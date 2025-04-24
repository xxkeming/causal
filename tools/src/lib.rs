use mcp_core::types::{ClientCapabilities, Implementation};
use serde::{Deserialize, Serialize};

use mcp_core::transport::ClientSseTransportBuilder;
use mcp_core::{client::ClientBuilder, protocol::RequestOptions};

mod error;
pub use error::Error;

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub content: String,
    pub raw_content: Option<String>,
    pub score: f32,
}

pub async fn tavily_search(
    key: &str, result_count: u32, query: &str,
) -> Result<Vec<SearchResult>, error::Error> {
    let tavily = tavily::Tavily::builder(key)
        .timeout(std::time::Duration::from_secs(60))
        .max_retries(result_count)
        .build()?;

    let results = tavily.search(query).await?;

    let results = results
        .results
        .into_iter()
        .map(|result| SearchResult {
            title: result.title,
            url: result.url,
            content: result.content,
            raw_content: result.raw_content,
            score: result.score,
        })
        .collect::<Vec<_>>();
    Ok(results)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolDescription {
    pub name: String,
    pub description: String,

    #[serde(rename = "inputSchema")]
    pub schema: serde_json::Value,
}

pub struct McpSseTool(String);

impl McpSseTool {
    pub fn new(name: String) -> Self {
        Self(name)
    }

    pub async fn description(&self) -> Result<Vec<ToolDescription>, error::Error> {
        let client =
            ClientBuilder::new(ClientSseTransportBuilder::new(self.0.clone()).build()).build();
        client.open().await.map_err(|e| error::Error::McpError(format!("{e}")))?;

        client
            .initialize(
                Implementation { name: "causal studio".to_string(), version: "1.0".to_string() },
                ClientCapabilities::default(),
            )
            .await
            .map_err(|e| error::Error::McpError(format!("{e}")))?;

        let mut response = client
            .request("tools/list", None, RequestOptions::default())
            .await
            .map_err(|e| error::Error::McpError(format!("{e}")))?;

        let tools = response.get_mut("tools").unwrap().take();
        let tools = serde_json::from_value::<Vec<ToolDescription>>(tools)
            .map_err(|e| error::Error::McpError(format!("{e}")))?;

        Ok(tools)
    }

    pub async fn call(
        &self, name: &str, param: serde_json::Value,
    ) -> Result<serde_json::Value, error::Error> {
        let client =
            ClientBuilder::new(ClientSseTransportBuilder::new(self.0.clone()).build()).build();
        client.open().await.map_err(|e| error::Error::McpError(format!("{e}")))?;

        client
            .initialize(
                Implementation { name: "causal studio".to_string(), version: "1.0".to_string() },
                ClientCapabilities::default(),
            )
            .await
            .map_err(|e| error::Error::McpError(format!("{e}")))?;

        let response = client
            .call_tool(name, Some(param))
            .await
            .map_err(|e| error::Error::McpError(format!("{e}")))?;

        Ok(serde_json::to_value(response).map_err(|e| error::Error::McpError(format!("{e}")))?)
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use mcp_core::types::{ClientCapabilities, Implementation};

    use super::*;

    // #[tokio::test]
    // async fn test() {
    //     js_run().await.unwrap();
    // }

    #[tokio::test]
    async fn test_tavily() {
        let results = tavily_search("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", 5, "rust是什么")
            .await
            .unwrap();
        println!("result: {:?}", results);
    }

    #[tokio::test]
    async fn mcp_test() {
        use mcp_core::transport::ClientSseTransportBuilder;
        use mcp_core::{client::ClientBuilder, protocol::RequestOptions};

        let client = ClientBuilder::new(
            ClientSseTransportBuilder::new(
                "https://mcp.amap.com/sse?key=0d36bfeaa2b89fab9d25172a404163f8".to_string(),
            )
            .build(),
        )
        .build();
        client.open().await.unwrap();

        client
            .initialize(
                Implementation { name: "causal studio".to_string(), version: "1.0".to_string() },
                ClientCapabilities::default(),
            )
            .await
            .unwrap();

        let mut response = client
            .request("tools/list", None, RequestOptions::default().timeout(Duration::from_secs(5)))
            .await
            .unwrap();

        let tools = response.get_mut("tools").unwrap().take();
        let tools = serde_json::from_value::<Vec<ToolDescription>>(tools).unwrap();

        for tool in &tools {
            println!("Tool: {} {}", tool.name, tool.description);
        }

        // println!("{:?}", transp)
        // https://mcp.amap.com/sse?key=0d36bfeaa2b89fab9d25172a404163f8
    }
}
