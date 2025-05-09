use std::{collections::HashMap, sync::Arc};

use async_openai::{
    Client,
    config::OpenAIConfig,
    types::{
        ChatCompletionMessageToolCall, ChatCompletionRequestAssistantMessageArgs,
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestToolMessageArgs, ChatCompletionToolType, CompletionUsage,
        CreateChatCompletionRequest, CreateChatCompletionRequestArgs, FinishReason, FunctionCall,
    },
};

use futures::StreamExt;
use serde::Serialize;
use serde_json::{Value, json};
use store::ChatMessage;
use tokio::{
    sync::{Mutex, RwLock, mpsc, watch},
    task::JoinSet,
};

use crate::{
    error,
    openai::chat,
    openai::tool::{Search, Tool, ToolObject},
};

pub struct MessageTask {
    pub exit: tokio::sync::watch::Sender<()>,
}

pub type MessageTasks = Arc<RwLock<HashMap<u64, MessageTask>>>;

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
    tools: Arc<Vec<Box<dyn ToolObject>>>, name: String, args: String,
) -> Result<Value, error::Error> {
    tracing::info!("Calling function: {} with args: {}", name, args);

    let function_args: serde_json::Value = args.parse()?;

    for tool in tools.iter() {
        if tool.description().iter().any(|desc| desc.name == name) {
            return Ok(tool.call(&name, function_args).await?);
        }
    }

    Ok(serde_json::json!({
        "error": "not func",
    }))
}

async fn event_chat_stream(
    client: &Client<OpenAIConfig>, tool_objects: Arc<Vec<Box<dyn ToolObject>>>,
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

            if let Some(content) = chat_choice.delta.reasoning_content.filter(|v| v.len() > 0) {
                on_event
                    .send(MessageEvent::ReasoningContent { content })
                    .await
                    .map_err(|e| error::Error::InvalidData(e.to_string()))?;
            }

            if let Some(content) = chat_choice.delta.content.filter(|v| v.len() > 0) {
                on_event
                    .send(MessageEvent::Content { content })
                    .await
                    .map_err(|e| error::Error::InvalidData(e.to_string()))?;
            }
        }
    }

    Ok((false, None))
}

