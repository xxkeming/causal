use std::ops::{Deref, DerefMut};

use async_openai::types::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
    ChatCompletionRequestUserMessageArgs,
};

use crate::error;

pub mod chat;
pub mod tool;

pub struct Message(store::ChatMessage);

impl Deref for Message {
    type Target = store::ChatMessage;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Message {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Message {
    pub fn new_user(message: store::ChatMessage) -> Result<Self, error::Error> {
        if message.role != store::Role::User {
            return Err(error::Error::InvalidData(format!(
                "Message with id {} is not a user message",
                message.id
            )));
        }
        Ok(Self::new(message, true))
    }

    pub fn new(mut message: store::ChatMessage, extend_context: bool) -> Self {
        if extend_context {
            if let Some(attachments) = message.attachments.as_ref() {
                let attachments = attachments
                    .iter()
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

impl TryFrom<Message> for ChatCompletionRequestMessage {
    type Error = error::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
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
