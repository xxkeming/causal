use bonsaidb::core::schema::Collection;
use serde::{Deserialize, Serialize};

/// 角色类型
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
    Tool,
}

/// 消息状态
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageStatus {
    Sending,
    Sent,
    Processing,
    Processed,
    Failed,
    Timeout,
    Success,
    Error,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attachment {
    pub name: String,
    pub size: u64,
    // 文件内容,经过base64编码
    pub data: String,
}

/// 工具结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolResult {
    /// 工具ID
    pub id: String,
    /// 工具名称
    pub name: String,
    /// 工具参数
    pub arguments: String,
    /// 工具结果
    pub result: String,
}

/// 聊天消息
#[derive(Debug, Serialize, Deserialize, Collection, Clone)]
#[collection(name = "chat_messages", primary_key = u64)]
pub struct ChatMessage {
    /// 消息ID
    #[natural_id]
    pub id: u64,
    /// 所属会话ID
    #[serde(rename = "sessionId")]
    pub session_id: u64,
    /// 角色
    pub role: Role,
    /// 内容
    pub content: String,
    /// 工具结果
    pub tools: Option<Vec<ToolResult>>,
    /// 附件
    pub attachments: Option<Vec<Attachment>>,
    /// 状态
    pub status: MessageStatus,

    /// 反馈 默认 = 0, 赞 = 1, 踩 = 2
    #[serde(default)]
    pub feedback: u32,

    /// 耗时
    pub cost: Option<i64>,

    /// prompt_tokens
    #[serde(rename = "promptTokens")]
    pub prompt_tokens: Option<u32>,

    /// completion_tokens
    #[serde(rename = "completionTokens")]
    pub completion_tokens: Option<u32>,

    /// 总tokens
    #[serde(rename = "totalTokens")]
    pub total_tokens: Option<u32>,

    /// 创建时间
    #[serde(rename = "createdAt")]
    pub created_at: i64,
}

/// 聊天会话
#[derive(Debug, Serialize, Deserialize, Collection, Clone)]
#[collection(name = "chat_sessions", primary_key = u64)]
pub struct ChatSession {
    /// 会话ID
    #[natural_id]
    pub id: u64,
    /// 智能体ID
    #[serde(rename = "agentId")]
    pub agent_id: u64,
    /// 会话主题
    pub topic: String,
    /// 创建时间
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    /// 更新时间
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,
}
