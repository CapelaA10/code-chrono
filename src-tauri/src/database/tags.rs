// database/tags.rs — Tag CRUD
//
// Tags use INSERT OR IGNORE so the same name can't be created twice.
// Deletion cascades to task_tags via ON DELETE CASCADE.

use rusqlite::{Connection, Result};
use super::models::Tag;

/// Create a tag (or silently skip if the name already exists) and return its id.
pub fn create(conn: &Connection, name: &str, color: Option<&str>) -> Result<i64> {
    conn.execute(
        "INSERT OR IGNORE INTO tags (name, color) VALUES (?1, ?2)",
        (name, color),
    )?;
    // IGNORE means last_insert_rowid() is 0 on duplicate; fetch by name instead
    conn.prepare("SELECT id FROM tags WHERE name = ?1")?
        .query_row([name], |row| row.get(0))
}

pub fn list(conn: &Connection) -> Result<Vec<Tag>> {
    let mut stmt = conn.prepare("SELECT id, name, color FROM tags ORDER BY name")?;
    let rows = stmt
        .query_map([], |row| {
            Ok(Tag { id: row.get(0)?, name: row.get(1)?, color: row.get(2)? })
        })?
        .filter_map(Result::ok)
        .collect();
    Ok(rows)
}

pub fn delete(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM tags WHERE id = ?1", [id])?;
    Ok(())
}
