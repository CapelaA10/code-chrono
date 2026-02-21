// commands/mod.rs
//
// Declares all command sub-modules.
// lib.rs references each via its full path (e.g. commands::timer::start_pomodoro)
// so Tauri's generate_handler! macro can find the correct function symbols.

pub mod data;
pub mod projects;
pub mod settings;
pub mod stats;
pub mod sync;
pub mod tags;
pub mod tasks;
pub mod timer;
