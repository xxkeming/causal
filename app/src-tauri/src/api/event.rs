use crate::error;

use futures::StreamExt;
use rig::{
    completion::Prompt,
    streaming::{StreamingChoice, StreamingPrompt},
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
    agent: rig::agent::Agent<rig::providers::openai::CompletionModel>, prompt: String,
    on_event: tauri::ipc::Channel<MessageEvent>,
) -> Result<serde_json::Value, error::Error> {
    let mut stream = agent.stream_prompt(&prompt).await?;

    on_event.send(MessageEvent::Started).map_err(|_| error::Error::Unknown)?;

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(StreamingChoice::Message(content)) => {
                print!("{}", content);
                on_event
                    .send(MessageEvent::Progress { content })
                    .map_err(|_| error::Error::Unknown)?;
            }
            Ok(StreamingChoice::ToolCall(name, _, params)) => {
                let res = agent
                    .tools
                    .call(&name, params.to_string())
                    .await
                    .map_err(|e| std::io::Error::other(e.to_string()))?;
                println!("\nResult: {}", res);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }

    on_event.send(MessageEvent::Finished).map_err(|_| error::Error::Unknown)?;

    Ok(serde_json::json!({
        "status": "success"
    }))
}

async fn event_chat(
    agent: rig::agent::Agent<rig::providers::openai::CompletionModel>, prompt: String,
    on_event: tauri::ipc::Channel<MessageEvent>,
) -> Result<serde_json::Value, error::Error> {
    on_event.send(MessageEvent::Started).map_err(|_| error::Error::Unknown)?;

    let content = agent.prompt(prompt).await?;

    on_event.send(MessageEvent::Progress { content }).map_err(|_| error::Error::Unknown)?;

    on_event.send(MessageEvent::Finished).map_err(|_| error::Error::Unknown)?;

    Ok(serde_json::json!({
        "status": "success"
    }))
}

pub async fn event(
    store: tauri::State<'_, store::Store>, agent_id: u64, _session_id: u64, message_id: u64,
    stream: bool, on_event: tauri::ipc::Channel<MessageEvent>,
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
        let mut contexts = String::new();
        for attachment in attachments {
            contexts.push_str(&format!(
                "\n\n<attachment><name>{}</name><data>{}</data></attachment>",
                attachment.name, attachment.data
            ));
        }
        client.context(&contexts)
    } else {
        client
    };

    if stream {
        event_chat_stream(client.build(), message.content, on_event).await
    } else {
        event_chat(client.build(), message.content, on_event).await
    }
}
