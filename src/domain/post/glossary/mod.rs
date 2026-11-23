//! N17: the glossary. A term is defined once, in one post of the user's ([`Definition`],
//! `extras.defines`: the term and what it means); every mention of it in any post shows the
//! definition on hover. The [`Glossary`] is the index of terms built from the posts; a locked
//! post never defines a term (its definition would speak past the lock).

mod index;
pub mod prompt;

pub use index::{Glossary, GlossaryEntry};

use jiff::Timestamp;
use serde::{Deserialize, Serialize};

/// The longest term, in characters and in words.
pub const MAX_TERM_CHARS: usize = 60;
pub const MAX_TERM_WORDS: usize = 4;
/// The longest definition, in characters.
pub const MAX_DEFINITION_CHARS: usize = 280;

/// The term a post defines, and what it means (`extras.defines`, boxed).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Definition {
    pub term: String,
    pub text: String,
    pub at: Timestamp,
}

/// Why a term could not be defined.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DefineProblem {
    /// The post is locked.
    Locked,
    /// Not the user's post, or not found.
    NotYours,
    NoTerm,
    NoDefinition,
    /// Another post defines the term already.
    Taken(crate::domain::PostId),
    Storage(String),
}

/// A word as the glossary compares it: lower case, without the punctuation around it.
pub fn word_key(word: &str) -> String {
    word.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase()
}

/// A term as the glossary compares it: its words' keys, one space apart.
pub fn term_key(term: &str) -> String {
    term.split_whitespace().map(word_key).filter(|word| !word.is_empty()).collect::<Vec<_>>().join(" ")
}

/// A term as kept: on one line, spaces collapsed, punctuation around it dropped; `None` when
/// empty, longer than [`MAX_TERM_CHARS`] or than [`MAX_TERM_WORDS`] words.
pub fn clean_term(raw: &str) -> Option<String> {
    let term = raw.split_whitespace().collect::<Vec<_>>().join(" ");
    let term = term.trim_matches(|c: char| !c.is_alphanumeric()).to_string();
    let words = term.split_whitespace().count();
    (!term_key(&term).is_empty() && term.chars().count() <= MAX_TERM_CHARS && words <= MAX_TERM_WORDS).then_some(term)
}

/// A definition as kept: lines trimmed, blank runs dropped; `None` when empty or too long.
pub fn clean_definition(raw: &str) -> Option<String> {
    let text = raw.lines().map(str::trim).filter(|line| !line.is_empty()).collect::<Vec<_>>().join("\n");
    let count = text.chars().count();
    (count > 0 && count <= MAX_DEFINITION_CHARS).then_some(text)
}

#[cfg(test)]
mod tests;
