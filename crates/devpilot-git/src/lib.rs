//! # devpilot-git
//!
//! Git adapter for DevPilot. Implements the git-reading ports from
//! `devpilot-core` on top of libgit2 (the `git2` crate).
//!
//! Planned contents (added with roadmap phase 2):
//!
//! - Opening local repositories and cloning remote ones.
//! - File tree extraction at a given commit.
//! - Commit history, authorship, and per-file churn statistics.
//!
//! ## Rules
//!
//! - No shelling out to a system `git` binary.
//! - `git2` types never leak out of this crate.
