use crate::error::StoreError;
use crate::models::{ChatMessage, ChatSession};
use crate::{MessageStatus, Store};
use bonsaidb::core::schema::SerializedCollection;
use chrono::Utc;

impl Store {
    // ===== 聊天会话操作 =====

    /// 添加聊天会话
    pub fn add_chat_session(&self, session: ChatSession) -> Result<ChatSession, StoreError> {
        let mut session_to_save = session;

        // 确保创建时间有值
        if session_to_save.created_at == 0 {
            session_to_save.created_at = Utc::now().timestamp();
        }

        session_to_save
            .clone()
            .push_into(&self.db)
            .map_err(|e| StoreError::Operator(format!("add chat session {e}")))?;
        Ok(session_to_save)
    }

    /// 获取所有聊天会话
    pub fn get_all_chat_sessions(&self) -> Result<Vec<ChatSession>, StoreError> {
        let all_sessions = ChatSession::all(&self.db);
        let query_result = all_sessions
            .query()
            .map_err(|e| StoreError::Operator(format!("query chat sessions {e}")))?;
        let sessions = query_result.iter().map(|doc| doc.contents.clone()).collect();

        Ok(sessions)
    }

    /// 通过ID获取聊天会话
    pub fn get_chat_session(&self, id: u64) -> Result<Option<ChatSession>, StoreError> {
        let session_doc = ChatSession::get(&id, &self.db)
            .map_err(|e| StoreError::Operator(format!("get chat session {e}")))?;
        Ok(session_doc.map(|doc| doc.contents))
    }

    /// 更新聊天会话
    pub fn update_chat_session(&self, session: ChatSession) -> Result<(), StoreError> {
        let id = session.id;

        // 检查是否存在该会话
        let mut doc = match ChatSession::get(&id, &self.db)
            .map_err(|e| StoreError::Operator(format!("get chat session {e}")))?
        {
            Some(doc) => doc,
            None => return Err(StoreError::NotFound(format!("ChatSession with id {}", id))),
        };

        let mut updated_session = session;

        // 确保更新时间有值
        updated_session.updated_at = Some(Utc::now().timestamp());

        doc.contents = updated_session;
        doc.update(&self.db)
            .map_err(|e| StoreError::Operator(format!("update chat session {e}")))?;

        Ok(())
    }

    /// 删除聊天会话
    pub fn delete_chat_session(&self, id: u64) -> Result<(), StoreError> {
        // 检查是否存在该会话
        let doc = match ChatSession::get(&id, &self.db)
            .map_err(|e| StoreError::Operator(format!("get chat session {e}")))?
        {
            Some(doc) => doc,
            None => return Err(StoreError::NotFound(format!("ChatSession with id {}", id))),
        };

        // 首先删除所有关联的聊天消息
        self.delete_messages_by_session(id)?;

        // 然后删除会话本身
        doc.delete(&self.db)
            .map_err(|e| StoreError::Operator(format!("delete chat session {e}")))?;
        Ok(())
    }

    /// 获取用户的所有会话
    pub fn get_chat_sessions_by_user(&self) -> Result<Vec<ChatSession>, StoreError> {
        let all_sessions = ChatSession::all(&self.db);
        let query_result = all_sessions
            .query()
            .map_err(|e| StoreError::Operator(format!("query chat sessions {e}")))?;

        let sessions: Vec<ChatSession> =
            query_result.iter().map(|doc| doc.contents.clone()).collect();

        Ok(sessions)
    }

    /// 获取特定智能体的所有会话
    pub fn get_chat_sessions_by_agent_id(
        &self, agent_id: u64,
    ) -> Result<Vec<ChatSession>, StoreError> {
        let all_sessions = ChatSession::all(&self.db);
        let query_result = all_sessions
            .query()
            .map_err(|e| StoreError::Operator(format!("query chat sessions {e}")))?;

        let sessions: Vec<ChatSession> = query_result
            .iter()
            .map(|doc| doc.contents.clone())
            .filter(|session: &ChatSession| session.agent_id == agent_id)
            .collect();

        Ok(sessions)
    }

