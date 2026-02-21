// commands/projects.rs — Project management

use std::sync::{Arc, Mutex};
use tauri::State;

use crate::database::{Database, Project};

#[tauri::command]
pub async fn create_project(
    db_state: State<'_, Arc<Mutex<Database>>>,
    name: String,
    color: Option<String>,
) -> Result<i64, String> {
    db_state.lock().unwrap().create_project(&name, color.as_deref()).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_projects(
    db_state: State<'_, Arc<Mutex<Database>>>,
) -> Result<Vec<Project>, String> {
    db_state.lock().unwrap().get_projects().map_err(|e| e.to_string())
}

/// Deletes a project. Tasks assigned to it are automatically un-assigned
/// by the SQLite ON DELETE SET NULL constraint on tasks.project_id.
#[tauri::command]
pub async fn delete_project(
    db_state: State<'_, Arc<Mutex<Database>>>,
    id: i64,
) -> Result<(), String> {
    db_state.lock().unwrap().delete_project(id).map_err(|e| e.to_string())
}
