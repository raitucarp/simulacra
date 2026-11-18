//! The AI flashcard draft request: the prompt through the configured LLM client, the reply
//! parsed into card drafts. Runs on a private single-thread runtime off the UI thread.

use crate::agent::LlmClient;
use crate::domain::review::ai::{draft_prompt, parse_draft_reply, DRAFT_SYSTEM_PROMPT};
use crate::domain::review::DraftKind;
use rust_i18n::t;

/// Characters of source text sent with one request.
pub const MAX_DRAFT_SOURCE_CHARS: usize = 4_000;

/// Drafts cards about `text` (in `language`, an English language name).
pub async fn request_drafts(client: &LlmClient, text: &str, language: &str) -> Result<Vec<DraftKind>, String> {
    if !client.has_api_key() {
        return Err(t!("review.maker.ai_no_key").to_string());
    }
    let text: String = text.chars().take(MAX_DRAFT_SOURCE_CHARS).collect();
    let reply = client.generate(DRAFT_SYSTEM_PROMPT, &draft_prompt(&text, language)).await.map_err(|raw| client.describe_failure(&raw).message)?;
    parse_draft_reply(&reply).map_err(|error| t!(error.message_key()).to_string())
}

/// [`request_drafts`] blocking on its own runtime (call from a background task).
pub fn request_drafts_blocking(client: &LlmClient, text: &str, language: &str) -> Result<Vec<DraftKind>, String> {
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().map_err(|e| e.to_string())?;
    runtime.block_on(request_drafts(client, text, language))
}
