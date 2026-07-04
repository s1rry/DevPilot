//! Application state: the concrete port implementations wired at startup.
//!
//! Held behind trait objects so command handlers depend only on the
//! `devpilot-core` ports, never on the adapters. Constructed in
//! [`crate::di`].

use std::sync::Arc;

use devpilot_core::ports::{CodeAnalyzer, GitReader, ProjectScanner, RecentProjectsStore};

/// Shared, injected dependencies available to every command handler.
pub struct AppState {
    /// Git access, backed by libgit2.
    pub git: Arc<dyn GitReader>,
    /// Recent-projects persistence, backed by a JSON file.
    pub recent: Arc<dyn RecentProjectsStore>,
    /// Project scanning, backed by manifest detection.
    pub scanner: Arc<dyn ProjectScanner>,
    /// AST parsing, backed by tree-sitter.
    pub analyzer: Arc<dyn CodeAnalyzer>,
}
