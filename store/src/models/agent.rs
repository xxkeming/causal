use bonsaidb::core::schema::Collection;
use serde::{Deserialize, Serialize};

/// 智能体类别
#[derive(Debug, Serialize, Deserialize, Collection, Clone)]
#[collection(name = "agent_categories", primary_key = u64)]
pub struct AgentCategory {
    /// 类别ID
    #[natural_id]
    pub id: u64,
    /// 类别名称
    pub name: String,
    /// 创建时间
    #[serde(rename = "createdAt")]
    pub created_at: i64,
}

/// 模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProviderModel {
    /// 主键ID（对应provider的ID）
    pub id: u64,
    /// 模型名称
    pub name: String,
}

/// 模型参数
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelParam {
    /// 参数名称
    pub name: String,
    /// 参数类型: string, number, boolean, object
    pub param_type: String,
    /// 参数值
    pub value: String,
}

/// 智能体
#[derive(Debug, Serialize, Deserialize, Collection, Clone)]
#[collection(name = "agents")]
pub struct Agent {
    /// 智能体ID
    #[natural_id]
    pub id: u64,
    /// 所属类别ID
    #[serde(rename = "categoryId")]
    pub category_id: u64,
    /// 图标ID
    #[serde(rename = "iconId")]
    pub icon_id: Option<u64>,
    /// 名称
    pub name: String,
    /// 描述
    pub description: Option<String>,
    /// 使用的模型
    pub model: Option<ProviderModel>,
    /// 提示词
    pub prompt: String,
    /// 生成文本的随机程度(0-2)
    pub temperature: f64,
    /// 词汇多样性(0-1)
    /// 最大消息长度，0表示不限制
    #[serde(rename = "maxTokens")]
    pub max_tokens: u32,
    /// 上下文保留消息数量
    #[serde(rename = "contextSize")]
    pub context_size: u32,
    /// 自定义参数
    pub params: Option<Vec<ModelParam>>,
    /// 工具集合
    pub tools: Option<Vec<u64>>,
    /// 创建时间, 单位秒
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    /// 更新时间
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,
}
