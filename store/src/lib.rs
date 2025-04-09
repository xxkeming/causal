mod error;
mod models;
mod schema;

pub use error::StoreError;
pub use models::*;
pub use schema::*;

use bonsaidb::core::schema::SerializedCollection;
use bonsaidb::local::{
    Database,
    config::{Builder, StorageConfiguration},
};

use std::path::Path;

/// 数据库管理器
#[derive(Debug, Clone)]
pub struct Store {
    db: Database,
}

impl Store {
    /// 创建或打开数据库
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, StoreError> {
        let db = Database::open::<schema::Schema>(
            StorageConfiguration::new(path).with_schema::<schema::Schema>()?,
        )?;

        Ok(Self { db })
    }

    /// 添加模型提供商
    pub fn add_provider(&self, provider: Provider) -> Result<(), StoreError> {
        provider
            .push_into(&self.db)
            .map_err(|e| StoreError::Operator(format!("add provider {e}")))?;
        Ok(())
    }

    /// 获取所有模型提供商
    pub fn get_all_providers(&self) -> Result<Vec<Provider>, StoreError> {
        let all_providers = Provider::all(&self.db);
        let query_result = all_providers.query().unwrap();
        let providers = query_result.iter().map(|doc| doc.contents.clone()).collect();

        Ok(providers)
    }

    /// 通过ID获取模型提供商
    pub fn get_provider(&self, id: u64) -> Result<Option<Provider>, StoreError> {
        let provider_doc = Provider::get(&id, &self.db).unwrap();
        Ok(provider_doc.map(|doc| doc.contents))
    }

    /// 更新模型提供商
    pub fn update_provider(&self, provider: Provider) -> Result<(), StoreError> {
        let id: u64 = provider.id;

        // 检查是否存在该提供商
        let mut doc = match Provider::get(&id, &self.db).unwrap() {
            Some(doc) => doc,
            None => return Err(StoreError::NotFound(format!("Provider with id {}", id))),
        };

        doc.contents = provider;
        doc.update(&self.db).unwrap();

        Ok(())
    }

    /// 删除模型提供商
    pub fn delete_provider(&self, id: u64) -> Result<(), StoreError> {
        // 检查是否存在该提供商
        let doc = match Provider::get(&id, &self.db).unwrap() {
            Some(doc) => doc,
            None => return Err(StoreError::NotFound(format!("Provider with id {}", id))),
        };

        doc.delete(&self.db).unwrap();
        Ok(())
    }

    // ==== 智能体类别 CRUD 操作 ====

    /// 添加智能体类别
    pub fn add_agent_category(&self, category: AgentCategory) -> Result<(), StoreError> {
        category
            .push_into(&self.db)
            .map_err(|e| StoreError::Operator(format!("add agent category {e}")))?;
        Ok(())
    }

    /// 获取所有智能体类别
    pub fn get_all_agent_categories(&self) -> Result<Vec<AgentCategory>, StoreError> {
        let all_categories = AgentCategory::all(&self.db);
        let query_result = all_categories
            .query()
            .map_err(|e| StoreError::Operator(format!("query agent categories {e}")))?;
        let categories = query_result.iter().map(|doc| doc.contents.clone()).collect();

        Ok(categories)
    }

    /// 通过ID获取智能体类别
    pub fn get_agent_category(&self, id: u64) -> Result<Option<AgentCategory>, StoreError> {
        let category_doc = AgentCategory::get(&id, &self.db)
            .map_err(|e| StoreError::Operator(format!("get agent category {e}")))?;
        Ok(category_doc.map(|doc| doc.contents))
    }

    /// 更新智能体类别
    pub fn update_agent_category(&self, category: AgentCategory) -> Result<(), StoreError> {
        let id = category.id;

        // 检查是否存在该类别
        let mut doc = match AgentCategory::get(&id, &self.db)
            .map_err(|e| StoreError::Operator(format!("get agent category {e}")))?
        {
            Some(doc) => doc,
            None => return Err(StoreError::NotFound(format!("AgentCategory with id {}", id))),
        };

        doc.contents = category;
        doc.update(&self.db)
            .map_err(|e| StoreError::Operator(format!("update agent category {e}")))?;

        Ok(())
    }

