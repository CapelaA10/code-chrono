// commands/programs.rs
//
// Program / IDE detection: scanning installed apps, managing the tracked-program
// list in SQLite, and the background process-polling loop that emits
// "program-opened" events to the frontend.

use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, State};

use crate::database::Database;

// ── Data types ────────────────────────────────────────────────────────────

/// A program found on the system during a scan.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DetectedProgram {
    pub name:       String,
    pub executable: String,
}

/// A program the user has chosen to track (stored in `tracked_programs` table).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TrackedProgram {
    pub id:         Option<i64>,
    pub name:       String,
    pub executable: String,
    pub enabled:    bool,
    pub is_custom:  bool,
}

// ── Well-known IDE / dev-tool list ────────────────────────────────────────

/// (lowercase executable stem → display name)
const KNOWN_IDES: &[(&str, &str)] = &[
    // Visual Studio Code variants
    ("code",                    "Visual Studio Code"),
    ("code-insiders",           "VS Code Insiders"),
    ("cursor",                  "Cursor"),
    ("windsurf",                "Windsurf"),
    // JetBrains
    ("idea",                    "IntelliJ IDEA"),
    ("pycharm",                 "PyCharm"),
    ("webstorm",                "WebStorm"),
    ("goland",                  "GoLand"),
    ("clion",                   "CLion"),
    ("rider",                   "Rider"),
    ("rubymine",                "RubyMine"),
    ("datagrip",                "DataGrip"),
    ("phpstorm",                "PhpStorm"),
    ("appcode",                 "AppCode"),
    ("fleet",                   "Fleet"),
    // Apple
    ("xcode",                   "Xcode"),
    // Google
    ("androidstudio",           "Android Studio"),
    ("studio",                  "Android Studio"),
    // Text editors
    ("sublime_text",            "Sublime Text"),
    ("sublimetext",             "Sublime Text"),
    ("zed",                     "Zed"),
    ("zeditor",                 "Zed"),
    ("atom",                    "Atom"),
    ("nova",                    "Nova"),
    ("bbedit",                  "BBEdit"),
    // Terminal editors
    ("nvim",                    "Neovim"),
    ("vim",                     "Vim"),
    ("emacs",                   "Emacs"),
    ("nano",                    "Nano"),
    // Misc dev tools
    ("tableplus",               "TablePlus"),
    ("dbeaver",                 "DBeaver"),
    ("insomnia",                "Insomnia"),
    ("postman",                 "Postman"),
    ("docker",                  "Docker Desktop"),
];

// ── Commands ──────────────────────────────────────────────────────────────

/// Scan the current OS for installed IDE/dev-tool applications.
/// Returns only programs present on-disk that match the known IDE list.
#[tauri::command]
pub async fn scan_installed_programs() -> Result<Vec<DetectedProgram>, String> {
    Ok(scan_platform())
}

/// Return all rows from the `tracked_programs` table.
#[tauri::command]
pub fn get_tracked_programs(
    db_state: State<'_, Arc<Mutex<Database>>>,
) -> Result<Vec<TrackedProgram>, String> {
    db_state
        .lock()
        .unwrap()
        .get_tracked_programs()
        .map_err(|e| e.to_string())
}

/// Upsert a tracked program (insert or update by executable path).
#[tauri::command]
pub fn save_tracked_program(
    db_state: State<'_, Arc<Mutex<Database>>>,
    program:  TrackedProgram,
) -> Result<(), String> {
    db_state
        .lock()
        .unwrap()
        .save_tracked_program(program)
        .map_err(|e| e.to_string())
}

/// Delete a tracked program by id.
#[tauri::command]
pub fn remove_tracked_program(
    db_state: State<'_, Arc<Mutex<Database>>>,
    id:       i64,
) -> Result<(), String> {
    db_state
        .lock()
        .unwrap()
        .remove_tracked_program(id)
        .map_err(|e| e.to_string())
}

/// Toggle the `enabled` flag of a tracked program.
#[tauri::command]
pub fn toggle_tracked_program(
    db_state: State<'_, Arc<Mutex<Database>>>,
    id:       i64,
    enabled:  bool,
) -> Result<(), String> {
    db_state
        .lock()
        .unwrap()
        .toggle_tracked_program(id, enabled)
        .map_err(|e| e.to_string())
}

// ── Background polling loop ───────────────────────────────────────────────

