//! Application use cases: the logic that orchestrates ports.
//!
//! Each use case is a small struct holding the ports it needs as
//! `Arc<dyn Trait>` and exposing a single `execute` method. They contain the
//! application's behavior and are fully testable against the mocks in
//! `devpilot-testing`.

mod open_project;
mod recent_projects;
mod scan_repository;

pub use open_project::OpenProject;
pub use recent_projects::{ListRecentProjects, RemoveRecentProject};
pub use scan_repository::ScanRepository;
