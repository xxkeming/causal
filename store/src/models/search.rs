use serde::{Deserialize, Serialize};

// 搜索服务商
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "name")]
/// 搜索服务商
pub enum SearchType {
    /// Tavily 搜索服务
    Tavily {
        /// Tavily API密钥
        #[serde(rename = "apiKey")]
        api_key: String,
    },
}

/// 搜索选项
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Search {
    pub r#type: SearchType,

    /// 模式 先搜索, 工具智能搜索
    pub mode: u32,

    /// 保留几条搜索结果
    #[serde(rename = "resultCount")]
    pub result_count: u32,
}
