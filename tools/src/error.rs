#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("MCP error: {0}")]
    Mcp(String),

    #[error("Tavily error: {0}")]
    Tavily(#[from] tavily::TavilyError),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}
