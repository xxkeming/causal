use crate::Store;
use crate::error::StoreError;
use crate::models::{Tool, ToolCategory};
use bonsaidb::core::schema::SerializedCollection;

impl Store {
    // ==== 工具类别 操作 ====

    /// 添加工具类别
    pub fn add_tool_category(&self, category: ToolCategory) -> Result<(), StoreError> {
        category
            .push_into(&self.db)
            .map_err(|e| StoreError::Operator(format!("add tool category {e}")))?;
        Ok(())
    }

    /// 获取所有工具类别
    pub fn get_all_tool_categories(&self) -> Result<Vec<ToolCategory>, StoreError> {
        let all_categories = ToolCategory::all(&self.db);
        let query_result = all_categories
            .query()
            .map_err(|e| StoreError::Operator(format!("query tool categories {e}")))?;
        let categories = query_result.iter().map(|doc| doc.contents.clone()).collect();

        Ok(categories)
    }

    /// 通过ID获取工具类别
    pub fn get_tool_category(&self, id: u64) -> Result<Option<ToolCategory>, StoreError> {
        let category_doc = ToolCategory::get(&id, &self.db)
            .map_err(|e| StoreError::Operator(format!("get tool category {e}")))?;
        Ok(category_doc.map(|doc| doc.contents))
    }

    /// 更新工具类别
    pub fn update_tool_category(&self, category: ToolCategory) -> Result<(), StoreError> {
        let id = category.id;

        // 检查是否存在该类别
        let mut doc = match ToolCategory::get(&id, &self.db)
            .map_err(|e| StoreError::Operator(format!("get tool category {e}")))?
        {
            Some(doc) => doc,
            None => return Err(StoreError::NotFound(format!("ToolCategory with id {}", id))),
        };

        doc.contents = category;
        doc.update(&self.db)
            .map_err(|e| StoreError::Operator(format!("update tool category {e}")))?;

        Ok(())
    }

    /// 删除工具类别
    pub fn delete_tool_category(&self, id: u64) -> Result<(), StoreError> {
        // 检查是否存在该类别
        let doc = match ToolCategory::get(&id, &self.db)
            .map_err(|e| StoreError::Operator(format!("get tool category {e}")))?
        {
            Some(doc) => doc,
            None => return Err(StoreError::NotFound(format!("ToolCategory with id {}", id))),
        };

        doc.delete(&self.db)
            .map_err(|e| StoreError::Operator(format!("delete tool category {e}")))?;
        Ok(())
    }

    /// 按类别ID获取工具
    pub fn get_tools_by_category(&self, category_id: u64) -> Result<Vec<Tool>, StoreError> {
        let all_tools = Tool::all(&self.db);
        let query_result =
            all_tools.query().map_err(|e| StoreError::Operator(format!("query tools {e}")))?;

        let tools: Vec<Tool> = query_result
            .iter()
            .map(|doc| doc.contents.clone())
            .filter(|tool: &Tool| tool.category_id == category_id)
            .collect();

        Ok(tools)
    }

    // ==== 工具 操作 ====

    /// 添加工具
    pub fn add_tool(&self, tool: Tool) -> Result<(), StoreError> {
        tool.push_into(&self.db).map_err(|e| StoreError::Operator(format!("add tool {e}")))?;
        Ok(())
    }

    /// 获取所有工具
    pub fn get_all_tools(&self) -> Result<Vec<Tool>, StoreError> {
        let all_tools = Tool::all(&self.db);
        let query_result =
            all_tools.query().map_err(|e| StoreError::Operator(format!("query tools {e}")))?;
        let tools = query_result.iter().map(|doc| doc.contents.clone()).collect();

        Ok(tools)
    }

    /// 通过ID获取工具
    pub fn get_tool(&self, id: u64) -> Result<Option<Tool>, StoreError> {
        let tool_doc =
            Tool::get(&id, &self.db).map_err(|e| StoreError::Operator(format!("get tool {e}")))?;
        Ok(tool_doc.map(|doc| doc.contents))
    }

