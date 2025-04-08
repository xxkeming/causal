use crate::models::*;

#[derive(bonsaidb::core::schema::Schema)]
#[schema(name = "causal", collections = [
    Provider, Agent, AgentCategory, 
    Tool, ToolCategory, 
    KnowledgeBase, KnowledgeBaseCategory, KnowledgeBaseDocument,
    ChatSession, ChatMessage
])]
pub struct Schema;
