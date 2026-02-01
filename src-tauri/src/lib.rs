pub mod commands;
pub mod database;

pub use commands::api::{start_pomodoro, pause_timer, get_timer, reset_timer, export_csv, import_csv, reset_database, get_unique_task_names};
pub use database::Database;

use tauri::ipc::Invoke;

pub fn get_invoke_handler() -> impl Fn(Invoke) -> bool + Send + Sync + 'static {
    tauri::generate_handler![start_pomodoro, pause_timer, get_timer, reset_timer, export_csv, import_csv, reset_database, get_unique_task_names]
}