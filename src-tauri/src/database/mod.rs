// database/mod.rs
//
// Declares the database module tree and exposes the `Database` façade.
//
// Each sub-module owns one concern:
//   models   — shared data structs (no logic)
//   sessions — pomodoro session log
//   tasks    — task CRUD + tag linking
//   projects — project CRUD
//   tags     — tag CRUD
//   settings — key-value settings store
//
// External code imports from `crate::database::*` and interacts only
// with the `Database` struct; sub-module internals stay private.

mod models;
mod projects;
mod sessions;
mod settings;
mod tags;
mod tasks;

// Re-export models so the rest of the crate can use them without
// reaching into sub-modules.
pub use models::{DailyStats, PomodoroRecord, Project, Tag, Task, TaskStats};

use rusqlite::{Connection, Result};

// ── Database façade ───────────────────────────────────────────────────────

/// Holds the open SQLite connection. All methods delegate to the appropriate
/// sub-module, keeping this struct lean and easy to test.
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Open (or create) the database at `path` and run schema migrations.
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        schema::migrate(&conn)?;
        Ok(Database { conn })
    }

    // ── Session log ───────────────────────────────────────────────────────

    pub fn log_action(&self, task_name: &str, action: &str, elapsed: u64, phase: u8) -> Result<()> {
        sessions::log_action(&self.conn, task_name, action, elapsed, phase)
    }

    pub fn log_session_complete(&self, task_name: &str, elapsed_seconds: u64, phase: u8) -> Result<()> {
        sessions::log_session_complete(&self.conn, task_name, elapsed_seconds, phase)
    }

    pub fn get_recent(&self, limit: i64) -> Result<Vec<PomodoroRecord>> {
        sessions::get_recent(&self.conn, limit)
    }

    pub fn get_all(&self) -> Result<Vec<PomodoroRecord>> {
        sessions::get_all(&self.conn)
    }

    pub fn insert_record(&self, task_name: &str, action: &str, elapsed: u64, phase: u8, timestamp: i64) -> Result<()> {
        sessions::insert_record(&self.conn, task_name, action, elapsed, phase, timestamp)
    }

    pub fn clear_all(&self) -> Result<()> {
        sessions::clear_all(&self.conn)
    }

    pub fn get_unique_task_names(&self, limit: i64) -> Result<Vec<String>> {
        sessions::get_unique_task_names(&self.conn, limit)
    }

    pub fn get_task_stats(&self, start: i64, end: i64) -> Result<Vec<TaskStats>> {
        sessions::get_task_stats(&self.conn, start, end)
    }

    pub fn get_daily_breakdown(&self, start: i64, end: i64) -> Result<Vec<DailyStats>> {
        sessions::get_daily_breakdown(&self.conn, start, end)
    }

    // ── Tasks ─────────────────────────────────────────────────────────────

    pub fn create_task(&self, task: Task) -> Result<i64> {
        tasks::create(&self.conn, task)
    }

    pub fn save_external_task(&self, task: Task) -> Result<i64> {
        tasks::save_external(&self.conn, task)
    }

    /// Returns true if a task with the given `external_id` and `source` already
    /// exists in the database (i.e. was previously imported).
    pub fn is_external_task_imported(&self, external_id: &str, source: &str) -> Result<bool> {
        tasks::is_imported(&self.conn, external_id, source)
    }

    pub fn update_task(&self, task: Task) -> Result<()> {
        tasks::update(&self.conn, task)
    }

    pub fn delete_task(&self, id: i64) -> Result<()> {
        tasks::delete(&self.conn, id)
    }

    pub fn get_tasks(&self, filter_project: Option<i64>, filter_tag: Option<i64>, filter_status: Option<String>) -> Result<Vec<Task>> {
        tasks::list(&self.conn, filter_project, filter_tag, filter_status)
    }

    pub fn search_tasks(&self, query: &str) -> Result<Vec<Task>> {
        tasks::search(&self.conn, query)
    }

    // ── Projects ──────────────────────────────────────────────────────────

    pub fn create_project(&self, name: &str, color: Option<&str>) -> Result<i64> {
        projects::create(&self.conn, name, color)
    }

    /// Find an existing project by name or create a new one. Returns the project id.
    pub fn find_or_create_project(&self, name: &str) -> Result<i64> {
        projects::find_or_create(&self.conn, name)
    }

    pub fn get_projects(&self) -> Result<Vec<Project>> {
        projects::list(&self.conn)
    }

    pub fn delete_project(&self, id: i64) -> Result<()> {
        projects::delete(&self.conn, id)
    }

    // ── Tags ──────────────────────────────────────────────────────────────

    pub fn create_tag(&self, name: &str, color: Option<&str>) -> Result<i64> {
        tags::create(&self.conn, name, color)
    }

    pub fn get_tags(&self) -> Result<Vec<Tag>> {
        tags::list(&self.conn)
    }

    pub fn delete_tag(&self, id: i64) -> Result<()> {
        tags::delete(&self.conn, id)
    }

    // ── Settings ──────────────────────────────────────────────────────────

    pub fn get_setting(&self, key: &str) -> Result<Option<String>> {
        settings::get(&self.conn, key)
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        settings::set(&self.conn, key, value)
    }
}

// ── Schema ────────────────────────────────────────────────────────────────

/// Schema management lives in its own inline module to keep mod.rs readable.
mod schema {
    use rusqlite::{Connection, Result};

    /// Create all tables if they don't exist, then run additive migrations.
    /// Safe to call on every startup — all statements are idempotent.
    pub fn migrate(conn: &Connection) -> Result<()> {
        conn.execute_batch("
            CREATE TABLE IF NOT EXISTS pomodoro_sessions (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                task_name       TEXT    NOT NULL,
                action          TEXT    NOT NULL,
                elapsed         INTEGER NOT NULL,
                phase           INTEGER NOT NULL,
                timestamp       INTEGER NOT NULL,
                end_timestamp   INTEGER
            );

            CREATE TABLE IF NOT EXISTS projects (
                id    INTEGER PRIMARY KEY AUTOINCREMENT,
                name  TEXT NOT NULL,
                color TEXT
            );

            CREATE TABLE IF NOT EXISTS tags (
                id    INTEGER PRIMARY KEY AUTOINCREMENT,
                name  TEXT NOT NULL UNIQUE,
                color TEXT
            );

            CREATE TABLE IF NOT EXISTS tasks (
                id           INTEGER PRIMARY KEY AUTOINCREMENT,
                title        TEXT    NOT NULL,
                description  TEXT,
                due_date     INTEGER,
                priority     INTEGER DEFAULT 0,
                status       TEXT    DEFAULT 'todo',
                project_id   INTEGER REFERENCES projects(id) ON DELETE SET NULL,
                parent_id    INTEGER REFERENCES tasks(id)    ON DELETE CASCADE,
                position     INTEGER DEFAULT 0,
                external_id  TEXT,
                source       TEXT,
                created_at   INTEGER NOT NULL,
                completed_at INTEGER
            );

            CREATE TABLE IF NOT EXISTS task_tags (
                task_id INTEGER NOT NULL REFERENCES tasks(id)  ON DELETE CASCADE,
                tag_id  INTEGER NOT NULL REFERENCES tags(id)   ON DELETE CASCADE,
                PRIMARY KEY (task_id, tag_id)
            );

            CREATE TABLE IF NOT EXISTS settings (
                key   TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
        ")?;

        // Additive migration: add end_timestamp column if it didn't exist in older DBs
        let _ = conn.execute(
            "ALTER TABLE pomodoro_sessions ADD COLUMN end_timestamp INTEGER",
            [],
        );

        Ok(())
    }
}
