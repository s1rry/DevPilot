//! Domain types of DevPilot.
//!
//! Entities are plain data with small, self-explanatory helper methods.
//! They carry `serde` derives so adapters can persist and transfer them
//! (ADR-0002), but contain no I/O and no framework types.

mod analysis;
mod ast;
mod graph;
mod history;
mod intel;
mod language;
mod llm;
mod metadata;
mod metrics;
mod project;
mod repository;
mod scan;
mod settings;
mod tree;

pub use analysis::{
    AnalysisProgress, AnalysisResult, Diagnostic, DiagnosticSeverity, FileAnalysis, SourceFile,
};
pub use ast::{ClassDef, ExportDecl, ExportKind, FileAst, FunctionDef, ImportDecl, InterfaceDef};
pub use graph::{ArchitectureModel, EdgeKind, Graph, GraphEdge, GraphNode, NodeKind};
pub use history::{AuthorStats, CommitHash, CommitInfo, FileChurn};
pub use intel::{
    CodeIntelligenceReport, Cycle, DeadSymbol, DuplicationGroup, DuplicationLocation, SearchHit,
};
pub use language::Language;
pub use llm::{ChatMessage, ChatRequest, ModelInfo, Role};
pub use metadata::{LanguageStat, ProjectMetadata};
pub use metrics::{FileMetrics, FunctionMetrics};
pub use project::RecentProject;
pub use repository::{Repository, RepositoryId, RepositorySource};
pub use scan::{
    Dependency, Detection, Ecosystem, FolderSummary, Framework, FrameworkCategory, GitSummary,
    ScanReport,
};
pub use settings::{AiSettings, ProviderKind};
pub use tree::{FileNode, FileTree};
