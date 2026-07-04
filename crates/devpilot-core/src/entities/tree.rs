use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::language::Language;

/// A node in a repository file tree.
///
/// All paths are relative to the repository root; the root itself is a
/// [`FileNode::Directory`] with an empty path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileNode {
    /// A regular file.
    File {
        /// Path relative to the repository root.
        path: PathBuf,
        /// File size in bytes.
        size_bytes: u64,
        /// Detected language, [`Language::Unknown`] when out of scope.
        language: Language,
    },
    /// A directory containing further nodes.
    Directory {
        /// Path relative to the repository root.
        path: PathBuf,
        /// Direct children, ordered by name.
        children: Vec<FileNode>,
    },
}

impl FileNode {
    /// Path of the node relative to the repository root.
    pub fn path(&self) -> &Path {
        match self {
            Self::File { path, .. } | Self::Directory { path, .. } => path,
        }
    }

    /// Number of files in this subtree; directories are not counted.
    pub fn file_count(&self) -> usize {
        match self {
            Self::File { .. } => 1,
            Self::Directory { children, .. } => children.iter().map(FileNode::file_count).sum(),
        }
    }

    /// Total size in bytes of all files in this subtree.
    pub fn total_size_bytes(&self) -> u64 {
        match self {
            Self::File { size_bytes, .. } => *size_bytes,
            Self::Directory { children, .. } => {
                children.iter().map(FileNode::total_size_bytes).sum()
            }
        }
    }

    /// Accumulates a per-language file count over this subtree into `counts`.
    fn accumulate_language_counts(&self, counts: &mut BTreeMap<Language, usize>) {
        match self {
            Self::File { language, .. } => {
                *counts.entry(*language).or_insert(0) += 1;
            }
            Self::Directory { children, .. } => {
                for child in children {
                    child.accumulate_language_counts(counts);
                }
            }
        }
    }
}

/// The complete file tree of a repository at a given commit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileTree {
    /// Root directory of the repository.
    pub root: FileNode,
}

impl FileTree {
    /// Total number of files in the tree.
    pub fn file_count(&self) -> usize {
        self.root.file_count()
    }

    /// Total size in bytes of all files in the tree.
    pub fn total_size_bytes(&self) -> u64 {
        self.root.total_size_bytes()
    }

    /// File count per language, ordered by language for determinism.
    pub fn language_counts(&self) -> BTreeMap<Language, usize> {
        let mut counts = BTreeMap::new();
        self.root.accumulate_language_counts(&mut counts);
        counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file(path: &str) -> FileNode {
        FileNode::File {
            path: PathBuf::from(path),
            size_bytes: 1,
            language: Language::Unknown,
        }
    }

    #[test]
    fn counts_files_recursively() {
        let tree = FileTree {
            root: FileNode::Directory {
                path: PathBuf::new(),
                children: vec![
                    file("a.rs"),
                    FileNode::Directory {
                        path: PathBuf::from("src"),
                        children: vec![file("src/b.rs"), file("src/c.rs")],
                    },
                ],
            },
        };
        assert_eq!(tree.file_count(), 3);
    }

    #[test]
    fn empty_directory_has_zero_files() {
        let tree = FileTree {
            root: FileNode::Directory {
                path: PathBuf::new(),
                children: vec![],
            },
        };
        assert_eq!(tree.file_count(), 0);
    }
}
