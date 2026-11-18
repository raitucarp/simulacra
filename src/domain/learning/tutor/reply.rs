//! The tutor's structured reply: the assessment of the last answer (level reached, verdict,
//! what was right, misconceptions) and the next question. Validated so the tutor asks rather
//! than lectures: the question must be a question and short.

use super::json::{key_field, list_field, object_in, text_field};
use super::{Assessment, Verdict};
use crate::domain::learning::bloom::BloomLevel;
use serde_json::Value;

/// Longest question accepted, in characters (longer reads as a lecture).
pub const MAX_QUESTION_CHARS: usize = 420;
/// Points kept per list of an assessment.
pub const MAX_POINTS: usize = 4;
/// Longest point kept, in characters.
pub const MAX_POINT_CHARS: usize = 240;
const QUESTION_MARKS: [char; 3] = ['?', '？', '؟'];
const QUOTES: [char; 4] = ['"', '\'', '“', '”'];
/// American spellings models use for the level keys.
const LEVEL_ALIASES: [(&str, BloomLevel); 2] = [("analyze", BloomLevel::Analyse), ("analysis", BloomLevel::Analyse)];

/// One parsed turn.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TurnReply {
    /// None for the opening question (nothing to assess yet).
    pub assessment: Option<Assessment>,
    pub question: String,
}

/// Why a reply cannot be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplyProblem {
    /// Nothing readable in it.
    Unreadable,
    /// No question to ask.
    NoQuestion,
    /// The question runs too long: a lecture, not a question.
    Lecture,
}

impl ReplyProblem {
    /// Locale key of the message.
    pub const fn message_key(self) -> &'static str {
        match self {
            Self::Unreadable => "learning.tutor.problem.unreadable",
            Self::NoQuestion => "learning.tutor.problem.no_question",
            Self::Lecture => "learning.tutor.problem.lecture",
        }
    }
}

/// A level key, tolerant of case and of American spelling.
pub fn level_of(key: &str) -> Option<BloomLevel> {
    let key = key.trim().to_lowercase();
    BloomLevel::from_key(&key).or_else(|| LEVEL_ALIASES.iter().find(|(alias, _)| *alias == key).map(|(_, level)| *level))
}

fn assessment_of(value: &Value) -> Option<Assessment> {
    let object = value.get("assessment").filter(|v| v.is_object())?;
    let verdict = key_field(object, "verdict").and_then(|key| Verdict::from_key(&key)).unwrap_or(Verdict::Partial);
    Some(Assessment {
        reached: key_field(object, "level").and_then(|key| level_of(&key)),
        verdict,
        right: list_field(object, "right", MAX_POINTS, MAX_POINT_CHARS),
        misconceptions: list_field(object, "misconceptions", MAX_POINTS, MAX_POINT_CHARS),
    })
}

/// `text` as a question, or why it is not one.
pub fn checked_question(text: &str) -> Result<String, ReplyProblem> {
    let question = text.trim().trim_matches(|c| QUOTES.contains(&c)).trim().to_string();
    match (question.contains(QUESTION_MARKS), question.chars().count()) {
        (false, _) => Err(ReplyProblem::NoQuestion),
        (true, length) if length > MAX_QUESTION_CHARS => Err(ReplyProblem::Lecture),
        _ => Ok(question),
    }
}

/// The last line of a plain-text reply that asks something.
fn last_question_line(text: &str) -> Option<&str> {
    text.lines().rev().map(str::trim).find(|line| line.contains(QUESTION_MARKS))
}

/// Parses a turn reply: a JSON object (`assessment`, `question`), or, failing that, the last
/// question in plain text.
pub fn parse_turn_reply(text: &str) -> Result<TurnReply, ReplyProblem> {
    match object_in(text) {
        Some(value) => {
            let question = text_field(&value, "question").ok_or(ReplyProblem::NoQuestion)?;
            Ok(TurnReply { assessment: assessment_of(&value), question: checked_question(&question)? })
        }
        None if text.trim().is_empty() => Err(ReplyProblem::Unreadable),
        None => {
            let line = last_question_line(text).ok_or(ReplyProblem::NoQuestion)?;
            Ok(TurnReply { assessment: None, question: checked_question(line)? })
        }
    }
}
