//! Configurable mock implementations of every `devpilot-core` port.
//!
//! Mocks are hand-written on purpose (ADR-0002): they read as ordinary
//! code and need no macro DSL. Each mock returns consistent fixtures from
//! [`crate::fixtures`] by default and is reconfigured through `with_*`
//! builder methods.

mod analyzer;
mod cache;
mod git;
mod progress;

pub use analyzer::MockCodeAnalyzer;
pub use cache::MockAnalysisCache;
pub use git::MockGitReader;
pub use progress::RecordingProgressReporter;
