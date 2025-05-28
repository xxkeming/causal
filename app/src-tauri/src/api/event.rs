use std::{collections::HashMap, sync::Arc};

use async_openai::{
    Client,
    config::OpenAIConfig,
    types::{ChatCompletionRequestSystemMessageArgs, CreateChatCompletionRequestArgs},
};

use serde_json::json;
use tokio::sync::{RwLock, mpsc, watch};

use store::ChatMessage;

use crate::{
    AppState, error,
    openai::{
        self,
        chat::{self, MessageEvent},
        tool::{Search, Tool},
    },
};

pub struct MessageTask {
    pub exit: tokio::sync::watch::Sender<()>,
}

pub type MessageTasks = RwLock<HashMap<u64, MessageTask>>;

pub async fn event(
    app: tauri::State<'_, AppState>, message: ChatMessage, search: bool, time: bool, stream: bool,
    on_event: tauri::ipc::Channel<MessageEvent>,
) -> Result<serde_json::Value, error::Error> {
    let session = app.store.get_chat_session(message.session_id)?.ok_or_else(|| {
        error::Error::InvalidData(format!("Session with id {} not found", message.id))
    })?;

    let agent = app.get_agent(session.agent_id).await?;

    let Some(model) = agent.model.as_ref() else {
        return Err(error::Error::InvalidData(format!("Model with {:?} not found", agent.model)));
    };

    let provider = app.get_provider(model.id).await?;

    let (sender_event, mut receiver_event) = mpsc::channel::<MessageEvent>(32);
    let task_store = app.store.clone();
    tokio::spawn(async move {
        let mut assistant: Option<ChatMessage> = None;
        while let Some(event) = receiver_event.recv().await {
            // tracing::info!("Event: {:?}", event);

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
                MessageEvent::Tool { id, name, arguments, result } => {
                    if let Some(assistant) = assistant.as_mut() {
                        let tool = store::ToolResult {
                            id: id.clone(),
                            name: name.clone(),
                            arguments: arguments.clone(),
                            result: result.clone(),
                        };

                        match assistant.tools {
                            Some(ref mut tools) => {
                                tools.push(tool);
                            }
                            None => {
                                assistant.tools = Some(vec![tool]);
                            }
                        }
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
        let histroy = app
            .store
            .get_latest_messages_by_session(message.session_id, agent.context_size as usize * 2)?;

        let message = app.store.add_chat_message(message)?;
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
        let mut histroy = app.store.get_latest_messages_by_session_and_message(
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

    let message_id = message.id;
    let mut message = openai::Message::new_user(message)?;

    let mut search = if search { Some(app.store.get_settings()?.search) } else { None };

    // 先联网搜索
    if let Some(search) = search.take_if(|v| v.mode == 1) {
        let search = app.get_search_tool_object(search).await?;

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

        message.content = format!("{}\n\nweb search results\n{}\n\n", message.content, search);
    }

    // 上下文附加当前时间
    if time {
        message.content =
            format!("{}\n\ncurrent utc time:{}\n\n", message.content, time::UtcDateTime::now());
        // tracing::info!("Message: {:?}", message.content);
    }

    let config = OpenAIConfig::new()
        .with_api_base(provider.url.clone())
        .with_api_key(provider.api_key.as_ref().map(|key| key.to_string()).unwrap_or_default());
    let client = Client::with_config(config);

    let mut tool_objects = Vec::new();
    for id in agent.tools.clone().into_iter().map(|tool| tool.into_iter()).flatten() {
        tool_objects.push(app.get_tool_object(id).await?);
    }

    let mut tools = Vec::new();

    // 先联网搜索工具方式
    if let Some(search) = search.take_if(|v| v.mode == 2) {
        tool_objects.push(app.get_search_tool_object(search).await?);
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
        messages.push(openai::Message::new(message, agent.context_extend).try_into()?);
    }
    messages.push(message.try_into()?);

    // on_event.send(MessageEvent::Started).map_err(|_| error::Error::Unknown)?;

    let mut usages = Vec::new();
    let cost = std::time::Instant::now();

    let (sender_exit, mut receiver_exit) = watch::channel(());
    app.tasks.write().await.insert(message_id, MessageTask { exit: sender_exit });

    let mut event_result = Ok(serde_json::json!({
        "status": "success"
    }));

    loop {
        let task = async {
            let mut request = CreateChatCompletionRequestArgs::default()
                .model(model.name.clone())
                .temperature(agent.temperature as f32)
                .top_p(agent.top_p as f32)
                .messages(messages.clone())
                .build()?;

            if !tools.is_empty() {
                request.tools = Some(tools.clone());
            }

            if agent.max_tokens > 0 {
                request.max_completion_tokens = Some(agent.max_tokens);
            }

            let (is_continue, usage) = if stream {
                chat::chat_stream(
                    &client,
                    tool_objects.clone(),
                    request,
                    &mut messages,
                    &sender_event,
                )
                .await?
            } else {
                chat::chat(&client, tool_objects.clone(), request, &mut messages, &sender_event)
                    .await?
            };
            Ok::<_, error::Error>((is_continue, usage))
        };

        let is_continue = tokio::select! {
            _ = receiver_exit.changed() => {
                tracing::info!("Message task {} exit", message_id);
                false
            }
            result = task => {
                tracing::info!("Message task {} finished {result:?}", message_id);
                match result {
                    Ok((is_continue, usage)) => {
                        if !is_continue {
                             app.tasks.write().await.remove(&message_id);
                        }
                        if let Some(usage) = usage {
                            usages.push(usage);
                        }
                        is_continue
                    },
                    Err(e) => {
                         app.tasks.write().await.remove(&message_id);
                        event_result = Err(e);
                        false
                    }
                }
            }
        };

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
        prompt_tokens,
        completion_tokens,
        total_tokens,
    };

    sender_event.send(finised).await.map_err(|_| error::Error::Unknown)?;

    event_result
}
