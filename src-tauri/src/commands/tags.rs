// commands/tags.rs — Tag management

use std::sync::{Arc, Mutex};
use tauri::State;

use crate::database::{Database, Tag};

#[tauri::command]
pub async fn create_tag(
    db_state: State<'_, Arc<Mutex<Database>>>,
    name: String,
    color: Option<String>,
) -> Result<i64, String> {
    db_state.lock().unwrap().create_tag(&name, color.as_deref()).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_tags(
    db_state: State<'_, Arc<Mutex<Database>>>,
) -> Result<Vec<Tag>, String> {
    db_state.lock().unwrap().get_tags().map_err(|e| e.to_string())
}

/// Deletes a tag. The tag is automatically removed from all tasks
/// via the SQLite ON DELETE CASCADE on task_tags.
#[tauri::command]
pub async fn delete_tag(
    db_state: State<'_, Arc<Mutex<Database>>>,
    id: i64,
) -> Result<(), String> {
    db_state.lock().unwrap().delete_tag(id).map_err(|e| e.to_string())
}
