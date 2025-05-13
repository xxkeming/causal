#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("store error: {0}")]
    Store(#[from] store::StoreError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("openai error: {0}")]
    OpenAI(#[from] async_openai::error::OpenAIError),

    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Mcp error: {0}")]
    Mcp(String),

    #[error("Tavily error: {0}")]
    Tavily(#[from] tavily::TavilyError),

    #[error("Unknown data")]
    Unknown,
}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl From<Error> for serde_json::Value {
    fn from(e: Error) -> Self {
        serde_json::json!({
            "status": "error",
            "error": e.to_string(),
        })
    }
}
