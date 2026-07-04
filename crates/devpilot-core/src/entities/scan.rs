use serde::{Deserialize, Serialize};

use super::history::{AuthorStats, CommitHash, CommitInfo};
use super::metadata::LanguageStat;
use super::tree::{FileNode, FileTree};

/// Package ecosystem a dependency belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Ecosystem {
    /// npm / Node.js (`package.json`).
    Npm,
    /// Rust / Cargo (`Cargo.toml`).
    Cargo,
    /// Python / PyPI (`requirements.txt`, `pyproject.toml`).
    PyPI,
    /// Go modules (`go.mod`).
    Go,
}

impl Ecosystem {
    /// Human-readable ecosystem name.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Npm => "npm",
            Self::Cargo => "Cargo",
            Self::PyPI => "PyPI",
            Self::Go => "Go",
        }
    }
}

/// A single declared dependency of the project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dependency {
    /// Package name.
    pub name: String,
    /// Declared version or version requirement, when available.
    pub version: Option<String>,
    /// Ecosystem the dependency comes from.
    pub ecosystem: Ecosystem,
}

/// Broad role a framework plays in a project.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrameworkCategory {
    /// UI / client frameworks (React, Vue, ...).
    Frontend,
    /// Server / web frameworks (Express, Axum, Django, ...).
    Backend,
    /// Frameworks that span both (Next.js, ...).
    Fullstack,
    /// Desktop shells (Tauri, ...).
    Desktop,
}

/// A framework detected from the project's manifests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Framework {
    /// Display name (e.g. `React`).
    pub name: String,
    /// Role the framework plays.
    pub category: FrameworkCategory,
    /// Manifest file the detection came from (e.g. `package.json`).
    pub source: String,
}

/// A summary of the repository's top-level folder structure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FolderSummary {
    /// Total number of files in the tree.
    pub total_files: usize,
    /// Total number of directories in the tree.
    pub total_directories: usize,
    /// Names of the top-level directories.
    pub top_level_dirs: Vec<String>,
    /// Recognized convention directories present (src, tests, docs, ...).
    pub notable: Vec<String>,
}

/// Directory names treated as notable project conventions.
const NOTABLE_DIRS: &[&str] = &[
    "src", "lib", "tests", "test", "docs", "examples", "benches", "scripts", ".github", "crates",
    "apps", "packages",
];

impl FolderSummary {
    /// Summarizes the top level of a file tree.
    pub fn from_tree(tree: &FileTree) -> Self {
        let mut top_level_dirs = Vec::new();
        if let FileNode::Directory { children, .. } = &tree.root {
            for child in children {
                if let FileNode::Directory { path, .. } = child {
                    if let Some(name) = path.file_name().and_then(|name| name.to_str()) {
                        top_level_dirs.push(name.to_string());
                    }
                }
            }
        }
        top_level_dirs.sort();

        let notable = top_level_dirs
            .iter()
            .filter(|name| NOTABLE_DIRS.contains(&name.as_str()))
            .cloned()
            .collect();

        Self {
            total_files: tree.file_count(),
            total_directories: tree.directory_count(),
            top_level_dirs,
            notable,
        }
    }
}

/// Git facts about the scanned repository.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitSummary {
    /// Current branch name.
    pub branch: String,
    /// Commit HEAD points to.
    pub head: CommitHash,
    /// Total number of commits reachable from HEAD.
    pub commit_count: usize,
    /// Most recent commit, if any.
    pub last_commit: Option<CommitInfo>,
    /// Top contributors by commit count, most first.
    pub contributors: Vec<AuthorStats>,
}

/// Frameworks and dependencies detected from manifests.
///
/// Produced by [`crate::ports::ProjectScanner`] and merged into a
/// [`ScanReport`] by the scan use case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Detection {
    /// Frameworks found across all manifests.
    pub frameworks: Vec<Framework>,
    /// Dependencies found across all manifests.
    pub dependencies: Vec<Dependency>,
}

/// The complete result of scanning a repository.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScanReport {
    /// Per-language file counts, most files first.
    pub languages: Vec<LanguageStat>,
    /// Detected frameworks.
    pub frameworks: Vec<Framework>,
    /// Declared dependencies.
    pub dependencies: Vec<Dependency>,
    /// Folder structure summary.
    pub structure: FolderSummary,
    /// Git facts.
    pub git: GitSummary,
}
