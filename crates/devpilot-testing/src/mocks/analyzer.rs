use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use async_trait::async_trait;
use devpilot_core::entities::{FileAnalysis, Language, SourceFile};
use devpilot_core::errors::AnalysisError;
use devpilot_core::ports::CodeAnalyzer;

/// Configurable [`CodeAnalyzer`] for tests.
///
/// Responses are keyed by file path. Files without a configured response
/// fail with [`AnalysisError::UnsupportedLanguage`], mirroring how the real
/// analyzer treats files it has no grammar for.
pub struct MockCodeAnalyzer {
    responses: HashMap<PathBuf, Result<FileAnalysis, AnalysisError>>,
    analyzed: Mutex<Vec<PathBuf>>,
}

impl MockCodeAnalyzer {
    /// Creates a mock with no configured responses.
    pub fn new() -> Self {
        Self {
            responses: HashMap::new(),
            analyzed: Mutex::new(Vec::new()),
        }
    }

    /// Configures a successful analysis, keyed by `analysis.path`.
    pub fn with_analysis(mut self, analysis: FileAnalysis) -> Self {
        self.responses.insert(analysis.path.clone(), Ok(analysis));
        self
    }

    /// Configures a failure for the given path.
    pub fn with_error(mut self, path: impl Into<PathBuf>, error: AnalysisError) -> Self {
        self.responses.insert(path.into(), Err(error));
        self
    }

    /// Paths passed to [`CodeAnalyzer::analyze_file`] so far, in order.
    pub fn analyzed_paths(&self) -> Vec<PathBuf> {
        self.analyzed.lock().expect("mock mutex poisoned").clone()
    }
}

impl Default for MockCodeAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CodeAnalyzer for MockCodeAnalyzer {
    async fn analyze_file(&self, file: &SourceFile) -> Result<FileAnalysis, AnalysisError> {
        self.analyzed
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
