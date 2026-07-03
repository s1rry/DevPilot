use std::fmt;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::history::CommitHash;

/// Unique identifier of a repository within DevPilot.
///
/// Produced by the git adapter from the repository source (for example a
/// normalized URL or an absolute path) and used as a cache key together
/// with [`CommitHash`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RepositoryId(String);

impl RepositoryId {
    /// Creates an identifier from any string-like value.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RepositoryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Where a repository comes from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RepositorySource {
    /// A repository that already exists on the local filesystem.
    LocalPath(PathBuf),
    /// A remote repository to clone, for example a GitHub URL.
    RemoteUrl(String),
}

/// A repository that has been opened and is ready for analysis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Repository {
    /// Stable identifier used for caching and cross-referencing.
    pub id: RepositoryId,
    /// Human-readable name, usually the project directory name.
    pub name: String,
    /// Location of the working copy on the local filesystem.
    pub local_path: PathBuf,
    /// Commit the analysis operates on (HEAD at the time of opening).
    pub head: CommitHash,
}
