//! # devpilot-core
//!
//! The domain heart of DevPilot. This crate defines *what* the application
//! does, independent of *how* it is done.
//!
//! - [`entities`] — domain types: repositories, file trees, metrics, history,
//!   analysis results.
//! - [`ports`] — traits implemented by adapter crates: git reading, code
//!   analysis, caching, progress reporting.
//! - [`errors`] — typed errors, one enum per port.
//!
//! Use cases orchestrating the ports arrive with the next roadmap step.
//!
//! ## Architecture rules (enforced in review and by the compiler)
//!
//! - No dependency on Tauri, tree-sitter, git2, HTTP clients or provider SDKs.
//! - Adapter crates depend on this crate, never the other way around.
//! - See ADR-0001 and ADR-0002 in `docs/adr/`.

pub mod architecture;
pub mod entities;
pub mod errors;
pub mod ports;
pub mod usecases;
