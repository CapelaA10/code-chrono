use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{AppHandle, State};
use tauri_plugin_notification::NotificationExt;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TimerState {
    pub remaining: u64,
    pub paused: bool,
    pub phase: u8,
    pub task_active: bool,
    #[serde(skip)]
    pub loop_running: bool,
    pub session_duration: u64,
    #[serde(skip)]
    pub session_start_time: u64,
}

impl Default for TimerState {
    fn default() -> Self {
        Self {
            remaining: 25 * 60,
            paused: true,
            phase: 0,
            task_active: false,
            loop_running: false,
            session_duration: 25 * 60,
            session_start_time: 0,
        }
    }
}

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
    use tauri::Emitter;

    use super::*;
    use crate::database::Database;

    #[tauri::command]
    pub async fn start_pomodoro(
        state: State<'_, Arc<Mutex<TimerState>>>,
        db_state: State<'_, Arc<Mutex<Database>>>,
        handle: AppHandle,
        task_name: String,
        duration_minutes: Option<u64>,
    ) -> Result<(), String> {
        let mut timer = state.lock().unwrap();
        if timer.task_active {
            return Ok(());
        }

        {
            let db = db_state.lock().unwrap();
            db.log_action(&task_name, "start", 0, 0).unwrap();
        }

        let duration_seconds = duration_minutes.unwrap_or(25) * 60;
        timer.remaining = duration_seconds;
        timer.session_duration = duration_seconds;
        timer.session_start_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        timer.paused = false;
        timer.phase = 0;
        timer.task_active = true;
        timer.loop_running = true;
        drop(timer);

        start_timer_loop(Arc::clone(&*state), Arc::clone(&*db_state), handle, task_name);
        Ok(())
    }

    #[tauri::command]
    pub fn pause_timer(
        state: State<'_, Arc<Mutex<TimerState>>>,
        db_state: State<'_, Arc<Mutex<Database>>>,
        handle: AppHandle,
        task_name: String,
    ) -> Result<(), String> {
        let should_resume = {
            let mut timer = state.lock().unwrap();
            let action = if timer.paused { "resume" } else { "pause" };
            {
                let db = db_state.lock().unwrap();
                db.log_action(&task_name, action, 0, timer.phase).unwrap();
            }
            timer.paused = !timer.paused;
            
            !timer.paused && timer.task_active && timer.remaining > 0 && !timer.loop_running
        };
        
        if should_resume {
            let mut timer = state.lock().unwrap();
            timer.loop_running = true;
            drop(timer);
            start_timer_loop(Arc::clone(&*state), Arc::clone(&*db_state), handle, task_name);
        }
        Ok(())
    }

    #[tauri::command]
    pub fn get_timer(state: State<'_, Arc<Mutex<TimerState>>>) -> Result<TimerState, String> {
        Ok(state.lock().unwrap().clone())
    }

    #[tauri::command]
    pub fn reset_timer(
        state: State<'_, Arc<Mutex<TimerState>>>,
        db_state: State<'_, Arc<Mutex<Database>>>,
        task_name: String,
    ) -> Result<(), String> {
        let elapsed_seconds = {
            let timer = state.lock().unwrap();
            if timer.task_active && !task_name.is_empty() {
                timer.session_duration - timer.remaining
            } else {
                0
            }
        };

        if elapsed_seconds > 0 {
            let timer = state.lock().unwrap();
            let db = db_state.lock().unwrap();
            let _ = db.log_session_complete(&task_name, elapsed_seconds, timer.phase);
        }

        let mut timer = state.lock().unwrap();
        timer.remaining = 25 * 60;
        timer.session_duration = 25 * 60;
        timer.session_start_time = 0;
        timer.paused = true;
        timer.phase = 0;
        timer.task_active = false;
        timer.loop_running = false;
        Ok(())
    }

    #[tauri::command]
    pub fn export_csv(db_state: State<'_, Arc<Mutex<Database>>>) -> Result<String, String> {
        let db = db_state.lock().unwrap();
        let records = db.get_all().map_err(|e| e.to_string())?;
        let mut csv = "id,task_name,action,elapsed,phase,timestamp\n".to_string();
        for record in records {
            let task_name = if record.task_name.contains(',') {
                format!("\"{}\"", record.task_name.replace("\"", "\"\""))
            } else {
                record.task_name
            };
            csv.push_str(&format!(
                "{},{},{},{},{},{}\n",
                record.id, task_name, record.action, record.elapsed, record.phase, record.timestamp
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

    #[tauri::command]
    pub fn get_task_stats(
        db_state: State<'_, Arc<Mutex<Database>>>,
        start_timestamp: i64,
        end_timestamp: i64,
    ) -> Result<Vec<crate::database::TaskStats>, String> {
        let db = db_state.lock().unwrap();
        db.get_task_stats(start_timestamp, end_timestamp).map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub fn get_daily_breakdown(
        db_state: State<'_, Arc<Mutex<Database>>>,
        start_timestamp: i64,
        end_timestamp: i64,
    ) -> Result<Vec<crate::database::DailyStats>, String> {
        let db = db_state.lock().unwrap();
        db.get_daily_breakdown(start_timestamp, end_timestamp).map_err(|e| e.to_string())
    }

    fn start_timer_loop(
        arc_state: Arc<Mutex<TimerState>>,
        db_state: Arc<Mutex<Database>>,
        handle: AppHandle,
        task_name: String,
    ) {
        tauri::async_runtime::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));
            loop {
                interval.tick().await;
                {
                    let mut timer = arc_state.lock().unwrap();
                    
                    if timer.paused || !timer.task_active {
                        timer.loop_running = false;
                        break;
                    }
                    
                    if timer.remaining > 0 {
                        timer.remaining -= 1;
                        let payload = timer.clone();
                        drop(timer);
                        let _ = handle.emit("timer-tick", payload);
                    } else {
                        let phase = timer.phase;
                        let elapsed_seconds = timer.session_duration;
                        timer.task_active = false;
                        timer.loop_running = false;
                        drop(timer);
                        
                        let db = db_state.lock().unwrap();
                        let _ = db.log_session_complete(&task_name, elapsed_seconds, phase);
                        drop(db);
                        
                        let _ = handle
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
            }
        });
    }
}