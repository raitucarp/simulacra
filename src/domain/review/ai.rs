//! The AI suggestion for the face the learner writes: a question the post or passage
//! answers ("Add question"), or a short answer to it as a prompt ("Add answer"). The
//! learner always edits or accepts the suggestion before saving.

use super::item::AnchorSide;
use serde::Deserialize;

/// System prompt of the suggestion request.
pub const SUGGEST_SYSTEM_PROMPT: &str = "You help a learner review their own notes with spaced repetition and reply with one JSON object only.";
/// Characters of a suggestion kept.
pub const MAX_SUGGESTION_CHARS: usize = 400;

/// The user prompt for a suggestion of the `side` the learner writes about `text`.
pub fn suggest_prompt(text: &str, side: AnchorSide, language: &str) -> String {
    let task = match side {
        AnchorSide::Back => "Write one question whose answer is the text below. Ask for understanding of its main idea, not trivia.",
        AnchorSide::Front => "The text below is a prompt. Write the short answer a learner should recall when they see it.",
    };
    format!(
        "{task}\nRules: one sentence; use the text's own ideas only; write in {language}.\n\
         Reply with {{\"text\":\"…\"}} and nothing else.\n\nText:\n\"\"\"\n{}\n\"\"\"",
        text.trim()
    )
}

#[derive(Deserialize)]
struct Reply {
    #[serde(default)]
    text: String,
}

/// Why a reply could not be used.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SuggestError {
    NoJsonObject,
    Malformed(String),
    Empty,
}

impl SuggestError {
    pub const fn message_key(&self) -> &'static str {
        match self {
            Self::NoJsonObject | Self::Malformed(_) => "review.learn.ai_unreadable",
            Self::Empty => "review.learn.ai_empty",
        }
    }
}

/// The outermost `{…}` of `reply` (code fences and chatter around it are ignored).
fn json_object(reply: &str) -> Option<&str> {
    let (start, end) = (reply.find('{')?, reply.rfind('}')?);
    (end > start).then(|| &reply[start..=end])
}

/// The suggested text of an AI reply, trimmed and capped.
pub fn parse_suggestion(reply: &str) -> Result<String, SuggestError> {
    let json = json_object(reply).ok_or(SuggestError::NoJsonObject)?;
    let parsed: Reply = serde_json::from_str(json).map_err(|e| SuggestError::Malformed(e.to_string()))?;
    let text: String = parsed.text.trim().chars().take(MAX_SUGGESTION_CHARS).collect();
    if text.is_empty() {
        return Err(SuggestError::Empty);
    }
    Ok(text)
}
