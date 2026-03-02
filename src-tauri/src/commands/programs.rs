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
    // ── AI-first IDEs ─────────────────────────────────────────────────────
    ("cursor",          "Cursor"),
    ("windsurf",        "Windsurf"),
    ("pearai",          "PearAI"),
    ("void",            "Void"),
    ("trae",            "Trae"),
    ("aide",            "Aide"),
    ("melty",           "Melty"),
    ("antigravity",     "Antigravity"),
    // ── Visual Studio Code variants ────────────────────────────────────────
    ("code",            "Visual Studio Code"),
    ("code-insiders",   "VS Code Insiders"),
    // ── JetBrains ─────────────────────────────────────────────────────────
    ("idea",            "IntelliJ IDEA"),
    ("pycharm",         "PyCharm"),
    ("webstorm",        "WebStorm"),
    ("goland",          "GoLand"),
    ("clion",           "CLion"),
    ("rider",           "Rider"),
    ("rubymine",        "RubyMine"),
    ("datagrip",        "DataGrip"),
    ("phpstorm",        "PhpStorm"),
    ("appcode",         "AppCode"),
    ("fleet",           "Fleet"),
    // ── Apple ─────────────────────────────────────────────────────────────
    ("xcode",           "Xcode"),
    // ── Google / Android ──────────────────────────────────────────────────
    ("androidstudio",   "Android Studio"),
    ("studio",          "Android Studio"),
    // ── Text editors ──────────────────────────────────────────────────────
    ("sublime_text",    "Sublime Text"),
    ("sublimetext",     "Sublime Text"),
    ("zed",             "Zed"),
    ("zeditor",         "Zed"),
    ("atom",            "Atom"),
    ("nova",            "Nova"),
    ("bbedit",          "BBEdit"),
    // ── Terminal editors ──────────────────────────────────────────────────
    ("nvim",            "Neovim"),
    ("vim",             "Vim"),
    ("emacs",           "Emacs"),
    ("nano",            "Nano"),
    // ── Misc dev tools ────────────────────────────────────────────────────
    ("tableplus",       "TablePlus"),
    ("dbeaver",         "DBeaver"),
    ("insomnia",        "Insomnia"),
    ("postman",         "Postman"),
    ("docker",          "Docker Desktop"),
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
/// emit "program-opened" ONLY when a program transitions from not-running → running.
/// This prevents the modal from re-appearing if the program was already open when
/// the 15-minute cooldown window expired without a real launch event.
pub fn spawn_program_watcher(db: Arc<Mutex<Database>>, handle: AppHandle) {
    tauri::async_runtime::spawn(async move {
        use std::collections::{HashMap, HashSet};
        use std::time::{Duration, Instant};
        use sysinfo::System;

        let mut sys = System::new_all();
        let mut last_notified: HashMap<String, Instant> = HashMap::new();
        let cooldown = Duration::from_secs(15 * 60);

        // prev_detected: the *tracked program executable keys* (not all process
        // names) that were observed running in the previous poll cycle.
        // Using only tracked-program keys avoids false matches from unrelated
        // system processes whose names happen to be substrings of IDE names.
        let mut prev_detected: HashSet<String> = HashSet::new();

        // Seed prev_detected from the programs that are already open at startup
        // so we don't immediately fire for an IDE the user already had running.
        sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
        if let Ok(initial) = db.lock().unwrap().get_tracked_programs() {
            for p in initial.iter().filter(|p| p.enabled) {
                if process_is_running(&sys, &p.executable) {
                    prev_detected.insert(p.executable.clone());
                }
            }
        }

        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;

            let notify_enabled = db
                .lock()
                .unwrap()
                .get_setting("notify_on_program_open")
                .unwrap_or(None)
                .map(|v| v != "false")
                .unwrap_or(true);

            // Refresh process list every cycle regardless of the setting so
            // prev_detected stays accurate and won't fire stale events when
            // the user re-enables the notification setting.
            sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

            if !notify_enabled {
                // Re-seed so the state is fresh when notifications are re-enabled.
                if let Ok(tracked) = db.lock().unwrap().get_tracked_programs() {
                    prev_detected.clear();
                    for p in tracked.iter().filter(|p| p.enabled) {
                        if process_is_running(&sys, &p.executable) {
                            prev_detected.insert(p.executable.clone());
                        }
                    }
                }
                continue;
            }

            let tracked = match db.lock().unwrap().get_tracked_programs() {
                Ok(t) => t,
                Err(_) => continue,
            };
            let enabled: Vec<TrackedProgram> = tracked.into_iter().filter(|p| p.enabled).collect();

            // curr_detected holds only the tracked executables running THIS cycle.
            let mut curr_detected: HashSet<String> = HashSet::new();

            for program in &enabled {
                let is_running = process_is_running(&sys, &program.executable);

                if !is_running {
                    // Emit a closed event if the program just stopped running.
                    if prev_detected.contains(&program.executable) {
                        let _ = handle.emit(
                            "program-closed",
                            serde_json::json!({ "name": program.name, "executable": program.executable }),
                        );
                    }
                    prev_detected.remove(&program.executable);
                    continue;
                }

                curr_detected.insert(program.executable.clone());

                // Only notify on a not-running → running transition.
                let just_opened = !prev_detected.contains(&program.executable);
                if !just_opened {
                    continue;
                }

                // Respect the 15-minute cooldown for genuine re-launches.
                if let Some(last) = last_notified.get(&program.executable) {
                    if last.elapsed() < cooldown {
                        continue;
                    }
                }

                let _ = handle.emit(
                    "program-opened",
                    serde_json::json!({ "name": program.name, "executable": program.executable }),
                );
                last_notified.insert(program.executable.clone(), Instant::now());
            }

            // Advance the tracked-only snapshot (not all system processes).
            prev_detected = curr_detected;
        }
    });
}

/// Returns true if any running process name matches the tracked executable.
/// Strips `.exe` / `.app` extensions before comparing so "code" matches "Code.exe".
fn process_is_running(sys: &sysinfo::System, executable: &str) -> bool {
    let exe_lower = executable.to_lowercase();
    // Strip common suffixes so platform-specific names compare cleanly.
    let exe_stem = exe_lower
        .trim_end_matches(".exe")
        .trim_end_matches(".app");

    sys.processes().values().any(|proc| {
        let raw = proc.name().to_string_lossy().to_lowercase();
        let name = raw
            .trim_end_matches(".exe")
            .trim_end_matches(".app");

        // Exact match after stripping extension.
        name == exe_stem
            // Process name starts with the tracked stem (e.g. "webstorm64" matches "webstorm",
            // "code-insiders" matches "code"). Only block if the next char is a plain letter
            // so "codecov" does NOT match "code", but "webstorm64" and "studio64" still do.
            || (name.starts_with(exe_stem)
                && name[exe_stem.len()..]
                    .chars()
                    .next()
                    .map(|c| !c.is_ascii_alphabetic())
                    .unwrap_or(true))
            // Executable contains the process name (e.g. "visual studio code" contains "code").
            || exe_stem.contains(name)
    })
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
