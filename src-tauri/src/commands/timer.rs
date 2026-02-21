// commands/timer.rs
//
// Pomodoro timer commands: start, pause, reset, and the background tick loop.
// The timer state is a globally shared Mutex. Only one session runs at a time.

use std::sync::{Arc, Mutex};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, State};
use tauri::Emitter;
use tauri_plugin_notification::NotificationExt;

use crate::database::Database;

// ── TimerState ────────────────────────────────────────────────────────────

/// The application timer state, shared across Tauri commands via a Mutex.
/// Fields annotated with `#[serde(skip)]` are internal and not sent to the frontend.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TimerState {
    pub remaining:        u64,
    pub paused:           bool,
    /// 0 = work, 1 = short break, 2 = long break
    pub phase:            u8,
    pub task_active:      bool,
    pub active_task_name: Option<String>,
    #[serde(skip)]
    pub loop_running:     bool,
    pub session_duration: u64,
    #[serde(skip)]
    pub session_start_time: u64,
    pub last_activity:    u64,
}

impl Default for TimerState {
    fn default() -> Self {
        Self {
            remaining:          25 * 60,
            paused:             true,
            phase:              0,
            task_active:        false,
            active_task_name:   None,
            loop_running:       false,
            session_duration:   25 * 60,
            session_start_time: 0,
            last_activity:      now_secs(),
        }
    }
}

// ── Commands ──────────────────────────────────────────────────────────────

/// Start a new Pomodoro session.
/// If a session is already running this is a no-op (call `pause_timer` first).
#[tauri::command]
pub async fn start_pomodoro(
    state:    State<'_, Arc<Mutex<TimerState>>>,
    db_state: State<'_, Arc<Mutex<Database>>>,
    handle:   AppHandle,
    task_name: String,
    duration_minutes: Option<u64>,
) -> Result<(), String> {
    // Finalize the previous session if one was active
    let previous_session = {
        let t = state.lock().unwrap();
        if t.task_active {
            let name = t.active_task_name.clone().unwrap_or_default();
            let elapsed = if !name.is_empty() {
                t.session_duration.saturating_sub(t.remaining)
            } else { 0 };
            Some((name, elapsed, t.phase))
        } else {
            None
        }
    };

    if let Some((old_name, elapsed, phase)) = previous_session {
        if elapsed > 0 {
            db_state.lock().unwrap().log_session_complete(&old_name, elapsed, phase).unwrap_or(());
        }
    }

    let should_spawn_loop = {
        let mut timer = state.lock().unwrap();
        let was_running = timer.loop_running;
        
        let duration_secs = duration_minutes.unwrap_or(25) * 60;
        let now = now_secs();
        timer.remaining         = duration_secs;
        timer.session_duration  = duration_secs;
        timer.session_start_time = now;
        timer.last_activity     = now;
        timer.paused            = false;
        timer.phase             = 0;
        timer.task_active       = true;
        timer.active_task_name  = Some(task_name.clone());
        timer.loop_running      = true;
        
        !was_running
    };

    db_state.lock().unwrap().log_action(&task_name, "start", 0, 0).unwrap();
    if should_spawn_loop {
        spawn_tick_loop(Arc::clone(&*state), Arc::clone(&*db_state), handle.clone());
    }

    let snapshot = state.lock().unwrap().clone();
    let _ = handle.emit("timer-tick", snapshot);

    Ok(())
}

/// Toggle pause/resume. Also restarts the tick loop if we're resuming.
#[tauri::command]
pub fn pause_timer(
    state:    State<'_, Arc<Mutex<TimerState>>>,
    db_state: State<'_, Arc<Mutex<Database>>>,
    handle:   AppHandle,
) -> Result<(), String> {
    let should_resume = {
        let mut timer = state.lock().unwrap();
        let action = if timer.paused { "resume" } else { "pause" };
        let task_name = timer.active_task_name.clone().unwrap_or_default();
        db_state.lock().unwrap().log_action(&task_name, action, 0, timer.phase).unwrap_or(());

        if !timer.task_active && timer.remaining > 0 {
            timer.task_active  = true;
            timer.last_activity = now_secs();
        }
        timer.paused = !timer.paused;
        !timer.paused && timer.task_active && timer.remaining > 0 && !timer.loop_running
    };

    if should_resume {
        state.lock().unwrap().loop_running = true;
        spawn_tick_loop(Arc::clone(&*state), Arc::clone(&*db_state), handle.clone());
    }

    // Force an immediate UI update, covering the case when it pauses and the tick loop stops
    let snapshot = state.lock().unwrap().clone();
    let _ = handle.emit("timer-tick", snapshot);

    Ok(())
}

