//! # devpilot-ai
//!
//! LLM provider adapters for DevPilot. Implements the `LlmProvider` port from
//! `devpilot-core` for four interchangeable backends:
//! Ollama (local, the reference adapter), Claude, OpenAI, and Gemini.
//!
//! Planned contents (added with roadmap phase 4):
//!
//! - One structurally identical adapter file per provider, over raw HTTP.
//! - Token streaming for all providers.
//! - Typed error mapping: auth, rate limits, context overflow, network.
//!
//! ## Rules
//!
//! - No provider SDKs and no LLM frameworks (ADR-0001).
//! - API keys are never logged and never appear in `Debug` output.
//! - Ollama must work with zero API keys so contributors can test locally.
