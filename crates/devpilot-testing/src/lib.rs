//! # devpilot-testing
//!
//! Shared test doubles for the DevPilot workspace. Used as a dev-dependency
//! by every other crate; never shipped in the application.
//!
//! Planned contents (added with roadmap phase 1, alongside the first ports):
//!
//! - Mock implementations of every `devpilot-core` port
//!   (`MockLlmProvider`, `MockGitReader`, `MockCodeAnalyzer`, `MockCache`).
//! - Fixture builders: temporary git repositories, sample source files.
//!
//! ## Rules
//!
//! - Mocks live only here; crates must not define ad-hoc duplicates.
//! - Fixtures are built programmatically, never committed as binary blobs.
