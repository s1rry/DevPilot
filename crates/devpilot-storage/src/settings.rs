use std::path::{Path, PathBuf};

use async_trait::async_trait;
use devpilot_core::entities::AiSettings;
use devpilot_core::errors::StoreError;
use devpilot_core::ports::SettingsStore;

/// A [`SettingsStore`] backed by a single JSON file.
///
/// API keys are stored in plaintext in the application data directory. This
/// is acceptable for a local desktop app; an OS keychain backend can replace
/// this store later without touching callers.
pub struct JsonSettingsStore {
    file_path: PathBuf,
}

impl JsonSettingsStore {
    /// Creates a store persisting to `file_path`.
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: file_path.into(),
        }
    }
}

/// Writes `bytes` to `path` via a temporary file and a rename.
async fn write_atomic(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let temp = path.with_extension("json.tmp");
    tokio::fs::write(&temp, bytes).await?;
    tokio::fs::rename(&temp, path).await
}

#[async_trait]
impl SettingsStore for JsonSettingsStore {
    async fn load(&self) -> Result<AiSettings, StoreError> {
        match tokio::fs::read(&self.file_path).await {
            Ok(bytes) => serde_json::from_slice(&bytes)
                .map_err(|error| StoreError::Backend(format!("parse: {error}"))),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(AiSettings::default()),
            Err(error) => Err(StoreError::Backend(format!("read: {error}"))),
        }
    }

    async fn save(&self, settings: &AiSettings) -> Result<(), StoreError> {
        if let Some(parent) = self.file_path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|error| StoreError::Backend(format!("mkdir: {error}")))?;
        }
        let bytes = serde_json::to_vec_pretty(settings)
            .map_err(|error| StoreError::Backend(format!("serialize: {error}")))?;
        write_atomic(&self.file_path, &bytes)
            .await
            .map_err(|error| StoreError::Backend(format!("write: {error}")))
    }
}
