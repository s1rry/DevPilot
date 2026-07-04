use serde::{Deserialize, Serialize};

/// Role of a chat message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// System / instruction message.
    System,
    /// End-user message.
    User,
    /// Model reply.
    Assistant,
}

/// One message in a chat conversation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Who authored the message.
    pub role: Role,
    /// Message text.
    pub content: String,
}

impl ChatMessage {
    /// Convenience constructor.
    pub fn new(role: Role, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }
}

/// A chat completion request, provider-neutral.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatRequest {
    /// Model identifier (provider-specific, e.g. `gpt-4o` or `llama3`).
    pub model: String,
    /// Conversation so far, oldest first.
    pub messages: Vec<ChatMessage>,
    /// Optional sampling temperature.
    pub temperature: Option<f32>,
}

/// Metadata about a model offered by a provider.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Model identifier.
    pub id: String,
    /// Context window in tokens, when known.
    pub context_window: Option<usize>,
}
