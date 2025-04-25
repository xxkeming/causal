use mcp_core::{
    client::{Client, ClientBuilder},
    protocol::RequestOptions,
    transport::{ClientSseTransport, ClientSseTransportBuilder},
    types::{ClientCapabilities, Implementation},
};

use super::{ToolDescription, ToolObject};

use crate::error;

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
