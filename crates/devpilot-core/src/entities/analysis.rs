use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::history::CommitHash;
use super::language::Language;
use super::metrics::FileMetrics;
use super::repository::RepositoryId;

/// A source file prepared for analysis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceFile {
    /// Path relative to the repository root.
    pub path: PathBuf,
    /// Full text content of the file.
    pub content: String,
}

/// Analysis output for one file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileAnalysis {
    /// Path relative to the repository root.
    pub path: PathBuf,
    /// Language the file was analyzed as.
    pub language: Language,
    /// Extracted metrics.
    pub metrics: FileMetrics,
}

/// Severity of a non-fatal analysis problem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    /// Expected limitation, e.g. an unsupported language.
    Info,
    /// Unexpected but recoverable problem, e.g. a parse failure.
    Warning,
}

/// A non-fatal problem encountered during analysis.
///
/// Diagnostics never abort a run: the analysis of other files continues
/// and the problems are reported alongside the result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    /// File the problem relates to, when applicable.
    pub path: Option<PathBuf>,
    /// How serious the problem is.
    pub severity: DiagnosticSeverity,
    /// Human-readable description, safe to show in the UI.
    pub message: String,
}

/// Complete result of analyzing a repository at one commit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnalysisResult {
    /// Repository the result belongs to.
    pub repository: RepositoryId,
    /// Commit the analysis was performed at; together with `repository`
    /// this forms the cache key.
    pub commit: CommitHash,
    /// Per-file results for every successfully analyzed file.
    pub files: Vec<FileAnalysis>,
    /// Problems that did not stop the analysis.
    pub diagnostics: Vec<Diagnostic>,
}

/// Progress event of a running analysis.
///
/// Emitted through [`crate::ports::ProgressReporter`] so the UI can render
/// incremental progress instead of a frozen spinner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnalysisProgress {
    /// Analysis started; `total_files` files will be processed.
    Started {
        /// Number of files scheduled for analysis.
        total_files: usize,
    },
    /// One more file finished.
    FileAnalyzed {
        /// Path of the file that was just processed.
        path: PathBuf,
        /// Files processed so far, including this one.
        analyzed: usize,
        /// Total files scheduled.
        total: usize,
    },
    /// All files processed; the result is being assembled.
    Finished,
}
