use chrono;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PomodoroRecord {
    pub id: i64,
    pub task_name: String,
    pub action: String,
    pub elapsed: u64,
    pub phase: u8,
    pub timestamp: i64,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS pomodoro_sessions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                task_name TEXT NOT NULL,
                action TEXT NOT NULL,
                elapsed INTEGER NOT NULL,
                phase INTEGER NOT NULL,
                timestamp INTEGER NOT NULL,
                end_timestamp INTEGER
            )",
            [],
        )?;
        
        let _ = conn.execute(
            "ALTER TABLE pomodoro_sessions ADD COLUMN end_timestamp INTEGER",
            [],
        );
        
        Ok(Database { conn })
    }

    pub fn log_action(&self, task_name: &str, action: &str, elapsed: u64, phase: u8) -> Result<()> {
        let timestamp = chrono::Utc::now().timestamp();
        self.conn.execute(
            "INSERT INTO pomodoro_sessions (task_name, action, elapsed, phase, timestamp, end_timestamp) 
         VALUES (?1, ?2, ?3, ?4, ?5, NULL)",
            (task_name, action, elapsed as i64, phase as i32, timestamp),
        )?;
        Ok(())
    }

    pub fn log_session_complete(&self, task_name: &str, elapsed_seconds: u64, phase: u8) -> Result<()> {
        let timestamp = chrono::Utc::now().timestamp();
        self.conn.execute(
            "INSERT INTO pomodoro_sessions (task_name, action, elapsed, phase, timestamp, end_timestamp) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (task_name, "complete", elapsed_seconds as i64, phase as i32, timestamp, timestamp),
        )?;
        Ok(())
    }

    pub fn get_recent(&self, limit: i64) -> Result<Vec<PomodoroRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, task_name, action, elapsed, phase, timestamp \
             FROM pomodoro_sessions ORDER BY timestamp DESC LIMIT ?1",
        )?;
        let records = stmt
            .query_map([&limit], |row| {
                Ok(PomodoroRecord {
                    id: row.get(0)?,
                    task_name: row.get(1)?,
                    action: row.get(2)?,
                    elapsed: row.get(3)?,
                    phase: row.get(4)?,
                    timestamp: row.get(5)?,
                })
            })?
            .filter_map(Result::ok)
            .collect();
        Ok(records)
    }

    pub fn get_all(&self) -> Result<Vec<PomodoroRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, task_name, action, elapsed, phase, timestamp \
             FROM pomodoro_sessions ORDER BY timestamp DESC",
        )?;
        let records = stmt
            .query_map([], |row| {
                Ok(PomodoroRecord {
                    id: row.get(0)?,
                    task_name: row.get(1)?,
                    action: row.get(2)?,
                    elapsed: row.get(3)?,
                    phase: row.get(4)?,
                    timestamp: row.get(5)?,
                })
            })?
            .filter_map(Result::ok)
            .collect();
        Ok(records)
    }

    pub fn insert_record(
        &self,
        task_name: &str,
        action: &str,
        elapsed: u64,
        phase: u8,
        timestamp: i64,
    ) -> Result<()> {
        self.conn.execute(
            "INSERT INTO pomodoro_sessions (task_name, action, elapsed, phase, timestamp) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
            (task_name, action, elapsed as i64, phase as i32, timestamp),
        )?;
        Ok(())
    }

    pub fn clear_all(&self) -> Result<()> {
        self.conn.execute("DELETE FROM pomodoro_sessions", [])?;
        Ok(())
    }

    pub fn get_unique_task_names(&self, limit: i64) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT task_name FROM (
                SELECT task_name, MAX(timestamp) AS last_ts
                FROM pomodoro_sessions
                WHERE task_name != '' AND TRIM(task_name) != ''
                GROUP BY task_name
                ORDER BY last_ts DESC
                LIMIT ?1
            )",
        )?;
        let names = stmt
            .query_map([&limit], |row| row.get(0))?
            .filter_map(Result::ok)
            .collect();
        Ok(names)
    }

    pub fn get_task_stats(&self, start_timestamp: i64, end_timestamp: i64) -> Result<Vec<TaskStats>> {
        let mut stmt = self.conn.prepare(
            "SELECT task_name, 
                    COUNT(*) as sessions,
                    SUM(elapsed) as total_seconds
             FROM pomodoro_sessions
             WHERE timestamp >= ?1 AND timestamp <= ?2
               AND task_name != '' AND TRIM(task_name) != ''
               AND action = 'complete'
             GROUP BY task_name
             ORDER BY total_seconds DESC",
        )?;
        let stats = stmt
            .query_map([&start_timestamp, &end_timestamp], |row| {
                Ok(TaskStats {
                    task_name: row.get(0)?,
                    sessions: row.get(1)?,
                    total_seconds: row.get(2)?,
                })
            })?
            .filter_map(Result::ok)
            .collect();
        Ok(stats)
    }

    pub fn get_daily_breakdown(&self, start_timestamp: i64, end_timestamp: i64) -> Result<Vec<DailyStats>> {
        let mut stmt = self.conn.prepare(
            "SELECT DATE(timestamp, 'unixepoch') as day,
                    task_name,
                    COUNT(*) as sessions,
                    SUM(elapsed) as total_seconds
             FROM pomodoro_sessions
             WHERE timestamp >= ?1 AND timestamp <= ?2
               AND task_name != '' AND TRIM(task_name) != ''
               AND action = 'complete'
             GROUP BY day, task_name
             ORDER BY day DESC, total_seconds DESC",
        )?;
        let stats = stmt
            .query_map([&start_timestamp, &end_timestamp], |row| {
                Ok(DailyStats {
                    day: row.get(0)?,
                    task_name: row.get(1)?,
                    sessions: row.get(2)?,
                    total_seconds: row.get(3)?,
                })
            })?
            .filter_map(Result::ok)
            .collect();
        Ok(stats)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TaskStats {
    pub task_name: String,
    pub sessions: i64,
    pub total_seconds: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DailyStats {
    pub day: String,
    pub task_name: String,
    pub sessions: i64,
    pub total_seconds: i64,
}