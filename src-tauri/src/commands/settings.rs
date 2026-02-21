// commands/settings.rs — Persistent key-value app settings (stored in SQLite)

use std::sync::{Arc, Mutex};
use tauri::State;

use crate::database::Database;

/// Retrieve a setting value by key. Returns `None` if the key doesn't exist yet.
#[tauri::command]
pub fn get_setting(
    db_state: State<'_, Arc<Mutex<Database>>>,
    key: String,
) -> Result<Option<String>, String> {
    db_state.lock().unwrap().get_setting(&key).map_err(|e| e.to_string())
}

/// Upsert a setting value.
#[tauri::command]
pub async fn set_setting(
    db_state: State<'_, Arc<Mutex<Database>>>,
    key: String,
    value: String,
) -> Result<(), String> {
    db_state.lock().unwrap().set_setting(&key, &value).map_err(|e| e.to_string())
}
