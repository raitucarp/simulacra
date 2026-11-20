//! Reading raw query parts as words, phrases and filters.

use super::lex::{is_quoted, lex, unquote};
use super::{HasFilter, IsFilter, PostQuery, QueryDate, QueryFilter};
use jiff::civil::Date;

const NEGATION: char = '-';
const KEY_SEPARATOR: char = ':';
const MENTION_SIGIL: char = '@';
/// Byte positions of the dashes in `YYYY-MM-DD`.
const ISO_DASHES: [usize; 2] = [4, 7];
const ISO_LEN: usize = 10;

/// One classified part of the query.
enum Part {
    Word(String),
    Phrase(String),
    Filter(QueryFilter),
}

/// Parses `input` into a [`PostQuery`] (see the module rules).
pub fn parse_query(input: &str) -> PostQuery {
    let mut query = PostQuery::default();
    for raw in lex(input) {
        let (negated, body) = match raw.strip_prefix(NEGATION) {
            Some(rest) if !rest.is_empty() => (true, rest),
            _ => (false, raw),
        };
        let Some(part) = classify(body) else { continue };
        match (part, negated) {
            (Part::Word(w), false) => query.words.push(w),
            (Part::Word(w), true) => query.excluded_words.push(w),
            (Part::Phrase(p), false) => query.phrases.push(p),
            (Part::Phrase(p), true) => query.excluded_phrases.push(p),
            (Part::Filter(f), false) => query.filters.push(f),
            (Part::Filter(f), true) => query.excluded_filters.push(f),
        }
    }
    query
}

/// `None` for an empty phrase (`""`).
fn classify(raw: &str) -> Option<Part> {
    if is_quoted(raw) {
        let phrase = unquote(raw).trim();
        return (!phrase.is_empty()).then(|| Part::Phrase(phrase.to_string()));
    }
    Some(filter(raw).map_or_else(|| Part::Word(raw.to_string()), Part::Filter))
}

fn filter(raw: &str) -> Option<QueryFilter> {
    let (key, value) = raw.split_once(KEY_SEPARATOR)?;
    let value = unquote(value).trim();
    if value.is_empty() {
        return None;
    }
    let text = || value.to_string();
    match key.to_ascii_lowercase().as_str() {
        "from" => named(value.trim_start_matches(MENTION_SIGIL)).map(QueryFilter::From),
        "kind" => Some(QueryFilter::Kind(text())),
        "symbol" => Some(QueryFilter::Symbol(text())),
        "tint" => Some(QueryFilter::Tint(text())),
        "has" => HasFilter::parse(value).map(QueryFilter::Has),
        "is" => IsFilter::parse(value).map(QueryFilter::Is),
        "before" => Some(QueryFilter::Before(QueryDate::parse(value))),
        "after" => Some(QueryFilter::After(QueryDate::parse(value))),
        _ => None,
    }
}

fn named(value: &str) -> Option<String> {
    (!value.is_empty()).then(|| value.to_string())
}

impl HasFilter {
    /// Reads `image`, `file`, `code` or `todo`, ignoring case.
    pub fn parse(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "image" => Some(Self::Image),
            "file" => Some(Self::File),
            "code" => Some(Self::Code),
            "todo" => Some(Self::Todo),
            _ => None,
        }
    }
}

impl IsFilter {
    /// Reads `open`, `done`, `scheduled`, `archived` or `locked`, ignoring case.
    pub fn parse(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "open" => Some(Self::Open),
            "done" => Some(Self::Done),
            "scheduled" => Some(Self::Scheduled),
            "archived" => Some(Self::Archived),
            "locked" => Some(Self::Locked),
            _ => None,
        }
    }
}

impl QueryDate {
    /// A day for a valid `YYYY-MM-DD`, otherwise the raw text.
    pub fn parse(value: &str) -> Self {
        iso_day(value).map_or_else(|| Self::Raw(value.to_string()), Self::Day)
    }
}

fn iso_day(value: &str) -> Option<Date> {
    let bytes = value.as_bytes();
    let shaped = bytes.len() == ISO_LEN
        && bytes.iter().enumerate().all(|(i, b)| {
            if ISO_DASHES.contains(&i) { *b == b'-' } else { b.is_ascii_digit() }
        });
    if !shaped {
        return None;
    }
    value.parse().ok()
}
