//! # devpilot-ai
//!
//! LLM provider adapters for DevPilot. Each adapter implements the single
//! [`LlmProvider`](devpilot_core::ports::LlmProvider) port from
//! `devpilot-core` over raw HTTP: [`OllamaProvider`] (local, the reference
//! adapter), [`ClaudeProvider`], [`OpenAiProvider`] and [`GeminiProvider`].
//!
//! This crate is the abstraction only: it turns provider-neutral requests
//! into each API's wire format, streams tokens back, and maps errors to the
//! typed `LlmError`. It holds no business logic — no context building, no
//! conversation management.
//!
//! ## Rules
//!
//! - No provider SDKs and no LLM frameworks; thin `reqwest` adapters only.
//! - API keys are never logged and never appear in `Debug` output.
//! - Ollama works with zero API keys so contributors and CI can test locally.

mod claude;
mod common;
mod gemini;
mod ollama;
mod openai;

use std::sync::Arc;

use devpilot_core::entities::{AiSettings, ProviderKind};
use devpilot_core::ports::LlmProvider;

pub use claude::ClaudeProvider;
pub use gemini::GeminiProvider;
pub use ollama::OllamaProvider;
pub use openai::OpenAiProvider;

/// Builds the provider selected in `settings`, using its configured API key.
///
/// This is the single place that maps a [`ProviderKind`] to a concrete
/// adapter, so wiring code stays free of provider `match`es.
pub fn build_provider(settings: &AiSettings) -> Arc<dyn LlmProvider> {
    let key = settings.active_key().unwrap_or_default().to_string();
    match settings.provider {
        ProviderKind::Ollama => Arc::new(OllamaProvider::new()),
        ProviderKind::Claude => Arc::new(ClaudeProvider::new(key)),
        ProviderKind::OpenAI => Arc::new(OpenAiProvider::new(key)),
        ProviderKind::Gemini => Arc::new(GeminiProvider::new(key)),
    }
}
