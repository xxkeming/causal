use crate::error;

use futures::StreamExt;
use rig::{
    completion::Completion,
    message::{AssistantContent, Message, ToolCall, ToolResult, ToolResultContent, UserContent},
    streaming::{StreamingChoice, StreamingCompletion, StreamingCompletionModel},
    OneOrMany,
};
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum MessageEvent {
    Started,
    Progress { content: String },
    Finished,
}

async fn event_chat_stream(
    agent: &rig::agent::Agent<rig::providers::openai::CompletionModel>, prompt: &str,
    messages: &mut Vec<Message>, on_event: &tauri::ipc::Channel<MessageEvent>,
) -> Result<bool, error::Error> {
    // let prompt = messages.pop().unwrap();

    println!("messages: {:?}", messages);

    let mut stream = agent.stream_completion(prompt, messages.clone()).await?.stream().await?;

    let mut tool_messages = vec![];
    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(StreamingChoice::Message(content)) => {
                print!("content: {}", content);
                on_event
                    .send(MessageEvent::Progress { content })
                    .map_err(|_| error::Error::Unknown)?;
            }
            Ok(StreamingChoice::ToolCall(name, id, params)) => {
                let res = agent
                    .tools
                    .call(&name, params.to_string())
                    .await
                    .map_err(|e| std::io::Error::other(e.to_string()))?;

                println!("tools call: {} {} {}", name, id, params.to_string());

                let result =
                    ToolResult { id, content: OneOrMany::one(ToolResultContent::text(res)) };
                tool_messages.push(Message::User {
                    content: OneOrMany::one(UserContent::ToolResult(result)),
                });

                // println!("tools: \nResult: {}", res);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }

    if tool_messages.len() > 0 {
        messages.extend(tool_messages);
        Ok(true)
    } else {
        Ok(false)
    }
}

async fn event_chat(
    agent: &rig::agent::Agent<rig::providers::openai::CompletionModel>, prompt: &str,
    messages: &mut Vec<Message>, on_event: &tauri::ipc::Channel<MessageEvent>,
) -> Result<bool, error::Error> {
    println!("messages: {:?}", messages);

    let prompt = messages.pop().unwrap();
    let resp = agent.completion(prompt.clone(), messages.clone()).await?.send().await?;
    messages.push(prompt);

    let mut tool_messages = vec![];
    for content in resp.choice.into_iter() {
        match content {
            AssistantContent::Text(content) => {
                print!("{}", content.text);
                on_event
                    .send(MessageEvent::Progress { content: content.text })
                    .map_err(|_| error::Error::Unknown)?;
            }
            AssistantContent::ToolCall(tool) => {
                let res = agent
                    .tools
                    .call(&tool.function.name, tool.function.arguments.to_string())
                    .await
                    .map_err(|e| std::io::Error::other(e.to_string()))?;

                println!(
                    "tools call: {} {} {}",
                    tool.function.name,
                    tool.id,
                    tool.function.arguments.to_string()
                );

                let call = ToolCall { id: tool.id.clone(), function: tool.function };
                tool_messages.push(Message::Assistant {
                    content: OneOrMany::one(AssistantContent::ToolCall(call)),
                });

                let result = ToolResult {
                    id: tool.id,
                    content: OneOrMany::one(ToolResultContent::text(format!("Result: {}", res))),
                };
                tool_messages.push(Message::User {
                    content: OneOrMany::one(UserContent::ToolResult(result)),
                });
            }
        }
    }

    // on_event.send(MessageEvent::Progress { content }).map_err(|_| error::Error::Unknown)?;
    if tool_messages.len() > 0 {
        messages.extend(tool_messages);
        Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn event(
    store: tauri::State<'_, store::Store>, agent_id: u64, _session_id: u64, message_id: u64,
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

    println!("Provider: {:?}", provider);

    let message = store.get_chat_message(message_id - 1)?;
    let Some(message) = message else {
        return Err(error::Error::InvalidData(format!(
            "Message with id {} not found",
            message_id - 1
        )));
    };

    let client = rig::providers::openai::Client::from_url(
        provider.api_key.as_ref().unwrap_or(&"".to_string()),
        &provider.url,
    );

    let client = client.agent(&model.name).temperature(agent.temperature);

    let client =
        if agent.max_tokens > 0 { client.max_tokens(agent.max_tokens as u64) } else { client };

    let client = if agent.prompt.len() > 0 { client.preamble(&agent.prompt) } else { client };

    // 添加附件 client.context(doc)
    let client = if let Some(attachments) = message.attachments {
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
        client.context(&attachments)
    } else {
        client
    };

    // 联网搜索结果
    let client = if search {
        let search = tools::tavily_search(message.content.as_ref()).await.unwrap();
        let search = search
            .into_iter()
            .map(|item| {
                format!("<search><name>{}</name><data>{}</data></search>", item.title, item.content)
            })
            .collect::<Vec<_>>()
            .join("\n");
        println!("Search: {:?}", search);
        client.context(&search)
    } else {
        client
    };

    let client = client.tool(tools::Adder);
    let client = client.build();

    // 添加工具
    // client.tools.add_tool(tools::Adder);

    on_event.send(MessageEvent::Started).map_err(|_| error::Error::Unknown)?;

    let mut messages: Vec<Message> = Vec::new();
    messages.push(Message::User {
        content: OneOrMany::one(UserContent::text(message.content.clone())),
    });

    if stream {
        while event_chat_stream(&client, &message.content, &mut messages, &on_event).await? {
            continue;
        }
    } else {
        while event_chat(&client, &message.content, &mut messages, &on_event).await? {
            continue;
        }
    };

    on_event.send(MessageEvent::Finished).map_err(|_| error::Error::Unknown)?;

    Ok(serde_json::json!({
        "status": "success"
    }))
}
