use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use async_trait::async_trait;
use devpilot_core::entities::{CommitInfo, FileChurn, FileTree, Repository, RepositorySource};
use devpilot_core::errors::GitError;
use devpilot_core::ports::GitReader;

use crate::fixtures;

/// Configurable in-memory [`GitReader`] for tests.
///
/// By default every method returns the consistent fixtures from
/// [`crate::fixtures`]. Override individual responses with the `with_*`
/// methods and assert interactions through [`MockGitReader::calls`].
///
/// # Examples
///
/// ```
/// use devpilot_testing::mocks::MockGitReader;
/// use devpilot_core::errors::GitError;
///
/// let reader = MockGitReader::new().with_open_error(GitError::EmptyRepository);
/// ```
pub struct MockGitReader {
    open_result: Result<Repository, GitError>,
    tree_result: Result<FileTree, GitError>,
    history_result: Result<Vec<CommitInfo>, GitError>,
    churn_result: Result<Vec<FileChurn>, GitError>,
    branch_result: Result<String, GitError>,
    files: HashMap<PathBuf, String>,
    calls: Mutex<Vec<String>>,
}

impl MockGitReader {
    /// Creates a mock backed by the standard sample repository fixtures.
    pub fn new() -> Self {
        Self {
            open_result: Ok(fixtures::sample_repository()),
            tree_result: Ok(fixtures::sample_tree()),
            history_result: Ok(fixtures::sample_history()),
            churn_result: Ok(vec![]),
            branch_result: Ok("main".to_string()),
            files: fixtures::sample_files(),
            calls: Mutex::new(Vec::new()),
        }
    }

    /// Makes [`GitReader::current_branch`] return the given branch name.
    pub fn with_branch(mut self, branch: impl Into<String>) -> Self {
        self.branch_result = Ok(branch.into());
        self
    }

    /// Makes [`GitReader::open`] return the given repository.
    pub fn with_repository(mut self, repository: Repository) -> Self {
        self.open_result = Ok(repository);
        self
    }

    /// Makes [`GitReader::open`] fail with the given error.
    pub fn with_open_error(mut self, error: GitError) -> Self {
        self.open_result = Err(error);
        self
    }

    /// Makes [`GitReader::file_tree`] return the given tree.
    pub fn with_tree(mut self, tree: FileTree) -> Self {
        self.tree_result = Ok(tree);
        self
    }

    /// Makes [`GitReader::file_tree`] fail with the given error.
    pub fn with_tree_error(mut self, error: GitError) -> Self {
        self.tree_result = Err(error);
        self
    }

    /// Makes [`GitReader::history`] return the given commits.
    pub fn with_history(mut self, history: Vec<CommitInfo>) -> Self {
        self.history_result = Ok(history);
        self
    }

    /// Makes [`GitReader::churn`] return the given statistics.
    pub fn with_churn(mut self, churn: Vec<FileChurn>) -> Self {
        self.churn_result = Ok(churn);
        self
    }

    /// Adds (or replaces) a file readable through [`GitReader::read_file`].
    pub fn with_file(mut self, path: impl Into<PathBuf>, content: impl Into<String>) -> Self {
        self.files.insert(path.into(), content.into());
        self
    }

    /// Removes all readable files.
    pub fn without_files(mut self) -> Self {
        self.files.clear();
        self
    }

    /// Names of the methods called so far, in order.
    pub fn calls(&self) -> Vec<String> {
        self.calls.lock().expect("mock mutex poisoned").clone()
    }

    fn record(&self, name: &str) {
        self.calls
            .lock()
            .expect("mock mutex poisoned")
            .push(name.to_string());
    }
}

impl Default for MockGitReader {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl GitReader for MockGitReader {
    async fn open(&self, _source: &RepositorySource) -> Result<Repository, GitError> {
        self.record("open");
        self.open_result.clone()
    }

    async fn file_tree(&self, _repository: &Repository) -> Result<FileTree, GitError> {
        self.record("file_tree");
        self.tree_result.clone()
    }

    async fn history(
        &self,
        _repository: &Repository,
        limit: usize,
    ) -> Result<Vec<CommitInfo>, GitError> {
        self.record("history");
        self.history_result.clone().map(|mut commits| {
            commits.truncate(limit);
            commits
        })
    }

    async fn churn(&self, _repository: &Repository) -> Result<Vec<FileChurn>, GitError> {
        self.record("churn");
        self.churn_result.clone()
    }

    async fn read_file(&self, _repository: &Repository, path: &Path) -> Result<String, GitError> {
        self.record("read_file");
        self.files
            .get(path)
            .cloned()
            .ok_or_else(|| GitError::FileNotFound {
                path: path.to_path_buf(),
            })
    }

    async fn current_branch(&self, _repository: &Repository) -> Result<String, GitError> {
        self.record("current_branch");
        self.branch_result.clone()
    }

    async fn commit_count(&self, _repository: &Repository) -> Result<usize, GitError> {
        self.record("commit_count");
        self.history_result.clone().map(|commits| commits.len())
    }
}
