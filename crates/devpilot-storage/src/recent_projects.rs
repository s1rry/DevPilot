use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use devpilot_core::entities::{RecentProject, RepositoryId};
use devpilot_core::errors::StoreError;
use devpilot_core::ports::RecentProjectsStore;

/// Maximum number of projects kept in the list; older entries are dropped.
const MAX_ENTRIES: usize = 20;

/// A [`RecentProjectsStore`] backed by a single JSON file.
///
/// The file holds a JSON array of projects ordered most-recent-first. Reads
/// tolerate a missing file (empty list); writes are whole-file rewrites,
/// which is more than fast enough for a list capped at
/// [`MAX_ENTRIES`](MAX_ENTRIES).
pub struct JsonRecentProjectsStore {
    file_path: PathBuf,
}

impl JsonRecentProjectsStore {
    /// Creates a store persisting to `file_path`.
    ///
    /// The parent directory is created on the first write; the file itself
    /// need not exist yet.
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: file_path.into(),
        }
    }

    /// Reads and parses the file, returning an empty list when it is absent.
    async fn read_all(&self) -> Result<Vec<RecentProject>, StoreError> {
        match tokio::fs::read(&self.file_path).await {
            Ok(bytes) => serde_json::from_slice(&bytes)
                .map_err(|error| StoreError::Backend(format!("parse: {error}"))),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
            Err(error) => Err(StoreError::Backend(format!("read: {error}"))),
        }
    }

    /// Serializes and atomically replaces the file, creating parents as needed.
    async fn write_all(&self, projects: &[RecentProject]) -> Result<(), StoreError> {
        if let Some(parent) = self.file_path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|error| StoreError::Backend(format!("mkdir: {error}")))?;
        }
        let bytes = serde_json::to_vec_pretty(projects)
            .map_err(|error| StoreError::Backend(format!("serialize: {error}")))?;
        write_atomic(&self.file_path, &bytes)
            .await
            .map_err(|error| StoreError::Backend(format!("write: {error}")))
    }
}

/// Current time as Unix seconds; clamps a pre-epoch clock to zero.
fn now_unix() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs() as i64)
        .unwrap_or(0)
}

/// Writes `bytes` to `path` via a temporary file and a rename, so a crash
/// mid-write cannot leave a truncated list behind.
async fn write_atomic(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let temp = path.with_extension("json.tmp");
    tokio::fs::write(&temp, bytes).await?;
    tokio::fs::rename(&temp, path).await
}

#[async_trait]
impl RecentProjectsStore for JsonRecentProjectsStore {
    async fn list(&self) -> Result<Vec<RecentProject>, StoreError> {
        self.read_all().await
    }

    async fn add(&self, project: &RecentProject) -> Result<(), StoreError> {
        let mut projects = self.read_all().await?;
        projects.retain(|existing| existing.id != project.id);

        let mut entry = project.clone();
        entry.last_opened = now_unix();
        projects.insert(0, entry);
        projects.truncate(MAX_ENTRIES);

        self.write_all(&projects).await
    }

    async fn remove(&self, id: &RepositoryId) -> Result<(), StoreError> {
        let mut projects = self.read_all().await?;
        let before = projects.len();
        projects.retain(|existing| &existing.id != id);
        if projects.len() == before {
            return Ok(()); // nothing to remove is not an error
        }
        self.write_all(&projects).await
    }
}
