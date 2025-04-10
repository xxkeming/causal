use bonsaidb::core::schema::Collection;
use serde::{Deserialize, Serialize};

/// 工具参数
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Param {
    /// 参数名称
    pub name: String,
    /// 参数类型: string, number, boolean, object
    #[serde(rename = "type")]
    pub param_type: String,
    /// 参数描述
    pub description: String,
    /// 是否必填
    pub required: bool,
    /// 测试值
    #[serde(rename = "testValue")]
    pub test_value: Option<String>,
}

/// JavaScript工具数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolScript {
    /// 参数
    pub param: Option<Vec<Param>>,
    /// JS代码
    pub code: String,
}

/// MCP-IO工具数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolMcpIo {
    /// 路径
    pub path: String,
}

/// MCP-SSE工具数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolMcpSse {
    /// URL
    pub url: String,
}

/// 工具数据（可能是JS脚本、MCP-IO或MCP-SSE）
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ToolData {
    Script(ToolScript),
    McpIo(ToolMcpIo),
    McpSse(ToolMcpSse),
}

/// 工具类别
#[derive(Debug, Serialize, Deserialize, Collection, Clone)]
#[collection(name = "tool_categories", primary_key = u64)]
pub struct ToolCategory {
    /// 类别ID
    #[natural_id]
    pub id: u64,
    /// 类别名称
    pub name: String,
    /// 创建时间
    #[serde(rename = "createdAt")]
    pub created_at: i64,
}

/// 工具
#[derive(Debug, Serialize, Deserialize, Collection, Clone)]
#[collection(name = "tools", primary_key = u64)]
pub struct Tool {
    /// 工具ID
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
    pub description: String,
    /// 工具类型: js, mcp-io, mcp-sse
    #[serde(rename = "type")]
    pub tool_type: String,
    /// 工具数据
    pub data: ToolData,
    /// 创建时间
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    /// 更新时间
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,
}

// 为了后向兼容，提供一些辅助方法
impl Tool {
    // 获取参数（如果是JS工具）
    pub fn params(&self) -> Option<&Vec<Param>> {
        match &self.data {
            ToolData::Script(script) => script.param.as_ref(),
            _ => None,
        }
    }

    // 获取代码（如果是JS工具）
    pub fn code(&self) -> Option<&str> {
        match &self.data {
            ToolData::Script(script) => Some(&script.code),
            _ => None,
        }
    }

    // 获取路径（如果是MCP-IO工具）
    pub fn path(&self) -> Option<&str> {
        match &self.data {
            ToolData::McpIo(mcp_io) => Some(&mcp_io.path),
            _ => None,
        }
    }

    // 获取URL（如果是MCP-SSE工具）
    pub fn url(&self) -> Option<&str> {
        match &self.data {
            ToolData::McpSse(mcp_sse) => Some(&mcp_sse.url),
            _ => None,
        }
    }
}
