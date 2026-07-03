use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use devpilot_core::entities::{AnalysisResult, CommitHash, RepositoryId};
use devpilot_core::errors::CacheError;
use devpilot_core::ports::AnalysisCache;

/// In-memory [`AnalysisCache`] for tests, with optional injected failure.
///
/// Behaves like a real cache (results survive `put` and come back from
/// `get`), which makes cache-hit scenarios trivial to arrange.
pub struct MockAnalysisCache {
    entries: Mutex<HashMap<(RepositoryId, CommitHash), AnalysisResult>>,
    error: Option<CacheError>,
}

impl MockAnalysisCache {
    /// Creates an empty, working cache.
    pub fn new() -> Self {
        Self {
            entries: Mutex::new(HashMap::new()),
            error: None,
        }
    }

    /// Creates a cache where every operation fails with `error`.
    pub fn failing(error: CacheError) -> Self {
        Self {
            entries: Mutex::new(HashMap::new()),
            error: Some(error),
        }
    }

    /// Pre-populates the cache with a result.
    pub fn with_result(self, result: AnalysisResult) -> Self {
        self.entries
            .lock()
            .expect("mock mutex poisoned")
            .insert((result.repository.clone(), result.commit.clone()), result);
        self
    }

    /// Number of stored results.
    pub fn len(&self) -> usize {
        self.entries.lock().expect("mock mutex poisoned").len()
    }

    /// Whether the cache holds no results.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for MockAnalysisCache {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AnalysisCache for MockAnalysisCache {
    async fn get(
        &self,
        repository: &RepositoryId,
        commit: &CommitHash,
    ) -> Result<Option<AnalysisResult>, CacheError> {
        if let Some(error) = &self.error {
            return Err(error.clone());
        }
        Ok(self
            .entries
            .lock()
            .expect("mock mutex poisoned")
            .get(&(repository.clone(), commit.clone()))
            .cloned())
    }

    async fn put(&self, result: &AnalysisResult) -> Result<(), CacheError> {
        if let Some(error) = &self.error {
            return Err(error.clone());
        }
        self.entries.lock().expect("mock mutex poisoned").insert(
            (result.repository.clone(), result.commit.clone()),
            result.clone(),
        );
        Ok(())
    }
}