    // ===== 聊天消息操作 =====

    /// 添加聊天消息
    pub fn add_chat_message(&self, message: ChatMessage) -> Result<ChatMessage, StoreError> {
        let mut message_to_save = message;

        // 确保时间戳有值
        if message_to_save.created_at == 0 {
            message_to_save.created_at = Utc::now().timestamp();
        }

        // 验证会话是否存在
        if self.get_chat_session(message_to_save.session_id)?.is_none() {
            return Err(StoreError::NotFound(format!(
                "ChatSession with id {}",
                message_to_save.session_id
            )));
        }

        message_to_save
            .clone()
            .push_into(&self.db)
            .map_err(|e| StoreError::Operator(format!("add chat message {e}")))?;
        Ok(message_to_save)
    }

    /// 通过ID获取聊天消息
    pub fn get_chat_message(&self, id: u64) -> Result<Option<ChatMessage>, StoreError> {
        let message_doc = ChatMessage::get(&id, &self.db)
            .map_err(|e| StoreError::Operator(format!("get chat message {e}")))?;
        Ok(message_doc.map(|doc| doc.contents))
    }

    /// 获取会话的所有消息
    pub fn get_messages_by_session(&self, session_id: u64) -> Result<Vec<ChatMessage>, StoreError> {
        let all_messages = ChatMessage::all(&self.db);
        let query_result = all_messages
            .query()
            .map_err(|e| StoreError::Operator(format!("query chat messages {e}")))?;

        let messages: Vec<ChatMessage> = query_result
            .iter()
            .map(|doc| doc.contents.clone())
            .filter(|message: &ChatMessage| message.session_id == session_id)
            .collect();

        Ok(messages)
    }

    /// 更新聊天消息
    pub fn update_chat_message(&self, message: ChatMessage) -> Result<(), StoreError> {
        let id = message.id;

        // 检查是否存在该消息
        let mut doc = match ChatMessage::get(&id, &self.db)
            .map_err(|e| StoreError::Operator(format!("get chat message {e}")))?
        {
            Some(doc) => doc,
            None => return Err(StoreError::NotFound(format!("ChatMessage with id {}", id))),
        };

        // 验证会话是否存在
        if self.get_chat_session(message.session_id)?.is_none() {
            return Err(StoreError::NotFound(format!(
                "ChatSession with id {}",
                message.session_id
            )));
        }

        doc.contents = message;
        doc.update(&self.db)
            .map_err(|e| StoreError::Operator(format!("update chat message {e}")))?;

        Ok(())
    }

    /// 删除聊天消息
    pub fn delete_chat_message(&self, id: u64) -> Result<(), StoreError> {
        // 检查是否存在该消息
        let doc = match ChatMessage::get(&id, &self.db)
            .map_err(|e| StoreError::Operator(format!("get chat message {e}")))?
        {
            Some(doc) => doc,
            None => return Err(StoreError::NotFound(format!("ChatMessage with id {}", id))),
        };

        doc.delete(&self.db)
            .map_err(|e| StoreError::Operator(format!("delete chat message {e}")))?;
        Ok(())
    }

    /// 删除会话的所有消息
    pub fn delete_messages_by_session(&self, session_id: u64) -> Result<(), StoreError> {
        let messages = self.get_messages_by_session(session_id)?;
        for message in messages {
            self.delete_chat_message(message.id)?;
        }
        Ok(())
    }

    /// 获取会话的最近消息
    pub fn get_latest_messages_by_session(
        &self, session_id: u64, limit: usize,
    ) -> Result<Vec<ChatMessage>, StoreError> {
        let mut messages = self.get_messages_by_session(session_id)?;

        // 按时间戳排序（降序）
        messages.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        // 限制返回数量
        if messages.len() > limit {
            messages.truncate(limit);
        }

        Ok(messages)
    }

