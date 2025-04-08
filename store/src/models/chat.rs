use bonsaidb::core::schema::Collection;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 角色类型
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ChatRole {
    User,
    Assistant,
    System,
}

/// 消息状态
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ChatMessageStatus {
    Sending,
    Success,
    Error,
}

/// 聊天消息
#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "chat_messages")]
pub struct ChatMessage {
    /// 所属会话ID
    pub session_id: u64,
    /// 角色
    pub role: ChatRole,
    /// 内容
    pub content: String,
    /// 状态
    pub status: Option<ChatMessageStatus>,
    /// 时间戳
    pub timestamp: DateTime<Utc>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
}

/// 聊天会话
#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "chat_sessions")]
pub struct ChatSession {
    /// 会话ID
    pub id: u64,
    /// 智能体ID
    pub agent_id: String,
    /// 会话主题
    pub topic: String,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: Option<DateTime<Utc>>,
}
