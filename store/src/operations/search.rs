use crate::Store;
use crate::error::StoreError;
use crate::models::Search;
use bonsaidb::core::keyvalue::*;

impl Store {
    pub fn get_search(&self) -> Result<Search, StoreError> {
        self.db
            .get_key("search")
            .query()?
            .map(|value| value.deserialize::<Search>().map_err(|e| e.into()))
            .unwrap_or(Err(StoreError::InvalidData("not search".to_string())))
    }

    pub fn set_search(&self, search: Search) -> Result<(), StoreError> {
        self.db.set_key("search", &search).execute().map(|_| ()).map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_search_operations() {
        // 创建临时测试目录
        let temp_dir = tempdir().unwrap();
        let store = Store::open(temp_dir.path()).unwrap();

        // 测试初始状态下获取搜索
        let result = store.get_search();
        assert!(result.is_err());

        // 测试设置搜索
        let search = Search {
            r#type: crate::SearchType::Tavily { api_key: "test".to_string() },
            mode: 1,
            result_count: 5,
        };
        let set_result = store.set_search(search.clone());
        assert!(set_result.is_ok());

        // 测试获取刚设置的搜索
        let get_result = store.get_search();
        assert!(get_result.is_ok());
        let retrieved_search = get_result.unwrap();
        assert_eq!(retrieved_search.mode, search.mode);
    }
}
