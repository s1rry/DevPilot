use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Which LLM provider is selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ProviderKind {
    /// Local Ollama (no API key).
    Ollama,
    /// Anthropic Claude.
    Claude,
    /// OpenAI.
    OpenAI,
    /// Google Gemini.
    Gemini,
}

impl ProviderKind {
    /// Whether the provider requires an API key.
    pub fn needs_api_key(&self) -> bool {
        !matches!(self, Self::Ollama)
    }
}

/// User AI settings: which provider and model to use, and the API keys.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiSettings {
    /// Active provider.
    pub provider: ProviderKind,
    /// Model identifier to request.
    pub model: String,
    /// API keys per provider. Absent for providers not configured yet.
    pub api_keys: BTreeMap<ProviderKind, String>,
}

impl Default for AiSettings {
    /// Defaults to local Ollama, which needs no key.
    fn default() -> Self {
        Self {
            provider: ProviderKind::Ollama,
            model: "llama3".to_string(),
            api_keys: BTreeMap::new(),
        }
    }
}

impl AiSettings {
    /// Returns the API key for the active provider, if any.
    pub fn active_key(&self) -> Option<&str> {
        self.api_keys.get(&self.provider).map(String::as_str)
    }
}
