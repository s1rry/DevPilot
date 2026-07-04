//! DevPilot desktop entry point.
//!
//! This binary is the composition root of the application. Business logic
//! lives in the workspace crates; here we only wire concrete adapters into
//! the `devpilot-core` ports (see [`di`]) and expose thin IPC commands
//! (see [`commands`]). See ADR-0001.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod di;
mod state;

use tauri::Manager;

fn main() {
    // A failure here means the app cannot start at all (broken config,
    // webview runtime, or unresolvable data directory), so aborting with the
    // cause is the correct behavior.
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let state = di::build_state(&app.handle().clone())?;
            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::open_folder,
            commands::clone_repository,
            commands::list_recent_projects,
            commands::remove_recent_project,
            commands::scan_folder,
            commands::parse_file,
            commands::analyze_architecture,
            commands::export_architecture,
        ])
        .run(tauri::generate_context!())
        .expect("failed to start DevPilot");
}
