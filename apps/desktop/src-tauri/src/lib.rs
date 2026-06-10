// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_app_version,
            get_daemon_status,
            get_daemon_token_path,
            get_default_api_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_app_version() -> String {
    "0.13.0".to_string()
}

#[tauri::command]
fn get_default_api_url() -> String {
    "http://127.0.0.1:47647".to_string()
}

#[tauri::command]
fn get_daemon_token_path() -> String {
    // Return standard token path based on platform
    let mut path = dirs::data_dir().unwrap_or_else(|| std::path::PathBuf::from("~/.local/share"));
    path.push("goat");
    path.push("daemon.token");
    path.to_string_lossy().into_owned()
}

#[tauri::command]
fn get_daemon_status() -> bool {
    // Check if daemon is responding
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_millis(500))
        .build()
        .unwrap();
        
    let res = client.get("http://127.0.0.1:47647/v1/status").send();
    res.is_ok()
}
