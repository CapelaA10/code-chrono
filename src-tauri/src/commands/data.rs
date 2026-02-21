// commands/data.rs — CSV import/export and database management

use std::sync::{Arc, Mutex};
use serde::Deserialize;
use tauri::State;

use crate::database::Database;

/// Intermediate struct used during CSV deserialization.
/// Mirrors the columns written by `export_csv`.
#[derive(Deserialize)]
struct CsvRow {
    #[allow(dead_code)]
    id: i64,
    task_name: String,
    action:    String,
    elapsed:   u64,
    phase:     u8,
    timestamp: i64,
}

/// Export all Pomodoro session records as a CSV string.
/// The frontend is responsible for triggering the browser "Save file" dialog.
#[tauri::command]
pub fn export_csv(
    db_state: State<'_, Arc<Mutex<Database>>>,
) -> Result<String, String> {
    let db      = db_state.lock().unwrap();
    let records = db.get_all().map_err(|e| e.to_string())?;

    let mut csv = "id,task_name,action,elapsed,phase,timestamp\n".to_string();
    for r in records {
        // Escape task names that contain commas
        let name = if r.task_name.contains(',') {
            format!("\"{}\"", r.task_name.replace('"', "\"\""))
        } else {
            r.task_name
        };
        csv.push_str(&format!("{},{},{},{},{},{}\n", r.id, name, r.action, r.elapsed, r.phase, r.timestamp));
    }
    Ok(csv)
}

/// Import Pomodoro session records from a CSV file at `path`.
/// Returns the number of records inserted.
#[tauri::command]
pub fn import_csv(
    db_state: State<'_, Arc<Mutex<Database>>>,
    path:     String,
) -> Result<u64, String> {
    let db = db_state.lock().unwrap();
    let mut reader = csv::Reader::from_path(&path).map_err(|e| e.to_string())?;
    let mut count  = 0u64;

    for result in reader.deserialize() {
        let row: CsvRow = result.map_err(|e| e.to_string())?;
        db.insert_record(&row.task_name, &row.action, row.elapsed, row.phase, row.timestamp)
            .map_err(|e| e.to_string())?;
        count += 1;
    }
    Ok(count)
}

/// Wipe all Pomodoro session logs (tasks, projects, and tags are preserved).
#[tauri::command]
pub fn reset_database(
    db_state: State<'_, Arc<Mutex<Database>>>,
) -> Result<(), String> {
    db_state.lock().unwrap().clear_all().map_err(|e| e.to_string())
}
