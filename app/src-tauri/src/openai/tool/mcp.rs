use std::collections::HashMap;

use rmcp::{
    RoleClient, ServiceExt,
    model::{
        CallToolRequestParam, ClientCapabilities, ClientInfo, Implementation,
        InitializeRequestParam, ListToolsResult,
    },
    service::RunningService,
    transport::{SseTransport, TokioChildProcess},
};
use tokio::process::Command;

use super::{ToolDescription, ToolObject};

use crate::error;

pub struct McpTool {
    client: RunningService<RoleClient, InitializeRequestParam>,
    description: Vec<ToolDescription>,
}

impl McpTool {
    fn new(
        client: RunningService<RoleClient, InitializeRequestParam>, tools: ListToolsResult,
    ) -> Self {
        let description = tools
            .tools
            .into_iter()
            .map(|tool| ToolDescription {
                name: tool.name.into_owned(),
                description: tool.description.map(|v| v.to_string()).unwrap_or_default(),
                schema: serde_json::to_value(&*tool.input_schema).unwrap(),
            })
            .collect();
        Self { client, description }
    }

    pub async fn try_new_io(
        command: String, args: Option<Vec<String>>, env: Option<HashMap<String, String>>,
    ) -> Result<Self, error::Error> {
        let mut path = std::env::var("PATH").unwrap_or_default();

        let mut cmd = Command::new(command);
        // cmd.current_dir("/tmp/");

        if let Some(args) = args {
            cmd.args(args);
        }

        if let Some(env) = env {
            for (key, value) in env {
                if key == "PATH" {
                    path = format!("{value}:{path}");
                } else {
                    cmd.env(key, value);
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            path = format!(
                "{}:{}",
                path,
                "/usr/local/bin:/usr/local/sbin:/opt/local/bin:/opt/local/sbin:/System/Cryptexes/App/usr/bin"
            );
        }

        // 添加系统环境变量
        cmd.env("PATH", path);

        let client_info = ClientInfo {
            protocol_version: Default::default(),
            capabilities: ClientCapabilities::default(),
            client_info: Implementation {
                name: env!("CARGO_PKG_NAME").to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
        };

        let client = client_info
            .serve(TokioChildProcess::new(&mut cmd)?)
            .await
            .map_err(|e| error::Error::Mcp(format!("{e}")))?;

        let tools = client
            .list_tools(Default::default())
            .await
            .map_err(|e| error::Error::Mcp(format!("{e}")))?;

        Ok(Self::new(client, tools))
    }

    pub async fn try_new_sse(url: String) -> Result<Self, error::Error> {
        let transport =
            SseTransport::start(url).await.map_err(|e| error::Error::Mcp(format!("{e}")))?;

        let client_info = ClientInfo {
            protocol_version: Default::default(),
            capabilities: ClientCapabilities::default(),
            client_info: Implementation {
                name: env!("CARGO_PKG_NAME").to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
        };

        let client =
            client_info.serve(transport).await.map_err(|e| error::Error::Mcp(format!("{e}")))?;

        let tools = client
            .list_tools(Default::default())
            .await
            .map_err(|e| error::Error::Mcp(format!("{e}")))?;

        Ok(Self::new(client, tools))
    }
}

impl ToolObject for McpTool {
    fn description(&self) -> Vec<ToolDescription> {
        self.description.clone()
    }
    fn call<'a>(
        &'a self, name: &'a str, param: serde_json::Value,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<serde_json::Value, error::Error>> + Send + 'a>,
    > {
        Box::pin(async move {
            let arguments = match param {
                serde_json::Value::Object(v) => Some(v),
                _ => None,
            };

            let response = self
                .client
                .call_tool(CallToolRequestParam { name: name.to_string().into(), arguments })
                .await
                .map_err(|e| error::Error::Mcp(format!("{e}")))?;

            serde_json::to_value(response).map_err(|e| error::Error::Mcp(format!("{e}")))
        })
    }
}
