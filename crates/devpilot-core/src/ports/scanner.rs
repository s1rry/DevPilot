use async_trait::async_trait;

use crate::entities::{Detection, Repository};
use crate::errors::ScanError;

/// Detects frameworks and dependencies of a project by reading its manifests.
///
/// Implemented by `devpilot-scan` (reading the working copy) and by
/// `MockProjectScanner` in `devpilot-testing`. Language and structure
/// detection is derived from the file tree by the use case, not here.
#[async_trait]
pub trait ProjectScanner: Send + Sync {
    /// Reads the manifests in the repository's working copy and returns the
    /// frameworks and dependencies it can recognize.
    ///
    /// A malformed or unreadable single manifest is skipped, not fatal; only
    /// an underlying I/O failure surfaces as [`ScanError`].
    async fn detect(&self, repository: &Repository) -> Result<Detection, ScanError>;
}
