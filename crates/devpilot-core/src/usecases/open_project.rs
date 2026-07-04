use std::sync::Arc;

use crate::entities::{ProjectMetadata, RecentProject, RepositorySource};
use crate::errors::ProjectError;
use crate::ports::{GitReader, RecentProjectsStore};

/// Opens a project from a local folder or a remote URL and records it in the
/// recent-projects list.
///
/// This is the single entry point behind both "Open folder" and "Clone
/// GitHub repository": the two differ only in the [`RepositorySource`] they
/// pass. Cloning, when needed, happens inside the git adapter's `open`.
pub struct OpenProject {
    git: Arc<dyn GitReader>,
    recent: Arc<dyn RecentProjectsStore>,
}

impl OpenProject {
    /// Creates the use case from its dependencies.
    pub fn new(git: Arc<dyn GitReader>, recent: Arc<dyn RecentProjectsStore>) -> Self {
        Self { git, recent }
    }

    /// Opens the project described by `source` and returns its metadata.
    ///
    /// Steps: open (cloning if remote), read branch, tree and commit count,
    /// assemble metadata, then record the project as recently opened. A
    /// failure at any step surfaces as a typed [`ProjectError`].
    pub async fn execute(&self, source: RepositorySource) -> Result<ProjectMetadata, ProjectError> {
        let repository = self.git.open(&source).await?;
        let branch = self.git.current_branch(&repository).await?;
        let tree = self.git.file_tree(&repository).await?;
        let commit_count = self.git.commit_count(&repository).await?;

        let metadata = ProjectMetadata::assemble(
            repository.name.clone(),
            repository.local_path.clone(),
            branch,
            repository.head.clone(),
            commit_count,
            &tree,
        );

        let recent = RecentProject {
            id: repository.id,
            name: metadata.name.clone(),
            source,
            local_path: metadata.local_path.clone(),
            last_opened: 0, // stamped by the store on insert
        };
        self.recent.add(&recent).await?;

        Ok(metadata)
    }
}
