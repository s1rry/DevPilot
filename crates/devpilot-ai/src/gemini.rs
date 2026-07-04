use std::fmt;

use async_trait::async_trait;
use devpilot_core::entities::{ChatRequest, ModelInfo, Role};
use devpilot_core::errors::LlmError;
use devpilot_core::ports::{LlmProvider, TokenStream};

use crate::common::{client, ensure_success, network, token_stream, LineMode};

/// Default Gemini API endpoint.
const DEFAULT_BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta";

/// Google Gemini provider (generateContent API).
#[derive(Clone)]
pub struct GeminiProvider {
    http: reqwest::Client,
    api_key: String,
    base_url: String,
}

impl GeminiProvider {
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
impl fmt::Debug for GeminiProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GeminiProvider")
            .field("base_url", &self.base_url)
            .field("api_key", &"***")
            .finish()
    }
}

#[async_trait]
impl LlmProvider for GeminiProvider {
    fn name(&self) -> &str {
        "gemini"
    }

    async fn models(&self) -> Result<Vec<ModelInfo>, LlmError> {
        let response = self
            .http
            .get(format!("{}/models", self.base_url))
            .query(&[("key", &self.api_key)])
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
        // Gemini uses `user`/`model` roles and a separate system instruction.
        let contents: Vec<serde_json::Value> = request
            .messages
            .iter()
            .filter(|message| message.role != Role::System)
            .map(|message| {
                let role = if message.role == Role::Assistant {
                    "model"
                } else {
                    "user"
                };
                serde_json::json!({
                    "role": role,
                    "parts": [{ "text": message.content }],
                })
            })
            .collect();

        let system: String = request
            .messages
            .iter()
            .filter(|message| message.role == Role::System)
            .map(|message| message.content.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        let mut body = serde_json::json!({ "contents": contents });
        if !system.is_empty() {
            body["systemInstruction"] = serde_json::json!({ "parts": [{ "text": system }] });
        }
        if let Some(temperature) = request.temperature {
            body["generationConfig"] = serde_json::json!({ "temperature": temperature });
        }

        let url = format!(
            "{}/models/{}:streamGenerateContent",
            self.base_url, request.model
        );
        let response = self
            .http
            .post(url)
            .query(&[("alt", "sse"), ("key", self.api_key.as_str())])
            .json(&body)
            .send()
            .await
            .map_err(network)?;
        let response = ensure_success(response).await?;

        Ok(token_stream(response, LineMode::Sse, |payload| {
            let value: serde_json::Value = serde_json::from_str(payload).ok()?;
            let text = value["candidates"][0]["content"]["parts"][0]["text"].as_str()?;
            (!text.is_empty()).then(|| text.to_string())
        }))
    }
}
