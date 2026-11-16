//! A user-presentable LLM failure: provider plus a short, secret-free message.

use super::LlmProvider;

/// Maximum characters of the provider message kept for display.
pub const MAX_FAILURE_MESSAGE_CHARS: usize = 180;
/// Replacement for any secret found in a provider message.
const REDACTED: &str = "[redacted]";
/// Appended when a message is shortened.
const ELLIPSIS: char = '…';
/// URL query parameter carrying the key for providers such as Gemini.
const KEY_QUERY_PARAM: &str = "key=";
/// Shortest configured key worth redacting verbatim (avoids mangling tiny strings).
const MIN_REDACTABLE_KEY_CHARS: usize = 4;

/// Why an LLM request did not produce text (the heuristic reply was used instead).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlmFailure {
    pub provider: LlmProvider,
    /// Short, single-line message with every API key redacted.
    pub message: String,
    /// The request was never sent because no API key is configured.
    pub missing_key: bool,
}

impl LlmFailure {
    /// Builds a failure from a raw provider error, redacting `api_key` and shortening it.
    pub fn new(provider: LlmProvider, raw: &str, api_key: &str, missing_key: bool) -> Self {
        let message = shorten(&redact(raw, api_key));
        Self { provider, message, missing_key }
    }
}

/// Removes the configured key and any `key=` query value, then collapses whitespace.
fn redact(raw: &str, api_key: &str) -> String {
    let key = api_key.trim();
    let mut text = if key.chars().count() >= MIN_REDACTABLE_KEY_CHARS {
        raw.replace(key, REDACTED)
    } else {
        raw.to_string()
    };
    text = redact_query_keys(&text);
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Replaces the value of every `key=` query parameter with [`REDACTED`].
fn redact_query_keys(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(pos) = rest.find(KEY_QUERY_PARAM) {
        let value_start = pos + KEY_QUERY_PARAM.len();
        out.push_str(&rest[..value_start]);
        let tail = &rest[value_start..];
        let value_len = tail.find(is_query_terminator).unwrap_or(tail.len());
        out.push_str(REDACTED);
        rest = &tail[value_len..];
    }
    out.push_str(rest);
    out
}

/// Characters that end a URL query value.
fn is_query_terminator(c: char) -> bool {
    c.is_whitespace() || matches!(c, '&' | ')' | '"' | '\'' | '>')
}

/// Truncates to [`MAX_FAILURE_MESSAGE_CHARS`] characters, marking the cut with an ellipsis.
fn shorten(text: &str) -> String {
    if text.chars().count() <= MAX_FAILURE_MESSAGE_CHARS {
        return text.to_string();
    }
    let mut short: String = text.chars().take(MAX_FAILURE_MESSAGE_CHARS).collect();
    short.push(ELLIPSIS);
    short
}
