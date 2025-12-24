mod graph;
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
) -> Result<String, String> {
    let graph: graph::Graph = serde_json::from_str(&graph_json).map_err(|e| e.to_string())?;

    println!("Received graph with {} nodes", graph.nodes.len());

    let result_bytes = state.render(graph).await?;
    println!("Rendered Result Bytes Length: {}", result_bytes.len());

    // For MVP debug: Print the first pixel's color
    if result_bytes.len() >= 4 {
        println!(
            "Rendered First Pixel: R={} G={} B={} A={}",
            result_bytes[0], result_bytes[1], result_bytes[2], result_bytes[3]
        );
    }

    Ok("Rendered successfully".to_string())
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
