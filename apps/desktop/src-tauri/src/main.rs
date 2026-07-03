//! DevPilot desktop entry point.
//!
//! This binary is the composition root of the application. As features land,
//! dependency wiring moves to a dedicated `di` module and IPC handlers to a
//! `commands` module; business logic always stays in the workspace crates
//! (see ADR-0001).

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // A failure here means the app cannot start at all (broken config or
    // webview runtime), so aborting with the cause is the correct behavior.
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("failed to start DevPilot");
}
