//! Integration tests for the libgit2-backed [`Git2Reader`], exercised against
//! real temporary repositories built programmatically (no committed `.git`).

use std::fs;
use std::path::{Path, PathBuf};

use devpilot_core::entities::{Language, RepositorySource};
use devpilot_core::errors::GitError;
use devpilot_core::ports::GitReader;
use devpilot_git::Git2Reader;
use tempfile::TempDir;

/// Writes `content` to `dir/relative`, creating parent directories.
fn write(dir: &Path, relative: &str, content: &str) {
    let path = dir.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create dirs");
    }
    fs::write(path, content).expect("write file");
}

/// Stages `relative` and creates a commit, returning the new commit id.
fn commit(repo: &git2::Repository, relative_paths: &[&str], message: &str) -> git2::Oid {
    let mut index = repo.index().expect("index");
    for relative in relative_paths {
        index.add_path(Path::new(relative)).expect("add_path");
    }
    index.write().expect("index write");
    let tree_oid = index.write_tree().expect("write_tree");
    let tree = repo.find_tree(tree_oid).expect("find_tree");
    let signature = git2::Signature::now("Tester", "tester@example.com").expect("signature");

    let parents = match repo.head().ok().and_then(|head| head.peel_to_commit().ok()) {
        Some(parent) => vec![parent],
        None => vec![],
    };
    let parent_refs: Vec<&git2::Commit> = parents.iter().collect();

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &parent_refs,
    )
    .expect("commit")
}

/// Builds a two-commit sample repository and returns its temp dir.
fn sample_repo() -> TempDir {
    let dir = TempDir::new().expect("tempdir");
    let repo = git2::Repository::init(dir.path()).expect("init");

    write(dir.path(), "README.md", "# sample\n");
    commit(&repo, &["README.md"], "Initial commit");

    write(
        dir.path(),
        "src/main.rs",
        "fn main() {\n    println!(\"hi\");\n}\n",
    );
    write(
        dir.path(),
        "src/lib.rs",
        "pub fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n",
    );
    commit(&repo, &["src/main.rs", "src/lib.rs"], "Add source files");

    dir
}

fn local(dir: &TempDir) -> RepositorySource {
    RepositorySource::LocalPath(dir.path().to_path_buf())
}

#[tokio::test]
async fn opens_local_repository_with_head() {
    let dir = sample_repo();
    let reader = Git2Reader::new(PathBuf::from("/tmp"));

    let repository = reader.open(&local(&dir)).await.expect("open");

    assert!(!repository.head.as_str().is_empty());
    assert!(!repository.name.is_empty());
}

#[tokio::test]
async fn reads_file_tree_with_languages() {
    let dir = sample_repo();
    let reader = Git2Reader::new(PathBuf::from("/tmp"));
    let repository = reader.open(&local(&dir)).await.expect("open");

    let tree = reader.file_tree(&repository).await.expect("tree");

    assert_eq!(tree.file_count(), 3);
    assert!(tree.total_size_bytes() > 0);
    let counts = tree.language_counts();
    assert_eq!(counts.get(&Language::Rust), Some(&2));
}

#[tokio::test]
async fn reads_history_and_commit_count() {
    let dir = sample_repo();
    let reader = Git2Reader::new(PathBuf::from("/tmp"));
    let repository = reader.open(&local(&dir)).await.expect("open");

    let history = reader.history(&repository, 10).await.expect("history");
    assert_eq!(history.len(), 2);
    assert_eq!(history[0].summary, "Add source files");
    assert_eq!(history[0].author_name, "Tester");

    let count = reader.commit_count(&repository).await.expect("count");
    assert_eq!(count, 2);
}

#[tokio::test]
async fn reports_current_branch() {
    let dir = sample_repo();
    let reader = Git2Reader::new(PathBuf::from("/tmp"));
    let repository = reader.open(&local(&dir)).await.expect("open");

    let branch = reader.current_branch(&repository).await.expect("branch");
    assert!(!branch.is_empty());
}

#[tokio::test]
async fn reads_and_misses_files() {
    let dir = sample_repo();
    let reader = Git2Reader::new(PathBuf::from("/tmp"));
    let repository = reader.open(&local(&dir)).await.expect("open");

    let content = reader
        .read_file(&repository, Path::new("src/lib.rs"))
        .await
        .expect("read_file");
    assert!(content.contains("pub fn add"));

    let missing = reader
        .read_file(&repository, Path::new("does/not/exist.rs"))
        .await;
    assert_eq!(
        missing,
        Err(GitError::FileNotFound {
            path: PathBuf::from("does/not/exist.rs")
        })
    );
}

#[tokio::test]
async fn computes_churn_per_file() {
    let dir = sample_repo();
    let reader = Git2Reader::new(PathBuf::from("/tmp"));
    let repository = reader.open(&local(&dir)).await.expect("open");

    let churn = reader.churn(&repository).await.expect("churn");

    // Three files, each touched by exactly one commit.
    assert_eq!(churn.len(), 3);
    assert!(churn.iter().all(|entry| entry.commit_count == 1));
}

#[tokio::test]
async fn open_on_non_repository_is_typed_error() {
    let dir = TempDir::new().expect("tempdir");
    let reader = Git2Reader::new(PathBuf::from("/tmp"));

    let result = reader
        .open(&RepositorySource::LocalPath(dir.path().to_path_buf()))
        .await;

    assert!(matches!(result, Err(GitError::NotARepository { .. })));
}
