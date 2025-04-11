mod api;
mod error;

#[tauri::command]
async fn fetch(
    store: tauri::State<'_, store::Store>, name: String, data: String,
) -> Result<serde_json::Value, serde_json::Value> {
    api::fetch::ftech(store, &name, &data).map_err(|e| e.into())
}

#[tauri::command]
async fn event(
    store: tauri::State<'_, store::Store>, agent: u64, session: u64, message: u64, content: String,
    on_event: tauri::ipc::Channel<api::event::MessageEvent>,
) -> Result<serde_json::Value, serde_json::Value> {
    api::event::event(store, agent, session, message, content, on_event).await.map_err(|e| e.into())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let store = store::Store::open("store-tmp").unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![fetch, event])
        .manage(store)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
