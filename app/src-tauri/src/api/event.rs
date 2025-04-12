use crate::error;
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequest, CreateChatCompletionRequestArgs,
    },
    Client,
};
use futures::StreamExt;
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum MessageEvent {
    Started,
    Progress { content: String },
    Finished,
}

async fn event_chat_stream(
    client: Client<OpenAIConfig>, request: CreateChatCompletionRequest,
    on_event: tauri::ipc::Channel<MessageEvent>,
) -> Result<serde_json::Value, error::Error> {
    let mut stream = client.chat().create_stream(request).await?;

    on_event.send(MessageEvent::Started).map_err(|_| error::Error::Unknown)?;

    while let Some(result) = stream.next().await {
        let response = result?;
        for chat_choice in response.choices {
            if let Some(content) = chat_choice.delta.content {
                // write!(lock, "{}", content).unwrap();
                on_event
                    .send(MessageEvent::Progress { content })
                    .map_err(|_| error::Error::Unknown)?;
            }
        }
    }

    on_event.send(MessageEvent::Finished).map_err(|_| error::Error::Unknown)?;

    Ok(serde_json::json!({
        "status": "success"
    }))
}

async fn event_chat(
    client: Client<OpenAIConfig>, request: CreateChatCompletionRequest,
    on_event: tauri::ipc::Channel<MessageEvent>,
) -> Result<serde_json::Value, error::Error> {
    let response = client.chat().create(request).await?;

    on_event.send(MessageEvent::Started).map_err(|_| error::Error::Unknown)?;
    for chat_choice in response.choices {
        if let Some(content) = chat_choice.message.content {
            on_event.send(MessageEvent::Progress { content }).map_err(|_| error::Error::Unknown)?;
        }
    }
    on_event.send(MessageEvent::Finished).map_err(|_| error::Error::Unknown)?;

    Ok(serde_json::json!({
        "status": "success"
    }))
}

pub async fn event(
    store: tauri::State<'_, store::Store>, agent_id: u64, _session_id: u64, message_id: u64,
    on_event: tauri::ipc::Channel<MessageEvent>,
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

    let content = match message.attachments {
        Some(attachments) => {
            let mut content = message.content;
            for attachment in attachments {
                content.push_str(&format!(
                    "\n\n<attachment><name>{}</name><data>{}</data></attachment>",
                    attachment.name, attachment.data
                ));
            }
            content
        }
        None => message.content,
    };

    // println!("Message: {:?}", content);

    let config = async_openai::config::OpenAIConfig::new()
        .with_api_base(provider.url.clone())
        .with_api_key(provider.api_key.map(|key| key.to_string()).unwrap_or_default());

    let client = Client::with_config(config);

    let request = CreateChatCompletionRequestArgs::default()
        .model(model.name.clone())
        // .max_tokens(agent.max_tokens)
        .temperature(agent.temperature as f32)
        .top_p(agent.top_p as f32)
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(agent.prompt.clone())
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default().content(content).build()?.into(),
        ])
        .build()?;

    if provider.stream {
        event_chat_stream(client, request, on_event).await
    } else {
        event_chat(client, request, on_event).await
    }
}
