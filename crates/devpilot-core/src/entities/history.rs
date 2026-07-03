use std::fmt;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Full hash of a git commit.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommitHash(String);

impl CommitHash {
    /// Creates a hash from any string-like value.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the hash as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CommitHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// One commit in the repository history.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitInfo {
    /// Full commit hash.
    pub hash: CommitHash,
    /// Author name as recorded in the commit.
    pub author_name: String,
    /// Author email as recorded in the commit.
    pub author_email: String,
    /// Commit time as Unix seconds (UTC).
    pub timestamp: i64,
    /// First line of the commit message.
    pub summary: String,
}

/// Contribution statistics of one author.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorStats {
    /// Author name.
    pub name: String,
    /// Author email; authors are aggregated by email.
    pub email: String,
    /// Number of commits by this author.
    pub commit_count: usize,
}

/// Change frequency of one file across the repository history.
///
/// High churn combined with high complexity marks a maintenance hotspot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileChurn {
    /// Path relative to the repository root.
    pub path: PathBuf,
    /// Number of commits that touched the file.
    pub commit_count: usize,
    /// Unix seconds (UTC) of the most recent change.
    pub last_modified: i64,
}
