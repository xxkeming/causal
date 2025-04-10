mod error;

fn ftech(
    store: tauri::State<'_, store::Store>, name: &str, data: &str,
) -> Result<serde_json::Value, error::Error> {
    println!("fetch, {} {}", name, data);

    match name {
        "provider.add" => {
            let provider: store::Provider = serde_json::from_str(data)?;
            store.add_provider(provider)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "provider.update" => {
            let provider: store::Provider = serde_json::from_str(data)?;
            store.update_provider(provider)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "provider.delete" => {
            let id: u64 = serde_json::from_str(data)?;
            store.delete_provider(id)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "provider.get" => {
            let id: u64 = serde_json::from_str(data)?;
            let provider = store.get_provider(id)?;
            match provider {
                Some(provider) => {
                    let parsed_data: serde_json::Value = serde_json::to_value(provider)?;
                    Ok(serde_json::json!({ "status": "success", "data": parsed_data }))
                }
                None => Ok(serde_json::json!({ "status": "error", "error": "Provider not found" })),
            }
        }
        "provider.list" => {
            let list = store.get_all_providers()?;
            let parsed_data: serde_json::Value = serde_json::to_value(list)?;
            Ok(serde_json::json!({ "status": "success", "data": parsed_data }))
        }
        "agent.add" => {
            let agent: store::Agent = serde_json::from_str(data)?;
            store.add_agent(agent)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "agent.update" => {
            let agent: store::Agent = serde_json::from_str(data)?;
            store.update_agent(agent)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "agent.delete" => {
            let id: u64 = serde_json::from_str(data)?;
            store.delete_agent(id)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "agent.delete.by.category" => {
            let id: u64 = serde_json::from_str(data)?;
            store.delete_agent_by_category(id)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "agent.get" => {
            let id: u64 = serde_json::from_str(data)?;
            let agent = store.get_agent(id)?;
            match agent {
                Some(agent) => {
                    let parsed_data: serde_json::Value = serde_json::to_value(agent)?;
                    Ok(serde_json::json!({ "status": "success", "data": parsed_data }))
                }
                None => Ok(serde_json::json!({ "status": "error", "error": "Agent not found" })),
            }
        }
        "agent.list" => {
            let list = store.get_all_agents()?;
            let parsed_data: serde_json::Value = serde_json::to_value(list)?;
            Ok(serde_json::json!({ "status": "success", "data": parsed_data }))
        }
        "agent.category.list" => {
            let list = store.get_all_agent_categories()?;
            let parsed_data: serde_json::Value = serde_json::to_value(list)?;
            Ok(serde_json::json!({ "status": "success", "data": parsed_data }))
        }
        "agent.category.get" => {
            let id: u64 = serde_json::from_str(data)?;
            let category = store.get_agent_category(id)?;
            match category {
                Some(category) => {
                    let parsed_data: serde_json::Value = serde_json::to_value(category)?;
                    Ok(serde_json::json!({ "status": "success", "data": parsed_data }))
                }
                None => Ok(
                    serde_json::json!({ "status": "error", "error": "Agent category not found" }),
                ),
            }
        }
        "agent.category.add" => {
            let category: store::AgentCategory = serde_json::from_str(data)?;
            store.add_agent_category(category)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "agent.category.update" => {
            let category: store::AgentCategory = serde_json::from_str(data)?;
            store.update_agent_category(category)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "agent.category.delete" => {
            let id: u64 = serde_json::from_str(data)?;
            store.delete_agent_category(id)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "tool.add" => {
            let tool: store::Tool = serde_json::from_str(data)?;
            store.add_tool(tool)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "tool.update" => {
            let tool: store::Tool = serde_json::from_str(data)?;
            store.update_tool(tool)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "tool.delete" => {
            let id: u64 = serde_json::from_str(data)?;
            store.delete_tool(id)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "tool.delete.by.category" => {
            let id: u64 = serde_json::from_str(data)?;
            store.delete_tool_by_category(id)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "tool.get" => {
            let id: u64 = serde_json::from_str(data)?;
            let tool = store.get_tool(id)?;
            match tool {
                Some(tool) => {
                    let parsed_data: serde_json::Value = serde_json::to_value(tool)?;
                    Ok(serde_json::json!({ "status": "success", "data": parsed_data }))
                }
                None => Ok(serde_json::json!({ "status": "error", "error": "Tool not found" })),
            }
        }
        "tool.list" => {
            let list = store.get_all_tools()?;
            let parsed_data: serde_json::Value = serde_json::to_value(list)?;
            Ok(serde_json::json!({ "status": "success", "data": parsed_data }))
        }
        "tool.category.list" => {
            let list = store.get_all_tool_categories()?;
            let parsed_data: serde_json::Value = serde_json::to_value(list)?;
            Ok(serde_json::json!({ "status": "success", "data": parsed_data }))
        }
        "tool.category.get" => {
            let id: u64 = serde_json::from_str(data)?;
            let category = store.get_tool_category(id)?;
            match category {
                Some(category) => {
                    let parsed_data: serde_json::Value = serde_json::to_value(category)?;
                    Ok(serde_json::json!({ "status": "success", "data": parsed_data }))
                }
                None => {
                    Ok(serde_json::json!({ "status": "error", "error": "Tool category not found" }))
                }
            }
        }
        "tool.category.add" => {
            let category: store::ToolCategory = serde_json::from_str(data)?;
            store.add_tool_category(category)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "tool.category.update" => {
            let category: store::ToolCategory = serde_json::from_str(data)?;
            store.update_tool_category(category)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "tool.category.delete" => {
            let id: u64 = serde_json::from_str(data)?;
            store.delete_tool_category(id)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        _ => Err(error::Error::Unknown),
    }
}

#[tauri::command]
fn fetch_local(
    store: tauri::State<'_, store::Store>, name: &str, data: &str,
) -> serde_json::value::Value {
    println!("Hello, {}!", name);

    match ftech(store, name, data) {
        Ok(value) => value,
        Err(e) => {
            let error_value: serde_json::Value = e.into();
            error_value
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let store = store::Store::open("store-tmp").unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![fetch_local])
        .manage(store)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
