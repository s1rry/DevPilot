use async_trait::async_trait;
use devpilot_core::entities::{ChatRequest, ModelInfo};
use devpilot_core::errors::LlmError;
use devpilot_core::ports::{LlmProvider, TokenStream};
use futures_util::stream;

/// Configurable [`LlmProvider`] for tests.
///
/// Streams a fixed list of tokens and returns a fixed model list, or fails
/// with a configured error.
pub struct MockLlmProvider {
    models: Vec<ModelInfo>,
    tokens: Vec<String>,
    error: Option<LlmError>,
}

impl MockLlmProvider {
    /// Creates a mock that streams `tokens`.
    pub fn new(tokens: impl IntoIterator<Item = &'static str>) -> Self {
        Self {
            models: Vec::new(),
            tokens: tokens.into_iter().map(str::to_string).collect(),
            error: None,
        }
    }

    /// Configures the models returned by [`LlmProvider::models`].
    pub fn with_models(mut self, models: Vec<ModelInfo>) -> Self {
        self.models = models;
        self
    }

    /// Creates a mock where every call fails with `error`.
    pub fn failing(error: LlmError) -> Self {
        Self {
            models: Vec::new(),
            tokens: Vec::new(),
            error: Some(error),
        }
    }
}

#[async_trait]
impl LlmProvider for MockLlmProvider {
    fn name(&self) -> &str {
        "mock"
    }

    async fn models(&self) -> Result<Vec<ModelInfo>, LlmError> {
        match &self.error {
            Some(error) => Err(error.clone()),
            None => Ok(self.models.clone()),
        }
    }

    async fn chat(&self, _request: ChatRequest) -> Result<TokenStream, LlmError> {
        if let Some(error) = &self.error {
            return Err(error.clone());
        }
        let tokens: Vec<Result<String, LlmError>> = self.tokens.iter().cloned().map(Ok).collect();
        Ok(Box::pin(stream::iter(tokens)))
    }
}
