use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use chrono; 

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
                timestamp INTEGER NOT NULL
            )",
            [],
        )?;
        Ok(Database { conn })
    }

    pub fn log_action(&self, task_name: &str, action: &str, elapsed: u64, phase: u8) -> Result<()> {
        self.conn.execute(
            "INSERT INTO pomodoro_sessions (task_name, action, elapsed, phase, timestamp) \
             VALUES (?1, ?2, ?3, ?4, ?5)",
            (&task_name, &action, &elapsed as &dyn rusqlite::ToSql, &phase as &dyn rusqlite::ToSql, &chrono::Utc::now().timestamp() as &dyn rusqlite::ToSql),
        )?;
        Ok(())
    }

    pub fn get_recent(&self, limit: i64) -> Result<Vec<PomodoroRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, task_name, action, elapsed, phase, timestamp \
             FROM pomodoro_sessions ORDER BY timestamp DESC LIMIT ?1"
        )?;
        let records = stmt.query_map([&limit], |row| {
            Ok(PomodoroRecord {
                id: row.get(0)?,
                task_name: row.get(1)?,
                action: row.get(2)?,
                elapsed: row.get(3)?,
                phase: row.get(4)?,
                timestamp: row.get(5)?,
            })
        })?.filter_map(Result::ok).collect();
        Ok(records)
    }

    pub fn get_all(&self) -> Result<Vec<PomodoroRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, task_name, action, elapsed, phase, timestamp \
             FROM pomodoro_sessions ORDER BY timestamp DESC"
        )?;
        let records = stmt.query_map([], |row| {
            Ok(PomodoroRecord {
                id: row.get(0)?,
                task_name: row.get(1)?,
                action: row.get(2)?,
                elapsed: row.get(3)?,
                phase: row.get(4)?,
                timestamp: row.get(5)?,
            })
        })?.filter_map(Result::ok).collect();
        Ok(records)
    }
}
