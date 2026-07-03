//! Ready-made domain objects for tests.
//!
//! All fixtures describe one small imaginary repository (`fixture/sample`)
//! so they stay consistent with each other: the tree, the history, the file
//! contents and the analysis result all refer to the same two Rust files.

use std::collections::HashMap;
use std::path::PathBuf;

use devpilot_core::entities::{
    AnalysisResult, CommitHash, CommitInfo, FileAnalysis, FileMetrics, FileNode, FileTree,
    FunctionMetrics, Language, Repository, RepositoryId, SourceFile,
};

/// Commit hash the sample repository is pinned at.
pub fn sample_commit() -> CommitHash {
    CommitHash::new("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
}

/// A minimal opened repository.
pub fn sample_repository() -> Repository {
    Repository {
        id: RepositoryId::new("fixture/sample"),
        name: "sample".to_string(),
        local_path: PathBuf::from("/tmp/fixture-sample"),
        head: sample_commit(),
    }
}

/// File tree of the sample repository: two Rust files and a README.
pub fn sample_tree() -> FileTree {
    FileTree {
        root: FileNode::Directory {
            path: PathBuf::new(),
            children: vec![
                FileNode::File {
                    path: PathBuf::from("README.md"),
                    size_bytes: 50,
                    language: Language::Unknown,
                },
                FileNode::Directory {
                    path: PathBuf::from("src"),
                    children: vec![
                        FileNode::File {
                            path: PathBuf::from("src/lib.rs"),
                            size_bytes: 120,
                            language: Language::Rust,
                        },
                        FileNode::File {
                            path: PathBuf::from("src/main.rs"),
                            size_bytes: 60,
                            language: Language::Rust,
                        },
                    ],
                },
            ],
        },
    }
}

/// Two commits of history, newest first.
pub fn sample_history() -> Vec<CommitInfo> {
    vec![
        CommitInfo {
            hash: sample_commit(),
            author_name: "Alice".to_string(),
            author_email: "alice@example.com".to_string(),
            timestamp: 1_750_000_000,
            summary: "Add library module".to_string(),
        },
        CommitInfo {
            hash: CommitHash::new("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"),
            author_name: "Bob".to_string(),
            author_email: "bob@example.com".to_string(),
            timestamp: 1_749_000_000,
            summary: "Initial commit".to_string(),
        },
    ]
}

/// Readable contents of the sample repository files.
pub fn sample_files() -> HashMap<PathBuf, String> {
    HashMap::from([
        (
            PathBuf::from("src/main.rs"),
            "fn main() {\n    println!(\"hello\");\n}\n".to_string(),
        ),
        (
            PathBuf::from("src/lib.rs"),
            "/// Adds two numbers.\npub fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n"
                .to_string(),
        ),
        (PathBuf::from("README.md"), "# sample\n".to_string()),
    ])
}

/// A source file matching `src/main.rs` from [`sample_files`].
pub fn sample_source_file() -> SourceFile {
    SourceFile {
        path: PathBuf::from("src/main.rs"),
        content: sample_files()[&PathBuf::from("src/main.rs")].clone(),
    }
}

/// A plausible analysis of one Rust file at `path`.
pub fn sample_file_analysis(path: &str) -> FileAnalysis {
    FileAnalysis {
        path: PathBuf::from(path),
        language: Language::Rust,
        metrics: FileMetrics {
            lines_total: 4,
            lines_code: 3,
            lines_comment: 1,
            functions: vec![FunctionMetrics {
                name: "add".to_string(),
                start_line: 2,
                line_count: 3,
                cyclomatic_complexity: 1,
                nesting_depth: 1,
            }],
        },
    }
}

/// A complete analysis result consistent with [`sample_repository`].
pub fn sample_analysis_result() -> AnalysisResult {
    AnalysisResult {
        repository: sample_repository().id,
        commit: sample_commit(),
        files: vec![
            sample_file_analysis("src/lib.rs"),
            sample_file_analysis("src/main.rs"),
        ],
        diagnostics: vec![],
    }
}
