#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("MCP error: {0}")]
    McpError(String),

    #[error("Tavily error: {0}")]
    TavilyError(#[from] tavily::TavilyError),
}
