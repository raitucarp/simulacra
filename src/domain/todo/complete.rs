//! Completing to-do tokens as they are typed (D11): on a to-do line, a word being typed that
//! starts with `^` offers due phrases (with the day each names), `^^` repeat rules, `&` the
//! contexts already in use and `@` the agents. Picking one replaces the word.

use super::due::resolve_due;
use super::line::is_todo_line;
use jiff::civil::Date;
use std::ops::Range;

/// The most suggestions offered at once.
pub const MAX_SUGGESTIONS: usize = 6;
const DUE_PHRASES: [&str; 13] = ["today", "tomorrow", "mon", "tue", "wed", "thu", "fri", "sat", "sun", "next week", "next month", "+1w", "eom"];
const REPEAT_RULES: [&str; 7] = ["daily", "weekdays", "weekly", "mon,wed", "biweekly", "monthly", "15th"];

/// What the word being typed is for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionKind {
    Due,
    Repeat,
    Context,
    Delegate,
}

impl CompletionKind {
    /// The sigil a completed token starts with.
    pub const fn sigil(self) -> &'static str {
        match self {
            Self::Due => "^",
            Self::Repeat => "^^",
            Self::Context => "&",
            Self::Delegate => "@",
        }
    }
}

/// The word being typed at the cursor: where it sits, what it is for and what follows its sigil.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Typing {
    pub range: Range<usize>,
    pub kind: CompletionKind,
    pub typed: String,
}

/// What can be offered besides the fixed phrases.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CompletionSources {
    /// Contexts in use, without `&`.
    pub contexts: Vec<String>,
    /// Agents' handles, without `@`.
    pub agents: Vec<String>,
}

/// One suggestion: the token it inserts and, for a due phrase, the day it names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Suggestion {
    pub token: String,
    pub day: Option<Date>,
}

/// The token being typed at byte `cursor` of `text`, when its line is a to-do.
pub fn typing_at(text: &str, cursor: usize) -> Option<Typing> {
    let cursor = cursor.min(text.len());
    let before = text.get(..cursor)?;
    let line_start = before.rfind('\n').map_or(0, |at| at + 1);
    let line_end = text[cursor..].find('\n').map_or(text.len(), |at| cursor + at);
    if !is_todo_line(&text[line_start..line_end]) {
        return None;
    }
    let start = before.char_indices().rev().find(|(_, c)| c.is_whitespace()).map_or(0, |(at, c)| at + c.len_utf8()).max(line_start);
    let word = &before[start..];
    let (kind, typed) = if let Some(rest) = word.strip_prefix("^^") {
        (CompletionKind::Repeat, rest)
    } else if let Some(rest) = word.strip_prefix('^') {
        (CompletionKind::Due, rest)
    } else if let Some(rest) = word.strip_prefix('&') {
        (CompletionKind::Context, rest)
    } else {
        (CompletionKind::Delegate, word.strip_prefix('@')?)
    };
    Some(Typing { range: start..cursor, kind, typed: typed.to_string() })
}

/// What to offer for `typing` on `today`: phrases and names starting with what was typed
/// (ignoring case), never the very word already typed.
pub fn suggestions(typing: &Typing, sources: &CompletionSources, today: Date) -> Vec<Suggestion> {
    let typed = typing.typed.to_lowercase();
    let starts = |candidate: &str| candidate.to_lowercase().starts_with(&typed) && candidate.to_lowercase() != typed;
    let names: Vec<String> = match typing.kind {
        CompletionKind::Due => DUE_PHRASES.iter().map(|phrase| phrase.to_string()).collect(),
        CompletionKind::Repeat => REPEAT_RULES.iter().map(|rule| rule.to_string()).collect(),
        CompletionKind::Context => sources.contexts.clone(),
        CompletionKind::Delegate => sources.agents.clone(),
    };
    names
        .into_iter()
        .filter(|name| starts(name))
        .take(MAX_SUGGESTIONS)
        .map(|name| {
            let day = (typing.kind == CompletionKind::Due).then(|| resolve_due(&name, today)).flatten();
            Suggestion { token: format!("{}{name}", typing.kind.sigil()), day }
        })
        .collect()
}

/// `text` with `typing` replaced by `token` and a space; the new cursor.
pub fn complete(text: &str, typing: &Typing, token: &str) -> (String, usize) {
    let inserted = format!("{token} ");
    let mut out = String::with_capacity(text.len() + inserted.len());
    out.push_str(&text[..typing.range.start]);
    out.push_str(&inserted);
    let cursor = out.len();
    out.push_str(text[typing.range.end..].strip_prefix(' ').unwrap_or(&text[typing.range.end..]));
    (out, cursor)
}
