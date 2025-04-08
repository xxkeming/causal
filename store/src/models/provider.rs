use bonsaidb::core::schema::Collection;
use serde::{Deserialize, Serialize};

/// 模型名及其标签
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model {
    pub name: String,
    pub tags: Vec<String>,
}

/// 模型提供商
#[derive(Debug, Serialize, Deserialize, Clone, Collection)]
#[collection(name = "providers", primary_key = u64)]
pub struct Provider {
    /// 主键ID
    #[natural_id]
    pub id: u64,
    /// 自定义名称
    pub name: String,
    /// 对应API类别名称
    #[serde(rename = "apiCategory")]
    pub api_category: String,
    /// API URL
    pub url: String,
    /// API密钥
    #[serde(rename = "apiKey")]
    pub api_key: Option<String>,
    /// 支持的模型
    pub models: Option<Vec<Model>>,
    /// 创建时间
    #[serde(rename = "createdAt")]
    pub created_at: Option<i64>,
    /// 更新时间
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,
}
