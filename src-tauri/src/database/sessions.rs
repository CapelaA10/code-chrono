// database/sessions.rs
//
// Pomodoro session log: writing actions, completing sessions, querying records.

use rusqlite::{Connection, Result};
use chrono::Utc;

use super::models::{DailyStats, PomodoroRecord, TaskStats};

/// Record a timer action (start, pause, resume) — elapsed is 0 for non-complete actions.
pub fn log_action(conn: &Connection, task_name: &str, action: &str, elapsed: u64, phase: u8) -> Result<()> {
    conn.execute(
        "INSERT INTO pomodoro_sessions (task_name, action, elapsed, phase, timestamp, end_timestamp)
         VALUES (?1, ?2, ?3, ?4, ?5, NULL)",
        (task_name, action, elapsed as i64, phase as i32, Utc::now().timestamp()),
    )?;
    Ok(())
}

/// Record a fully completed session with elapsed time and timestamp.
pub fn log_session_complete(conn: &Connection, task_name: &str, elapsed_seconds: u64, phase: u8) -> Result<()> {
    let now = Utc::now().timestamp();
    conn.execute(
        "INSERT INTO pomodoro_sessions (task_name, action, elapsed, phase, timestamp, end_timestamp)
         VALUES (?1, 'complete', ?2, ?3, ?4, ?4)",
        (task_name, elapsed_seconds as i64, phase as i32, now),
    )?;
    Ok(())
}

/// Return the N most recent records (newest first).
pub fn get_recent(conn: &Connection, limit: i64) -> Result<Vec<PomodoroRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, task_name, action, elapsed, phase, timestamp
         FROM pomodoro_sessions ORDER BY timestamp DESC LIMIT ?1",
    )?;
    query_records(&mut stmt, rusqlite::params![limit])
}

/// Return all session records (newest first).
pub fn get_all(conn: &Connection) -> Result<Vec<PomodoroRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, task_name, action, elapsed, phase, timestamp
         FROM pomodoro_sessions ORDER BY timestamp DESC",
    )?;
    query_records(&mut stmt, [])
}

/// Insert a raw record (used during CSV import).
pub fn insert_record(conn: &Connection, task_name: &str, action: &str, elapsed: u64, phase: u8, timestamp: i64) -> Result<()> {
    conn.execute(
        "INSERT INTO pomodoro_sessions (task_name, action, elapsed, phase, timestamp)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        (task_name, action, elapsed as i64, phase as i32, timestamp),
    )?;
    Ok(())
}

/// Delete all session records (used by reset_database).
pub fn clear_all(conn: &Connection) -> Result<()> {
    conn.execute("DELETE FROM pomodoro_sessions", [])?;
    Ok(())
}

/// Return the N most recently used distinct task names, ordered by last use.
pub fn get_unique_task_names(conn: &Connection, limit: i64) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT task_name FROM (
             SELECT task_name, MAX(timestamp) AS last_ts
             FROM   pomodoro_sessions
             WHERE  task_name != '' AND TRIM(task_name) != ''
             GROUP  BY task_name
             ORDER  BY last_ts DESC
             LIMIT  ?1
         )",
    )?;
    let names = stmt
        .query_map([limit], |row| row.get(0))?
        .filter_map(Result::ok)
        .collect();
    Ok(names)
}

/// Aggregate total time and session count per task for a given time window.
pub fn get_task_stats(conn: &Connection, start: i64, end: i64) -> Result<Vec<TaskStats>> {
    let mut stmt = conn.prepare(
        "SELECT task_name,
                COUNT(*)    AS sessions,
                SUM(elapsed) AS total_seconds
         FROM   pomodoro_sessions
         WHERE  timestamp  >= ?1
           AND  timestamp  <= ?2
           AND  task_name  != '' AND TRIM(task_name) != ''
           AND  action      = 'complete'
         GROUP  BY task_name
         ORDER  BY total_seconds DESC",
    )?;
    let rows = stmt
        .query_map([start, end], |row| {
            Ok(TaskStats {
                task_name:     row.get(0)?,
                sessions:      row.get(1)?,
                total_seconds: row.get(2)?,
            })
        })?
        .filter_map(Result::ok)
        .collect();
    Ok(rows)
}

/// Same as `get_task_stats` but broken down by calendar day.
pub fn get_daily_breakdown(conn: &Connection, start: i64, end: i64) -> Result<Vec<DailyStats>> {
    let mut stmt = conn.prepare(
        "SELECT DATE(timestamp, 'unixepoch') AS day,
                task_name,
                COUNT(*)    AS sessions,
                SUM(elapsed) AS total_seconds
         FROM   pomodoro_sessions
         WHERE  timestamp  >= ?1
           AND  timestamp  <= ?2
           AND  task_name  != '' AND TRIM(task_name) != ''
           AND  action      = 'complete'
         GROUP  BY day, task_name
         ORDER  BY day DESC, total_seconds DESC",
    )?;
    let rows = stmt
        .query_map([start, end], |row| {
            Ok(DailyStats {
                day:           row.get(0)?,
                task_name:     row.get(1)?,
                sessions:      row.get(2)?,
                total_seconds: row.get(3)?,
            })
        })?
        .filter_map(Result::ok)
        .collect();
    Ok(rows)
}

// ── Shared helper ─────────────────────────────────────────────────────────

fn query_records(stmt: &mut rusqlite::Statement<'_>, params: impl rusqlite::Params) -> Result<Vec<PomodoroRecord>> {
    let records = stmt
        .query_map(params, |row| {
            Ok(PomodoroRecord {
                id:        row.get(0)?,
                task_name: row.get(1)?,
                action:    row.get(2)?,
                elapsed:   row.get(3)?,
                phase:     row.get(4)?,
                timestamp: row.get(5)?,
            })
        })?
        .filter_map(Result::ok)
        .collect();
    Ok(records)
}
