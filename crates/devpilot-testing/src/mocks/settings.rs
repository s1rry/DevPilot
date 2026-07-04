use std::sync::Mutex;

use async_trait::async_trait;
use devpilot_core::entities::AiSettings;
use devpilot_core::errors::StoreError;
use devpilot_core::ports::SettingsStore;

/// In-memory [`SettingsStore`] for tests.
pub struct MockSettingsStore {
    settings: Mutex<AiSettings>,
    error: Option<StoreError>,
}

impl MockSettingsStore {
    /// Creates a store holding the given settings.
    pub fn new(settings: AiSettings) -> Self {
        Self {
            settings: Mutex::new(settings),
            error: None,
        }
    }

    /// Creates a store where every operation fails with `error`.
    pub fn failing(error: StoreError) -> Self {
        Self {
            settings: Mutex::new(AiSettings::default()),
            error: Some(error),
        }
    }
}

impl Default for MockSettingsStore {
    fn default() -> Self {
        Self::new(AiSettings::default())
    }
}

#[async_trait]
impl SettingsStore for MockSettingsStore {
    async fn load(&self) -> Result<AiSettings, StoreError> {
        match &self.error {
            Some(error) => Err(error.clone()),
            None => Ok(self.settings.lock().expect("mock mutex poisoned").clone()),
        }
    }

    async fn save(&self, settings: &AiSettings) -> Result<(), StoreError> {
        if let Some(error) = &self.error {
            return Err(error.clone());
        }
        *self.settings.lock().expect("mock mutex poisoned") = settings.clone();
        Ok(())
    }
}
