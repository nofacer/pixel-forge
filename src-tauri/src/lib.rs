mod state;
use state::AppState;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn init_wgpu(state: tauri::State<'_, AppState>) -> Result<String, String> {
    state.initialize().await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![greet, init_wgpu])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
