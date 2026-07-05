//! Application use cases: the logic that orchestrates ports.
//!
//! Each use case is a small struct holding the ports it needs as
//! `Arc<dyn Trait>` and exposing a single `execute` method. They contain the
//! application's behavior and are fully testable against the mocks in
//! `devpilot-testing`.

mod analyze_architecture;
mod chat_with_repository;
mod code_intelligence;
mod open_project;
mod recent_projects;
mod scan_repository;

pub use analyze_architecture::AnalyzeArchitecture;
pub use chat_with_repository::ChatWithRepository;
pub use code_intelligence::{AnalyzeCodeIntelligence, SearchCode};
pub use open_project::OpenProject;
pub use recent_projects::{ListRecentProjects, RemoveRecentProject};
pub use scan_repository::ScanRepository;
