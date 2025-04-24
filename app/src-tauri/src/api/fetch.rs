use crate::error;

pub async fn ftech(
    store: tauri::State<'_, store::Store>, name: &str, data: &str,
) -> Result<serde_json::Value, error::Error> {
    // println!("fetch, {} {}", name, data);

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
        "search.set" => {
            let search: store::Search = serde_json::from_str(data)?;
            store.set_search(search)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "search.get" => {
            let search = store.get_search()?;
            let search: serde_json::Value = serde_json::to_value(search)?;
            Ok(serde_json::json!({ "status": "success", "data": search }))
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
        "tool.mcp.sse.tools" => {
            let url: String = serde_json::from_str(data)?;
            match tools::McpSseTool::new(url).description().await {
                Ok(description) => {
                    Ok(serde_json::json!({ "status": "success", "data": description }))
                }
                Err(e) => Ok(serde_json::json!({ "status": "error", "error": e.to_string() })),
            }
        }
        "chat.session.add" => {
            let session: store::ChatSession = serde_json::from_str(data)?;
            let session = store.add_chat_session(session)?;
            Ok(serde_json::json!({ "status": "success", "data": session }))
        }
        "chat.session.update" => {
            let session: store::ChatSession = serde_json::from_str(data)?;
            store.update_chat_session(session)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "chat.session.delete" => {
            let id: u64 = serde_json::from_str(data)?;
            store.delete_chat_session(id)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "chat.session.get" => {
            let id: u64 = serde_json::from_str(data)?;
            let session = store.get_chat_session(id)?;
            match session {
                Some(session) => {
                    let parsed_data: serde_json::Value = serde_json::to_value(session)?;
                    Ok(serde_json::json!({ "status": "success", "data": parsed_data }))
                }
                None => {
                    Ok(serde_json::json!({ "status": "error", "error": "Chat session not found" }))
                }
            }
        }
        "chat.session.list" => {
            let list = store.get_all_chat_sessions()?;
            let parsed_data: serde_json::Value = serde_json::to_value(list)?;
            Ok(serde_json::json!({ "status": "success", "data": parsed_data }))
        }
        "chat.message.add" => {
            let message: store::ChatMessage = serde_json::from_str(data)?;
            let message = store.add_chat_message(message)?;
            Ok(serde_json::json!({ "status": "success", "data": message }))
        }
        "chat.message.update" => {
            let message: store::ChatMessage = serde_json::from_str(data)?;
            store.update_chat_message(message)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "chat.message.delete" => {
            let id: u64 = serde_json::from_str(data)?;
            store.delete_chat_message(id)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "chat.message.list.by.session" => {
            let id: u64 = serde_json::from_str(data)?;
            let list = store.get_messages_by_session(id)?;
            let parsed_data: serde_json::Value = serde_json::to_value(list)?;
            Ok(serde_json::json!({ "status": "success", "data": parsed_data }))
        }
        "chat.message.delete.by.session" => {
            let id: u64 = serde_json::from_str(data)?;
            store.delete_messages_by_session(id)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "file.convert" => {
            #[derive(serde::Deserialize)]
            struct Convert {
                name: String,
                data: String,
            }

            let convert = serde_json::from_str::<Convert>(data)?;
            // println!("convert name: {} data: {}", convert.name, convert.data.len());

            if let Some((title, data)) =
                document::loader_from_data_base64(convert.name, convert.data)
            {
                println!("convert title: {:?} data: {}", title, data.len());
                if data.len() > 0 {
                    return Ok(serde_json::json!({
                        "status": "success",
                        "data": data,
                    }));
                }
            }

            return Ok(serde_json::json!({
                "status": "error",
                "message": "convert failed",
            }));
        }
        _ => Err(error::Error::Unknown),
    }
}
