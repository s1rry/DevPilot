//! Shared HTTP and streaming helpers for the provider adapters.

use devpilot_core::errors::LlmError;
use devpilot_core::ports::TokenStream;
use futures_util::StreamExt;

/// How a streaming response is framed on the wire.
#[derive(Clone, Copy)]
pub(crate) enum LineMode {
    /// Server-Sent Events: payloads on `data:` lines.
    Sse,
    /// Newline-delimited JSON: one object per line.
    Ndjson,
}

/// A shared HTTP client.
pub(crate) fn client() -> reqwest::Client {
    reqwest::Client::new()
}

/// Maps a transport error to [`LlmError::Network`].
pub(crate) fn network(error: reqwest::Error) -> LlmError {
    LlmError::Network(error.to_string())
}

/// Maps a non-success HTTP status and body to a typed [`LlmError`].
pub(crate) fn status_error(status: reqwest::StatusCode, body: &str) -> LlmError {
    match status.as_u16() {
        401 | 403 => LlmError::AuthFailed,
        429 => LlmError::RateLimited {
            retry_after_seconds: None,
        },
        413 => LlmError::ContextTooLong,
        400 if body.to_lowercase().contains("context") => LlmError::ContextTooLong,
        _ => LlmError::Backend(format!("HTTP {}: {}", status.as_u16(), body.trim())),
    }
}

/// Fails the request if the response status is not successful, mapping the
/// body to a typed error.
pub(crate) async fn ensure_success(
    response: reqwest::Response,
) -> Result<reqwest::Response, LlmError> {
    let status = response.status();
    if status.is_success() {
        Ok(response)
    } else {
        let body = response.text().await.unwrap_or_default();
        Err(status_error(status, &body))
    }
}

/// Extracts the meaningful payload from one wire line, or `None` to skip it.
fn payload_of(line: &str, mode: LineMode) -> Option<&str> {
    let line = line.trim_end();
    match mode {
        LineMode::Sse => line.strip_prefix("data:").map(str::trim),
        LineMode::Ndjson => {
            let trimmed = line.trim();
            (!trimmed.is_empty()).then_some(trimmed)
        }
    }
}

/// Turns a streaming HTTP response into a token stream.
///
/// The response body is buffered by line; each payload (SSE `data:` value or
/// NDJSON object) is passed to `extract`, which returns the token text to
/// emit or `None` to skip control frames. An SSE `[DONE]` sentinel ends the
/// stream.
pub(crate) fn token_stream<F>(
    response: reqwest::Response,
    mode: LineMode,
    extract: F,
) -> TokenStream
where
    F: Fn(&str) -> Option<String> + Send + 'static,
{
    let stream = async_stream::stream! {
        let mut bytes = response.bytes_stream();
        let mut buffer = String::new();

        while let Some(chunk) = bytes.next().await {
            let chunk = match chunk {
                Ok(chunk) => chunk,
                Err(error) => {
                    yield Err(network(error));
                    return;
                }
            };
            buffer.push_str(&String::from_utf8_lossy(&chunk));

            while let Some(newline) = buffer.find('\n') {
                let line: String = buffer.drain(..=newline).collect();
                if let Some(payload) = payload_of(&line, mode) {
                    if payload == "[DONE]" {
                        return;
                    }
                    if let Some(token) = extract(payload) {
                        yield Ok(token);
                    }
                }
            }
        }

        // Trailing line without a newline terminator.
        if let Some(payload) = payload_of(&buffer, mode) {
            if payload != "[DONE]" {
                if let Some(token) = extract(payload) {
                    yield Ok(token);
                }
            }
        }
    };

    Box::pin(stream)
}
