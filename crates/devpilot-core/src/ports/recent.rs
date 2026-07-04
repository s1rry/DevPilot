use async_trait::async_trait;

use crate::entities::{RecentProject, RepositoryId};
use crate::errors::StoreError;

/// Persistent store of the recent-projects list.
///
/// Implemented by `devpilot-storage` (a JSON file) and by
/// `MockRecentProjectsStore` in `devpilot-testing`. Implementations stamp
/// `last_opened` on [`add`](RecentProjectsStore::add) and keep the list
/// bounded and ordered by recency.
#[async_trait]
pub trait RecentProjectsStore: Send + Sync {
    /// Returns the recent projects, most recently opened first.
    async fn list(&self) -> Result<Vec<RecentProject>, StoreError>;

    /// Inserts or refreshes a project, stamping it as opened now.
    ///
    /// If a project with the same id already exists it is moved to the top;
    /// otherwise it is added. The stored `last_opened` value is set by the
    /// implementation, so the `last_opened` field of `project` is ignored.
    async fn add(&self, project: &RecentProject) -> Result<(), StoreError>;

    /// Removes a project from the list; a missing id is not an error.
    async fn remove(&self, id: &RepositoryId) -> Result<(), StoreError>;
}
