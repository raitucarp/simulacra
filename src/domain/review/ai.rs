//! The AI flashcard draft: a prompt asking for question/answer and cloze cards about a text,
//! and a tolerant parser of the JSON reply. The learner always reviews a draft before saving.

use super::card::cloze::cloze_ordinals;
use super::card::DraftKind;
use serde::Deserialize;

/// System prompt of the draft request.
pub const DRAFT_SYSTEM_PROMPT: &str = "You write excellent spaced-repetition flashcards and reply with one JSON object only.";
/// Most cards a draft may hold.
pub const MAX_DRAFT_CARDS: usize = 5;

/// The user prompt for drafting cards about `text`, written in `language` (an English name).
pub fn draft_prompt(text: &str, language: &str) -> String {
    format!(
        "Write at most {MAX_DRAFT_CARDS} flashcards that test the key ideas of the text below.\n\
         Rules: one idea per card; ask for understanding, not trivia; keep answers short; \
         use the text's own facts only; write in {language}.\n\
         Use either a question card {{\"type\":\"qa\",\"question\":\"…\",\"answer\":\"…\"}} \
         or a cloze card {{\"type\":\"cloze\",\"text\":\"… {{{{c1::hidden words}}}} …\"}}.\n\
         Reply with {{\"cards\":[…]}} and nothing else.\n\nText:\n\"\"\"\n{}\n\"\"\"",
        text.trim()
    )
}

#[derive(Deserialize)]
struct Reply {
    #[serde(default)]
    cards: Vec<serde_json::Value>,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum WireCard {
    Qa { question: String, answer: String },
    Cloze { text: String },
}

/// Why a reply could not be used.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DraftParseError {
    NoJsonObject,
    Malformed(String),
    NoCards,
}

impl DraftParseError {
    pub const fn message_key(&self) -> &'static str {
        match self {
            Self::NoJsonObject | Self::Malformed(_) => "review.maker.ai_unreadable",
            Self::NoCards => "review.maker.ai_empty",
        }
    }
}

/// The outermost `{…}` of `reply` (code fences and chatter around it are ignored).
fn json_object(reply: &str) -> Option<&str> {
    let (start, end) = (reply.find('{')?, reply.rfind('}')?);
    (end > start).then(|| &reply[start..=end])
}

fn to_draft(card: WireCard) -> Option<DraftKind> {
    match card {
        WireCard::Qa { question, answer } if !question.trim().is_empty() && !answer.trim().is_empty() => {
            Some(DraftKind::Basic { front: question.trim().into(), back: answer.trim().into() })
        }
        WireCard::Cloze { text } if !cloze_ordinals(&text).is_empty() => Some(DraftKind::Cloze { text: text.trim().into() }),
        _ => None,
    }
}

/// The usable cards of an AI reply (invalid ones skipped).
pub fn parse_draft_reply(reply: &str) -> Result<Vec<DraftKind>, DraftParseError> {
    let json = json_object(reply).ok_or(DraftParseError::NoJsonObject)?;
    let parsed: Reply = serde_json::from_str(json).map_err(|e| DraftParseError::Malformed(e.to_string()))?;
    let drafts: Vec<DraftKind> = parsed.cards.into_iter().filter_map(|card| serde_json::from_value(card).ok()).filter_map(to_draft).take(MAX_DRAFT_CARDS).collect();
    if drafts.is_empty() {
        return Err(DraftParseError::NoCards);
    }
    Ok(drafts)
}
