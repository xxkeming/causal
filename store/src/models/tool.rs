use bonsaidb::core::schema::Collection;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 工具参数
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolParam {
    /// 参数名称
    pub name: String,
    /// 参数类型: string, number, boolean, object
    pub param_type: String,
    /// 参数描述
    pub description: String,
    /// 是否必填
    pub required: bool,
    /// 测试值
    pub test_value: Option<String>,
}

/// 工具类别
#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "tool_categories")]
pub struct ToolCategory {
    /// 类别ID
    pub id: u64,
    /// 类别名称
    pub name: String,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: Option<DateTime<Utc>>,
}

/// 工具
#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "tools")]
pub struct Tool {
    /// 工具ID
    pub id: u64,
    /// 所属类别ID
    pub category_id: Option<u64>,
    /// 图标ID
    pub icon_id: Option<u64>,
    /// 名称
    pub name: String,
    /// 描述
    pub description: String,
    /// 参数
    pub params: Option<Vec<ToolParam>>,
    /// JS代码(执行工具的代码)
    pub code: String,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: Option<DateTime<Utc>>,
}
