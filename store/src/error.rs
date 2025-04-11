use thiserror::Error;

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("Database error: {0}")]
    DbLocal(#[from] bonsaidb::local::Error),

    #[error("Database error: {0}")]
    DbCore(#[from] bonsaidb::core::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("insert error: {0}")]
    Operator(String),

    #[error("Item not found: {0}")]
    NotFound(String),

    #[error("Invalid data: {0}")]
    InvalidData(String),
}
