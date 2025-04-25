use mcp_core::client::Client;
use mcp_core::types::{ClientCapabilities, Implementation};
use serde::{Deserialize, Serialize};

use mcp_core::transport::{ClientSseTransport, ClientSseTransportBuilder};
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

pub trait ToolObject {
    fn description(&self) -> Vec<ToolDescription>;
    fn call<'a>(
        &'a self, name: &'a str, param: serde_json::Value,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<serde_json::Value, error::Error>> + Send + 'a>,
    >
    where
        Self: Send + Sync + 'static;
}

pub struct SearchTaivlyTool {
    taivly: tavily::Tavily,
    description: Vec<ToolDescription>,
}

impl SearchTaivlyTool {
    pub fn try_new(api_key: String) -> Result<Self, error::Error> {
        let taivly = tavily::Tavily::builder(api_key)
            .timeout(std::time::Duration::from_secs(60))
            .max_retries(5)
            .build()?;

        let description = ToolDescription {
            name: "tavily_web_search".to_string(),
            description: "Useful for when you need to answer questions by searching the web."
                .to_string(),
            schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string"
                    }
                }
            }),
        };

        Ok(Self { taivly, description: vec![description] })
    }
}

impl ToolObject for SearchTaivlyTool {
    fn description(&self) -> Vec<ToolDescription> {
        self.description.clone()
    }
    fn call<'a>(
        &'a self, _name: &'a str, param: serde_json::Value,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<serde_json::Value, error::Error>> + Send + 'a>,
    > {
        Box::pin(async move {
            let query = param.get("query").and_then(|value| value.as_str()).unwrap_or_default();

            let results = self.taivly.search(query).await?;

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
            Ok(serde_json::to_value(results)?)
        })
    }
}

pub struct McpSseTool {
    client: Client<ClientSseTransport>,
    description: Vec<ToolDescription>,
}

impl McpSseTool {
    pub async fn try_new(url: String) -> Result<Self, error::Error> {
        let client = ClientBuilder::new(ClientSseTransportBuilder::new(url).build()).build();
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

        Ok(Self { client, description: tools })
    }
}

impl ToolObject for McpSseTool {
    fn description(&self) -> Vec<ToolDescription> {
        self.description.clone()
    }
    fn call<'a>(
        &'a self, name: &'a str, param: serde_json::Value,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<serde_json::Value, error::Error>> + Send + 'a>,
    > {
        Box::pin(async move {
            let response = self
                .client
                .call_tool(name, Some(param))
                .await
                .map_err(|e| error::Error::McpError(format!("{e}")))?;

            Ok(serde_json::to_value(response)
                .map_err(|e| error::Error::McpError(format!("{e}")))?)
        })
    }
}

#[cfg(test)]
mod test {
    use mcp_core::types::{ClientCapabilities, Implementation};
    use std::time::Duration;

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
        let test: Box<dyn ToolObject> = Box::new(
            McpSseTool::try_new(
                "https://mcp.amap.com/sse?key=0d36bfeaa2b89fab9d25172a404163f8".to_string(),
            )
            .await
            .unwrap(),
        );

        for tool in &test.description() {
            println!("Tool: {} {}", tool.name, tool.description);
        }

        // println!("{:?}", transp)
        // https://mcp.amap.com/sse?key=0d36bfeaa2b89fab9d25172a404163f8
    }
}
