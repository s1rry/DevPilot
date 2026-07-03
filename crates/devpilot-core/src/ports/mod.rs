//! Ports: traits the domain needs implemented by the outside world.
//!
//! Each port is implemented by exactly one adapter crate and mocked in
//! `devpilot-testing`. Ports are object-safe (`dyn`-compatible) because
//! dependency injection wires them as `Arc<dyn Trait>` in the composition
//! root (ADR-0001, ADR-0002).

mod analyzer;
mod cache;
mod git;
mod progress;

pub use analyzer::CodeAnalyzer;
pub use cache::AnalysisCache;
pub use git::GitReader;
pub use progress::ProgressReporter;
