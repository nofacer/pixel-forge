mod graph;
mod shaders;
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

#[tauri::command]
async fn sync_graph(
    state: tauri::State<'_, AppState>,
    graph_json: String,
) -> Result<Vec<u8>, String> {
    let graph: graph::Graph = serde_json::from_str(&graph_json).map_err(|e| e.to_string())?;

    println!("Received graph with {} nodes", graph.nodes.len());

    let result_bytes = state.render(graph).await?;

    Ok(result_bytes)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![greet, init_wgpu, sync_graph])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
