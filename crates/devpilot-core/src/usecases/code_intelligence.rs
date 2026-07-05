use std::path::PathBuf;
use std::sync::Arc;

use crate::architecture;
use crate::entities::{
    CodeIntelligenceReport, FileAst, FileNode, FileTree, Language, Repository, RepositorySource,
    SearchHit, SourceFile,
};
use crate::errors::GitError;
use crate::intel;
use crate::ports::{CodeAnalyzer, GitReader};

/// Computes the deterministic code-intelligence report (cyclic dependencies,
/// dead code, duplication) for a repository.
pub struct AnalyzeCodeIntelligence {
    git: Arc<dyn GitReader>,
    analyzer: Arc<dyn CodeAnalyzer>,
}

impl AnalyzeCodeIntelligence {
    /// Creates the use case from its dependencies.
    pub fn new(git: Arc<dyn GitReader>, analyzer: Arc<dyn CodeAnalyzer>) -> Self {
        Self { git, analyzer }
    }

    /// Opens the project, parses its supported files, and runs the detectors.
    pub async fn execute(
        &self,
        source: RepositorySource,
    ) -> Result<CodeIntelligenceReport, GitError> {
        let repository = self.git.open(&source).await?;
        let tree = self.git.file_tree(&repository).await?;
        let (asts, contents) = parse_repo(&*self.git, &*self.analyzer, &repository, &tree).await;

        let model = architecture::build(&asts, &tree);

        Ok(CodeIntelligenceReport {
            cyclic_dependencies: intel::find_cycles(&model.dependency_graph),
            dead_code: intel::find_dead_code(&model.call_graph, &asts),
            duplication: intel::find_duplication(&contents),
        })
    }
}

/// Searches a repository's symbols and paths for a query.
pub struct SearchCode {
    git: Arc<dyn GitReader>,
    analyzer: Arc<dyn CodeAnalyzer>,
}

impl SearchCode {
    /// Creates the use case from its dependencies.
    pub fn new(git: Arc<dyn GitReader>, analyzer: Arc<dyn CodeAnalyzer>) -> Self {
        Self { git, analyzer }
    }

    /// Opens the project, parses its files, and ranks matches for `query`.
    pub async fn execute(
        &self,
        source: RepositorySource,
        query: String,
    ) -> Result<Vec<SearchHit>, GitError> {
        let repository = self.git.open(&source).await?;
        let tree = self.git.file_tree(&repository).await?;
        let (asts, _) = parse_repo(&*self.git, &*self.analyzer, &repository, &tree).await;
        Ok(intel::search_code(&query, &asts))
    }
}

/// Parses every supported file in the tree, returning the ASTs and the raw
/// `(path, content)` pairs (the latter feed the duplication detector).
async fn parse_repo(
    git: &dyn GitReader,
    analyzer: &dyn CodeAnalyzer,
    repository: &Repository,
    tree: &FileTree,
) -> (Vec<FileAst>, Vec<(String, String)>) {
    let mut asts = Vec::new();
    let mut contents = Vec::new();
    for path in supported_files(tree, analyzer) {
        let Ok(content) = git.read_file(repository, &path).await else {
            continue;
        };
        let file = SourceFile {
            path: path.clone(),
            content: content.clone(),
        };
        if let Ok(ast) = analyzer.parse(&file).await {
            contents.push((path.to_string_lossy().replace('\\', "/"), content));
            asts.push(ast);
        }
    }
    (asts, contents)
}

/// Paths of files whose language the analyzer supports.
fn supported_files(tree: &FileTree, analyzer: &dyn CodeAnalyzer) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    collect(&tree.root, analyzer, &mut paths);
    paths
}

/// Recursively gathers supported file paths.
fn collect(node: &FileNode, analyzer: &dyn CodeAnalyzer, paths: &mut Vec<PathBuf>) {
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
