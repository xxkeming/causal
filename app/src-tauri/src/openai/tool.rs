use async_openai::types::{
    ChatCompletionTool, ChatCompletionToolArgs, ChatCompletionToolType, FunctionObjectArgs,
};
use serde::{Deserialize, Serialize};

mod search;
use search::SearchTaivlyTool;

mod mcp;
pub use mcp::McpSseTool;

use crate::error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolDescription {
    pub name: String,
    pub description: String,

    #[serde(rename = "inputSchema")]
    pub schema: serde_json::Value,
}

impl TryFrom<ToolDescription> for ChatCompletionTool {
    type Error = async_openai::error::OpenAIError;
    fn try_from(description: ToolDescription) -> Result<Self, Self::Error> {
        ChatCompletionToolArgs::default()
            .r#type(ChatCompletionToolType::Function)
            .function(
                FunctionObjectArgs::default()
                    .name(description.name)
                    .description(description.description)
                    .parameters(description.schema)
                    .build()?,
            )
            .build()
    }
}

pub trait ToolObject: Send + Sync {
    fn description(&self) -> Vec<ToolDescription>;
    fn call<'a>(
        &'a self, name: &'a str, param: serde_json::Value,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<serde_json::Value, error::Error>> + Send + 'a>,
    >
    where
        Self: Send + Sync + 'static;
}

pub struct Tool(store::Tool);

impl Tool {
    pub fn new(tool: store::Tool) -> Self {
        Self(tool)
    }

    pub async fn into_tool_object(self) -> Result<Box<dyn ToolObject>, error::Error> {
        if let store::ToolData::McpSse(ref sse) = self.0.data {
            // MCP-SSE工具
            return Ok(Box::new(McpSseTool::try_new(sse.url.clone()).await?));
        }
        Err(error::Error::InvalidData("Unsupported tool type".to_string()))
    }
}

pub struct Search(store::Search);

impl Search {
    pub fn new(search: store::Search) -> Self {
        Self(search)
    }

    pub fn into_tool_object(self) -> Result<Box<dyn ToolObject>, error::Error> {
        match self.0.r#type {
            store::SearchType::Tavily { api_key } => {
                // Taivly搜索工具
                return Ok(Box::new(SearchTaivlyTool::try_new(api_key)?));
            }
        }
    }
}