async fn event_chat(
    client: &Client<OpenAIConfig>, tool_objects: Arc<Vec<Box<dyn ToolObject>>>,
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

pub async fn event(
    tasks: tauri::State<'_, MessageTasks>, store: tauri::State<'_, store::Store>,
    message: ChatMessage, search: bool, time: bool, stream: bool,
    on_event: tauri::ipc::Channel<MessageEvent>,
) -> Result<serde_json::Value, error::Error> {
    let session = store.get_chat_session(message.session_id)?.ok_or_else(|| {
        error::Error::InvalidData(format!("Session with id {} not found", message.id))
    })?;

    let agent = store.get_agent(session.agent_id)?.ok_or_else(|| {
        error::Error::InvalidData(format!("Agent with id {} not found", session.agent_id))
    })?;

    let Some(model) = agent.model.as_ref() else {
        return Err(error::Error::InvalidData(format!("Model with {:?} not found", agent.model)));
    };

    let provider = store
        .get_provider(model.id)?
        .ok_or_else(|| error::Error::InvalidData(format!("Provider with {:?} not found", model)))?;

    let (sender_event, mut receiver_event) = mpsc::channel::<MessageEvent>(32);
    let task_store = (*store).clone();
    tokio::spawn(async move {
        let mut assistant: Option<ChatMessage> = None;
        while let Some(event) = receiver_event.recv().await {
            tracing::info!("Event: {:?}", event);

            match &event {
                MessageEvent::AssistantMessage { message } => {
                    assistant = task_store.add_chat_message(message.clone()).ok();
                }
                MessageEvent::RetryAssistantMessage { message } => {
                    assistant = Some(message.clone());
                }
                MessageEvent::ReasoningContent { content } => {
                    if let Some(assistant) = assistant.as_mut() {
                        match assistant.reasoning_content {
                            Some(ref mut reasoning_content) => {
                                reasoning_content.push_str(content.as_str());
                            }
                            None => {
                                assistant.reasoning_content = Some(content.clone());
                            }
                        }
                    }
                }
                MessageEvent::Content { content } => {
                    if let Some(assistant) = assistant.as_mut() {
                        assistant.content.push_str(content.as_str());
                    }
                }
                MessageEvent::Finished { cost, prompt_tokens, completion_tokens, total_tokens } => {
                    if let Some(assistant) = assistant.as_mut() {
                        assistant.cost = Some(*cost);
                        assistant.prompt_tokens = Some(*prompt_tokens);
                        assistant.completion_tokens = Some(*completion_tokens);
                        assistant.total_tokens = Some(*total_tokens);
                        assistant.status = store::MessageStatus::Success;
                        if let Err(e) = task_store.update_chat_message(assistant.clone()) {
                            tracing::error!("Error updating message: {:?}", e);
                        }
                    }
                }
                _ => {}
            }

            if let Err(e) = on_event.send(event) {
                tracing::error!("Error sending event: {:?}", e);
            }
        }
        // tracing::info!("Event task {} exit", message_id);
    });

    let (message, histroy) = if message.id == 0 {
        let histroy = store
            .get_latest_messages_by_session(message.session_id, agent.context_size as usize * 2)?;

        let message = store.add_chat_message(message)?;
        sender_event
            .send(MessageEvent::UserMessage { message: message.clone() })
            .await
            .map_err(|_| error::Error::Unknown)?;

        let assistant = ChatMessage::new_assistant(message.id + 1, message.session_id);
        sender_event
            .send(MessageEvent::AssistantMessage { message: assistant })
            .await
            .map_err(|_| error::Error::Unknown)?;

        (message, histroy)
    } else {
        let mut histroy = store.get_messages_by_session_and_message(
            message.session_id,
            message.id + 1,
            agent.context_size as usize * 2 + 2,
        )?;
        // tracing::info!("histroy: {:?}", histroy);

        let Some(assistant) = histroy.pop() else {
            return Err(error::Error::InvalidData(format!(
                "Message Assistant {} with id {} not found",
                histroy.len(),
                message.id - 1
            )));
        };
        let assistant = ChatMessage::new_assistant(assistant.id, assistant.session_id);
        sender_event
            .send(MessageEvent::RetryAssistantMessage { message: assistant })
            .await
            .map_err(|_| error::Error::Unknown)?;

        let Some(message) = histroy.pop() else {
            return Err(error::Error::InvalidData(format!(
                "Message User {} with id {} not found",
                histroy.len(),
                message.id - 1
            )));
        };
        (message, histroy)
    };

    // chat(tasks, store, agent, provider, message, histroy, search, time, stream, sender_event).await
    let message_id = message.id;
    let mut message = chat::Message::new_user(message)?;

    let mut search = if search { Some(store.get_search()?) } else { None };

    // 先联网搜索
    if let Some(search) = search.take_if(|v| v.mode == 1) {
        let search = Search::new(search).into_tool_object()?;

        let query = json!({"query": message.content.clone()});
        let search = search.call("search", query.clone()).await?;

        sender_event
            .send(MessageEvent::Tool {
                id: "0".to_string(),
                name: "web search".to_string(),
                arguments: query.to_string(),
                result: search.to_string(),
            })
            .await
            .map_err(|_| error::Error::Unknown)?;

        message.content =
            format!("{}\n\nweb search results\n{}\n\n", message.content, search.to_string());
    }

    // 上下文附加当前时间
    if time {
        message.content =
            format!("{}\n\ncurrent utc time:{}\n\n", message.content, time::UtcDateTime::now());
        // tracing::info!("Message: {:?}", message.content);
    }

    let config = async_openai::config::OpenAIConfig::new()
        .with_api_base(provider.url.clone())
        .with_api_key(provider.api_key.map(|key| key.to_string()).unwrap_or_default());
    let client = Client::with_config(config);

    let tools = agent
        .tools
        .map(|id| {
            id.iter().filter_map(|id| store.get_tool(*id).unwrap_or(None)).collect::<Vec<_>>()
        })
        .unwrap_or(Vec::new());

    let mut tool_objects = Vec::new();
    for tool in tools.into_iter() {
        tool_objects.push(Tool::new(tool).into_tool_object().await?);
    }

    let mut tools = Vec::new();

    // 先联网搜索工具方式
    if let Some(search) = search.take_if(|v| v.mode == 2) {
        tool_objects.push(Search::new(search).into_tool_object()?);
    }

    for tool in tool_objects.iter() {
        let tool = tool
            .description()
            .into_iter()
            .map(|tool| tool.try_into())
            .collect::<Result<Vec<_>, _>>()?;
        tools.extend(tool);
    }
    let tool_objects = Arc::new(tool_objects);

    let mut messages = vec![
        ChatCompletionRequestSystemMessageArgs::default()
            .content(agent.prompt.clone())
            .build()?
            .into(),
    ];

    for message in histroy.into_iter() {
        messages.push(chat::Message::new(message, agent.context_extend).try_into()?);
    }
    messages.push(message.try_into()?);

    // on_event.send(MessageEvent::Started).map_err(|_| error::Error::Unknown)?;

    let mut usages = Vec::new();
    let cost = std::time::Instant::now();

    let (sender_exit, mut receiver_exit) = watch::channel(());
    tasks.write().await.insert(message_id, MessageTask { exit: sender_exit });

    loop {
        let task = async {
            let mut request = CreateChatCompletionRequestArgs::default()
                .model(model.name.clone())
                .temperature(agent.temperature as f32)
                .top_p(agent.top_p as f32)
                .messages(messages.clone())
                .build()?;

            if tools.len() > 0 {
                request.tools = Some(tools.clone());
            }

            if agent.max_tokens > 0 {
                request.max_completion_tokens = Some(agent.max_tokens);
            }

            let (is_continue, usage) = if stream {
                event_chat_stream(
                    &client,
                    tool_objects.clone(),
                    request,
                    &mut messages,
                    &sender_event,
                )
                .await?
            } else {
                event_chat(&client, tool_objects.clone(), request, &mut messages, &sender_event)
                    .await?
            };
            Ok::<_, error::Error>((is_continue, usage))
        };

        let (is_continue, usage) = tokio::select! {
            _ = receiver_exit.changed() => {
                tracing::info!("Message task {} exit", message_id);
                (false, None)
            }
            result = task => {
                tracing::info!("Message task {} finished {result:?}", message_id);
                if result.as_ref().map(|r|!r.0).unwrap_or(true) {
                    tasks.write().await.remove(&message_id);
                }
                match result {
                    Ok((is_continue, usage)) => (is_continue, usage),
                    Err(e) => {
                        let (prompt_tokens, completion_tokens, total_tokens) =
                            usages.into_iter().fold((0, 0, 0), |(prompt, completion, total), usage: CompletionUsage| {
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

                        let _ = sender_event.send(finised).await;
                        return Err(e);
                    }
                }
            }
        };

        if let Some(usage) = usage {
            usages.push(usage);
        }

        if !is_continue {
            break;
        }
    }

    let (prompt_tokens, completion_tokens, total_tokens) =
        usages.into_iter().fold((0, 0, 0), |(prompt, completion, total), usage| {
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

    sender_event.send(finised).await.map_err(|_| error::Error::Unknown)?;

    Ok(serde_json::json!({
        "status": "success"
    }))
}
