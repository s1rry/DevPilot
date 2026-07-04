//! Adapter tests against recorded HTTP responses (wiremock). No live APIs.

use devpilot_ai::{ClaudeProvider, GeminiProvider, OllamaProvider, OpenAiProvider};
use devpilot_core::entities::{ChatMessage, ChatRequest, Role};
use devpilot_core::errors::LlmError;
use devpilot_core::ports::{LlmProvider, TokenStream};
use futures_util::StreamExt;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Collects a token stream into a vector, panicking on any stream error.
async fn drain(mut stream: TokenStream) -> Vec<String> {
    let mut tokens = Vec::new();
    while let Some(item) = stream.next().await {
        tokens.push(item.expect("token"));
    }
    tokens
}

fn request(model: &str) -> ChatRequest {
    ChatRequest {
        model: model.to_string(),
        messages: vec![
            ChatMessage::new(Role::System, "be brief"),
            ChatMessage::new(Role::User, "hi"),
        ],
        temperature: Some(0.2),
    }
}

#[tokio::test]
async fn ollama_streams_ndjson_content() {
    let server = MockServer::start().await;
    let body = "{\"message\":{\"role\":\"assistant\",\"content\":\"Hel\"},\"done\":false}\n\
                {\"message\":{\"role\":\"assistant\",\"content\":\"lo\"},\"done\":false}\n\
                {\"message\":{\"role\":\"assistant\",\"content\":\"\"},\"done\":true}\n";
    Mock::given(method("POST"))
        .and(path("/api/chat"))
        .respond_with(ResponseTemplate::new(200).set_body_string(body))
        .mount(&server)
        .await;

    let provider = OllamaProvider::new().with_base_url(server.uri());
    let stream = provider.chat(request("llama3")).await.expect("chat");
    assert_eq!(drain(stream).await, vec!["Hel", "lo"]);
}

#[tokio::test]
async fn ollama_lists_models() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/tags"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("{\"models\":[{\"name\":\"llama3\"},{\"name\":\"qwen\"}]}"),
        )
        .mount(&server)
        .await;

    let provider = OllamaProvider::new().with_base_url(server.uri());
    let models = provider.models().await.expect("models");
    let ids: Vec<&str> = models.iter().map(|m| m.id.as_str()).collect();
    assert_eq!(ids, vec!["llama3", "qwen"]);
}

#[tokio::test]
async fn openai_streams_sse_content() {
    let server = MockServer::start().await;
    let body = "data: {\"choices\":[{\"delta\":{\"content\":\"Hi\"}}]}\n\
                data: {\"choices\":[{\"delta\":{\"content\":\" there\"}}]}\n\
                data: [DONE]\n";
    Mock::given(method("POST"))
        .and(path("/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_string(body))
        .mount(&server)
        .await;

    let provider = OpenAiProvider::new("sk-test").with_base_url(server.uri());
    let stream = provider.chat(request("gpt-4o")).await.expect("chat");
    assert_eq!(drain(stream).await, vec!["Hi", " there"]);
}

#[tokio::test]
async fn openai_maps_401_to_auth_failed() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/chat/completions"))
        .respond_with(ResponseTemplate::new(401).set_body_string("{\"error\":\"bad key\"}"))
        .mount(&server)
        .await;

    let provider = OpenAiProvider::new("sk-bad").with_base_url(server.uri());
    let result = provider.chat(request("gpt-4o")).await;
    assert_eq!(result.err(), Some(LlmError::AuthFailed));
}

#[tokio::test]
async fn claude_streams_content_block_deltas() {
    let server = MockServer::start().await;
    let body = "data: {\"type\":\"message_start\"}\n\
                data: {\"type\":\"content_block_delta\",\"delta\":{\"type\":\"text_delta\",\"text\":\"Hello\"}}\n\
                data: {\"type\":\"content_block_delta\",\"delta\":{\"type\":\"text_delta\",\"text\":\" world\"}}\n\
                data: {\"type\":\"message_stop\"}\n";
    Mock::given(method("POST"))
        .and(path("/messages"))
        .respond_with(ResponseTemplate::new(200).set_body_string(body))
        .mount(&server)
        .await;

    let provider = ClaudeProvider::new("sk-ant").with_base_url(server.uri());
    let stream = provider.chat(request("claude-sonnet")).await.expect("chat");
    assert_eq!(drain(stream).await, vec!["Hello", " world"]);
}

#[tokio::test]
async fn gemini_streams_sse_candidates() {
    let server = MockServer::start().await;
    let body = "data: {\"candidates\":[{\"content\":{\"parts\":[{\"text\":\"Ge\"}]}}]}\n\
                data: {\"candidates\":[{\"content\":{\"parts\":[{\"text\":\"mini\"}]}}]}\n";
    Mock::given(method("POST"))
        .and(path("/models/gemini-pro:streamGenerateContent"))
        .respond_with(ResponseTemplate::new(200).set_body_string(body))
        .mount(&server)
        .await;

    let provider = GeminiProvider::new("key").with_base_url(server.uri());
    let stream = provider.chat(request("gemini-pro")).await.expect("chat");
    assert_eq!(drain(stream).await, vec!["Ge", "mini"]);
}

#[tokio::test]
async fn rate_limit_maps_to_typed_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/chat"))
        .respond_with(ResponseTemplate::new(429).set_body_string("slow down"))
        .mount(&server)
        .await;

    let provider = OllamaProvider::new().with_base_url(server.uri());
    let result = provider.chat(request("llama3")).await;
    assert!(matches!(result.err(), Some(LlmError::RateLimited { .. })));
}
