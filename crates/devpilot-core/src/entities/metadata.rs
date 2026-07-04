use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::history::CommitHash;
use super::language::Language;
use super::tree::FileTree;

/// Number of files written in one language.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanguageStat {
    /// The language.
    pub language: Language,
    /// How many files use it.
    pub file_count: usize,
}

/// Descriptive metadata of an opened project.
///
/// Everything here is derived from git and the file tree — no code analysis
/// and no AI. It is the payload the Repository Manager shows for a project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectMetadata {
    /// Project name, usually the directory name.
    pub name: String,
    /// Absolute path of the working copy.
    pub local_path: PathBuf,
    /// Current branch name at HEAD.
    pub branch: String,
    /// Commit HEAD points to.
    pub head: CommitHash,
    /// Total number of commits reachable from HEAD.
    pub commit_count: usize,
    /// Total number of files in the working tree.
    pub file_count: usize,
    /// Total size of all files in bytes.
    pub total_size_bytes: u64,
    /// Per-language file counts, most files first.
    pub languages: Vec<LanguageStat>,
}

impl ProjectMetadata {
    /// Assembles metadata from the pieces the git adapter provides.
    ///
    /// `languages` is computed from `tree` and sorted by descending file
    /// count, with the language name as a tiebreaker for determinism.
    pub fn assemble(
        name: impl Into<String>,
        local_path: impl Into<PathBuf>,
        branch: impl Into<String>,
        head: CommitHash,
        commit_count: usize,
        tree: &FileTree,
    ) -> Self {
        let mut languages: Vec<LanguageStat> = tree
            .language_counts()
            .into_iter()
            .map(|(language, file_count)| LanguageStat {
                language,
                file_count,
            })
            .collect();
        languages.sort_by(|a, b| {
            b.file_count
                .cmp(&a.file_count)
                .then_with(|| a.language.name().cmp(b.language.name()))
        });

        Self {
            name: name.into(),
            local_path: local_path.into(),
            branch: branch.into(),
            head,
            commit_count,
            file_count: tree.file_count(),
            total_size_bytes: tree.total_size_bytes(),
            languages,
        }
    }
}
