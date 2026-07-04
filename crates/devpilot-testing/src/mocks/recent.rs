use std::sync::Mutex;

use async_trait::async_trait;
use devpilot_core::entities::{RecentProject, RepositoryId};
use devpilot_core::errors::StoreError;
use devpilot_core::ports::RecentProjectsStore;

/// In-memory [`RecentProjectsStore`] for tests, with optional injected error.
///
/// Mirrors the real store's contract: `add` upserts by id and moves the
/// entry to the front, stamping a fixed `last_opened` so assertions are
/// deterministic.
pub struct MockRecentProjectsStore {
    projects: Mutex<Vec<RecentProject>>,
    error: Option<StoreError>,
    stamp: i64,
}

impl MockRecentProjectsStore {
    /// Creates an empty, working store. `add` stamps `last_opened` to `1`.
    pub fn new() -> Self {
        Self {
            projects: Mutex::new(Vec::new()),
            error: None,
            stamp: 1,
        }
    }

    /// Creates a store where every operation fails with `error`.
    pub fn failing(error: StoreError) -> Self {
        Self {
            projects: Mutex::new(Vec::new()),
            error: Some(error),
            stamp: 1,
        }
    }

    /// Pre-populates the store with a project.
    pub fn with_project(self, project: RecentProject) -> Self {
        self.projects
            .lock()
            .expect("mock mutex poisoned")
            .push(project);
        self
    }

    /// Number of stored projects.
    pub fn len(&self) -> usize {
        self.projects.lock().expect("mock mutex poisoned").len()
    }

    /// Whether the store holds no projects.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for MockRecentProjectsStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RecentProjectsStore for MockRecentProjectsStore {
    async fn list(&self) -> Result<Vec<RecentProject>, StoreError> {
        if let Some(error) = &self.error {
            return Err(error.clone());
        }
        Ok(self.projects.lock().expect("mock mutex poisoned").clone())
    }

    async fn add(&self, project: &RecentProject) -> Result<(), StoreError> {
        if let Some(error) = &self.error {
            return Err(error.clone());
        }
        let mut stored = self.projects.lock().expect("mock mutex poisoned");
        stored.retain(|existing| existing.id != project.id);
        let mut entry = project.clone();
        entry.last_opened = self.stamp;
        stored.insert(0, entry);
        Ok(())
    }

    async fn remove(&self, id: &RepositoryId) -> Result<(), StoreError> {
        if let Some(error) = &self.error {
            return Err(error.clone());
        }
        self.projects
            .lock()
            .expect("mock mutex poisoned")
            .retain(|existing| &existing.id != id);
        Ok(())
    }
}
