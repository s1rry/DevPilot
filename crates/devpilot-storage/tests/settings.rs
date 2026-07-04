//! Integration tests for the JSON settings store.

use devpilot_core::entities::{AiSettings, ProviderKind};
use devpilot_core::ports::SettingsStore;
use devpilot_storage::JsonSettingsStore;
use tempfile::TempDir;

fn store(dir: &TempDir) -> JsonSettingsStore {
    JsonSettingsStore::new(dir.path().join("state").join("ai-settings.json"))
}

#[tokio::test]
async fn missing_file_yields_defaults() {
    let dir = TempDir::new().expect("tempdir");
    let settings = store(&dir).load().await.expect("load");
    assert_eq!(settings, AiSettings::default());
    assert_eq!(settings.provider, ProviderKind::Ollama);
}

#[tokio::test]
async fn saves_and_loads_settings_with_keys() {
    let dir = TempDir::new().expect("tempdir");
    let store = store(&dir);

    let mut settings = AiSettings {
        provider: ProviderKind::Claude,
        model: "claude-sonnet".into(),
        ..AiSettings::default()
    };
    settings
        .api_keys
        .insert(ProviderKind::Claude, "sk-ant-123".into());
    settings
        .api_keys
        .insert(ProviderKind::OpenAI, "sk-openai-456".into());

    store.save(&settings).await.expect("save");
    let loaded = store.load().await.expect("load");

    assert_eq!(loaded, settings);
    assert_eq!(loaded.active_key(), Some("sk-ant-123"));
}