    /// 删除智能体类别
    pub fn delete_agent_category(&self, id: u64) -> Result<(), StoreError> {
        // 检查是否存在该类别
        let doc = match AgentCategory::get(&id, &self.db)
            .map_err(|e| StoreError::Operator(format!("get agent category {e}")))?
        {
            Some(doc) => doc,
            None => return Err(StoreError::NotFound(format!("AgentCategory with id {}", id))),
        };

        doc.delete(&self.db)
            .map_err(|e| StoreError::Operator(format!("delete agent category {e}")))?;
        Ok(())
    }

    /// 按类别ID获取智能体
    pub fn get_agents_by_category(&self, category_id: u64) -> Result<Vec<Agent>, StoreError> {
        let all_agents = Agent::all(&self.db);
        let query_result =
            all_agents.query().map_err(|e| StoreError::Operator(format!("query agents {e}")))?;

        let agents: Vec<Agent> = query_result
            .iter()
            .map(|doc| doc.contents.clone())
            .filter(|agent: &Agent| agent.category_id == category_id)
            .collect();

        Ok(agents)
    }

    // ==== 智能体 CRUD 操作 ====

    /// 添加智能体
    pub fn add_agent(&self, agent: Agent) -> Result<(), StoreError> {
        agent.push_into(&self.db).map_err(|e| StoreError::Operator(format!("add agent {e}")))?;
        Ok(())
    }

    /// 获取所有智能体
    pub fn get_all_agents(&self) -> Result<Vec<Agent>, StoreError> {
        let all_agents = Agent::all(&self.db);
        let query_result =
            all_agents.query().map_err(|e| StoreError::Operator(format!("query agents {e}")))?;
        let agents = query_result.iter().map(|doc| doc.contents.clone()).collect();

        Ok(agents)
    }

    /// 通过ID获取智能体
    pub fn get_agent(&self, id: u64) -> Result<Option<Agent>, StoreError> {
        let agent_doc = Agent::get(&id, &self.db)
            .map_err(|e| StoreError::Operator(format!("get agent {e}")))?;
        Ok(agent_doc.map(|doc| doc.contents))
    }

    /// 更新智能体
    pub fn update_agent(&self, agent: Agent) -> Result<(), StoreError> {
        let id = agent.id;

        // 检查是否存在该智能体
        let mut doc = match Agent::get(&id, &self.db)
            .map_err(|e| StoreError::Operator(format!("get agent {e}")))?
        {
            Some(doc) => doc,
            None => return Err(StoreError::NotFound(format!("Agent with id {}", id))),
        };

        doc.contents = agent;
        doc.update(&self.db).map_err(|e| StoreError::Operator(format!("update agent {e}")))?;

        Ok(())
    }

    /// 删除智能体
    pub fn delete_agent(&self, id: u64) -> Result<(), StoreError> {
        // 检查是否存在该智能体
        let doc = match Agent::get(&id, &self.db)
            .map_err(|e| StoreError::Operator(format!("get agent {e}")))?
        {
            Some(doc) => doc,
            None => return Err(StoreError::NotFound(format!("Agent with id {}", id))),
        };

        doc.delete(&self.db).map_err(|e| StoreError::Operator(format!("delete agent {e}")))?;
        Ok(())
    }

