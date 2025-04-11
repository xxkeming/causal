use crate::error;
use async_openai::{
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
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

pub async fn event(
    store: tauri::State<'_, store::Store>, agent_id: u64, _session_id: u64, _message_id: u64,
    content: String, on_event: tauri::ipc::Channel<MessageEvent>,
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

    let config = async_openai::config::OpenAIConfig::new()
        .with_api_base(provider.url.clone())
        .with_api_key(provider.api_key.map(|key| key.to_string()).unwrap_or_default());

    let client = Client::with_config(config);

    let request = CreateChatCompletionRequestArgs::default()
        .model(model.name.clone())
        .max_tokens(agent.max_tokens)
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

    let mut stream = client.chat().create_stream(request).await?;

    println!("message Agent: {:?}", agent);

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

    println!("message Finished");

    on_event.send(MessageEvent::Finished).map_err(|_| error::Error::Unknown)?;

    Ok(serde_json::json!({
        "status": "success"
    }))
}
