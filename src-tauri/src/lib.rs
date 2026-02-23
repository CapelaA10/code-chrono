// lib.rs — Application wiring
//
// This file registers all backend modules and exposes the Tauri commands
// to the frontend. Business logic lives in the sub-modules, not here.

pub mod commands;
pub mod database;
pub mod integrations;

use commands::timer::TimerState;
use database::Database;
use std::sync::{Arc, Mutex};
use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        // ── Plugins ────────────────────────────────────────────────────────
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        // ── Shared state ───────────────────────────────────────────────────
        .setup(|app| {
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to resolve app data directory");
            std::fs::create_dir_all(&app_dir).ok();
            let db_path = app_dir.join("code_chrono.db");

            let db = Database::new(db_path.to_str().unwrap())
                .expect("Failed to open database");

            app.manage(Arc::new(Mutex::new(db)));
            app.manage(Arc::new(Mutex::new(TimerState::default())));
            Ok(())
        })
        // ── Tauri commands (sub-module paths required by generate_handler!) ─
        .invoke_handler(tauri::generate_handler![
            // Timer
            commands::timer::start_pomodoro,
            commands::timer::start_break,
            commands::timer::pause_timer,
            commands::timer::get_timer,
            commands::timer::reset_timer,
            commands::timer::record_activity,
            commands::timer::get_unique_task_names,
            // Tasks
            commands::tasks::create_task,
            commands::tasks::update_task,
            commands::tasks::delete_task,
            commands::tasks::get_tasks,
            commands::tasks::search_tasks,
            // Projects
            commands::projects::create_project,
            commands::projects::get_projects,
            commands::projects::delete_project,
            // Tags
            commands::tags::create_tag,
            commands::tags::get_tags,
            commands::tags::delete_tag,
            // Statistics
            commands::stats::get_task_stats,
            commands::stats::get_daily_breakdown,
            // Settings
            commands::settings::get_setting,
            commands::settings::set_setting,
            // Data
            commands::data::export_csv,
            commands::data::import_csv,
            commands::data::reset_database,
            // Integrations — legacy one-shot sync
            commands::sync::sync_github,
            commands::sync::sync_jira,
            commands::sync::sync_gitlab,
            // Integrations — selective import (preview → pick → import)
            commands::sync::preview_sync_github,
            commands::sync::preview_sync_jira,
            commands::sync::preview_sync_gitlab,
            commands::sync::import_selected,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running application");
}
