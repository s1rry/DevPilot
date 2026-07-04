use std::fmt;

use async_trait::async_trait;
use devpilot_core::entities::{ChatRequest, ModelInfo, Role};
use devpilot_core::errors::LlmError;
use devpilot_core::ports::{LlmProvider, TokenStream};

use crate::common::{client, ensure_success, network, token_stream, LineMode};

/// Default Anthropic API endpoint.
const DEFAULT_BASE_URL: &str = "https://api.anthropic.com/v1";
/// Anthropic API version header value.
const API_VERSION: &str = "2023-06-01";
/// Default response length cap (Claude requires `max_tokens`).
const DEFAULT_MAX_TOKENS: u32 = 4096;

/// Anthropic Claude provider (Messages API).
#[derive(Clone)]
pub struct ClaudeProvider {
    http: reqwest::Client,
    api_key: String,
    base_url: String,
}

impl ClaudeProvider {
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
impl fmt::Debug for ClaudeProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClaudeProvider")
            .field("base_url", &self.base_url)
            .field("api_key", &"***")
            .finish()
    }
}

#[async_trait]
impl LlmProvider for ClaudeProvider {
    fn name(&self) -> &str {
        "claude"
    }

    async fn models(&self) -> Result<Vec<ModelInfo>, LlmError> {
        let response = self
            .http
            .get(format!("{}/models", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", API_VERSION)
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
        // Claude takes system prompts in a dedicated field; the messages list
        // holds only user/assistant turns.
        let system: String = request
            .messages
            .iter()
            .filter(|message| message.role == Role::System)
            .map(|message| message.content.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        let turns: Vec<_> = request
            .messages
            .iter()
            .filter(|message| message.role != Role::System)
            .collect();

        let mut body = serde_json::json!({
            "model": request.model,
            "max_tokens": DEFAULT_MAX_TOKENS,
            "messages": turns,
            "stream": true,
        });
        if !system.is_empty() {
            body["system"] = serde_json::json!(system);
        }
        if let Some(temperature) = request.temperature {
            body["temperature"] = serde_json::json!(temperature);
        }

        let response = self
            .http
            .post(format!("{}/messages", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", API_VERSION)
            .json(&body)
            .send()
            .await
            .map_err(network)?;
        let response = ensure_success(response).await?;

        Ok(token_stream(response, LineMode::Sse, |payload| {
            let value: serde_json::Value = serde_json::from_str(payload).ok()?;
            if value["type"] == serde_json::json!("content_block_delta") {
                let text = value["delta"]["text"].as_str()?;
                return (!text.is_empty()).then(|| text.to_string());
            }
            None
        }))
    }
}
