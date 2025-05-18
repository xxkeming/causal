use std::{collections::HashMap, sync::Arc};

use async_openai::{
    Client,
    config::OpenAIConfig,
    types::{
        ChatCompletionMessageToolCall, ChatCompletionRequestAssistantMessageArgs,
        ChatCompletionRequestMessage, ChatCompletionRequestToolMessageArgs, ChatCompletionToolType,
        CompletionUsage, CreateChatCompletionRequest, FinishReason, FunctionCall,
    },
};

use futures::StreamExt;
use serde::Serialize;
use serde_json::Value;

use tokio::{
    sync::{Mutex, mpsc},
    task::JoinSet,
};

use crate::{error, openai::tool::ToolObject};

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum MessageEvent {
    UserMessage {
        message: store::ChatMessage,
    },
    AssistantMessage {
        message: store::ChatMessage,
    },
    RetryAssistantMessage {
        message: store::ChatMessage,
    },
    ReasoningContent {
        content: String,
    },
    Content {
        content: String,
    },
    Tool {
        id: String,
        name: String,
        arguments: String,
        result: String,
    },
    Finished {
        cost: i64,
        #[serde(rename = "promptTokens")]
        prompt_tokens: u32,
        #[serde(rename = "completionTokens")]
        completion_tokens: u32,
        #[serde(rename = "totalTokens")]
        total_tokens: u32,
    },
}

async fn call_tools(
    tools: Arc<Vec<Arc<Box<dyn ToolObject>>>>, name: String, args: String,
) -> Result<Value, error::Error> {
    tracing::info!("Calling function: {} with args: {}", name, args);

    let function_args: serde_json::Value = args.parse()?;

    for tool in tools.iter() {
        if tool.description().iter().any(|desc| desc.name == name) {
            return tool.call(&name, function_args).await;
        }
    }

    Ok(serde_json::json!({
        "error": "not func",
    }))
}

pub async fn chat_stream(
    client: &Client<OpenAIConfig>, tool_objects: Arc<Vec<Arc<Box<dyn ToolObject>>>>,
    request: CreateChatCompletionRequest, messages: &mut Vec<ChatCompletionRequestMessage>,
    on_event: &mpsc::Sender<MessageEvent>,
) -> Result<(bool, Option<CompletionUsage>), error::Error> {
    // let prompt = messages.pop().unwrap();

    // tracing::info!("messages: {:?}", messages);

    let mut stream = client.chat().create_stream(request).await?;

    let mut tool_call_states: HashMap<(u32, u32), ChatCompletionMessageToolCall> = HashMap::new();

    let tool_call_responses: Arc<Mutex<Vec<(ChatCompletionMessageToolCall, Value)>>> =
        Arc::new(Mutex::new(Vec::new()));

    while let Some(result) = stream.next().await {
        let response = result?;
        // tracing::info!("response: {:?}", response.usage);

        for chat_choice in response.choices {
            // tracing::info!("reasoning_content: {:?}", chat_choice.delta.reasoning_content);

            if let Some(tool_calls) = chat_choice.delta.tool_calls {
                for tool_call_chunk in tool_calls.into_iter() {
                    let key = (chat_choice.index, tool_call_chunk.index);

                    let state = tool_call_states.entry(key).or_insert_with(|| {
                        ChatCompletionMessageToolCall {
                            id: tool_call_chunk.id.clone().unwrap_or_default(),
                            r#type: ChatCompletionToolType::Function,
                            function: FunctionCall {
                                name: tool_call_chunk
                                    .function
                                    .as_ref()
                                    .and_then(|f| f.name.clone())
                                    .unwrap_or_default(),
                                arguments: String::with_capacity(128),
                            },
                        }
                    });

                    if let Some(arguments) =
                        tool_call_chunk.function.as_ref().and_then(|f| f.arguments.as_ref())
                    {
                        state.function.arguments.push_str(arguments);
                    }
                }
            }

            if let Some(finish_reason) = &chat_choice.finish_reason {
                // tracing::info!("finish_reason: {:?} {:?}", finish_reason, tool_call_states);

                if matches!(finish_reason, FinishReason::Stop) {
                    return Ok((false, response.usage));
                }

                // on_event.send(MessageEvent::Finished).map_err(|_| error::Error::Unknown)?;
                if matches!(finish_reason, FinishReason::ToolCalls) {
                    let mut sets = JoinSet::new();

                    for (_, tool_call) in tool_call_states.into_iter() {
                        let tool_objects = tool_objects.clone();
                        let tool_call_responses = tool_call_responses.clone();
                        sets.spawn(async move {
                            let response_content = call_tools(
                                tool_objects,
                                tool_call.function.name.clone(),
                                tool_call.function.arguments.clone(),
                            )
                            .await
                            .unwrap_or_default();
                            tool_call_responses.lock().await.push((tool_call, response_content));
                        });
                    }

                    sets.join_all().await;

                    let tool_call_responses = tool_call_responses.lock().await;
                    let tool_calls: Vec<ChatCompletionMessageToolCall> =
                        tool_call_responses.iter().map(|tc| tc.0.clone()).collect();

                    for tc in tool_call_responses.iter() {
                        on_event
                            .send(MessageEvent::Tool {
                                id: tc.0.id.clone(),
                                name: tc.0.function.name.clone(),
                                arguments: tc.0.function.arguments.clone(),
                                result: tc.1.to_string(),
                            })
                            .await
                            .map_err(|e| error::Error::InvalidData(e.to_string()))?;
                    }

                    let assistant_messages: ChatCompletionRequestMessage =
                        ChatCompletionRequestAssistantMessageArgs::default()
                            .tool_calls(tool_calls)
                            .build()?
                            .into();
                    messages.push(assistant_messages);

                    for tc in tool_call_responses.iter() {
                        let tool_message = ChatCompletionRequestToolMessageArgs::default()
                            .content(tc.1.to_string())
                            .tool_call_id(tc.0.id.clone())
                            .build()?
                            .into();
                        messages.push(tool_message);
                    }
                    return Ok((true, response.usage));
                }
            }

            if let Some(content) = chat_choice.delta.reasoning_content.filter(|v| !v.is_empty()) {
                on_event
                    .send(MessageEvent::ReasoningContent { content })
                    .await
                    .map_err(|e| error::Error::InvalidData(e.to_string()))?;
            }

            if let Some(content) = chat_choice.delta.content.filter(|v| !v.is_empty()) {
                on_event
                    .send(MessageEvent::Content { content })
                    .await
                    .map_err(|e| error::Error::InvalidData(e.to_string()))?;
            }
        }
    }

    Ok((false, None))
}

