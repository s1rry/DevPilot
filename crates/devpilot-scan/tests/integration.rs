//! Integration tests for the filesystem-backed scanner, against real manifest
//! files in a temporary directory.

use std::fs;
use std::path::PathBuf;

use devpilot_core::entities::{CommitHash, Ecosystem, Repository, RepositoryId};
use devpilot_core::ports::ProjectScanner;
use devpilot_scan::FsProjectScanner;
use tempfile::TempDir;

fn repository_at(dir: &TempDir) -> Repository {
    Repository {
        id: RepositoryId::new("test"),
        name: "test".to_string(),
        local_path: dir.path().to_path_buf(),
        head: CommitHash::new("0"),
    }
}

#[tokio::test]
async fn detects_across_multiple_manifests() {
    let dir = TempDir::new().expect("tempdir");
    fs::write(
        dir.path().join("package.json"),
        r#"{ "dependencies": { "react": "^18" } }"#,
    )
    .unwrap();
    fs::write(
        dir.path().join("Cargo.toml"),
        "[dependencies]\ntauri = \"2\"\n",
    )
    .unwrap();

    let scanner = FsProjectScanner::new();
    let detection = scanner.detect(&repository_at(&dir)).await.expect("detect");

    let names: Vec<&str> = detection
        .frameworks
        .iter()
        .map(|f| f.name.as_str())
        .collect();
    assert!(names.contains(&"React"));
    assert!(names.contains(&"Tauri"));

    assert!(detection
        .dependencies
        .iter()
        .any(|d| d.name == "react" && d.ecosystem == Ecosystem::Npm));
    assert!(detection
        .dependencies
        .iter()
        .any(|d| d.name == "tauri" && d.ecosystem == Ecosystem::Cargo));
}

#[tokio::test]
async fn empty_project_yields_empty_detection() {
    let dir = TempDir::new().expect("tempdir");
    let scanner = FsProjectScanner::new();

    let detection = scanner.detect(&repository_at(&dir)).await.expect("detect");

    assert!(detection.frameworks.is_empty());
    assert!(detection.dependencies.is_empty());
}

#[tokio::test]
async fn malformed_manifest_is_skipped_not_failed() {
    let dir = TempDir::new().expect("tempdir");
    fs::write(dir.path().join("package.json"), "{ not json").unwrap();
    let _ = PathBuf::new();

    let scanner = FsProjectScanner::new();
    let detection = scanner.detect(&repository_at(&dir)).await.expect("detect");

    assert!(detection.dependencies.is_empty());
}
