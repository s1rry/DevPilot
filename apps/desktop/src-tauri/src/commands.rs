//! Thin IPC command handlers.
//!
//! Each handler deserializes its input, clones the injected ports out of
//! state, delegates to a `devpilot-core` use case, and maps the typed error
//! to a user-facing string. No business logic lives here (ADR-0001).

use std::path::PathBuf;

use devpilot_core::entities::{ProjectMetadata, RecentProject, RepositoryId, RepositorySource};
use devpilot_core::usecases::{ListRecentProjects, OpenProject, RemoveRecentProject};
use tauri::State;

use crate::state::AppState;

/// Opens a local folder as a project and records it as recently opened.
#[tauri::command]
pub async fn open_folder(
    path: String,
    state: State<'_, AppState>,
) -> Result<ProjectMetadata, String> {
    let use_case = OpenProject::new(state.git.clone(), state.recent.clone());
    use_case
        .execute(RepositorySource::LocalPath(PathBuf::from(path)))
        .await
        .map_err(|error| error.to_string())
}

/// Clones a remote repository, opens it, and records it as recently opened.
#[tauri::command]
pub async fn clone_repository(
    url: String,
    state: State<'_, AppState>,
) -> Result<ProjectMetadata, String> {
    let use_case = OpenProject::new(state.git.clone(), state.recent.clone());
    use_case
        .execute(RepositorySource::RemoteUrl(url))
        .await
        .map_err(|error| error.to_string())
}

/// Returns the recent-projects list, most recently opened first.
#[tauri::command]
pub async fn list_recent_projects(
    state: State<'_, AppState>,
) -> Result<Vec<RecentProject>, String> {
    let use_case = ListRecentProjects::new(state.recent.clone());
    use_case.execute().await.map_err(|error| error.to_string())
}

/// Removes one project from the recent-projects list.
#[tauri::command]
pub async fn remove_recent_project(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let use_case = RemoveRecentProject::new(state.recent.clone());
    use_case
        .execute(&RepositoryId::new(id))
        .await
        .map_err(|error| error.to_string())
}
