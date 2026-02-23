// database/tasks.rs
//
// Task CRUD: create, read, update, delete, search, and tag-linking helpers.

use chrono::Utc;
use rusqlite::{Connection, Result};

use super::models::{Tag, Task};

/// Insert a new task row and attach its tags. Returns the new row id.
pub fn create(conn: &Connection, task: Task) -> Result<i64> {
    conn.execute(
        "INSERT INTO tasks
             (title, description, due_date, priority, status,
              project_id, parent_id, position, external_id, source, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        (
            &task.title,
            &task.description,
            task.due_date,
            task.priority,
            &task.status,
            task.project_id,
            task.parent_id,
            task.position,
            &task.external_id,
            &task.source,
            Utc::now().timestamp(),
        ),
    )?;
    let task_id = conn.last_insert_rowid();
    set_tags(conn, task_id, &task.tags);
    Ok(task_id)
}

/// Upsert an external task (from GitHub/GitLab/Jira).
/// If a task with the same external_id + source already exists, only the
/// title and description are updated (status, position, tags are preserved).
/// Returns the task's id (existing or newly created).
pub fn save_external(conn: &Connection, task: Task) -> Result<i64> {
    if let (Some(ext_id), Some(source)) = (&task.external_id, &task.source) {
        let existing: Option<i64> = conn
            .prepare("SELECT id FROM tasks WHERE external_id = ?1 AND source = ?2")?
            .query_row([ext_id, source], |row| row.get(0))
            .ok();

        if let Some(id) = existing {
            conn.execute(
                "UPDATE tasks SET title = ?1, description = ?2 WHERE id = ?3",
                (&task.title, &task.description, id),
            )?;
            return Ok(id);
        }
    }
    create(conn, task)
}

/// Returns true when a task with the same `external_id` + `source` pair already
/// lives in the tasks table (i.e. it was previously imported).
pub fn is_imported(conn: &Connection, external_id: &str, source: &str) -> Result<bool> {
    let count: i64 = conn
        .prepare("SELECT COUNT(*) FROM tasks WHERE external_id = ?1 AND source = ?2")?
        .query_row([external_id, source], |row| row.get(0))?;
    Ok(count > 0)
}

/// Overwrite all editable fields of an existing task.
/// `completed_at` is set on first completion and preserved on subsequent edits;
/// it is cleared when a task moves out of 'done'.
pub fn update(conn: &Connection, task: Task) -> Result<()> {
    // Preserve the original completion timestamp
    let existing_completed_at: Option<i64> = conn
        .prepare("SELECT completed_at FROM tasks WHERE id = ?1")?
        .query_row([task.id], |row| row.get(0))
        .ok()
        .flatten();

    let completed_at = if task.status == "done" {
        existing_completed_at.or_else(|| Some(Utc::now().timestamp()))
    } else {
        None
    };

    conn.execute(
        "UPDATE tasks
         SET title = ?1, description = ?2, due_date = ?3, priority = ?4,
             status = ?5, project_id = ?6, parent_id = ?7,
             position = ?8, completed_at = ?9
         WHERE id = ?10",
        (
            &task.title, &task.description, task.due_date, task.priority,
            &task.status, task.project_id, task.parent_id, task.position,
            completed_at, task.id,
        ),
    )?;

    // Replace the tag list atomically: delete all then re-insert
    conn.execute("DELETE FROM task_tags WHERE task_id = ?1", [task.id])?;
    set_tags(conn, task.id, &task.tags);

    Ok(())
}

/// Delete a task by id. Tags are removed automatically via ON DELETE CASCADE.
pub fn delete(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM tasks WHERE id = ?1", [id])?;
    Ok(())
}

/// Return tasks with optional project / tag / status filters.
/// Filter values are interpolated as literals rather than params because
/// rusqlite doesn't support optional bind parameters natively.
pub fn list(
    conn: &Connection,
    filter_project: Option<i64>,
    filter_tag:     Option<i64>,
    filter_status:  Option<String>,
) -> Result<Vec<Task>> {
    // Build the SQL dynamically based on which filters are active
    let mut sql = "SELECT t.id, t.title, t.description, t.due_date, t.priority,
                          t.status, t.project_id, t.parent_id, t.position,
                          t.external_id, t.source, t.created_at, t.completed_at
                   FROM tasks t ".to_string();

    if filter_tag.is_some() {
        sql.push_str("JOIN task_tags tt ON t.id = tt.task_id ");
    }
    sql.push_str("WHERE 1=1 ");

    if let Some(id) = filter_tag     { sql.push_str(&format!("AND tt.tag_id = {id} ")); }
    if let Some(id) = filter_project { sql.push_str(&format!("AND t.project_id = {id} ")); }
    if let Some(s)  = filter_status  { sql.push_str(&format!("AND t.status = '{s}' ")); }

    sql.push_str("ORDER BY t.position ASC, t.created_at DESC");

    map_task_rows(conn, &sql, [])
}

/// Full-text search across task titles and descriptions.
pub fn search(conn: &Connection, query: &str) -> Result<Vec<Task>> {
    let sql = "SELECT id, title, description, due_date, priority, status,
                      project_id, parent_id, position, external_id, source,
                      created_at, completed_at
               FROM tasks
               WHERE title LIKE ?1 OR description LIKE ?1";
    map_task_rows(conn, sql, [format!("%{query}%")])
}

// ── Private helpers ───────────────────────────────────────────────────────

/// Attach a list of tags to a task (INSERT OR IGNORE).
fn set_tags(conn: &Connection, task_id: i64, tags: &[Tag]) {
    for tag in tags {
        let _ = conn.execute(
            "INSERT OR IGNORE INTO task_tags (task_id, tag_id) VALUES (?1, ?2)",
            (task_id, tag.id),
        );
    }
}

/// Fetch the tags attached to a single task.
pub(super) fn get_tags_for(conn: &Connection, task_id: i64) -> Vec<Tag> {
    let Ok(mut stmt) = conn.prepare(
        "SELECT id, name, color FROM tags
         JOIN task_tags ON tags.id = task_tags.tag_id
         WHERE task_tags.task_id = ?1",
    ) else { return vec![]; };

    stmt.query_map([task_id], |row| {
        Ok(Tag { id: row.get(0)?, name: row.get(1)?, color: row.get(2)? })
    })
    .map(|rows| rows.filter_map(Result::ok).collect())
    .unwrap_or_default()
}

/// Execute a task SELECT query and map every row to a Task (including its tags).
fn map_task_rows(
    conn: &Connection,
    sql:  &str,
    params: impl rusqlite::Params,
) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare(sql)?;
    let tasks = stmt
        .query_map(params, |row| {
            let id: i64 = row.get(0)?;
            Ok(Task {
                id,
                title:        row.get(1)?,
                description:  row.get(2)?,
                due_date:     row.get(3)?,
                priority:     row.get(4)?,
                status:       row.get(5)?,
                project_id:   row.get(6)?,
                parent_id:    row.get(7)?,
                position:     row.get(8)?,
                external_id:  row.get(9)?,
                source:       row.get(10)?,
                created_at:   row.get(11)?,
                completed_at: row.get(12)?,
                tags:         vec![], // populated below, outside the borrow
            })
        })?
        .filter_map(Result::ok)
        // Load each task's tags after the statement borrow ends
        .map(|mut t| { t.tags = get_tags_for(conn, t.id); t })
        .collect();
    Ok(tasks)
}
