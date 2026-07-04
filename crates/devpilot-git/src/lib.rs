//! # devpilot-git
//!
//! Git adapter for DevPilot. Implements the [`GitReader`] port from
//! `devpilot-core` on top of libgit2 (the `git2` crate).
//!
//! libgit2 handles are blocking and not `Send`, so every method re-opens the
//! repository from its path inside `spawn_blocking`. This keeps the async
//! runtime unblocked and avoids carrying non-`Send` values across `.await`.
//!
//! ## Rules
//!
//! - No shelling out to a system `git` binary.
//! - `git2` types never leak out of this crate.

use std::path::{Path, PathBuf};

use async_trait::async_trait;
use devpilot_core::entities::{
    CommitHash, CommitInfo, FileChurn, FileNode, FileTree, Language, Repository, RepositoryId,
    RepositorySource,
};
use devpilot_core::errors::GitError;
use devpilot_core::ports::GitReader;

mod convert;

use convert::{
    build_file_tree, build_history, collect_churn, count_commits, describe_repository,
    read_file_at_head, short_branch,
};

/// A [`GitReader`] backed by libgit2.
///
/// Remote repositories are cloned into `clone_base`; local repositories are
/// read in place.
pub struct Git2Reader {
    clone_base: PathBuf,
}

impl Git2Reader {
    /// Creates a reader that clones remote repositories under `clone_base`.
    pub fn new(clone_base: impl Into<PathBuf>) -> Self {
        Self {
            clone_base: clone_base.into(),
        }
    }
}

/// Runs a blocking git operation off the async runtime.
///
/// The closure receives nothing; it opens the repository itself so no
/// non-`Send` handle crosses the thread boundary.
async fn blocking<T, F>(work: F) -> Result<T, GitError>
where
    T: Send + 'static,
    F: FnOnce() -> Result<T, GitError> + Send + 'static,
{
    tokio::task::spawn_blocking(work)
        .await
        .map_err(|join| GitError::Backend(format!("git task failed: {join}")))?
}

/// Opens a local repository, mapping a missing repository to a typed error.
fn open_local(path: &Path) -> Result<git2::Repository, GitError> {
    git2::Repository::open(path).map_err(|error| match error.code() {
        git2::ErrorCode::NotFound => GitError::NotARepository {
            path: path.to_path_buf(),
        },
        _ => GitError::Backend(error.message().to_string()),
    })
}

impl Git2Reader {
    /// Resolves the destination directory for cloning `url`.
    fn clone_destination(&self, url: &str) -> PathBuf {
        let name = url
            .trim_end_matches('/')
            .rsplit('/')
            .next()
            .unwrap_or("repository")
            .trim_end_matches(".git");
        self.clone_base
            .join(if name.is_empty() { "repository" } else { name })
    }
}

#[async_trait]
impl GitReader for Git2Reader {
    async fn open(&self, source: &RepositorySource) -> Result<Repository, GitError> {
        match source {
            RepositorySource::LocalPath(path) => {
                let path = path.clone();
                blocking(move || {
                    let repo = open_local(&path)?;
                    describe_repository(&repo, &path)
                })
                .await
            }
            RepositorySource::RemoteUrl(url) => {
                let url = url.clone();
                let dest = self.clone_destination(&url);
                blocking(move || {
                    // Reopen an existing clone instead of failing on re-open.
                    let repo = if dest.join(".git").exists() {
                        open_local(&dest)?
                    } else {
                        git2::Repository::clone(&url, &dest).map_err(|error| {
                            GitError::CloneFailed {
                                url: url.clone(),
                                reason: error.message().to_string(),
                            }
                        })?
                    };
                    describe_repository(&repo, &dest)
                })
                .await
            }
        }
    }

    async fn file_tree(&self, repository: &Repository) -> Result<FileTree, GitError> {
        let path = repository.local_path.clone();
        blocking(move || {
            let repo = open_local(&path)?;
            build_file_tree(&repo)
        })
        .await
    }

    async fn history(
        &self,
        repository: &Repository,
        limit: usize,
    ) -> Result<Vec<CommitInfo>, GitError> {
        let path = repository.local_path.clone();
        blocking(move || {
            let repo = open_local(&path)?;
            build_history(&repo, limit)
        })
        .await
    }

    async fn churn(&self, repository: &Repository) -> Result<Vec<FileChurn>, GitError> {
        let path = repository.local_path.clone();
        blocking(move || {
            let repo = open_local(&path)?;
            collect_churn(&repo)
        })
        .await
    }

    async fn read_file(&self, repository: &Repository, path: &Path) -> Result<String, GitError> {
        let repo_path = repository.local_path.clone();
        let file_path = path.to_path_buf();
        blocking(move || {
            let repo = open_local(&repo_path)?;
            read_file_at_head(&repo, &file_path)
        })
        .await
    }

    async fn current_branch(&self, repository: &Repository) -> Result<String, GitError> {
        let path = repository.local_path.clone();
        blocking(move || {
            let repo = open_local(&path)?;
            short_branch(&repo)
        })
        .await
    }

    async fn commit_count(&self, repository: &Repository) -> Result<usize, GitError> {
        let path = repository.local_path.clone();
        blocking(move || {
            let repo = open_local(&path)?;
            count_commits(&repo)
        })
        .await
    }
}

/// Detects the language of a path from its extension.
fn language_of(path: &Path) -> Language {
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(Language::from_extension)
        .unwrap_or(Language::Unknown)
}

/// Builds a [`RepositoryId`] from a filesystem path, canonicalizing when
/// possible so the same repository always maps to the same id.
fn repository_id(path: &Path) -> RepositoryId {
    let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    RepositoryId::new(canonical.to_string_lossy().to_string())
}

/// Extracts a display name from a repository path.
fn repository_name(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("repository")
        .to_string()
}

/// Converts a git2 commit id into a [`CommitHash`].
fn commit_hash(oid: git2::Oid) -> CommitHash {
    CommitHash::new(oid.to_string())
}

/// Wraps a blob and its path into a [`FileNode::File`].
fn file_node(path: PathBuf, size_bytes: u64) -> FileNode {
    let language = language_of(&path);
    FileNode::File {
        path,
        size_bytes,
        language,
    }
}
