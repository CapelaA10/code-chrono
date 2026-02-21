// commands/stats.rs — Session statistics queries

use std::sync::{Arc, Mutex};
use tauri::State;

use crate::database::{Database, DailyStats, TaskStats};

/// Returns total time tracked and session count, grouped by task name.
/// Useful for the "Time by Task" bar chart.
#[tauri::command]
pub fn get_task_stats(
    db_state:        State<'_, Arc<Mutex<Database>>>,
    start_timestamp: i64,
    end_timestamp:   i64,
) -> Result<Vec<TaskStats>, String> {
    db_state
        .lock()
        .unwrap()
        .get_task_stats(start_timestamp, end_timestamp)
        .map_err(|e| e.to_string())
}

/// Returns the same aggregated stats broken down day by day.
/// Useful for the "Daily Breakdown" panel.
#[tauri::command]
pub fn get_daily_breakdown(
    db_state:        State<'_, Arc<Mutex<Database>>>,
    start_timestamp: i64,
    end_timestamp:   i64,
) -> Result<Vec<DailyStats>, String> {
    db_state
        .lock()
        .unwrap()
        .get_daily_breakdown(start_timestamp, end_timestamp)
        .map_err(|e| e.to_string())
}
