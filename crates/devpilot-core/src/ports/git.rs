use std::path::Path;

use async_trait::async_trait;

use crate::entities::{CommitInfo, FileChurn, FileTree, Repository, RepositorySource};
use crate::errors::GitError;

/// Read access to git repositories.
///
/// Implemented by `devpilot-git` on top of libgit2 and by `MockGitReader`
/// in `devpilot-testing`. All paths are relative to the repository root;
/// all data is read at the repository's HEAD commit.
#[async_trait]
pub trait GitReader: Send + Sync {
    /// Opens a repository, cloning it first when the source is remote.
    ///
    /// Fails with [`GitError::NotARepository`] for paths that are not a
    /// repository and [`GitError::EmptyRepository`] when there are no
    /// commits to analyze.
    async fn open(&self, source: &RepositorySource) -> Result<Repository, GitError>;

    /// Returns the file tree at the repository's HEAD commit.
    async fn file_tree(&self, repository: &Repository) -> Result<FileTree, GitError>;

    /// Returns up to `limit` most recent commits, newest first.
    async fn history(
        &self,
        repository: &Repository,
        limit: usize,
    ) -> Result<Vec<CommitInfo>, GitError>;

    /// Returns per-file change statistics across the whole history.
    async fn churn(&self, repository: &Repository) -> Result<Vec<FileChurn>, GitError>;

    /// Reads the text content of one file at HEAD.
    async fn read_file(&self, repository: &Repository, path: &Path) -> Result<String, GitError>;
}