    /// 根据会话里面的一条消息,查找指定条数的消息
    pub fn get_messages_by_session_and_message(
        &self, session_id: u64, message_id: u64, limit: usize,
    ) -> Result<Vec<ChatMessage>, StoreError> {
        let messages = self.get_messages_by_session(session_id)?;

        let error_messages = messages
            .iter()
            .filter_map(|m| (m.status != MessageStatus::Success).then_some(m.id))
            .collect::<Vec<_>>();

        // 按时间戳排序（降序）
        let mut sorted_messages: Vec<ChatMessage> = messages
            .into_iter()
            .filter(|m| {
                m.id <= message_id
                    && (!error_messages.contains(&m.id) || !error_messages.contains(&(m.id - 1)))
            })
            .collect();

        sorted_messages.sort_by(|a, b| b.id.cmp(&a.id));

        // 限制返回数量
        if sorted_messages.len() > limit {
            sorted_messages.truncate(limit);
        }

        sorted_messages.sort_by(|a, b| a.id.cmp(&b.id));

        Ok(sorted_messages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ChatMessage, ChatSession, MessageStatus, Role};
    use chrono::Utc;
    use tempfile::tempdir;

    #[test]
    fn test_chat_session_crud() {
        // 创建临时测试目录
        let temp_dir = tempdir().unwrap();
        let store = Store::open(temp_dir.path()).unwrap();

        // 准备测试数据
        let session = ChatSession {
            id: 1,
            agent_id: 200,
            topic: "测试聊天会话".to_string(),
            created_at: 0, // 将被自动设置
            updated_at: None,
        };

        // 测试添加
        let add_result = store.add_chat_session(session.clone());
        assert!(add_result.is_ok());

        // 测试获取单个
        let fetched = store.get_chat_session(1).unwrap().unwrap();
        assert_eq!(fetched.topic, "测试聊天会话");
        assert!(fetched.created_at > 0);

        // 测试获取所有
        let all_sessions = store.get_all_chat_sessions().unwrap();
        assert_eq!(all_sessions.len(), 1);

        // 测试获取用户的所有会话
        let user_sessions = store.get_chat_sessions_by_user().unwrap();
        assert_eq!(user_sessions.len(), 1);

        // 测试更新
        let mut updated_session = fetched.clone();
        updated_session.topic = "更新后的会话主题".to_string();
        let update_result = store.update_chat_session(updated_session);
        assert!(update_result.is_ok());

        let fetched_after_update = store.get_chat_session(1).unwrap().unwrap();
        assert_eq!(fetched_after_update.topic, "更新后的会话主题");
        assert!(fetched_after_update.updated_at.is_some());

        // 测试删除
        let delete_result = store.delete_chat_session(1);
        assert!(delete_result.is_ok());
        assert!(store.get_chat_session(1).unwrap().is_none());
    }

    #[test]
    fn test_chat_message_crud() {
        // 创建临时测试目录
        let temp_dir = tempdir().unwrap();
        let store = Store::open(temp_dir.path()).unwrap();

        // 先创建一个会话
        let session = ChatSession {
            id: 1,
            agent_id: 200,
            topic: "测试会话".to_string(),
            created_at: 0,
            updated_at: None,
        };
        store.add_chat_session(session).unwrap();

        // 准备测试消息
        let message = ChatMessage {
            id: 1,
            session_id: 1,
            role: Role::User,
            content: "你好，这是一条测试消息".to_string(),
            status: MessageStatus::Sending,
            cost: None,
            feedback: 0,
            prompt_tokens: None,
            completion_tokens: None,
            total_tokens: None,
            created_at: 0, // 将被自动设置
            tools: None,
            attachments: None,
        };

        // 测试添加
        let add_result = store.add_chat_message(message.clone());
        assert!(add_result.is_ok());

        // 测试获取单个
        let fetched = store.get_chat_message(1).unwrap().unwrap();
        assert_eq!(fetched.content, "你好，这是一条测试消息");
        assert!(fetched.created_at > 0);

        // 测试获取会话的所有消息
        let session_messages = store.get_messages_by_session(1).unwrap();
        assert_eq!(session_messages.len(), 1);
        assert_eq!(session_messages[0].role, Role::User);

        // 测试更新
        let mut updated_message = fetched.clone();
        updated_message.content = "已更新的消息内容".to_string();
        updated_message.status = MessageStatus::Success;
        let update_result = store.update_chat_message(updated_message);
        assert!(update_result.is_ok());

        let fetched_after_update = store.get_chat_message(1).unwrap().unwrap();
        assert_eq!(fetched_after_update.content, "已更新的消息内容");
        assert_eq!(fetched_after_update.status, MessageStatus::Success);

        // 添加更多消息用于测试获取最近消息
        for i in 2..6 {
            let msg = ChatMessage {
                id: i,
                session_id: 1,
                role: if i % 2 == 0 { Role::User } else { Role::Assistant },
                content: format!("测试消息 {}", i),
                status: MessageStatus::Success,
                cost: Some(i as i64),
                feedback: 0,
                prompt_tokens: Some(i as u32),
                completion_tokens: Some(i as u32),
                total_tokens: Some(i as u32),
                created_at: Utc::now().timestamp() + i as i64, // 递增的时间戳
                tools: None,
                attachments: None,
            };
            store.add_chat_message(msg).unwrap();
        }

        // 测试获取最近消息
        let latest_messages = store.get_latest_messages_by_session(1, 3).unwrap();
        assert_eq!(latest_messages.len(), 3);
        assert_eq!(latest_messages[0].id, 5); // 最新的消息应该在前面
        assert_eq!(latest_messages[2].id, 3);

        // 测试删除单条消息
        let delete_result = store.delete_chat_message(1);
        assert!(delete_result.is_ok());
        assert!(store.get_chat_message(1).unwrap().is_none());

        // 此时会话中还有4条消息
        assert_eq!(store.get_messages_by_session(1).unwrap().len(), 4);

        // 测试删除会话的所有消息
        let delete_all_result = store.delete_messages_by_session(1);
        assert!(delete_all_result.is_ok());
        assert_eq!(store.get_messages_by_session(1).unwrap().len(), 0);

        // 测试删除会话（包括级联删除消息）
        // 首先再添加一些消息
        let msg = ChatMessage {
            id: 10,
            session_id: 1,
            role: Role::User,
            content: "新的测试消息".to_string(),
            status: MessageStatus::Sending,
            cost: None,
            feedback: 0,
            prompt_tokens: None,
            completion_tokens: None,
            total_tokens: None,
            created_at: Utc::now().timestamp(),
            tools: None,
            attachments: None,
        };
        store.add_chat_message(msg).unwrap();

        // 验证消息添加成功
        assert_eq!(store.get_messages_by_session(1).unwrap().len(), 1);

        // 删除会话
        store.delete_chat_session(1).unwrap();

        // 验证会话和关联的消息都被删除
        assert!(store.get_chat_session(1).unwrap().is_none());
        assert_eq!(store.get_messages_by_session(1).unwrap().len(), 0);
    }

    #[test]
    fn test_chat_message_with_nonexistent_session() {
        // 创建临时测试目录
        let temp_dir = tempdir().unwrap();
        let store = Store::open(temp_dir.path()).unwrap();

        // 准备测试消息，但不创建对应的会话
        let message = ChatMessage {
            id: 1,
            session_id: 999, // 不存在的会话ID
            role: Role::User,
            content: "测试消息".to_string(),
            status: MessageStatus::Success,
            cost: None,
            feedback: 0,
            prompt_tokens: None,
            completion_tokens: None,
            total_tokens: None,
            created_at: 0,
            tools: None,
            attachments: None,
        };

        // 测试添加 - 应该失败
        let add_result = store.add_chat_message(message.clone());
        assert!(add_result.is_err());

        // 测试更新 - 也应该失败
        let update_result = store.update_chat_message(message);
        assert!(update_result.is_err());
    }
}