    /// 更新工具
    pub fn update_tool(&self, tool: Tool) -> Result<(), StoreError> {
        let id = tool.id;

        // 检查是否存在该工具
        let mut doc = match Tool::get(&id, &self.db)
            .map_err(|e| StoreError::Operator(format!("get tool {e}")))?
        {
            Some(doc) => doc,
            None => return Err(StoreError::NotFound(format!("Tool with id {}", id))),
        };

        doc.contents = tool;
        doc.update(&self.db).map_err(|e| StoreError::Operator(format!("update tool {e}")))?;

        Ok(())
    }

    /// 删除工具
    pub fn delete_tool(&self, id: u64) -> Result<(), StoreError> {
        // 检查是否存在该工具
        let doc = match Tool::get(&id, &self.db)
            .map_err(|e| StoreError::Operator(format!("get tool {e}")))?
        {
            Some(doc) => doc,
            None => return Err(StoreError::NotFound(format!("Tool with id {}", id))),
        };

        doc.delete(&self.db).map_err(|e| StoreError::Operator(format!("delete tool {e}")))?;
        Ok(())
    }

    /// 通过category_id删除工具
    pub fn delete_tool_by_category(&self, category_id: u64) -> Result<(), StoreError> {
        let tools = self.get_tools_by_category(category_id)?;
        for tool in tools {
            self.delete_tool(tool.id)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{McpTool, Param, ToolData, ToolJavaScript, ToolMcpSse};
    use chrono::Utc;
    use tempfile::tempdir;

    #[test]
    fn test_tool_category_crud() {
        // 创建临时测试目录
        let temp_dir = tempdir().unwrap();
        let store = Store::open(temp_dir.path()).unwrap();

        // 添加类别
        let category = ToolCategory {
            id: 1,
            name: "测试工具类别".to_string(),
            created_at: Utc::now().timestamp(),
        };

        let result = store.add_tool_category(category);
        assert!(result.is_ok());

        // 获取单个类别
        let fetched = store.get_tool_category(1).unwrap().unwrap();
        assert_eq!(fetched.name, "测试工具类别");

        // 获取所有类别
        let all_categories = store.get_all_tool_categories().unwrap();
        assert_eq!(all_categories.len(), 1);

        // 更新类别
        let updated_category = ToolCategory {
            id: 1,
            name: "更新后的工具类别".to_string(),
            created_at: fetched.created_at,
        };

        let update_result = store.update_tool_category(updated_category);
        assert!(update_result.is_ok());

        let fetched_after_update = store.get_tool_category(1).unwrap().unwrap();
        assert_eq!(fetched_after_update.name, "更新后的工具类别");

        // 删除类别
        let delete_result = store.delete_tool_category(1);
        assert!(delete_result.is_ok());
        assert!(store.get_tool_category(1).unwrap().is_none());
    }

    #[test]
    fn test_tool_crud() {
        // 创建临时测试目录
        let temp_dir = tempdir().unwrap();
        let store = Store::open(temp_dir.path()).unwrap();

        // 先添加一个类别
        let category = ToolCategory {
            id: 1,
            name: "测试工具类别".to_string(),
            created_at: Utc::now().timestamp(),
        };
        store.add_tool_category(category).unwrap();

        // 添加工具
        let tool = Tool {
            id: 1,
            category_id: 1,
            icon_id: Some(10),
            name: "测试工具".to_string(),
            description: "这是一个测试工具".to_string(),
            data: ToolData::JavsScript(ToolJavaScript {
                param: Some(vec![Param {
                    name: "param1".to_string(),
                    param_type: "string".to_string(),
                    description: "参数1".to_string(),
                    required: true,
                    test_value: Some("test".to_string()),
                }]),
                code: "function test() { return 'test'; }".to_string(),
            }),
            created_at: Utc::now().timestamp(),
            updated_at: None,
        };

        let result = store.add_tool(tool);
        assert!(result.is_ok());

        // 获取单个工具
        let fetched = store.get_tool(1).unwrap().unwrap();
        assert_eq!(fetched.name, "测试工具");

        // 获取所有工具
        let all_tools = store.get_all_tools().unwrap();
        assert_eq!(all_tools.len(), 1);

        // 按类别获取工具
        let tools_by_category = store.get_tools_by_category(1).unwrap();
        assert_eq!(tools_by_category.len(), 1);
        assert_eq!(tools_by_category[0].category_id, 1);

        // 更新工具
        let mut updated_tool = fetched.clone();
        updated_tool.name = "更新后的工具".to_string();
        updated_tool.updated_at = Some(Utc::now().timestamp());

        let update_result = store.update_tool(updated_tool);
        assert!(update_result.is_ok());

        let fetched_after_update = store.get_tool(1).unwrap().unwrap();
        assert_eq!(fetched_after_update.name, "更新后的工具");
        assert!(fetched_after_update.updated_at.is_some());

        // 删除工具
        let delete_result = store.delete_tool(1);
        assert!(delete_result.is_ok());
        assert!(store.get_tool(1).unwrap().is_none());
    }

    #[test]
    fn test_tool_category_relationship() {
        // 创建临时测试目录
        let temp_dir = tempdir().unwrap();
        let store = Store::open(temp_dir.path()).unwrap();

        // 添加两个类别
        let category1 =
            ToolCategory { id: 1, name: "类别1".to_string(), created_at: Utc::now().timestamp() };

        let category2 =
            ToolCategory { id: 2, name: "类别2".to_string(), created_at: Utc::now().timestamp() };

        store.add_tool_category(category1).unwrap();
        store.add_tool_category(category2).unwrap();

        // 添加工具
        let tool1 = Tool {
            id: 1,
            category_id: 1,
            icon_id: None,
            name: "工具1".to_string(),
            description: "工具1描述".to_string(),
            data: ToolData::JavsScript(ToolJavaScript {
                param: None,
                code: "function test() {}".to_string(),
            }),
            created_at: Utc::now().timestamp(),
            updated_at: None,
        };

        let tool2 = Tool {
            id: 2,
            category_id: 1,
            icon_id: None,
            name: "工具2".to_string(),
            description: "工具2描述".to_string(),
            data: ToolData::McpSse(ToolMcpSse {
                url: "http://example.com".to_string(),
                tools: vec![McpTool {
                    name: "工具2".to_string(),
                    description: "工具2描述".to_string(),
                    input_schema: Some(serde_json::json!({
                        "type": "object",
                        "properties": {
                            "param1": { "type": "string" },
                            "param2": { "type": "number" }
                        }
                    })),
                }],
            }),
            created_at: Utc::now().timestamp(),
            updated_at: None,
        };

        let tool3 = Tool {
            id: 3,
            category_id: 2,
            icon_id: None,
            name: "工具3".to_string(),
            description: "工具3描述".to_string(),
            data: ToolData::JavsScript(ToolJavaScript {
                param: None,
                code: "function test() {}".to_string(),
            }),
            created_at: Utc::now().timestamp(),
            updated_at: None,
        };

        store.add_tool(tool1).unwrap();
        store.add_tool(tool2).unwrap();
        store.add_tool(tool3).unwrap();

        // 测试按类别获取工具
        let category1_tools = store.get_tools_by_category(1).unwrap();
        assert_eq!(category1_tools.len(), 2);

        let category2_tools = store.get_tools_by_category(2).unwrap();
        assert_eq!(category2_tools.len(), 1);
        assert_eq!(category2_tools[0].name, "工具3");

        // 测试删除分类关联的工具
        store.delete_tool_by_category(1).unwrap();

        // 验证类别1的工具被删除
        let empty_tools = store.get_tools_by_category(1).unwrap();
        assert_eq!(empty_tools.len(), 0);

        // 类别2的工具仍然存在
        let remaining_tools = store.get_tools_by_category(2).unwrap();
        assert_eq!(remaining_tools.len(), 1);
    }

    #[test]
    fn serial_test() {
        let tool2 = Tool {
            id: 2,
            category_id: 1,
            icon_id: None,
            name: "工具2".to_string(),
            description: "工具2描述".to_string(),
            data: ToolData::McpSse(ToolMcpSse {
                url: "http://example.com".to_string(),
                tools: vec![McpTool {
                    name: "工具2".to_string(),
                    description: "工具2描述".to_string(),
                    input_schema: Some(serde_json::json!({
                        "type": "object",
                        "properties": {
                            "param1": { "type": "string" },
                            "param2": { "type": "number" }
                        }
                    })),
                }],
            }),
            created_at: Utc::now().timestamp(),
            updated_at: None,
        };
        let serialized = serde_json::to_string(&tool2).unwrap();
        println!("Serialized Tool: {}", serialized);

        let deserialized: Tool = serde_json::from_str(&serialized).unwrap();
        println!("Deserialized Tool: {:?}", deserialized);
    }
}
