use crate::Store;
use crate::error::StoreError;
use crate::models::Provider;
use bonsaidb::core::schema::SerializedCollection;

impl Store {
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
        let query_result = all_providers.query()?;
        let providers = query_result.iter().map(|doc| doc.contents.clone()).collect();

        Ok(providers)
    }

    /// 通过ID获取模型提供商
    pub fn get_provider(&self, id: u64) -> Result<Option<Provider>, StoreError> {
        let provider_doc = Provider::get(&id, &self.db)?;
        Ok(provider_doc.map(|doc| doc.contents))
    }

    /// 更新模型提供商
    pub fn update_provider(&self, provider: Provider) -> Result<(), StoreError> {
        let id: u64 = provider.id;

        // 检查是否存在该提供商
        let mut doc = match Provider::get(&id, &self.db)? {
            Some(doc) => doc,
            None => return Err(StoreError::NotFound(format!("Provider with id {}", id))),
        };

        doc.contents = provider;
        doc.update(&self.db)?;

        Ok(())
    }

    /// 删除模型提供商
    pub fn delete_provider(&self, id: u64) -> Result<(), StoreError> {
        // 检查是否存在该提供商
        let doc = match Provider::get(&id, &self.db)? {
            Some(doc) => doc,
            None => return Err(StoreError::NotFound(format!("Provider with id {}", id))),
        };

        doc.delete(&self.db)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Model;
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
            stream: false,
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
            stream: false,
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
}
