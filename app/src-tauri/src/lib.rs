mod api;
mod error;
mod openai;

use std::{collections::HashMap, sync::Arc};

use openai::tool::ToolObject;
use store::Search;
use tauri::Manager;
// use tauri_plugin_updater::UpdaterExt;
use tauri_plugin_window_state::{AppHandleExt, StateFlags};
use tokio::sync::RwLock;

struct AppState {
    causal_dir: String,
    store: store::Store,
    tasks: api::event::MessageTasks,

    providers: RwLock<HashMap<u64, Arc<store::Provider>>>,
    agents: RwLock<HashMap<u64, Arc<store::Agent>>>,
    tools: RwLock<HashMap<u64, Arc<Box<dyn ToolObject>>>>,
    search: RwLock<Option<Arc<Box<dyn ToolObject>>>>,
}

impl AppState {
    async fn get_provider(&self, id: u64) -> Result<Arc<store::Provider>, error::Error> {
        if let Some(provider) = self.providers.read().await.get(&id) {
            return Ok(provider.clone());
        }
        let provider = self
            .store
            .get_provider(id)?
            .ok_or(error::Error::InvalidData(format!("Provider with id {} not found", id)))?;
        let provider = Arc::new(provider);
        self.providers.write().await.insert(id, provider.clone());
        Ok(provider)
    }

    async fn get_agent(&self, id: u64) -> Result<Arc<store::Agent>, error::Error> {
        if let Some(agent) = self.agents.read().await.get(&id) {
            return Ok(agent.clone());
        }
        let agent = self
            .store
            .get_agent(id)?
            .ok_or(error::Error::InvalidData(format!("Agent with id {} not found", id)))?;
        let agent = Arc::new(agent);
        self.agents.write().await.insert(id, agent.clone());
        Ok(agent)
    }

    pub async fn get_tool_object(&self, id: u64) -> Result<Arc<Box<dyn ToolObject>>, error::Error> {
        if let Some(tool) = self.tools.read().await.get(&id) {
            return Ok(tool.clone());
        }
        let tool = self
            .store
            .get_tool(id)?
            .ok_or(error::Error::InvalidData(format!("Tool with id {} not found", id)))?;
        let tool_object = openai::tool::Tool::new(tool).into_tool_object().await?;
        let tool_object = Arc::new(tool_object);
        self.tools.write().await.insert(id, tool_object.clone());
        Ok(tool_object)
    }

    pub async fn get_search_tool_object(
        &self, search: Search,
    ) -> Result<Arc<Box<dyn ToolObject>>, error::Error> {
        if let Some(search) = self.search.read().await.as_ref() {
            return Ok(search.clone());
        }
        let search = openai::tool::Search::new(search).into_tool_object()?;
        let search = Arc::new(search);
        self.search.write().await.replace(search.clone());
        Ok(search)
    }
}

#[tauri::command]
async fn app_name() -> String {
    "Causal AI".to_string()
}

#[tauri::command]
async fn app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
async fn app_date() -> String {
    env!("COMPILE_TIME").to_string()
}

#[tauri::command]
async fn fetch(
    app: tauri::State<'_, AppState>, name: String, data: String,
) -> Result<serde_json::Value, serde_json::Value> {
    api::fetch::ftech(app, &name, &data).await.map_err(|e| {
        tracing::error!("fetch error: {}", e.to_string());
        e.into()
    })
}

#[tauri::command]
async fn event(
    app: tauri::State<'_, AppState>, message: store::ChatMessage, search: bool, time: bool,
    stream: bool, on_event: tauri::ipc::Channel<openai::chat::MessageEvent>,
) -> Result<serde_json::Value, serde_json::Value> {
    api::event::event(app, message, search, time, stream, on_event).await.map_err(|e| {
        tracing::error!("event error: {}", e.to_string());
        e.into()
    })
}

#[tauri::command]
async fn event_exit(
    app: tauri::State<'_, AppState>, message: u64,
) -> Result<serde_json::Value, serde_json::Value> {
    let mut tasks = app.tasks.write().await;
    if let Some(task) = tasks.remove(&message) {
        let _ = task.exit.send(());
    }
    Ok(serde_json::json!({
        "status": "success"
    }))
}

// async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
//     tracing::info!("checking for updates");

//     // Check for updates
//     if let Some(update) = app.updater()?.check().await? {
//         let mut downloaded = 0;

//         // alternatively we could also call update.download() and update.install() separately
//         update
//             .download_and_install(
//                 |chunk_length, content_length| {
//                     downloaded += chunk_length;
//                     tracing::info!("downloaded {downloaded} from {content_length:?}");
//                 },
//                 || {
//                     tracing::info!("download finished");
//                 },
//             )
//             .await?;

//         tracing::info!("update installed");
//         app.restart();
//     }
//     Ok(())
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    // 获取当前用户的目录, 拼接.causal, 如果没有则创建
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| {
        std::env::var("USERPROFILE").unwrap_or_else(|_| std::env::var("HOMEPATH").unwrap())
    });

    let causal_dir = format!("{}/.causal", home_dir);
    if !std::path::Path::new(&causal_dir).exists() {
        std::fs::create_dir_all(&causal_dir).unwrap();
    }

    // tracing日志
    tracing_subscriber::fmt()
        .with_writer(std::fs::File::create(format!("{}/run.log", causal_dir)).unwrap())
        .with_max_level(tracing::Level::INFO)
        .init();

    let app = AppState {
        causal_dir: causal_dir.clone(),
        store: store::Store::open(format!("{}/store", causal_dir)).unwrap(),
        tasks: api::event::MessageTasks::default(),

        providers: RwLock::new(HashMap::new()),
        agents: RwLock::new(HashMap::new()),
        tools: RwLock::new(HashMap::new()),
        search: RwLock::new(None),
    };

    tauri::Builder::default()
        // .setup(|app| {
        //     let handle = app.handle().clone();
        //     tauri::async_runtime::spawn(async move {
        //         update(handle).await.unwrap();
        //     });
        //     Ok(())
        // })
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            app_name,
            app_version,
            app_date,
            fetch,
            event,
            event_exit
        ])
        .manage(app)
        .on_window_event(|window, event| {
            if matches!(event, tauri::WindowEvent::Destroyed) {
                let _ = window.app_handle().save_window_state(StateFlags::all());
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
