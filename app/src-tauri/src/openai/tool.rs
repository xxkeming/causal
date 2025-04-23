use crate::error;

pub struct Tool {
    pub description: Option<Vec<tools::ToolDescription>>,
    tool: store::Tool,
}

impl Tool {
    pub fn new(tool: store::Tool) -> Self {
        Self { tool, description: None }
    }

    pub async fn description(&mut self) -> Result<Vec<tools::ToolDescription>, error::Error> {
        if let Some(ref description) = self.description {
            return Ok(description.clone());
        }

        if let store::ToolData::McpSse(ref sse) = self.tool.data {
            // MCP-SSE工具
            return match tools::McpSseTool::new(sse.url.clone()).description().await {
                Ok(description) => {
                    self.description = Some(description.clone());
                    Ok(description)
                }
                Err(e) => Err(error::Error::from(e)),
            };
        }

        Err(error::Error::InvalidData("Unsupported tool type".to_string()))
    }

    pub async fn call(
        &self, name: &str, param: serde_json::Value,
    ) -> Result<serde_json::Value, error::Error> {
        if let store::ToolData::McpSse(ref sse) = self.tool.data {
            // MCP-SSE工具
            return tools::McpSseTool::new(sse.url.clone())
                .call(name, param)
                .await
                .map_err(error::Error::from);
        }

        Err(error::Error::InvalidData("Unsupported tool type".to_string()))
    }
}
