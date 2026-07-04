use std::sync::Arc;

use crate::architecture;
use crate::entities::{ArchitectureModel, FileAst, FileNode, FileTree, Language, RepositorySource};
use crate::errors::GitError;
use crate::ports::{CodeAnalyzer, GitReader};

/// Analyzes a repository's architecture: parses every supported source file
/// and builds the folder, dependency, module and call graphs.
pub struct AnalyzeArchitecture {
    git: Arc<dyn GitReader>,
    analyzer: Arc<dyn CodeAnalyzer>,
}

impl AnalyzeArchitecture {
    /// Creates the use case from its dependencies.
    pub fn new(git: Arc<dyn GitReader>, analyzer: Arc<dyn CodeAnalyzer>) -> Self {
        Self { git, analyzer }
    }

    /// Opens the project, parses its supported files and builds the model.
    ///
    /// Files that fail to read or parse are skipped, so one bad file never
    /// fails the whole analysis; only opening the repository or reading its
    /// tree is fatal.
    pub async fn execute(&self, source: RepositorySource) -> Result<ArchitectureModel, GitError> {
        let repository = self.git.open(&source).await?;
        let tree = self.git.file_tree(&repository).await?;

        let mut asts: Vec<FileAst> = Vec::new();
        for path in supported_files(&tree, self.analyzer.as_ref()) {
            let Ok(content) = self.git.read_file(&repository, &path).await else {
                continue; // unreadable file: skip
            };
            let file = crate::entities::SourceFile { path, content };
            if let Ok(ast) = self.analyzer.parse(&file).await {
                asts.push(ast);
            }
        }

        Ok(architecture::build(&asts, &tree))
    }
}

/// Collects the paths of files whose language the analyzer supports.
fn supported_files(tree: &FileTree, analyzer: &dyn CodeAnalyzer) -> Vec<std::path::PathBuf> {
    let mut paths = Vec::new();
    collect(&tree.root, analyzer, &mut paths);
    paths
}

/// Recursively gathers supported file paths from the tree.
fn collect(node: &FileNode, analyzer: &dyn CodeAnalyzer, paths: &mut Vec<std::path::PathBuf>) {
    match node {
        FileNode::File { path, .. } => {
            if analyzer.detect_language(path) != Language::Unknown {
                paths.push(path.clone());
            }
        }
        FileNode::Directory { children, .. } => {
            for child in children {
                collect(child, analyzer, paths);
            }
        }
    }
}