pub async fn chat(
    client: &Client<OpenAIConfig>, tool_objects: Arc<Vec<Arc<Box<dyn ToolObject>>>>,
    request: CreateChatCompletionRequest, messages: &mut Vec<ChatCompletionRequestMessage>,
    on_event: &mpsc::Sender<MessageEvent>,
) -> Result<(bool, Option<CompletionUsage>), error::Error> {
    // tracing::info!("messages: {:?}", messages);

    let response = client.chat().create(request).await?;

    // tracing::info!("response: {:?}", response.usage);

    for choice in response.choices {
        if let Some(tool_calls) = choice.message.tool_calls {
            let mut sets = JoinSet::new();

            for tool_call in tool_calls {
                let name = tool_call.function.name.clone();
                let args = tool_call.function.arguments.clone();

                let tool_objects = tool_objects.clone();
                let tool_call_clone = tool_call.clone();

                sets.spawn(async move {
                    (
                        tool_call_clone,
                        call_tools(tool_objects, name, args).await.unwrap_or_default(),
                    )
                });
            }

            let tool_responses = sets.join_all().await;

            let tool_calls: Vec<ChatCompletionMessageToolCall> = tool_responses
                .iter()
                .map(|(tool_call, _response_content)| tool_call.clone())
                .collect();

            let assistant_messages: ChatCompletionRequestMessage =
                ChatCompletionRequestAssistantMessageArgs::default()
                    .tool_calls(tool_calls)
                    .build()?
                    .into();
            messages.push(assistant_messages);

            for (tool_call, response_content) in tool_responses {
                let tool_message = ChatCompletionRequestToolMessageArgs::default()
                    .content(response_content.to_string())
                    .tool_call_id(tool_call.id.clone())
                    .build()?
                    .into();
                messages.push(tool_message);
            }
            return Ok((true, response.usage));
        }

        if let Some(content) = choice.message.reasoning_content {
            on_event
                .send(MessageEvent::ReasoningContent { content })
                .await
                .map_err(|e| error::Error::InvalidData(e.to_string()))?;
        }

        if let Some(content) = choice.message.content {
            on_event
                .send(MessageEvent::Content { content })
                .await
                .map_err(|e| error::Error::InvalidData(e.to_string()))?;
        }
    }

    Ok((false, response.usage))
}
