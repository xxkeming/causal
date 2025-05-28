use crate::Store;
use crate::error::StoreError;
use crate::models::Settings;

use bonsaidb::core::keyvalue::*;

impl Store {
    pub fn get_settings(&self) -> Result<Settings, StoreError> {
        self.db
            .get_key("settings")
            .query()?
            .map(|value| value.deserialize::<Settings>().map_err(|e| e.into()))
            .unwrap_or(Err(StoreError::InvalidData("not settings".to_string())))
    }

    pub fn set_settings(&self, settings: Settings) -> Result<(), StoreError> {
        self.db.set_key("settings", &settings).execute().map(|_| ()).map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Search, SearchType, Settings};
    use tempfile::tempdir;

    #[test]
    fn test_settings_operations() {
        // 创建临时测试目录
        let temp_dir = tempdir().unwrap();
        let store = Store::open(temp_dir.path()).unwrap();

        // 测试初始状态下获取 settings
        let result = store.get_settings();
        assert!(result.is_err());

        // 构造 Settings 对象
        let search = Search {
            r#type: SearchType::Tavily {
                api_key: "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".to_string(),
            },
            mode: 1,
            result_count: 5,
        };
        let settings = Settings { search: search.clone(), transcriptions: None };
        // 设置 settings
        let set_result = store.set_settings(settings.clone());
        assert!(set_result.is_ok());

        // 获取刚设置的 settings
        let get_result = store.get_settings();
        assert!(get_result.is_ok());
        let retrieved_settings = get_result.unwrap();
        // 检查 search 字段
        match &retrieved_settings.search.r#type {
            SearchType::Tavily { api_key } => assert_eq!(api_key, "test_api_key"),
        }
        assert_eq!(retrieved_settings.search.mode, 1);
        assert_eq!(retrieved_settings.search.result_count, 5);
        assert!(retrieved_settings.transcriptions.is_none());
    }
}
