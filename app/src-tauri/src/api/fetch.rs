use crate::openai::tool::{McpTool, ToolObject};
use crate::{AppState, error};

pub async fn ftech(
    app: tauri::State<'_, AppState>, name: &str, data: &str,
) -> Result<serde_json::Value, error::Error> {
    // println!("fetch, {} {}", name, data);

    match name {
        "provider.add" => {
            let provider: store::Provider = serde_json::from_str(data)?;
            app.store.add_provider(provider)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "provider.update" => {
            let provider: store::Provider = serde_json::from_str(data)?;
            let _ = app.providers.write().await.remove(&provider.id);
            app.store.update_provider(provider)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "provider.delete" => {
            let id: u64 = serde_json::from_str(data)?;
            let _ = app.providers.write().await.remove(&id);
            app.store.delete_provider(id)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "provider.get" => {
            let id: u64 = serde_json::from_str(data)?;
            let provider = app
                .store
                .get_provider(id)?
                .ok_or(error::Error::InvalidData("Provider not found".to_string()))?;

            let parsed_data: serde_json::Value = serde_json::to_value(provider)?;
            Ok(serde_json::json!({ "status": "success", "data": parsed_data }))
        }
        "provider.list" => {
            let list = app.store.get_all_providers()?;
            let parsed_data: serde_json::Value = serde_json::to_value(list)?;
            Ok(serde_json::json!({ "status": "success", "data": parsed_data }))
        }
        "search.set" => {
            let search: store::Search = serde_json::from_str(data)?;
            let _ = app.search.write().await.take();
            app.store.set_search(search)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "search.get" => {
            let search = app.store.get_search()?;
            let search: serde_json::Value = serde_json::to_value(search)?;
            Ok(serde_json::json!({ "status": "success", "data": search }))
        }
        "agent.add" => {
            let agent: store::Agent = serde_json::from_str(data)?;
            app.store.add_agent(agent)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "agent.update" => {
            let agent: store::Agent = serde_json::from_str(data)?;
            let _ = app.agents.write().await.remove(&agent.id);
            app.store.update_agent(agent)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "agent.delete" => {
            let id: u64 = serde_json::from_str(data)?;
            let _ = app.agents.write().await.remove(&id);
            app.store.delete_agent(id)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "agent.delete.by.category" => {
            let id: u64 = serde_json::from_str(data)?;
            app.store.delete_agent_by_category(id)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "agent.get" => {
            let id: u64 = serde_json::from_str(data)?;
            let agent = app
                .store
                .get_agent(id)?
                .ok_or(error::Error::InvalidData("Agent not found".to_string()))?;

            let parsed_data: serde_json::Value = serde_json::to_value(agent)?;
            Ok(serde_json::json!({ "status": "success", "data": parsed_data }))
        }
        "agent.list" => {
            let list = app.store.get_all_agents()?;
            let parsed_data: serde_json::Value = serde_json::to_value(list)?;
            Ok(serde_json::json!({ "status": "success", "data": parsed_data }))
        }
        "agent.category.list" => {
            let list = app.store.get_all_agent_categories()?;
            let parsed_data: serde_json::Value = serde_json::to_value(list)?;
            Ok(serde_json::json!({ "status": "success", "data": parsed_data }))
        }
        "agent.category.get" => {
            let id: u64 = serde_json::from_str(data)?;
            let category = app
                .store
                .get_agent_category(id)?
                .ok_or(error::Error::InvalidData("Agent category not found".to_string()))?;

            let parsed_data: serde_json::Value = serde_json::to_value(category)?;
            Ok(serde_json::json!({ "status": "success", "data": parsed_data }))
        }
        "agent.category.add" => {
            let category: store::AgentCategory = serde_json::from_str(data)?;
            app.store.add_agent_category(category)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "agent.category.update" => {
            let category: store::AgentCategory = serde_json::from_str(data)?;
            app.store.update_agent_category(category)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "agent.category.delete" => {
            let id: u64 = serde_json::from_str(data)?;
            app.store.delete_agent_category(id)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "tool.add" => {
            let tool: store::Tool = serde_json::from_str(data)?;
            app.store.add_tool(tool)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "tool.update" => {
            let tool: store::Tool = serde_json::from_str(data)?;
            let _ = app.tools.write().await.remove(&tool.id);
            app.store.update_tool(tool)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "tool.delete" => {
            let id: u64 = serde_json::from_str(data)?;
            let _ = app.tools.write().await.remove(&id);
            app.store.delete_tool(id)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "tool.delete.by.category" => {
            let id: u64 = serde_json::from_str(data)?;
            app.store.delete_tool_by_category(id)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "tool.get" => {
            let id: u64 = serde_json::from_str(data)?;
            let tool = app
                .store
                .get_tool(id)?
                .ok_or(error::Error::InvalidData("Tool not found".to_string()))?;

            let parsed_data: serde_json::Value = serde_json::to_value(tool)?;
            Ok(serde_json::json!({ "status": "success", "data": parsed_data }))
        }
        "tool.list" => {
            let list = app.store.get_all_tools()?;
            let parsed_data: serde_json::Value = serde_json::to_value(list)?;
            Ok(serde_json::json!({ "status": "success", "data": parsed_data }))
        }
        "tool.category.list" => {
            let list = app.store.get_all_tool_categories()?;
            let parsed_data: serde_json::Value = serde_json::to_value(list)?;
            Ok(serde_json::json!({ "status": "success", "data": parsed_data }))
        }
        "tool.category.get" => {
            let id: u64 = serde_json::from_str(data)?;
            let category = app
                .store
                .get_tool_category(id)?
                .ok_or(error::Error::InvalidData("Tool category not found".to_string()))?;

            let parsed_data: serde_json::Value = serde_json::to_value(category)?;
            Ok(serde_json::json!({ "status": "success", "data": parsed_data }))
        }
        "tool.category.add" => {
            let category: store::ToolCategory = serde_json::from_str(data)?;
            app.store.add_tool_category(category)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "tool.category.update" => {
            let category: store::ToolCategory = serde_json::from_str(data)?;
            app.store.update_tool_category(category)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "tool.category.delete" => {
            let id: u64 = serde_json::from_str(data)?;
            app.store.delete_tool_category(id)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "tool.mcp.sse.tools" => {
            let url: String = serde_json::from_str(data)?;
            let tool = McpTool::try_new_sse(url).await?;
            Ok(serde_json::json!({ "status": "success", "data": tool.description() }))
        }
        "tool.mcp.io.tools" => {
            let io: store::ToolMcpIo = serde_json::from_str(data)?;
            let tool = McpTool::try_new_io(io.command, io.args, io.env).await?;
            Ok(serde_json::json!({ "status": "success", "data": tool.description() }))
        }
        "chat.session.add" => {
            let session: store::ChatSession = serde_json::from_str(data)?;
            let session = app.store.add_chat_session(session)?;
            Ok(serde_json::json!({ "status": "success", "data": session }))
        }
        "chat.session.update" => {
            let session: store::ChatSession = serde_json::from_str(data)?;
            app.store.update_chat_session(session)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "chat.session.delete" => {
            let id: u64 = serde_json::from_str(data)?;
            app.store.delete_chat_session(id)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "chat.session.get" => {
            let id: u64 = serde_json::from_str(data)?;
            let session = app
                .store
                .get_chat_session(id)?
                .ok_or(error::Error::InvalidData("Chat session not found".to_string()))?;

            let parsed_data: serde_json::Value = serde_json::to_value(session)?;
            Ok(serde_json::json!({ "status": "success", "data": parsed_data }))
        }
        "chat.session.list" => {
            let list = app.store.get_all_chat_sessions()?;
            let parsed_data: serde_json::Value = serde_json::to_value(list)?;
            Ok(serde_json::json!({ "status": "success", "data": parsed_data }))
        }
        "chat.message.add" => {
            let message: store::ChatMessage = serde_json::from_str(data)?;
            let message = app.store.add_chat_message(message)?;
            Ok(serde_json::json!({ "status": "success", "data": message }))
        }
        "chat.message.update" => {
            let message: store::ChatMessage = serde_json::from_str(data)?;
            app.store.update_chat_message(message)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "chat.message.delete" => {
            let id: u64 = serde_json::from_str(data)?;
            app.store.delete_chat_message(id)?;
            Ok(serde_json::json!({ "status": "success" }))
        }
        "chat.message.list.by.session" => {
            #[derive(serde::Deserialize)]
            struct Options {
                session: u64,
                message: Option<u64>,
                limit: usize,
            }
            let opt: Options = serde_json::from_str(data)?;
            let list = app.store.get_latest_messages_by_session_and_message(
                opt.session,
                opt.message.unwrap_or(u64::MAX),
                opt.limit,
            )?;
            let parsed_data: serde_json::Value = serde_json::to_value(list)?;
            Ok(serde_json::json!({ "status": "success", "data": parsed_data }))
        }
        "chat.message.delete.by.session" => {
            let id: u64 = serde_json::from_str(data)?;
            app.store.delete_messages_by_session(id)?;
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
                document::loader_from_data_base64(&app.causal_dir, convert.name, convert.data)
            {
                tracing::info!("convert title: {:?} data: {}", title, data.len());
                if !data.is_empty() {
                    return Ok(serde_json::json!({
                        "status": "success",
                        "data": data,
                    }));
                }
            }

            Err(error::Error::InvalidData("convert failed".to_string()))
        }
        _ => Err(error::Error::Unknown),
    }
}
