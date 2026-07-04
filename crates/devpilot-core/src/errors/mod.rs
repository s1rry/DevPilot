//! Typed errors of the DevPilot domain, one enum per port.
//!
//! Errors derive `Clone` and `PartialEq` so tests and mocks can configure
//! and assert them directly. Messages are written for developers; the UI
//! layer maps them to user-facing text.

use std::path::PathBuf;

use thiserror::Error;

/// Errors produced by [`crate::ports::GitReader`] implementations.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum GitError {
    /// The given path exists but is not a git repository.
    #[error("not a git repository: {}", path.display())]
    NotARepository {
        /// Path that was checked.
        path: PathBuf,
    },

    /// Cloning a remote repository failed.
    #[error("failed to clone {url}: {reason}")]
    CloneFailed {
        /// Remote URL that was requested.
        url: String,
        /// Backend-provided reason.
        reason: String,
    },

    /// The repository contains no commits, so there is nothing to analyze.
    #[error("repository has no commits")]
    EmptyRepository,

    /// A requested file does not exist at the repository's HEAD.
    #[error("file not found in repository: {}", path.display())]
    FileNotFound {
        /// Path that was requested, relative to the repository root.
        path: PathBuf,
    },

    /// Any other backend failure (I/O, libgit2, permissions).
    #[error("git operation failed: {0}")]
    Backend(String),
}

/// Errors produced by [`crate::ports::CodeAnalyzer`] implementations.
///
/// These are per-file errors: callers convert them into diagnostics and
/// continue with the rest of the repository.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum AnalysisError {
    /// The file's language has no AST support.
    #[error("language of {} is not supported for AST analysis", path.display())]
    UnsupportedLanguage {
        /// File that was skipped.
        path: PathBuf,
    },

    /// The parser could not process the file.
    #[error("failed to parse {}: {reason}", path.display())]
    ParseFailed {
        /// File that failed to parse.
        path: PathBuf,
        /// Parser-provided reason.
        reason: String,
    },

    /// The file exceeds the analyzer's size limit and was skipped.
    #[error("file too large for analysis: {} ({size_bytes} bytes)", path.display())]
    FileTooLarge {
        /// File that was skipped.
        path: PathBuf,
        /// Actual size of the file.
        size_bytes: u64,
    },
}

/// Errors produced by [`crate::ports::AnalysisCache`] implementations.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CacheError {
    /// Any backend failure (I/O, database, serialization).
    #[error("cache backend error: {0}")]
    Backend(String),
}

/// Errors produced by [`crate::ports::RecentProjectsStore`] implementations.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum StoreError {
    /// Any backend failure (I/O, serialization).
    #[error("recent projects store error: {0}")]
    Backend(String),
}

/// Errors of the project use cases, aggregating the ports they depend on.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ProjectError {
    /// A git operation failed.
    #[error(transparent)]
    Git(#[from] GitError),

    /// Persisting the recent-projects list failed.
    #[error(transparent)]
    Store(#[from] StoreError),
}