    /// 通过category_id删除智能体
    pub fn delete_agent_by_category(&self, category_id: u64) -> Result<(), StoreError> {
        let agents = self.get_agents_by_category(category_id)?;
        for agent in agents {
            self.delete_agent(agent.id)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Model, Provider};
    use chrono::Utc;
    use tempfile::tempdir;

    #[test]
    fn test_provider_crud() {
        // 创建临时测试目录
        let temp_dir = tempdir().unwrap();
        let store = Store::open(temp_dir.path()).unwrap();

        // 准备测试数据
        let models = vec![
            Model { name: "gpt-3.5-turbo".to_string(), tags: vec!["chat".to_string()] },
            Model {
                name: "gpt-4".to_string(),
                tags: vec!["chat".to_string(), "advanced".to_string()],
            },
        ];

        let provider = Provider {
            id: 1,
            name: "OpenAI".to_string(),
            api_category: "openai".to_string(),
            url: "https://api.openai.com/v1".to_string(),
            api_key: Some("sk-test-key".to_string()),
            models: Some(models),
            created_at: Some(Utc::now().timestamp()),
            updated_at: None,
        };

        // 测试添加
        let added = store.add_provider(provider.clone());
        assert!(added.is_ok());

        // 测试获取单个
        let fetched = store.get_provider(1).unwrap().unwrap();
        assert_eq!(fetched.name, "OpenAI");
        assert_eq!(fetched.api_category, "openai");
        assert_eq!(fetched.models.as_ref().unwrap().len(), 2);

        // 测试获取所有
        let all_providers = store.get_all_providers().unwrap();
        assert_eq!(all_providers.len(), 1);
        println!("All providers: {:?}", all_providers);

        // 测试更新
        let mut updated_provider = fetched.clone();
        updated_provider.name = "OpenAI API".to_string();
        updated_provider.url = "https://api.openai.com/v2".to_string();

        let updated = store.update_provider(updated_provider);
        assert!(updated.is_ok());

        let fetched_after_update = store.get_provider(1).unwrap().unwrap();
        assert_eq!(fetched_after_update.name, "OpenAI API");
        assert_eq!(fetched_after_update.url, "https://api.openai.com/v2");
        assert!(fetched_after_update.updated_at.is_some());

        // 测试删除
        store.delete_provider(1).unwrap();
        assert!(store.get_provider(1).unwrap().is_none());
    }

    #[test]
    fn test_provider_not_found() {
        let temp_dir = tempdir().unwrap();
        let store = Store::open(temp_dir.path()).unwrap();

        // 测试获取不存在的提供商
        let not_found = store.get_provider(999).unwrap();
        assert!(not_found.is_none());

        // 测试更新不存在的提供商
        let non_existent = Provider {
            id: 999,
            name: "Non Existent".to_string(),
            api_category: "none".to_string(),
            url: "https://example.com".to_string(),
            api_key: None,
            models: None,
            created_at: Some(Utc::now().timestamp()),
            updated_at: None,
        };

        let update_result = store.update_provider(non_existent);
        assert!(update_result.is_err());

        // 测试删除不存在的提供商
        let delete_result = store.delete_provider(999);
        assert!(delete_result.is_err());
    }

    #[test]
    fn test_agent_category_crud() {
        // 创建临时测试目录
        let temp_dir = tempdir().unwrap();
        let store = Store::open(temp_dir.path()).unwrap();

        // 添加类别
        let category = AgentCategory {
            id: 1,
            name: "测试类别".to_string(),
            created_at: 0, // 将被覆盖
        };

        let result = store.add_agent_category(category);
        assert!(result.is_ok());

        // 获取单个类别
        let fetched = store.get_agent_category(1).unwrap().unwrap();
        assert_eq!(fetched.name, "测试类别");
        assert!(fetched.created_at > 0);

        // 获取所有类别
        let all_categories = store.get_all_agent_categories().unwrap();
        assert_eq!(all_categories.len(), 1);

        // 更新类别
        let updated_category = AgentCategory {
            id: 1,
            name: "更新后的类别".to_string(),
            created_at: fetched.created_at,
        };

        let update_result = store.update_agent_category(updated_category);
        assert!(update_result.is_ok());

        let fetched_after_update = store.get_agent_category(1).unwrap().unwrap();
        assert_eq!(fetched_after_update.name, "更新后的类别");

        // 删除类别
        let delete_result = store.delete_agent_category(1);
        assert!(delete_result.is_ok());
        assert!(store.get_agent_category(1).unwrap().is_none());
    }

    #[test]
    fn test_agent_crud() {
        // 创建临时测试目录
        let temp_dir = tempdir().unwrap();
        let store = Store::open(temp_dir.path()).unwrap();

        // 先添加一个类别
        let category = AgentCategory {
            id: 1,
            name: "测试类别".to_string(),
            created_at: Utc::now().timestamp(),
        };
        store.add_agent_category(category).unwrap();

        // 添加模型提供商
        let provider = Provider {
            id: 1,
            name: "OpenAI".to_string(),
            api_category: "openai".to_string(),
            url: "https://api.openai.com/v1".to_string(),
            api_key: Some("sk-test-key".to_string()),
            models: Some(vec![Model {
                name: "gpt-3.5-turbo".to_string(),
                tags: vec!["chat".to_string()],
            }]),
            created_at: Some(Utc::now().timestamp()),
            updated_at: None,
        };
        store.add_provider(provider).unwrap();

        // 添加智能体
        let agent = Agent {
            id: 1,
            category_id: 1, // 关联到刚创建的类别
            icon_id: Some(100),
            name: "测试智能体".to_string(),
            description: Some("这是一个测试智能体".to_string()),
            model: Some(ProviderModel { id: 1, name: "gpt-3.5-turbo".to_string() }),
            prompt: "你是一个测试助手".to_string(),
            temperature: 0.7,
            top_p: 0.9,
            top_k: 50,
            max_tokens: 2000,
            context_size: 10,
            params: Some(vec![ModelParam {
                name: "test_param".to_string(),
                param_type: "string".to_string(),
                value: "test_value".to_string(),
            }]),
            tools: Some(vec![1, 2, 3]),
            created_at: 0, // 将被覆盖
            updated_at: None,
        };

        let result = store.add_agent(agent);
        assert!(result.is_ok());

        // 获取单个智能体
        let fetched = store.get_agent(1).unwrap().unwrap();
        assert_eq!(fetched.name, "测试智能体");
        assert!(fetched.created_at > 0);
        assert_eq!(fetched.model.as_ref().unwrap().name, "gpt-3.5-turbo");

        // 获取所有智能体
        let all_agents = store.get_all_agents().unwrap();
        assert_eq!(all_agents.len(), 1);

        // 按类别获取智能体
        let agents_by_category = store.get_agents_by_category(1).unwrap();
        assert_eq!(agents_by_category.len(), 1);
        assert_eq!(agents_by_category[0].category_id, 1);

        // 更新智能体
        let mut updated_agent = fetched.clone();
        updated_agent.name = "更新后的智能体".to_string();
        updated_agent.temperature = 0.8;

        let update_result = store.update_agent(updated_agent);
        assert!(update_result.is_ok());

        let fetched_after_update = store.get_agent(1).unwrap().unwrap();
        assert_eq!(fetched_after_update.name, "更新后的智能体");
        assert_eq!(fetched_after_update.temperature, 0.8);
        assert!(fetched_after_update.updated_at.is_some());

        // 删除智能体
        let delete_result = store.delete_agent(1);
        assert!(delete_result.is_ok());
        assert!(store.get_agent(1).unwrap().is_none());
    }

    #[test]
    fn test_agent_category_relationship() {
        // 创建临时测试目录
        let temp_dir = tempdir().unwrap();
        let store = Store::open(temp_dir.path()).unwrap();

        // 添加两个类别
        let category1 =
            AgentCategory {
                id: 1, name: "类别1".to_string(), created_at: Utc::now().timestamp()
            };

        let category2 =
            AgentCategory {
                id: 2, name: "类别2".to_string(), created_at: Utc::now().timestamp()
            };

        store.add_agent_category(category1).unwrap();
        store.add_agent_category(category2).unwrap();

        // 添加智能体
        let agent1 = Agent {
            id: 1,
            category_id: 1,
            icon_id: None,
            name: "智能体1".to_string(),
            description: None,
            model: None,
            prompt: "测试提示词".to_string(),
            temperature: 0.7,
            top_p: 0.9,
            top_k: 50,
            max_tokens: 2000,
            context_size: 10,
            params: None,
            tools: None,
            created_at: 0,
            updated_at: None,
        };

        let agent2 = Agent {
            id: 2,
            category_id: 1,
            icon_id: None,
            name: "智能体2".to_string(),
            description: None,
            model: None,
            prompt: "测试提示词".to_string(),
            temperature: 0.7,
            top_p: 0.9,
            top_k: 50,
            max_tokens: 2000,
            context_size: 10,
            params: None,
            tools: None,
            created_at: 0,
            updated_at: None,
        };

        let agent3 = Agent {
            id: 3,
            category_id: 2,
            icon_id: None,
            name: "智能体3".to_string(),
            description: None,
            model: None,
            prompt: "测试提示词".to_string(),
            temperature: 0.7,
            top_p: 0.9,
            top_k: 50,
            max_tokens: 2000,
            context_size: 10,
            params: None,
            tools: None,
            created_at: 0,
            updated_at: None,
        };

        store.add_agent(agent1).unwrap();
        store.add_agent(agent2).unwrap();
        store.add_agent(agent3).unwrap();

        // 测试按类别获取智能体
        let category1_agents = store.get_agents_by_category(1).unwrap();
        assert_eq!(category1_agents.len(), 2);

        let category2_agents = store.get_agents_by_category(2).unwrap();
        assert_eq!(category2_agents.len(), 1);
        assert_eq!(category2_agents[0].name, "智能体3");

        // 测试删除类别后查询智能体
        // 注意：我们不实现级联删除，所以类别删除后智能体仍存在
        store.delete_agent_category(2).unwrap();

        // 类别2不存在了
        assert!(store.get_agent_category(2).unwrap().is_none());

        // 但是类别2的智能体仍然可以通过ID获取
        let orphaned_agent = store.get_agent(3).unwrap().unwrap();
        assert_eq!(orphaned_agent.name, "智能体3");

        // 但是按类别2查询将返回空列表
        let deleted_category_agents = store.get_agents_by_category(2).unwrap();
        assert_eq!(deleted_category_agents.len(), 0);
    }

    #[test]
    fn test_agent_not_found() {
        let temp_dir = tempdir().unwrap();
        let store = Store::open(temp_dir.path()).unwrap();

        // 测试获取不存在的智能体
        let not_found = store.get_agent(999).unwrap();
        assert!(not_found.is_none());

        // 测试更新不存在的智能体
        let non_existent = Agent {
            id: 999,
            category_id: 1,
            icon_id: None,
            name: "不存在的智能体".to_string(),
            description: None,
            model: None,
            prompt: "测试提示词".to_string(),
            temperature: 0.7,
            top_p: 0.9,
            top_k: 50,
            max_tokens: 2000,
            context_size: 10,
            params: None,
            tools: None,
            created_at: Utc::now().timestamp(),
            updated_at: None,
        };

        let update_result = store.update_agent(non_existent);
        assert!(update_result.is_err());

        // 测试删除不存在的智能体
        let delete_result = store.delete_agent(999);
        assert!(delete_result.is_err());
    }

    #[test]
    fn test_agent_category_not_found() {
        let temp_dir = tempdir().unwrap();
        let store = Store::open(temp_dir.path()).unwrap();

        // 测试获取不存在的类别
        let not_found = store.get_agent_category(999).unwrap();
        assert!(not_found.is_none());

        // 测试更新不存在的类别
        let non_existent = AgentCategory {
            id: 999,
            name: "不存在的类别".to_string(),
            created_at: Utc::now().timestamp(),
        };

        let update_result = store.update_agent_category(non_existent);
        assert!(update_result.is_err());

        // 测试删除不存在的类别
        let delete_result = store.delete_agent_category(999);
        assert!(delete_result.is_err());
    }
}
