#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use code_chrono_lib::{Database, get_invoke_handler};
use std::sync::{Arc, Mutex};
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let db = {
                let app_data_dir = app.path().app_data_dir()
                    .expect("failed to resolve app data dir");
                std::fs::create_dir_all(&app_data_dir)
                    .expect("failed to create app data dir");
                let db_path = app_data_dir.join("code_chrono.db");
                Arc::new(Mutex::new(
                    Database::new(db_path.to_str().unwrap()).unwrap()
                ))
            };
            app.manage(db);
            Ok(())
        })
        .manage(Arc::new(Mutex::new(code_chrono_lib::commands::TimerState::default())))  // Use lib
        .invoke_handler(get_invoke_handler())
        .run(tauri::generate_context!())
        .expect("error running Code Chrono");
}