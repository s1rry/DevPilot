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

pub use claude::ClaudeProvider;
pub use gemini::GeminiProvider;
pub use ollama::OllamaProvider;
pub use openai::OpenAiProvider;
