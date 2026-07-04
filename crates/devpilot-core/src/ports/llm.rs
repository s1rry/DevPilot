use std::pin::Pin;

use async_trait::async_trait;
use futures_core::Stream;

use crate::entities::{ChatRequest, ModelInfo};
use crate::errors::LlmError;

/// A stream of response token fragments from a chat completion.
///
/// Each item is either the next piece of assistant text or a terminal error.
/// The stream ends when the completion is finished.
pub type TokenStream = Pin<Box<dyn Stream<Item = Result<String, LlmError>> + Send>>;

/// A large language model provider.
///
/// The single interface every backend implements — OpenAI, Claude, Gemini and
/// Ollama. Provider-specific request shapes, wire formats and error codes are
/// hidden inside the adapters; callers see only this trait
/// (ADR-0001, ADR-0002). Implemented in `devpilot-ai` and mocked in
/// `devpilot-testing`.
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Short provider name, e.g. `"openai"` or `"ollama"`.
    fn name(&self) -> &str;

    /// Lists the models the provider currently offers. Doubles as a health
    /// and credentials check.
    async fn models(&self) -> Result<Vec<ModelInfo>, LlmError>;

    /// Starts a streaming chat completion, yielding tokens as they arrive.
    ///
    /// Buffering the whole response instead of streaming is a bug: the UI
    /// relies on incremental tokens.
    async fn chat(&self, request: ChatRequest) -> Result<TokenStream, LlmError>;
}
