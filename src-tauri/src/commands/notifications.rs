// commands/notifications.rs
//
// Notification helpers: wraps tauri_plugin_notification with
// settings-aware guards. All notification firing from the frontend
// should go through these commands so the master-toggle is respected.

use std::sync::{Arc, Mutex};
use tauri::{AppHandle, State};
use tauri_plugin_notification::NotificationExt;

use crate::database::Database;

// ── Commands ──────────────────────────────────────────────────────────────

/// Request OS notification permission (required on macOS at runtime).
/// Returns true if permission is granted (or already was).
/// On Linux / Windows this is a no-op that always returns true.
#[tauri::command]
pub async fn request_notification_permission(handle: AppHandle) -> Result<bool, String> {
    // On desktop (macOS/Win/Linux) the desktop implementation always returns Granted.
    // We call request_permission anyway to handle future mobile targets gracefully.
    let _ = handle
        .notification()
        .request_permission()
        .map_err(|e| e.to_string())?;
    Ok(true)
}

/// Fire a system notification if the master toggle (`notifications_enabled`)
/// is on. Both title and body are supplied by the caller.
#[tauri::command]
pub fn show_notification(
    handle:   AppHandle,
    db_state: State<'_, Arc<Mutex<Database>>>,
    title: String,
    body:  String,
) -> Result<(), String> {
    if !notifications_enabled(&db_state) {
        return Ok(());
    }
    handle
        .notification()
        .builder()
        .title(&title)
        .body(&body)
        .show()
        .map_err(|e| e.to_string())
}

// ── Internal helper ───────────────────────────────────────────────────────

/// Reads the `notifications_enabled` setting. Defaults to true if not set.
pub fn notifications_enabled(db_state: &State<'_, Arc<Mutex<Database>>>) -> bool {
    db_state
        .lock()
        .unwrap()
        .get_setting("notifications_enabled")
        .unwrap_or(None)
        .map(|v| v != "false")
        .unwrap_or(true)
}

/// Read any boolean setting, defaulting to `default_val`.
pub fn bool_setting(db_state: &Arc<Mutex<Database>>, key: &str, default_val: bool) -> bool {
    db_state
        .lock()
        .unwrap()
        .get_setting(key)
        .unwrap_or(None)
        .map(|v| v != "false")
        .unwrap_or(default_val)
}
