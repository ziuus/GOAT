// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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

#[tauri::command]
fn start_daemon() -> Result<String, String> {
    use std::process::Command;
    
    // Spawn goat daemon start in the background safely
    // Not using shell, just invoking binary
    match Command::new("goat")
        .arg("daemon")
        .arg("start")
        .spawn() 
    {
        Ok(child) => {
            Ok(format!("Daemon started with PID {}", child.id()))
        }
        Err(e) => {
            Err(format!("Failed to start daemon: {}. Try running `goat daemon start` manually.", e))
        }
    }
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            // Scaffold Tray menu
            #[cfg(desktop)]
            {
                use tauri::menu::{Menu, MenuItem};
                use tauri::tray::TrayIconBuilder;

                // Simple scaffold for menu items
                let show_i = MenuItem::with_id(app, "show", "Show GOAT", true, None::<&str>)?;
                let status_i = MenuItem::with_id(app, "status", "Daemon Status", true, None::<&str>)?;
                let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
                
                let menu = Menu::with_items(app, &[&show_i, &status_i, &quit_i])?;

                let _tray = TrayIconBuilder::new()
                    .menu(&menu)
                    .on_menu_event(|app, event| {
                        match event.id.as_ref() {
                            "quit" => {
                                std::process::exit(0);
                            }
                            "show" => {
                                if let Some(window) = app.get_webview_window("main") {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                }
                            }
                            _ => {}
                        }
                    })
                    .build(app)?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_app_version,
            get_daemon_status,
            get_daemon_token_path,
            get_default_api_url,
            start_daemon
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
