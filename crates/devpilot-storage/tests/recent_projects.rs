//! Integration tests for the JSON recent-projects store, against real files
//! in a temporary directory.

use std::path::PathBuf;

use devpilot_core::entities::{RecentProject, RepositoryId, RepositorySource};
use devpilot_core::ports::RecentProjectsStore;
use devpilot_storage::JsonRecentProjectsStore;
use tempfile::TempDir;

fn project(id: &str) -> RecentProject {
    RecentProject {
        id: RepositoryId::new(id),
        name: id.to_string(),
        source: RepositorySource::LocalPath(PathBuf::from(format!("/tmp/{id}"))),
        local_path: PathBuf::from(format!("/tmp/{id}")),
        last_opened: 0,
    }
}

fn store(dir: &TempDir) -> JsonRecentProjectsStore {
    // Nested path exercises parent-directory creation.
    JsonRecentProjectsStore::new(dir.path().join("state").join("recent.json"))
}

#[tokio::test]
async fn missing_file_yields_empty_list() {
    let dir = TempDir::new().expect("tempdir");
    let store = store(&dir);

    assert!(store.list().await.expect("list").is_empty());
}

#[tokio::test]
async fn add_persists_and_stamps_last_opened() {
    let dir = TempDir::new().expect("tempdir");
    let store = store(&dir);

    store.add(&project("alpha")).await.expect("add");

    let listed = store.list().await.expect("list");
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, RepositoryId::new("alpha"));
    assert!(listed[0].last_opened > 0);
}

#[tokio::test]
async fn add_moves_existing_project_to_front_without_duplicating() {
    let dir = TempDir::new().expect("tempdir");
    let store = store(&dir);

    store.add(&project("alpha")).await.expect("add alpha");
    store.add(&project("beta")).await.expect("add beta");
    store.add(&project("alpha")).await.expect("re-add alpha");

    let listed = store.list().await.expect("list");
    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].id, RepositoryId::new("alpha"));
    assert_eq!(listed[1].id, RepositoryId::new("beta"));
}

#[tokio::test]
async fn remove_deletes_and_tolerates_missing() {
    let dir = TempDir::new().expect("tempdir");
    let store = store(&dir);
    store.add(&project("alpha")).await.expect("add");

    store
        .remove(&RepositoryId::new("alpha"))
        .await
        .expect("remove");
    assert!(store.list().await.expect("list").is_empty());

    // Removing something that is not there is not an error.
    store
        .remove(&RepositoryId::new("ghost"))
        .await
        .expect("remove missing");
}

#[tokio::test]
async fn list_is_capped_at_twenty_most_recent() {
    let dir = TempDir::new().expect("tempdir");
    let store = store(&dir);

    for index in 0..25 {
        store
            .add(&project(&format!("p{index}")))
            .await
            .expect("add");
    }

    let listed = store.list().await.expect("list");
    assert_eq!(listed.len(), 20);
    // The most recently added stays at the front.
    assert_eq!(listed[0].id, RepositoryId::new("p24"));
}
