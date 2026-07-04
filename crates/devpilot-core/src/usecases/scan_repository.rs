use std::collections::HashMap;
use std::sync::Arc;

use crate::entities::{
    AuthorStats, CommitInfo, FolderSummary, GitSummary, LanguageStat, RepositorySource, ScanReport,
};
use crate::errors::RepoScanError;
use crate::ports::{GitReader, ProjectScanner};

/// Maximum commits inspected to derive contributors and the last commit.
const HISTORY_DEPTH: usize = 1000;

/// Maximum number of contributors reported.
const MAX_CONTRIBUTORS: usize = 10;

/// Scans a repository into a [`ScanReport`]: languages and structure from the
/// file tree, git facts from history, and frameworks and dependencies from
/// the project's manifests.
pub struct ScanRepository {
    git: Arc<dyn GitReader>,
    scanner: Arc<dyn ProjectScanner>,
}

impl ScanRepository {
    /// Creates the use case from its dependencies.
    pub fn new(git: Arc<dyn GitReader>, scanner: Arc<dyn ProjectScanner>) -> Self {
        Self { git, scanner }
    }

    /// Opens the project described by `source` and scans it.
    pub async fn execute(&self, source: RepositorySource) -> Result<ScanReport, RepoScanError> {
        let repository = self.git.open(&source).await?;

        let tree = self.git.file_tree(&repository).await?;
        let branch = self.git.current_branch(&repository).await?;
        let commit_count = self.git.commit_count(&repository).await?;
        let history = self.git.history(&repository, HISTORY_DEPTH).await?;
        let detection = self.scanner.detect(&repository).await?;

        let languages = language_stats(&tree.language_counts());
        let structure = FolderSummary::from_tree(&tree);
        let contributors = contributors(&history);
        let last_commit = history.into_iter().next();

        Ok(ScanReport {
            languages,
            frameworks: detection.frameworks,
            dependencies: detection.dependencies,
            structure,
            git: GitSummary {
                branch,
                head: repository.head,
                commit_count,
                last_commit,
                contributors,
            },
        })
    }
}

/// Converts per-language counts into a list sorted by descending count.
fn language_stats(
    counts: &std::collections::BTreeMap<crate::entities::Language, usize>,
) -> Vec<LanguageStat> {
    let mut stats: Vec<LanguageStat> = counts
        .iter()
        .map(|(language, count)| LanguageStat {
            language: *language,
            file_count: *count,
        })
        .collect();
    stats.sort_by(|a, b| {
        b.file_count
            .cmp(&a.file_count)
            .then_with(|| a.language.name().cmp(b.language.name()))
    });
    stats
}

/// Aggregates commits into contributor statistics, keyed by author email,
/// sorted by descending commit count and capped at [`MAX_CONTRIBUTORS`].
fn contributors(history: &[CommitInfo]) -> Vec<AuthorStats> {
    let mut by_email: HashMap<&str, AuthorStats> = HashMap::new();
    for commit in history {
        by_email
            .entry(&commit.author_email)
            .and_modify(|stats| stats.commit_count += 1)
            .or_insert_with(|| AuthorStats {
                name: commit.author_name.clone(),
                email: commit.author_email.clone(),
                commit_count: 1,
            });
    }

    let mut stats: Vec<AuthorStats> = by_email.into_values().collect();
    stats.sort_by(|a, b| {
        b.commit_count
            .cmp(&a.commit_count)
            .then_with(|| a.name.cmp(&b.name))
    });
    stats.truncate(MAX_CONTRIBUTORS);
    stats
}
