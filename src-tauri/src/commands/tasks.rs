// commands/tasks.rs — Task CRUD operations

use std::sync::{Arc, Mutex};
use tauri::State;

use crate::database::{Database, Task};

/// Create a new task and return its generated id.
#[tauri::command]
pub async fn create_task(
    db_state: State<'_, Arc<Mutex<Database>>>,
    task: Task,
) -> Result<i64, String> {
    db_state.lock().unwrap().create_task(task).map_err(|e| e.to_string())
}

/// Overwrite all mutable fields of an existing task.
/// `completed_at` is handled in the database layer to preserve the original
/// completion timestamp when a task is re-edited without re-completing it.
#[tauri::command]
pub async fn update_task(
    db_state: State<'_, Arc<Mutex<Database>>>,
    task: Task,
) -> Result<(), String> {
    db_state.lock().unwrap().update_task(task).map_err(|e| e.to_string())
}

/// Delete a task by id.
#[tauri::command]
pub async fn delete_task(
    db_state: State<'_, Arc<Mutex<Database>>>,
    id: i64,
) -> Result<(), String> {
    db_state.lock().unwrap().delete_task(id).map_err(|e| e.to_string())
}

/// Return tasks with optional project / tag / status filters.
#[tauri::command]
pub async fn get_tasks(
    db_state:      State<'_, Arc<Mutex<Database>>>,
    filter_project: Option<i64>,
    filter_tag:     Option<i64>,
    filter_status:  Option<String>,
) -> Result<Vec<Task>, String> {
    db_state
        .lock()
        .unwrap()
        .get_tasks(filter_project, filter_tag, filter_status)
        .map_err(|e| e.to_string())
}

/// Full-text search across task titles and descriptions.
#[tauri::command]
pub async fn search_tasks(
    db_state: State<'_, Arc<Mutex<Database>>>,
    query: String,
) -> Result<Vec<Task>, String> {
    db_state.lock().unwrap().search_tasks(&query).map_err(|e| e.to_string())
}
