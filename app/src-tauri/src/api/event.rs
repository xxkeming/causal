use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionMessageToolCall, ChatCompletionRequestAssistantMessageArgs,
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestToolMessageArgs, ChatCompletionRequestUserMessageArgs,
        ChatCompletionToolArgs, ChatCompletionToolType, CompletionUsage,
        CreateChatCompletionRequest, CreateChatCompletionRequestArgs, FinishReason, FunctionCall,
        FunctionObjectArgs,
    },
    Client,
};

use futures::StreamExt;
use serde::Serialize;
use serde_json::{json, Value};
use tokio::{sync::Mutex, task::JoinSet};

use crate::error;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum MessageEvent {
    Started,
    Chat {
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

async fn call_tools(name: &str, args: &str) -> Result<Value, Box<dyn std::error::Error>> {
    println!("Calling function: {} with args: {}", name, args);

    let function_args: serde_json::Value = args.parse().unwrap();

    // let x = function_args["x"].as_number().unwrap().as_i64().unwrap() as i32;
    // let y = function_args["y"].as_number().unwrap().as_i64().unwrap() as i32;
    // let sum = x + y;

    // Ok(serde_json::json!({
    //     "result": sum,
    // }))

    let query = function_args["query"].as_str().unwrap_or_default();
    let search = tools::tavily_search(query).await?;
    Ok(serde_json::to_value(search)?)
}

async fn event_chat_stream(
    client: &Client<OpenAIConfig>, request: CreateChatCompletionRequest,
    messages: &mut Vec<ChatCompletionRequestMessage>, on_event: &tauri::ipc::Channel<MessageEvent>,
) -> Result<(bool, Option<CompletionUsage>), error::Error> {
    // let prompt = messages.pop().unwrap();

    // println!("messages: {:?}", messages);

    let mut stream = client.chat().create_stream(request).await?;

    let mut tool_call_states: HashMap<(u32, u32), ChatCompletionMessageToolCall> = HashMap::new();

    let tool_call_responses: Arc<Mutex<Vec<(ChatCompletionMessageToolCall, Value)>>> =
        Arc::new(Mutex::new(Vec::new()));

    while let Some(result) = stream.next().await {
        let response = result?;
        // println!("response: {:?}", response.usage);

        for chat_choice in response.choices {
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
                // println!("finish_reason: {:?} {:?}", finish_reason, tool_call_states);

                if matches!(finish_reason, FinishReason::Stop) {
                    return Ok((false, response.usage));
                }

                // on_event.send(MessageEvent::Finished).map_err(|_| error::Error::Unknown)?;
                if matches!(finish_reason, FinishReason::ToolCalls) {
                    let mut sets = JoinSet::new();

                    for (_, tool_call) in tool_call_states.into_iter() {
                        let tool_call_responses = tool_call_responses.clone();
                        sets.spawn(async move {
                            let response_content =
                                call_tools(&tool_call.function.name, &tool_call.function.arguments)
                                    .await
                                    .unwrap();
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
                            .map_err(|_| error::Error::Unknown)?;
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

            // println!("index: {:?}", chat_choice.index);

            if let Some(content) = chat_choice.delta.content {
                on_event.send(MessageEvent::Chat { content }).map_err(|_| error::Error::Unknown)?;
            }
        }
    }

    Ok((false, None))
}

async fn event_chat(
    client: &Client<OpenAIConfig>, request: CreateChatCompletionRequest,
    messages: &mut Vec<ChatCompletionRequestMessage>, on_event: &tauri::ipc::Channel<MessageEvent>,
) -> Result<(bool, Option<CompletionUsage>), error::Error> {
    // println!("messages: {:?}", messages);

    let response = client.chat().create(request).await?;

    // println!("response: {:?}", response.usage);

    for choice in response.choices {
        if let Some(tool_calls) = choice.message.tool_calls {
            let mut sets = JoinSet::new();

            for tool_call in tool_calls {
                let name = tool_call.function.name.clone();
                let args = tool_call.function.arguments.clone();
                let tool_call_clone = tool_call.clone();

                sets.spawn(async move {
                    (tool_call_clone, call_tools(&name, &args).await.unwrap_or_default())
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

        if let Some(content) = choice.message.content {
            on_event.send(MessageEvent::Chat { content }).map_err(|_| error::Error::Unknown)?;
        }
    }

    Ok((false, response.usage))
}

struct ChatMessage(store::ChatMessage);

impl Deref for ChatMessage {
    type Target = store::ChatMessage;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ChatMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ChatMessage {
    pub fn new_user(message: store::ChatMessage) -> Result<Self, error::Error> {
        if message.role != store::Role::User {
            return Err(error::Error::InvalidData(format!(
                "Message with id {} is not a user message",
                message.id
            )));
        }
        Ok(Self::new(message, true))
    }

    pub fn new(mut message: store::ChatMessage, context_extend: bool) -> Self {
        if context_extend {
            if let Some(attachments) = message.attachments.as_ref() {
                let attachments = attachments
                    .into_iter()
                    .map(|attachment| {
                        format!(
                            "<attachment><name>{}</name><data>{}</data></attachment>",
                            attachment.name, attachment.data
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                message.content = format!("{}\n\n{}", attachments, message.content);
            }
        }

        Self(message)
    }
}

impl TryFrom<ChatMessage> for ChatCompletionRequestMessage {
    type Error = error::Error;

    fn try_from(message: ChatMessage) -> Result<Self, Self::Error> {
        if message.0.role == store::Role::User {
            Ok(ChatCompletionRequestUserMessageArgs::default()
                .content(message.0.content)
                .build()?
                .into())
        } else {
            Ok(ChatCompletionRequestAssistantMessageArgs::default()
                .content(message.0.content)
                .build()?
                .into())
        }
    }
}

pub async fn event(
    store: tauri::State<'_, store::Store>, agent_id: u64, session_id: u64, message_id: u64,
    search: bool, stream: bool, on_event: tauri::ipc::Channel<MessageEvent>,
) -> Result<serde_json::Value, error::Error> {
    let Some(agent) = store.get_agent(agent_id)? else {
        return Err(error::Error::InvalidData(format!("Agent with id {agent_id} not found")));
    };

    let Some(model) = agent.model.as_ref() else {
        return Err(error::Error::InvalidData(format!("Model with {:?} not found", agent.model)));
    };

    let provider = store.get_provider(model.id)?;
    let Some(provider) = provider else {
        return Err(error::Error::InvalidData(format!("Provider with {:?} not found", model)));
    };

    // println!("Provider: {:?}", provider);

    let mut contexts = store.get_messages_by_session_and_message(
        session_id,
        message_id - 1,
        agent.context_size as usize * 2 + 1,
    )?;
    for context in contexts.iter() {
        println!("Message: {:?}", context.content);
    }

    let Some(context) = contexts.pop() else {
        return Err(error::Error::InvalidData(format!(
            "Message with id {} not found",
            message_id - 1
        )));
    };
    let mut context = ChatMessage::new_user(context)?;

    // 联网搜索结果
    if search {
        let search = tools::tavily_search(context.content.as_ref())
            .await
            .map_err(|e| error::Error::InvalidData(format!("{:?}", e)))?;
        let search = search
            .into_iter()
            .map(|item| {
                format!("<search><name>{}</name><data>{}</data></search>", item.title, item.content)
            })
            .collect::<Vec<_>>()
            .join("\n");
        println!("Search: {:?}", search);
        context.content = format!("{}\n\n{}", context.content, search);
    }

    let config = async_openai::config::OpenAIConfig::new()
        .with_api_base(provider.url.clone())
        .with_api_key(provider.api_key.map(|key| key.to_string()).unwrap_or_default());
    let client = Client::with_config(config);

    // let tools = vec![ChatCompletionToolArgs::default()
    //     .r#type(ChatCompletionToolType::Function)
    //     .function(
    //         FunctionObjectArgs::default()
    //             .name("fn_100000")
    //             .description("Add x and y together")
    //             .parameters(json!({
    //                 "type": "object",
    //                 "properties": {
    //                     "x": {
    //                         "type": "number",
    //                         "description": "The first number to add"
    //                     },
    //                     "y": {
    //                         "type": "number",
    //                         "description": "The second number to add"
    //                     }
    //                 }
    //             }))
    //             .build()?,
    //     )
    //     .build()?];

    let tools = vec![ChatCompletionToolArgs::default()
        .r#type(ChatCompletionToolType::Function)
        .function(
            FunctionObjectArgs::default()
                .name("fn_100000")
                .description("Useful for when you need to answer questions by searching the web.")
                .parameters(json!({
                    "type": "object",
                    "properties": {
                        "query": {
                            "type": "string"
                        }
                    }
                }))
                .build()?,
        )
        .build()?];

    let mut messages = vec![ChatCompletionRequestSystemMessageArgs::default()
        .content(agent.prompt.clone())
        .build()?
        .into()];

    for context in contexts.into_iter() {
        messages.push(ChatMessage::new(context, agent.context_extend).try_into()?);
    }
    messages.push(context.try_into()?);

    on_event.send(MessageEvent::Started).map_err(|_| error::Error::Unknown)?;

    let mut usages = Vec::new();
    let cost = std::time::Instant::now();
    loop {
        let mut request = CreateChatCompletionRequestArgs::default()
            .model(model.name.clone())
            .temperature(agent.temperature as f32)
            .messages(messages.clone())
            .tools(tools.clone())
            .build()?;

        if agent.max_tokens > 0 {
            request.max_completion_tokens = Some(agent.max_tokens);
        }

        // .top_p(agent.top_p as f32);

        let (is_continue, usage) = if stream {
            event_chat_stream(&client, request, &mut messages, &on_event).await?
        } else {
            event_chat(&client, request, &mut messages, &on_event).await?
        };

        usages.push(usage);
        if !is_continue {
            break;
        }
    }

    let (prompt_tokens, completion_tokens, total_tokens) = usages
        .into_iter()
        .filter_map(|usage| usage)
        .fold((0, 0, 0), |(prompt, completion, total), usage| {
            (
                prompt + usage.prompt_tokens,
                completion + usage.completion_tokens,
                total + usage.total_tokens,
            )
        });

    let finised = MessageEvent::Finished {
        cost: cost.elapsed().as_millis() as i64,
        prompt_tokens: prompt_tokens,
        completion_tokens: completion_tokens,
        total_tokens: total_tokens,
    };
    println!("Finished: {:?}", finised);

    on_event.send(finised).map_err(|_| error::Error::Unknown)?;

    Ok(serde_json::json!({
        "status": "success"
    }))
}
