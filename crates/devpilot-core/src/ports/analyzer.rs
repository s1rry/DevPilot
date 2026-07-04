use std::path::Path;

use async_trait::async_trait;

use crate::entities::{FileAst, Language, SourceFile};
use crate::errors::AnalysisError;

/// Parses source files into a structural AST model.
///
/// Implemented by `devpilot-analysis` on top of tree-sitter and by
/// `MockCodeAnalyzer` in `devpilot-testing`. Produces the internal
/// [`FileAst`] model (functions, classes, interfaces, imports, exports);
/// there is no semantic resolution.
#[async_trait]
pub trait CodeAnalyzer: Send + Sync {
    /// Parses one file into its [`FileAst`].
    ///
    /// An error affects only this file: callers record it as a diagnostic
    /// and continue with the rest of the repository. Implementations must
    /// not panic on malformed input.
    async fn parse(&self, file: &SourceFile) -> Result<FileAst, AnalysisError>;

    /// Detects the language of a file from its path.
    fn detect_language(&self, path: &Path) -> Language;
}
