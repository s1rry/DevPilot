//! # devpilot-analysis
//!
//! Code analysis adapter for DevPilot. Implements the analysis ports from
//! `devpilot-core` using tree-sitter.
//!
//! Planned contents (added with roadmap phase 2):
//!
//! - Language detection and lazy grammar loading.
//! - AST metrics: cyclomatic complexity, function length, nesting depth.
//! - Structural facts: imports, public API surface, duplication.
//! - A parallel, incremental analysis pipeline.
//!
//! ## Rules
//!
//! - tree-sitter types never leak out of this crate; the public API speaks
//!   `devpilot-core` entities only.
//! - A failure to parse one file must never fail a whole analysis run.
