use std::path::Path;

use async_trait::async_trait;

use crate::entities::{FileAnalysis, Language, SourceFile};
use crate::errors::AnalysisError;

/// Static analysis of source files.
///
/// Implemented by `devpilot-analysis` on top of tree-sitter and by
/// `MockCodeAnalyzer` in `devpilot-testing`.
#[async_trait]
pub trait CodeAnalyzer: Send + Sync {
    /// Analyzes one file and returns its metrics.
    ///
    /// An error affects only this file: callers record it as a diagnostic
    /// and continue with the rest of the repository. Implementations must
    /// not panic on malformed input.
    async fn analyze_file(&self, file: &SourceFile) -> Result<FileAnalysis, AnalysisError>;

    /// Detects the language of a file from its path.
    fn detect_language(&self, path: &Path) -> Language;
}
