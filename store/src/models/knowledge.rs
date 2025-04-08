use bonsaidb::core::schema::Collection;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 知识库类别
#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "knowledge_base_categories")]
pub struct KnowledgeBaseCategory {
    /// 类别ID
    pub id: String,
    /// 类别名称
    pub name: String,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: Option<DateTime<Utc>>,
}

/// 知识库
#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "knowledge_bases")]
pub struct KnowledgeBase {
    /// 知识库ID
    pub id: String,
    /// 所属类别ID
    pub category_id: String,
    /// 图标ID
    pub icon_id: Option<u64>,
    /// 名称
    pub name: String,
    /// 描述
    pub description: String,
    /// 站点地址(仅当category_id=2时有效)
    pub site: Option<String>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: Option<DateTime<Utc>>,
}

/// 知识库文档
#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "knowledge_base_documents")]
pub struct KnowledgeBaseDocument {
    /// 文档ID
    pub id: String,
    /// 所属知识库ID
    pub knowledge_base_id: String,
    /// 标题
    pub title: String,
    /// 内容
    pub content: String,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: Option<DateTime<Utc>>,
}
