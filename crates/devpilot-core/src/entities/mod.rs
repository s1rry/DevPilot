//! Domain types of DevPilot.
//!
//! Entities are plain data with small, self-explanatory helper methods.
//! They carry `serde` derives so adapters can persist and transfer them
//! (ADR-0002), but contain no I/O and no framework types.

mod analysis;
mod history;
mod language;
mod metadata;
mod metrics;
mod project;
mod repository;
mod scan;
mod tree;

pub use analysis::{
    AnalysisProgress, AnalysisResult, Diagnostic, DiagnosticSeverity, FileAnalysis, SourceFile,
};
pub use history::{AuthorStats, CommitHash, CommitInfo, FileChurn};
pub use language::Language;
pub use metadata::{LanguageStat, ProjectMetadata};
pub use metrics::{FileMetrics, FunctionMetrics};
pub use project::RecentProject;
pub use repository::{Repository, RepositoryId, RepositorySource};
pub use scan::{
    Dependency, Detection, Ecosystem, FolderSummary, Framework, FrameworkCategory, GitSummary,
    ScanReport,
};
pub use tree::{FileNode, FileTree};
