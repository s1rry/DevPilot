use async_trait::async_trait;

use crate::entities::{AnalysisResult, CommitHash, RepositoryId};
use crate::errors::CacheError;

/// Persistent cache of analysis results.
///
/// Keyed by `(repository, commit)`: a result is valid for exactly one
/// commit, so invalidation is implicit — a new commit is a new key.
/// Implemented by `devpilot-storage` on top of SQLite and by
/// `MockAnalysisCache` in `devpilot-testing`.
#[async_trait]
pub trait AnalysisCache: Send + Sync {
    /// Returns the stored result for the exact repository and commit, if any.
    async fn get(
        &self,
        repository: &RepositoryId,
        commit: &CommitHash,
    ) -> Result<Option<AnalysisResult>, CacheError>;

    /// Stores a result, replacing a previous one for the same key.
    async fn put(&self, result: &AnalysisResult) -> Result<(), CacheError>;
}
