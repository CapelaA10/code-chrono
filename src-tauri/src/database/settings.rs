// database/settings.rs — Persistent key-value store (backed by the settings table)

use rusqlite::{Connection, Result};

/// Return a setting value, or `None` if the key doesn't exist yet.
pub fn get(conn: &Connection, key: &str) -> Result<Option<String>> {
    let res = conn
        .prepare("SELECT value FROM settings WHERE key = ?1")?
        .query_row([key], |row| row.get(0))
        .ok();
    Ok(res)
}

/// Insert or overwrite a setting value (upsert via INSERT OR REPLACE).
pub fn set(conn: &Connection, key: &str, value: &str) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
        (key, value),
    )?;
    Ok(())
}
