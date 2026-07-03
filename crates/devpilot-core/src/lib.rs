//! # devpilot-core
//!
//! The domain heart of DevPilot. This crate defines *what* the application
//! does, independent of *how* it is done.
//!
//! It will contain (added incrementally, see the roadmap):
//!
//! - `entities` — domain types: repository, file tree, metrics, insights.
//! - `ports` — traits implemented by adapter crates: code analysis,
//!   git reading, LLM providers, caching.
//! - `usecases` — application logic orchestrating the ports.
//! - `errors` — typed domain errors.
//!
//! ## Architecture rules (enforced in review and by the compiler)
//!
//! - No dependency on Tauri, tree-sitter, git2, HTTP clients or provider SDKs.
//! - Adapter crates depend on this crate, never the other way around.
//! - See ADR-0001 (`docs/adr/0001-clean-architecture-workspace.md`).
