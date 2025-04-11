mod error;
mod models;
mod operations;
mod schema;

pub use error::StoreError;
pub use models::*;
pub use schema::*;

use bonsaidb::local::{
    Database,
    config::{Builder, StorageConfiguration},
};

use std::path::Path;

/// 数据库管理器
#[derive(Debug, Clone)]
pub struct Store {
    db: Database,
}

impl Store {
    /// 创建或打开数据库
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, StoreError> {
        let db = Database::open::<schema::Schema>(
            StorageConfiguration::new(path).with_schema::<schema::Schema>()?,
        )?;

        Ok(Self { db })
    }
}
