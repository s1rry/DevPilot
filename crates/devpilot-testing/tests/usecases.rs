//! Tests for the `devpilot-core` project use cases, driven by the shared
//! mocks. These live here so `devpilot-core` needs no dev-dependency on
//! `devpilot-testing`.

use std::sync::Arc;

use devpilot_core::entities::{
    Dependency, Detection, Ecosystem, Framework, FrameworkCategory, Language, RepositoryId,
};
use devpilot_core::errors::{GitError, ProjectError, RepoScanError, ScanError, StoreError};
use devpilot_core::ports::RecentProjectsStore;
use devpilot_core::usecases::{
    ListRecentProjects, OpenProject, RemoveRecentProject, ScanRepository,
};
use devpilot_testing::fixtures;
use devpilot_testing::mocks::{MockGitReader, MockProjectScanner, MockRecentProjectsStore};

#[tokio::test]
async fn open_project_builds_metadata_and_records_recent() {
    let git = Arc::new(MockGitReader::new().with_branch("develop"));
    let store = Arc::new(MockRecentProjectsStore::new());
    let use_case = OpenProject::new(git.clone(), store.clone());

    let metadata = use_case
        .execute(fixtures::sample_local_source())
        .await
        .expect("open should succeed");

    // Metadata assembled from the sample fixtures (3 files: 2 Rust, 1 unknown).
    assert_eq!(metadata.name, "sample");
    assert_eq!(metadata.branch, "develop");
    assert_eq!(metadata.file_count, 3);
    assert_eq!(metadata.commit_count, 2);
    assert_eq!(metadata.total_size_bytes, 230);
    assert_eq!(metadata.languages[0].language, Language::Rust);
    assert_eq!(metadata.languages[0].file_count, 2);

    // The project was recorded exactly once, with a store-stamped timestamp.
    let recent = store.list().await.expect("list");
    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0].id, RepositoryId::new("fixture/sample"));
    assert_eq!(recent[0].last_opened, 1);
}

#[tokio::test]
async fn open_project_propagates_git_error_without_recording() {
    let git = Arc::new(MockGitReader::new().with_open_error(GitError::EmptyRepository));
    let store = Arc::new(MockRecentProjectsStore::new());
    let use_case = OpenProject::new(git, store.clone());

    let result = use_case.execute(fixtures::sample_local_source()).await;

    assert_eq!(result, Err(ProjectError::Git(GitError::EmptyRepository)));
    assert!(store.is_empty());
}

#[tokio::test]
async fn open_project_propagates_store_error() {
    let git = Arc::new(MockGitReader::new());
    let store = Arc::new(MockRecentProjectsStore::failing(StoreError::Backend(
        "disk full".into(),
    )));
    let use_case = OpenProject::new(git, store);

    let result = use_case.execute(fixtures::sample_local_source()).await;

    assert_eq!(
        result,
        Err(ProjectError::Store(StoreError::Backend("disk full".into())))
    );
}

#[tokio::test]
async fn list_recent_projects_returns_stored_entries() {
    let store =
        Arc::new(MockRecentProjectsStore::new().with_project(fixtures::sample_recent_project()));
    let use_case = ListRecentProjects::new(store);

    let projects = use_case.execute().await.expect("list");

    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].name, "sample");
}

#[tokio::test]
async fn remove_recent_project_deletes_entry() {
    let store =
        Arc::new(MockRecentProjectsStore::new().with_project(fixtures::sample_recent_project()));
    let use_case = RemoveRecentProject::new(store.clone());

    use_case
        .execute(&RepositoryId::new("fixture/sample"))
        .await
        .expect("remove");

    assert!(store.is_empty());
}

#[tokio::test]
async fn scan_repository_assembles_full_report() {
    let git = Arc::new(MockGitReader::new().with_branch("main"));
    let detection = Detection {
        frameworks: vec![Framework {
            name: "React".into(),
            category: FrameworkCategory::Frontend,
            source: "package.json".into(),
        }],
        dependencies: vec![Dependency {
            name: "react".into(),
            version: Some("^18".into()),
            ecosystem: Ecosystem::Npm,
        }],
    };
    let scanner = Arc::new(MockProjectScanner::new().with_detection(detection));
    let use_case = ScanRepository::new(git, scanner);

    let report = use_case
        .execute(fixtures::sample_local_source())
        .await
        .expect("scan should succeed");

    // Languages from the sample tree: 2 Rust, 1 unknown.
    assert_eq!(report.languages[0].language, Language::Rust);
    assert_eq!(report.languages[0].file_count, 2);

    // Structure: sample tree has one top-level dir `src`, three files.
    assert_eq!(report.structure.total_files, 3);
    assert_eq!(report.structure.top_level_dirs, vec!["src".to_string()]);
    assert_eq!(report.structure.notable, vec!["src".to_string()]);

    // Git: two commits, two contributors, newest commit is last_commit.
    assert_eq!(report.git.branch, "main");
    assert_eq!(report.git.commit_count, 2);
    assert_eq!(report.git.contributors.len(), 2);
    assert_eq!(
        report.git.last_commit.as_ref().unwrap().summary,
        "Add library module"
    );

    // Detection passed through.
    assert_eq!(report.frameworks[0].name, "React");
    assert_eq!(report.dependencies[0].name, "react");
}

#[tokio::test]
async fn scan_repository_propagates_git_error() {
    let git = Arc::new(MockGitReader::new().with_open_error(GitError::EmptyRepository));
    let scanner = Arc::new(MockProjectScanner::new());
    let use_case = ScanRepository::new(git, scanner);

    let result = use_case.execute(fixtures::sample_local_source()).await;

    assert_eq!(result, Err(RepoScanError::Git(GitError::EmptyRepository)));
}

#[tokio::test]
async fn scan_repository_propagates_scan_error() {
    let git = Arc::new(MockGitReader::new());
    let scanner = Arc::new(MockProjectScanner::failing(ScanError::Backend("io".into())));
    let use_case = ScanRepository::new(git, scanner);

    let result = use_case.execute(fixtures::sample_local_source()).await;

    assert_eq!(
        result,
        Err(RepoScanError::Scan(ScanError::Backend("io".into())))
    );
}
