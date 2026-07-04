//! # devpilot-storage
//!
//! Local persistence adapters for DevPilot.
//!
//! - [`JsonRecentProjectsStore`] — the recent-projects list.
//! - [`JsonSettingsStore`] — the user's AI settings.
//!
//! Both keep a small JSON file in the application data directory. A
//! SQLite-backed analysis cache will join this crate in a later phase, when
//! it earns its complexity.
//!
//! ## Rules
//!
//! - Storage formats and file layout never leak out of this crate; the
//!   public API speaks `devpilot-core` entities only.

mod recent_projects;
mod settings;

pub use recent_projects::JsonRecentProjectsStore;
pub use settings::JsonSettingsStore;
