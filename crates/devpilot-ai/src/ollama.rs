use async_trait::async_trait;
use devpilot_core::entities::{ChatRequest, ModelInfo};
use devpilot_core::errors::LlmError;
use devpilot_core::ports::{LlmProvider, TokenStream};

use crate::common::{client, ensure_success, network, token_stream, LineMode};

/// Default Ollama endpoint.
const DEFAULT_BASE_URL: &str = "http://localhost:11434";

/// Local Ollama provider. Needs no API key, so it is the reference adapter
/// for contributors and CI.
#[derive(Debug, Clone)]
pub struct OllamaProvider {
    http: reqwest::Client,
    base_url: String,
}

impl OllamaProvider {
    /// Creates a provider pointing at the default local endpoint.
    pub fn new() -> Self {
        Self {
            http: client(),
            base_url: DEFAULT_BASE_URL.to_string(),
        }
    }

    /// Overrides the base URL (used in tests).
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }
}

impl Default for OllamaProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LlmProvider for OllamaProvider {
    fn name(&self) -> &str {
        "ollama"
    }

    async fn models(&self) -> Result<Vec<ModelInfo>, LlmError> {
        let response = self
            .http
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await
            .map_err(network)?;
        let response = ensure_success(response).await?;
        let body: serde_json::Value = response.json().await.map_err(network)?;

        let models = body["models"]
            .as_array()
            .map(|items| {
                items
                    .iter()
                    .filter_map(|item| item["name"].as_str())
                    .map(|id| ModelInfo {
                        id: id.to_string(),
                        context_window: None,
                    })
                    .collect()
            })
            .unwrap_or_default();
        Ok(models)
    }

    async fn chat(&self, request: ChatRequest) -> Result<TokenStream, LlmError> {
        let mut body = serde_json::json!({
            "model": request.model,
            "messages": request.messages,
            "stream": true,
        });
        if let Some(temperature) = request.temperature {
            body["options"] = serde_json::json!({ "temperature": temperature });
        }

        let response = self
            .http
            .post(format!("{}/api/chat", self.base_url))
            .json(&body)
            .send()
            .await
            .map_err(network)?;
        let response = ensure_success(response).await?;

        Ok(token_stream(response, LineMode::Ndjson, |payload| {
            let value: serde_json::Value = serde_json::from_str(payload).ok()?;
            let content = value["message"]["content"].as_str()?;
            (!content.is_empty()).then(|| content.to_string())
        }))
    }
}
