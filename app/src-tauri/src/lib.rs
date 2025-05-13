mod api;
mod error;
mod openai;

use tauri::Manager;
use tauri_plugin_window_state::{AppHandleExt, StateFlags};

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
    store: tauri::State<'_, store::Store>, name: String, data: String,
) -> Result<serde_json::Value, serde_json::Value> {
    api::fetch::ftech(store, &name, &data).await.map_err(|e| {
        tracing::error!("fetch error: {}", e.to_string());
        e.into()
    })
}

#[tauri::command]
async fn event(
    tasks: tauri::State<'_, api::event::MessageTasks>, store: tauri::State<'_, store::Store>,
    message: store::ChatMessage, search: bool, time: bool, stream: bool,
    on_event: tauri::ipc::Channel<openai::chat::MessageEvent>,
) -> Result<serde_json::Value, serde_json::Value> {
    api::event::event(tasks, store, message, search, time, stream, on_event).await.map_err(|e| {
        tracing::error!("event error: {}", e.to_string());
        e.into()
    })
}

#[tauri::command]
async fn event_exit(
    tasks: tauri::State<'_, api::event::MessageTasks>, message: u64,
) -> Result<serde_json::Value, serde_json::Value> {
    let mut tasks = tasks.write().await;
    if let Some(task) = tasks.remove(&message) {
        let _ = task.exit.send(());
    }
    Ok(serde_json::json!({
        "status": "success"
    }))
}

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

    // 打开数据库
    let store = store::Store::open(format!("{}/store", causal_dir)).unwrap();

    // 任务列表
    let tasks = api::event::MessageTasks::default();

    tauri::Builder::default()
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
        .manage(store)
        .manage(tasks)
        // .manage(causal_dir)
        .on_window_event(|window, event| {
            if matches!(event, tauri::WindowEvent::Destroyed) {
                let _ = window.app_handle().save_window_state(StateFlags::all());
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
