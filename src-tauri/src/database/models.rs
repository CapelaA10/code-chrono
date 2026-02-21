// database/models.rs
//
// Plain data structs that are shared across all database sub-modules
// and serialised to/from the frontend via Tauri IPC.

use serde::{Deserialize, Serialize};

// ── Session log ───────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PomodoroRecord {
    pub id:        i64,
    pub task_name: String,
    pub action:    String,
    pub elapsed:   u64,
    pub phase:     u8,
    pub timestamp: i64,
}

// ── Tasks ─────────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub id:           i64,
    pub title:        String,
    pub description:  Option<String>,
    pub due_date:     Option<i64>,
    pub priority:     i32,
    pub status:       String,
    pub project_id:   Option<i64>,
    pub parent_id:    Option<i64>,
    pub position:     i32,
    pub external_id:  Option<String>,
    pub source:       Option<String>,
    pub created_at:   i64,
    pub completed_at: Option<i64>,
    pub tags:         Vec<Tag>,
}

// ── Projects & Tags ───────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    pub id:    i64,
    pub name:  String,
    pub color: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tag {
    pub id:    i64,
    pub name:  String,
    pub color: Option<String>,
}

// ── Statistics ────────────────────────────────────────────────────────────

/// Totals per task, returned by `get_task_stats`.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TaskStats {
    pub task_name:     String,
    pub sessions:      i64,
    pub total_seconds: i64,
}

/// Per-day totals per task, returned by `get_daily_breakdown`.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DailyStats {
    /// ISO date "YYYY-MM-DD"
    pub day:           String,
    pub task_name:     String,
    pub sessions:      i64,
    pub total_seconds: i64,
}
