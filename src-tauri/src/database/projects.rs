// database/projects.rs — Project CRUD

use rusqlite::{Connection, Result};
use super::models::Project;

pub fn create(conn: &Connection, name: &str, color: Option<&str>) -> Result<i64> {
    conn.execute(
        "INSERT INTO projects (name, color) VALUES (?1, ?2)",
        (name, color),
    )?;
    Ok(conn.last_insert_rowid())
}

/// Return the id of the project with the given name, creating it if it does not exist.
pub fn find_or_create(conn: &Connection, name: &str) -> Result<i64> {
    let existing: Option<i64> = conn
        .prepare("SELECT id FROM projects WHERE name = ?1")?
        .query_row([name], |row| row.get(0))
        .ok();
    if let Some(id) = existing {
        return Ok(id);
    }
    create(conn, name, None)
}

pub fn list(conn: &Connection) -> Result<Vec<Project>> {
    let mut stmt = conn.prepare("SELECT id, name, color FROM projects ORDER BY name")?;
    let rows = stmt
        .query_map([], |row| {
            Ok(Project { id: row.get(0)?, name: row.get(1)?, color: row.get(2)? })
        })?
        .filter_map(Result::ok)
        .collect();
    Ok(rows)
}

/// Tasks whose project_id matches are automatically un-assigned
/// via the ON DELETE SET NULL foreign key constraint.
pub fn delete(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM projects WHERE id = ?1", [id])?;
    Ok(())
}