/// Return a snapshot of the current timer state.
#[tauri::command]
pub fn get_timer(state: State<'_, Arc<Mutex<TimerState>>>) -> Result<TimerState, String> {
    Ok(state.lock().unwrap().clone())
}

/// Stop the timer, log any elapsed time, and reset to the last used duration.
#[tauri::command]
pub fn reset_timer(
    state:    State<'_, Arc<Mutex<TimerState>>>,
    db_state: State<'_, Arc<Mutex<Database>>>,
    handle:   AppHandle,
) -> Result<(), String> {
    let (elapsed, phase, task_name) = {
        let t = state.lock().unwrap();
        let name = t.active_task_name.clone().unwrap_or_default();
        let elapsed = if t.task_active && !name.is_empty() {
            t.session_duration.saturating_sub(t.remaining)
        } else { 0 };
        (elapsed, t.phase, name)
    };

    if elapsed > 0 {
        db_state.lock().unwrap().log_session_complete(&task_name, elapsed, phase).unwrap_or(());
    }

    let mut t = state.lock().unwrap();
    let duration = t.session_duration; // preserve custom duration
    t.remaining          = duration;
    t.session_start_time = 0;
    t.paused             = true;
    t.phase              = 0;
    t.task_active        = false;
    t.active_task_name   = None;
    t.loop_running       = false;

    // Force an immediate UI update
    let snapshot = t.clone();
    drop(t); // avoid holding lock during emit if handle.emit blocks, although it's minimal
    let _ = handle.emit("timer-tick", snapshot);

    Ok(())
}

/// Record user activity (prevents idle auto-pause).
#[tauri::command]
pub fn record_activity(state: State<'_, Arc<Mutex<TimerState>>>) -> Result<(), String> {
    state.lock().unwrap().last_activity = now_secs();
    Ok(())
}

/// Return the N most recently tracked task names (for autocomplete).
#[tauri::command]
pub fn get_unique_task_names(
    db_state: State<'_, Arc<Mutex<Database>>>,
    limit: i64,
) -> Result<Vec<String>, String> {
    db_state.lock().unwrap().get_unique_task_names(limit).map_err(|e| e.to_string())
}

// ── Private helpers ───────────────────────────────────────────────────────

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Spawns a one-second-tick background loop in Tauri's async runtime.
/// The loop stops automatically when the timer is paused, complete, or idle.
fn spawn_tick_loop(
    state:    Arc<Mutex<TimerState>>,
    db:       Arc<Mutex<Database>>,
    handle:   AppHandle,
) {
    tauri::async_runtime::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;

            let mut timer = state.lock().unwrap();

            // Exit conditions
            if timer.paused || !timer.task_active {
                timer.loop_running = false;
                break;
            }

            // Auto-pause on 2 minutes of inactivity
            if now_secs() - timer.last_activity > 120 {
                timer.paused       = true;
                timer.loop_running = false;
                let payload = timer.clone();
                drop(timer);
                let _ = handle.emit("timer-tick", payload);
                break;
            }

            if timer.remaining > 0 {
                timer.remaining -= 1;
                let payload = timer.clone();
                drop(timer);
                let _ = handle.emit("timer-tick", payload);
            } else {
                // Session complete
                let phase    = timer.phase;
                let duration = timer.session_duration;
                let task_name = timer.active_task_name.clone().unwrap_or_default();
                timer.task_active  = false;
                timer.active_task_name = None;
                timer.loop_running = false;
                drop(timer);

                db.lock().unwrap().log_session_complete(&task_name, duration, phase).unwrap_or(());

                let _ = handle
                    .notification()
                    .builder()
                    .title("Code Chrono")
                    .body(if phase == 0 { "Work session done! 🎉" } else { "Break over! 💪" })
                    .show();

                break;
            }
        }
    });
}
