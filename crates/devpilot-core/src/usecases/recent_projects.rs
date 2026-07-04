use std::sync::Arc;

use crate::entities::{RecentProject, RepositoryId};
use crate::errors::StoreError;
use crate::ports::RecentProjectsStore;

/// Returns the recent-projects list, most recent first.
pub struct ListRecentProjects {
    recent: Arc<dyn RecentProjectsStore>,
}

impl ListRecentProjects {
    /// Creates the use case from its dependency.
    pub fn new(recent: Arc<dyn RecentProjectsStore>) -> Self {
        Self { recent }
    }

    /// Executes the query.
    pub async fn execute(&self) -> Result<Vec<RecentProject>, StoreError> {
        self.recent.list().await
    }
}

/// Removes one project from the recent-projects list.
pub struct RemoveRecentProject {
    recent: Arc<dyn RecentProjectsStore>,
}

impl RemoveRecentProject {
    /// Creates the use case from its dependency.
    pub fn new(recent: Arc<dyn RecentProjectsStore>) -> Self {
        Self { recent }
    }

    /// Removes the project with the given id; a missing id is not an error.
    pub async fn execute(&self, id: &RepositoryId) -> Result<(), StoreError> {
        self.recent.remove(id).await
    }
}
