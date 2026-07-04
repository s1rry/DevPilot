//! Composition root: the one place that knows every concrete adapter and
//! wires them into [`AppState`]. Changing an implementation (e.g. swapping
//! the JSON store for SQLite) touches only this file.

use std::sync::Arc;

use devpilot_analysis::TreeSitterAnalyzer;
use devpilot_git::Git2Reader;
use devpilot_scan::FsProjectScanner;
use devpilot_storage::JsonRecentProjectsStore;
use tauri::{AppHandle, Manager};

use crate::state::AppState;

/// Builds the application state, resolving storage locations under the
/// platform's application data directory.
///
/// - Remote repositories clone into `<app-data>/clones`.
/// - The recent-projects list lives in `<app-data>/recent-projects.json`.
pub fn build_state(app: &AppHandle) -> Result<AppState, String> {
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("could not resolve app data directory: {error}"))?;

    let git = Arc::new(Git2Reader::new(data_dir.join("clones")));
    let recent = Arc::new(JsonRecentProjectsStore::new(
        data_dir.join("recent-projects.json"),
    ));
    let scanner = Arc::new(FsProjectScanner::new());
    let analyzer = Arc::new(TreeSitterAnalyzer::new());

    Ok(AppState {
        git,
        recent,
        scanner,
        analyzer,
    })
}
