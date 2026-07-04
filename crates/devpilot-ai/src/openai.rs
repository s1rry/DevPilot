use std::fmt;

use async_trait::async_trait;
use devpilot_core::entities::{ChatRequest, ModelInfo};
use devpilot_core::errors::LlmError;
use devpilot_core::ports::{LlmProvider, TokenStream};

use crate::common::{client, ensure_success, network, token_stream, LineMode};

/// Default OpenAI API endpoint.
const DEFAULT_BASE_URL: &str = "https://api.openai.com/v1";

/// OpenAI provider (chat completions API).
#[derive(Clone)]
pub struct OpenAiProvider {
    http: reqwest::Client,
    api_key: String,
    base_url: String,
}

impl OpenAiProvider {
    /// Creates a provider with the given API key.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            http: client(),
            api_key: api_key.into(),
            base_url: DEFAULT_BASE_URL.to_string(),
        }
    }

    /// Overrides the base URL (used in tests).
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }
}

/// Redacts the API key so it never appears in logs.
impl fmt::Debug for OpenAiProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OpenAiProvider")
            .field("base_url", &self.base_url)
            .field("api_key", &"***")
            .finish()
    }
}

#[async_trait]
impl LlmProvider for OpenAiProvider {
    fn name(&self) -> &str {
        "openai"
    }

    async fn models(&self) -> Result<Vec<ModelInfo>, LlmError> {
        let response = self
            .http
            .get(format!("{}/models", self.base_url))
            .bearer_auth(&self.api_key)
            .send()
            .await
            .map_err(network)?;
        let response = ensure_success(response).await?;
        let body: serde_json::Value = response.json().await.map_err(network)?;

        let models = body["data"]
            .as_array()
            .map(|items| {
                items
                    .iter()
                    .filter_map(|item| item["id"].as_str())
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
            body["temperature"] = serde_json::json!(temperature);
        }

        let response = self
            .http
            .post(format!("{}/chat/completions", self.base_url))
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(network)?;
        let response = ensure_success(response).await?;

        Ok(token_stream(response, LineMode::Sse, |payload| {
            let value: serde_json::Value = serde_json::from_str(payload).ok()?;
            let content = value["choices"][0]["delta"]["content"].as_str()?;
            (!content.is_empty()).then(|| content.to_string())
        }))
    }
}
