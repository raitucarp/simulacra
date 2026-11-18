//! The AI suggestion of the written face (a question for "Add question", a short answer for
//! "Add answer") through the configured LLM client. Runs on a private single-thread
//! runtime off the UI thread.

use crate::agent::LlmClient;
use crate::domain::review::ai::{parse_suggestion, suggest_prompt, SUGGEST_SYSTEM_PROMPT};
use crate::domain::review::AnchorSide;
use rust_i18n::t;

/// Characters of source text sent with one request.
pub const MAX_SUGGEST_SOURCE_CHARS: usize = 4_000;

/// Suggests the `side` the learner writes about `text` (in `language`, an English name).
pub async fn request_suggestion(client: &LlmClient, text: &str, side: AnchorSide, language: &str) -> Result<String, String> {
    if !client.has_api_key() {
        return Err(t!("review.learn.ai_no_key").to_string());
    }
    let text: String = text.chars().take(MAX_SUGGEST_SOURCE_CHARS).collect();
    let reply = client.generate(SUGGEST_SYSTEM_PROMPT, &suggest_prompt(&text, side, language)).await.map_err(|raw| client.describe_failure(&raw).message)?;
    parse_suggestion(&reply).map_err(|error| t!(error.message_key()).to_string())
}

/// [`request_suggestion`] blocking on its own runtime (call from a background task).
pub fn request_suggestion_blocking(client: &LlmClient, text: &str, side: AnchorSide, language: &str) -> Result<String, String> {
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().map_err(|e| e.to_string())?;
    runtime.block_on(request_suggestion(client, text, side, language))
}
