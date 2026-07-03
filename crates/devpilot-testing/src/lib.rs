//! # devpilot-testing
//!
//! Shared test doubles for the DevPilot workspace. Used as a dev-dependency
//! by other crates; never shipped in the application.
//!
//! - [`mocks`] — configurable implementations of every `devpilot-core` port.
//! - [`fixtures`] — ready-made domain objects for tests.
//!
//! ## Rules
//!
//! - Mocks live only here; crates must not define ad-hoc duplicates.
//! - Fixtures are built programmatically, never committed as binary blobs.

pub mod fixtures;
pub mod mocks;
