//! # devpilot-storage
//!
//! Local persistence adapter for DevPilot. Implements the cache and settings
//! ports from `devpilot-core` on top of SQLite (`rusqlite`).
//!
//! Planned contents (added with roadmap phases 2-3):
//!
//! - Analysis result cache keyed by commit hash.
//! - Application settings, including AI provider configuration.
//! - Secure handling of API keys.
//!
//! ## Rules
//!
//! - SQLite types and SQL never leak out of this crate.
//! - Schema migrations are versioned and forward-only.
