use async_trait::async_trait;

use crate::entities::AiSettings;
use crate::errors::StoreError;

/// Persistent store of the user's AI settings (provider, model, API keys).
///
/// Implemented by `devpilot-storage` (a JSON file) and by `MockSettingsStore`
/// in `devpilot-testing`.
#[async_trait]
pub trait SettingsStore: Send + Sync {
    /// Loads the settings, returning defaults when none are stored yet.
    async fn load(&self) -> Result<AiSettings, StoreError>;

    /// Persists the settings, replacing any previous values.
    async fn save(&self, settings: &AiSettings) -> Result<(), StoreError>;
}
