use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use async_trait::async_trait;
use devpilot_core::entities::{FileAst, Language, SourceFile};
use devpilot_core::errors::AnalysisError;
use devpilot_core::ports::CodeAnalyzer;

/// Configurable [`CodeAnalyzer`] for tests.
///
/// Responses are keyed by file path. Files without a configured response
/// fail with [`AnalysisError::UnsupportedLanguage`], mirroring how the real
/// analyzer treats files it has no grammar for.
pub struct MockCodeAnalyzer {
    responses: HashMap<PathBuf, Result<FileAst, AnalysisError>>,
    parsed: Mutex<Vec<PathBuf>>,
}

impl MockCodeAnalyzer {
    /// Creates a mock with no configured responses.
    pub fn new() -> Self {
        Self {
            responses: HashMap::new(),
            parsed: Mutex::new(Vec::new()),
        }
    }

    /// Configures a successful parse, keyed by `ast.path`.
    pub fn with_ast(mut self, ast: FileAst) -> Self {
        self.responses.insert(ast.path.clone(), Ok(ast));
        self
    }

    /// Configures a failure for the given path.
    pub fn with_error(mut self, path: impl Into<PathBuf>, error: AnalysisError) -> Self {
        self.responses.insert(path.into(), Err(error));
        self
    }

    /// Paths passed to [`CodeAnalyzer::parse`] so far, in order.
    pub fn parsed_paths(&self) -> Vec<PathBuf> {
        self.parsed.lock().expect("mock mutex poisoned").clone()
    }
}

impl Default for MockCodeAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CodeAnalyzer for MockCodeAnalyzer {
    async fn parse(&self, file: &SourceFile) -> Result<FileAst, AnalysisError> {
        self.parsed
            .lock()
            .expect("mock mutex poisoned")
            .push(file.path.clone());
        self.responses.get(&file.path).cloned().unwrap_or_else(|| {
            Err(AnalysisError::UnsupportedLanguage {
                path: file.path.clone(),
            })
        })
    }

    fn detect_language(&self, path: &Path) -> Language {
        path.extension()
            .and_then(|extension| extension.to_str())
            .map(Language::from_extension)
            .unwrap_or(Language::Unknown)
    }
}
