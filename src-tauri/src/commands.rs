use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{AppHandle, Emitter, State};
use tauri_plugin_notification::NotificationExt;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TimerState {
    pub remaining: u64,
    pub paused: bool,
    pub phase: u8,
}

impl Default for TimerState {
    fn default() -> Self {
        Self {
            remaining: 25 * 60, // 25:00 when app starts
            paused: true,
            phase: 0,
        }
    }
}

/// Row shape for CSV import (matches export header).
#[derive(serde::Deserialize)]
struct PomodoroRecordRow {
    #[allow(dead_code)]
    id: i64,
    task_name: String,
    action: String,
    elapsed: u64,
    phase: u8,
    timestamp: i64,
}

pub mod api {
    use super::*; 
    use crate::database::Database; 

    #[tauri::command]
    pub async fn start_pomodoro(
        state: State<'_, Arc<Mutex<TimerState>>>,
        db_state: State<'_, Arc<Mutex<Database>>>,
        handle: AppHandle,
        task_name: String,
    ) -> Result<(), String> {
        {
            let db = db_state.lock().unwrap();
            db.log_action(&task_name, "start", 0, 0).unwrap();
        }

        {
            let mut timer = state.lock().unwrap();
            timer.remaining = 25 * 60;
            timer.paused = false;
            timer.phase = 0;
        }

        let arc_state = Arc::clone(&*state);
        let handle_clone = handle.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));
            loop {
                interval.tick().await;
                let mut timer = arc_state.lock().unwrap();
                if timer.paused {
                    continue;
                }
                if timer.remaining > 0 {
                    timer.remaining -= 1;
                    let payload = timer.clone(); // Clone for emit
                    drop(timer);
                    let _ = handle_clone.emit("timer-tick", payload);
                } else {
                    let phase = timer.phase;
                    drop(timer);
                    let _ = handle_clone
                        .notification()
                        .builder()
                        .title("Code Chrono")
                        .body(if phase == 0 {
                            "Work session done!"
                        } else {
                            "Break over!"
                        })
                        .show();
                    break;
                }
            }
        });
        Ok(())
    }

    #[tauri::command]
    pub fn pause_timer(
        state: State<'_, Arc<Mutex<TimerState>>>,
        db_state: State<'_, Arc<Mutex<Database>>>,
        task_name: String,
    ) -> Result<(), String> {
        let mut timer = state.lock().unwrap();
        let action = if timer.paused { "resume" } else { "pause" };
        let db = db_state.lock().unwrap();
        db.log_action(&task_name, action, 0, timer.phase).unwrap();
        timer.paused = !timer.paused;
        Ok(())
    }

    #[tauri::command]
    pub fn get_timer(state: State<'_, Arc<Mutex<TimerState>>>) -> Result<TimerState, String> {
        Ok(state.lock().unwrap().clone())
    }

    #[tauri::command]
    pub fn reset_timer(state: State<'_, Arc<Mutex<TimerState>>>) -> Result<(), String> {
        let mut timer = state.lock().unwrap();
        timer.remaining = 25 * 60;
        timer.paused = true;
        timer.phase = 0;
        Ok(())
    }

    #[tauri::command]
    pub fn export_csv(db_state: State<'_, Arc<Mutex<Database>>>) -> Result<String, String> {
        let db = db_state.lock().unwrap();
        let records = db.get_all().map_err(|e| e.to_string())?;
        let mut csv = "id,task_name,action,elapsed,phase,timestamp\n".to_string();
        for record in records {
            // Escape commas in task_name by wrapping in quotes if it contains comma
            let task_name = if record.task_name.contains(',') {
                format!("\"{}\"", record.task_name.replace("\"", "\"\""))
            } else {
                record.task_name
            };
            csv.push_str(&format!("{},{},{},{},{},{}\n",
                record.id,
                task_name,
                record.action,
                record.elapsed,
                record.phase,
                record.timestamp
            ));
        }
        Ok(csv)
    }

    #[tauri::command]
    pub fn import_csv(
        db_state: State<'_, Arc<Mutex<Database>>>,
        path: String,
    ) -> Result<u64, String> {
        let mut reader = csv::Reader::from_path(&path).map_err(|e| e.to_string())?;
        let db = db_state.lock().unwrap();
        let mut count = 0u64;
        for result in reader.deserialize() {
            let record: PomodoroRecordRow = result.map_err(|e| e.to_string())?;
            db.insert_record(
                &record.task_name,
                &record.action,
                record.elapsed,
                record.phase,
                record.timestamp,
            )
            .map_err(|e| e.to_string())?;
            count += 1;
        }
        Ok(count)
    }

    #[tauri::command]
    pub fn reset_database(db_state: State<'_, Arc<Mutex<Database>>>) -> Result<(), String> {
        let db = db_state.lock().unwrap();
        db.clear_all().map_err(|e| e.to_string())?;
        Ok(())
    }

    #[tauri::command]
    pub fn get_unique_task_names(
        db_state: State<'_, Arc<Mutex<Database>>>,
        limit: i64,
    ) -> Result<Vec<String>, String> {
        let db = db_state.lock().unwrap();
        db.get_unique_task_names(limit).map_err(|e| e.to_string())
    }
}
