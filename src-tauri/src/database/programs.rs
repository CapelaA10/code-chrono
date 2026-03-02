// database/programs.rs
//
// CRUD for the `tracked_programs` table.
// Each row represents an IDE / dev-tool the user wants to monitor.

use rusqlite::{params, Connection, Result};
use crate::commands::programs::TrackedProgram;

pub fn list(conn: &Connection) -> Result<Vec<TrackedProgram>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, executable, enabled, is_custom
           FROM tracked_programs
          ORDER BY name ASC",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(TrackedProgram {
            id:         Some(row.get(0)?),
            name:       row.get(1)?,
            executable: row.get(2)?,
            enabled:    row.get::<_, i64>(3)? != 0,
            is_custom:  row.get::<_, i64>(4)? != 0,
        })
    })?;
    rows.collect()
}

/// Insert or update a tracked program by executable path.
pub fn upsert(conn: &Connection, program: &TrackedProgram) -> Result<()> {
    conn.execute(
        "INSERT INTO tracked_programs (name, executable, enabled, is_custom)
              VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT(executable) DO UPDATE SET
              name      = excluded.name,
              enabled   = excluded.enabled,
              is_custom = excluded.is_custom",
        params![
            program.name,
            program.executable,
            program.enabled as i64,
            program.is_custom as i64,
        ],
    )?;
    Ok(())
}

pub fn delete(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM tracked_programs WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn set_enabled(conn: &Connection, id: i64, enabled: bool) -> Result<()> {
    conn.execute(
        "UPDATE tracked_programs SET enabled = ?1 WHERE id = ?2",
        params![enabled as i64, id],
    )?;
    Ok(())
}
