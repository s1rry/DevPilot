use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::repository::{RepositoryId, RepositorySource};

/// An entry in the "recent projects" list.
///
/// Persisted by the recent-projects store so a user can reopen a project
/// with one click. `last_opened` is stamped by the store on insert, so the
/// domain never needs a clock to record it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecentProject {
    /// Stable identifier, shared with [`RepositoryId`].
    pub id: RepositoryId,
    /// Display name of the project.
    pub name: String,
    /// Where the project came from (local folder or remote clone).
    pub source: RepositorySource,
    /// Absolute path of the working copy on disk.
    pub local_path: PathBuf,
    /// Unix seconds (UTC) of the last time the project was opened.
    pub last_opened: i64,
}