/// Spawn the process-watcher loop. Called once from `lib.rs` setup.
/// Every 5 s: list running processes, cross-reference tracked+enabled programs,
/// emit "program-opened" to the frontend (debounced: once per program per 15 min session).
pub fn spawn_program_watcher(db: Arc<Mutex<Database>>, handle: AppHandle) {
    tauri::async_runtime::spawn(async move {
        use std::collections::HashMap;
        use std::time::{Duration, Instant};
        use sysinfo::System;

        let mut sys = System::new_all();
        // last_notified: executable → Instant of last notification
        let mut last_notified: HashMap<String, Instant> = HashMap::new();
        let cooldown = Duration::from_secs(15 * 60); // 15 minute cooldown

        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;

            // Check the setting before doing any work
            let notify_enabled = db
                .lock()
                .unwrap()
                .get_setting("notify_on_program_open")
                .unwrap_or(None)
                .map(|v| v != "false")
                .unwrap_or(true);

            if !notify_enabled {
                continue;
            }

            // Fetch enabled tracked programs
            let tracked = match db.lock().unwrap().get_tracked_programs() {
                Ok(t) => t,
                Err(_) => continue,
            };
            let enabled: Vec<TrackedProgram> = tracked.into_iter().filter(|p| p.enabled).collect();
            if enabled.is_empty() {
                continue;
            }

            // Refresh process list
            sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

            for program in &enabled {
                let exe_lower = program.executable.to_lowercase();
                // Check if any running process name matches the tracked executable
                let is_running = sys.processes().values().any(|proc| {
                    let proc_name = proc.name().to_string_lossy().to_lowercase();
                    proc_name == exe_lower
                        || proc_name.starts_with(&exe_lower)
                        || exe_lower.contains(&proc_name.as_str())
                });

                if !is_running {
                    continue;
                }

                // Debounce: skip if we already notified recently
                if let Some(last) = last_notified.get(&program.executable) {
                    if last.elapsed() < cooldown {
                        continue;
                    }
                }

                // Emit event to frontend
                let _ = handle.emit(
                    "program-opened",
                    serde_json::json!({ "name": program.name, "executable": program.executable }),
                );
                last_notified.insert(program.executable.clone(), Instant::now());
            }
        }
    });
}

// ── Platform-specific scanning ────────────────────────────────────────────

#[cfg(target_os = "macos")]
fn scan_platform() -> Vec<DetectedProgram> {
    let mut found = Vec::new();

    let dirs = ["/Applications", &format!("{}/Applications", std::env::var("HOME").unwrap_or_default())];

    for dir in &dirs {
        let Ok(entries) = std::fs::read_dir(dir) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("app") {
                continue;
            }
            // Executable name = app-bundle stem, lower-cased
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_lowercase()
                .replace(' ', "");

            if let Some(&(_, display)) = KNOWN_IDES.iter().find(|(k, _)| {
                let k = k.to_lowercase();
                stem == k || stem.contains(&k) || k.contains(&stem)
            }) {
                // Store the .app bundle stem as the executable name.
                // sysinfo reports the running process by this name (e.g. "Xcode", not the full path).
                let bin_name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string();
                if !found.iter().any(|p: &DetectedProgram| p.name == display) {
                    found.push(DetectedProgram { name: display.to_string(), executable: bin_name });
                }
            }
        }
    }
    found
}

#[cfg(target_os = "linux")]
fn scan_platform() -> Vec<DetectedProgram> {
    use std::io::{BufRead, BufReader};
    let mut found = Vec::new();

    let home = std::env::var("HOME").unwrap_or_default();
    let dirs = [
        "/usr/share/applications".to_string(),
        format!("{}/.local/share/applications", home),
    ];

    for dir in &dirs {
        let Ok(entries) = std::fs::read_dir(dir) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("desktop") {
                continue;
            }
            // Parse the .desktop file for Name= and Exec= fields
            let Ok(file) = std::fs::File::open(&path) else { continue };
            let mut name = String::new();
            let mut exec = String::new();
            for line in BufReader::new(file).lines().flatten() {
                if line.starts_with("Name=") && name.is_empty() {
                    name = line[5..].trim().to_string();
                } else if line.starts_with("Exec=") && exec.is_empty() {
                    // Strip arguments like %U, %f, etc.
                    exec = line[5..]
                        .split_whitespace()
                        .next()
                        .unwrap_or("")
                        .to_string();
                }
            }
            if name.is_empty() || exec.is_empty() { continue; }
            let exec_stem = std::path::Path::new(&exec)
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or(&exec)
                .to_lowercase();

            if KNOWN_IDES.iter().any(|(k, _)| exec_stem.contains(k) || k.contains(&exec_stem.as_str())) {
                if !found.iter().any(|p: &DetectedProgram| p.name == name) {
                    found.push(DetectedProgram { name, executable: exec });
                }
            }
        }
    }
    found
}

#[cfg(target_os = "windows")]
fn scan_platform() -> Vec<DetectedProgram> {
    // On Windows we check the uninstall registry keys for DisplayName + InstallLocation / DisplayIcon
    // Falls back to an empty list if the registry is inaccessible.
    use std::process::Command;

    let mut found = Vec::new();

    // Use `reg query` to enumerate uninstall keys (safe, no extra crate needed)
    let keys = [
        r"HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
    ];

    for key in &keys {
        let Ok(output) = Command::new("reg")
            .args(["query", key, "/s", "/v", "DisplayName"])
            .output()
        else {
            continue;
        };
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let line = line.trim();
            if !line.contains("REG_SZ") { continue; }
            // Line format: "    DisplayName    REG_SZ    <value>"
            let parts: Vec<&str> = line.splitn(3, "REG_SZ").collect();
            if parts.len() < 2 { continue; }
            let display_name = parts[1].trim().to_string();
            let name_lower = display_name.to_lowercase().replace(' ', "");

            if let Some(&(exe, display)) = KNOWN_IDES.iter().find(|(k, _)| {
                name_lower.contains(k) || k.contains(&name_lower.as_str())
            }) {
                if !found.iter().any(|p: &DetectedProgram| p.name == display) {
                    found.push(DetectedProgram {
                        name: display.to_string(),
                        executable: exe.to_string(),
                    });
                }
            }
        }
    }
    found
}
