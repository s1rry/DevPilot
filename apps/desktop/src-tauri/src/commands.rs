//! Thin IPC command handlers.
//!
//! Each handler deserializes its input, clones the injected ports out of
//! state, delegates to a `devpilot-core` use case, and maps the typed error
//! to a user-facing string. No business logic lives here (ADR-0001).

use std::path::PathBuf;

use devpilot_core::entities::{
    ArchitectureModel, FileAst, ProjectMetadata, RecentProject, RepositoryId, RepositorySource,
    ScanReport, SourceFile,
};
use devpilot_core::usecases::{
    AnalyzeArchitecture, ListRecentProjects, OpenProject, RemoveRecentProject, ScanRepository,
};
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
pub async fn remove_recent_project(id: String, state: State<'_, AppState>) -> Result<(), String> {
    let use_case = RemoveRecentProject::new(state.recent.clone());
    use_case
        .execute(&RepositoryId::new(id))
        .await
        .map_err(|error| error.to_string())
}

/// Scans a local project folder: languages, frameworks, dependencies,
/// structure and git information.
#[tauri::command]
pub async fn scan_folder(path: String, state: State<'_, AppState>) -> Result<ScanReport, String> {
    let use_case = ScanRepository::new(state.git.clone(), state.scanner.clone());
    use_case
        .execute(RepositorySource::LocalPath(PathBuf::from(path)))
        .await
        .map_err(|error| error.to_string())
}

/// Parses one source file into its AST model (functions, classes,
/// interfaces, imports, exports).
#[tauri::command]
pub async fn parse_file(path: String, state: State<'_, AppState>) -> Result<FileAst, String> {
    let path = PathBuf::from(path);
    let content = tokio::fs::read_to_string(&path)
        .await
        .map_err(|error| format!("reading {}: {error}", path.display()))?;
    let file = SourceFile { path, content };
    state
        .analyzer
        .parse(&file)
        .await
        .map_err(|error| error.to_string())
}

/// Analyzes a project's architecture into the folder, dependency, module and
/// call graphs.
#[tauri::command]
pub async fn analyze_architecture(
    path: String,
    state: State<'_, AppState>,
) -> Result<ArchitectureModel, String> {
    let use_case = AnalyzeArchitecture::new(state.git.clone(), state.analyzer.clone());
    use_case
        .execute(RepositorySource::LocalPath(PathBuf::from(path)))
        .await
        .map_err(|error| error.to_string())
}

/// Analyzes a project's architecture and writes the model to `out_path` as
/// pretty-printed JSON. Returns the path written.
#[tauri::command]
pub async fn export_architecture(
    path: String,
    out_path: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let use_case = AnalyzeArchitecture::new(state.git.clone(), state.analyzer.clone());
    let model = use_case
        .execute(RepositorySource::LocalPath(PathBuf::from(path)))
        .await
        .map_err(|error| error.to_string())?;
    let json =
        serde_json::to_string_pretty(&model).map_err(|error| format!("serialize: {error}"))?;
    tokio::fs::write(&out_path, json)
        .await
        .map_err(|error| format!("writing {out_path}: {error}"))?;
    Ok(out_path)
}
