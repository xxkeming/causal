mod error;

fn ftech(
    store: tauri::State<'_, store::Store>, name: &str, data: &str,
) -> Result<serde_json::Value, error::Error> {
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
