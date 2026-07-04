use std::path::PathBuf;
use std::sync::Arc;

use crate::chat::{self, RelevantFile, RepoContext, DEFAULT_TOKEN_BUDGET};
use crate::entities::{ChatMessage, FileNode, FileTree, Language, RepositorySource, Role};
use crate::errors::ChatError;
use crate::ports::{GitReader, LlmProvider, TokenStream};

/// Maximum number of relevant files pulled into the context.
const MAX_RELEVANT_FILES: usize = 6;

/// Answers a question about a repository, streaming the model's reply.
///
/// Gathers repository context (summary plus files relevant to the latest
/// question), fits it and the conversation into a token budget, and streams
/// the selected provider's response.
pub struct ChatWithRepository {
    git: Arc<dyn GitReader>,
    provider: Arc<dyn LlmProvider>,
}

impl ChatWithRepository {
    /// Creates the use case from its dependencies. The provider is chosen by
    /// the composition root from the user's settings.
    pub fn new(git: Arc<dyn GitReader>, provider: Arc<dyn LlmProvider>) -> Self {
        Self { git, provider }
    }

    /// Runs one chat turn. `history` is the full conversation so far, oldest
    /// first, ending with the user's latest message.
    pub async fn execute(
        &self,
        source: RepositorySource,
        model: String,
        history: Vec<ChatMessage>,
    ) -> Result<TokenStream, ChatError> {
        let repository = self.git.open(&source).await?;
        let tree = self.git.file_tree(&repository).await?;
        let branch = self
            .git
            .current_branch(&repository)
            .await
            .unwrap_or_else(|_| "HEAD".to_string());

        let question = history
            .iter()
            .rev()
            .find(|message| message.role == Role::User)
            .map(|message| message.content.clone())
            .unwrap_or_default();

        let candidate_paths = file_paths(&tree);
        let relevant = chat::select_relevant(&candidate_paths, &question, MAX_RELEVANT_FILES);

        let mut relevant_files = Vec::new();
        for path in relevant {
            if let Ok(content) = self.git.read_file(&repository, &PathBuf::from(&path)).await {
                relevant_files.push(RelevantFile { path, content });
            }
        }

        let context = RepoContext {
            name: repository.name,
            branch,
            languages: language_counts(&tree),
            top_level_dirs: top_level_dirs(&tree),
            relevant_files,
        };

        let messages = chat::build_messages(&context, &history, DEFAULT_TOKEN_BUDGET);
        let request = crate::entities::ChatRequest {
            model,
            messages,
            temperature: None,
        };

        Ok(self.provider.chat(request).await?)
    }
}

/// Collects all file paths in the tree as `/`-separated strings.
fn file_paths(tree: &FileTree) -> Vec<String> {
    let mut paths = Vec::new();
    collect_paths(&tree.root, &mut paths);
    paths
}

/// Recursively gathers file paths.
fn collect_paths(node: &FileNode, paths: &mut Vec<String>) {
    match node {
        FileNode::File { path, .. } => paths.push(path.to_string_lossy().replace('\\', "/")),
        FileNode::Directory { children, .. } => {
            for child in children {
                collect_paths(child, paths);
            }
        }
    }
}

/// Per-language file counts, most first.
fn language_counts(tree: &FileTree) -> Vec<(Language, usize)> {
    let mut counts: Vec<(Language, usize)> = tree.language_counts().into_iter().collect();
    counts.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.name().cmp(b.0.name())));
    counts
}

/// Names of the top-level directories.
fn top_level_dirs(tree: &FileTree) -> Vec<String> {
    let mut dirs = Vec::new();
    if let FileNode::Directory { children, .. } = &tree.root {
        for child in children {
            if let FileNode::Directory { path, .. } = child {
                if let Some(name) = path.file_name().and_then(|name| name.to_str()) {
                    dirs.push(name.to_string());
                }
            }
        }
    }
    dirs.sort();
    dirs
}
