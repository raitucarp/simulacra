//! The search query language: free text plus filters.
//!
//! ```text
//! sourdough "starter recipe" from:sam kind:note has:image is:open -burnt -from:bot
//! symbol:task tint:amber before:2026-12-01 after:last-week
//! ```
//!
//! - Words are separated by whitespace; `"..."` makes a phrase, and a quoted value
//!   (`from:"Sam Lee"`) may contain spaces. An unclosed quote runs to the end.
//! - `from:` `kind:` `symbol:` `tint:` take any value (`from:@sam` drops the `@`).
//! - `has:` takes `image|file|code|todo`; `is:` takes `open|done|scheduled|archived|locked`.
//! - `before:` / `after:` take a date: `YYYY-MM-DD` is read as a day, anything else is kept
//!   raw for later resolution.
//! - A leading `-` negates a word, a phrase or a filter. A lone `-` is a word.
//! - Keys and `has:`/`is:` values ignore case. An unknown key, an unknown `has:`/`is:` value
//!   or an empty value leaves the whole `key:value` as free text.

mod lex;
mod parse;
mod write; // Q4: back to text for filter chips
pub mod saved; // Q5: saved searches
#[cfg(test)]
mod tests;

pub use parse::parse_query;
pub use saved::{SavedSearch, SavedSearches};

use jiff::civil::Date;

/// What a post must contain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HasFilter {
    Image,
    File,
    Code,
    Todo,
}

/// A state the post is in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IsFilter {
    Open,
    Done,
    Scheduled,
    Archived,
    Locked,
}

/// A `before:` / `after:` bound.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryDate {
    /// A `YYYY-MM-DD` day.
    Day(Date),
    /// Anything else, as typed (`yesterday`, `last-week`, `2026-13-40`).
    Raw(String),
}

/// One `key:value` filter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryFilter {
    From(String),
    Kind(String),
    Symbol(String),
    Tint(String),
    Has(HasFilter),
    Is(IsFilter),
    Before(QueryDate),
    After(QueryDate),
}

/// A parsed search query. Negated parts go to the `excluded_*` lists.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PostQuery {
    pub words: Vec<String>,
    pub phrases: Vec<String>,
    pub filters: Vec<QueryFilter>,
    pub excluded_words: Vec<String>,
    pub excluded_phrases: Vec<String>,
    pub excluded_filters: Vec<QueryFilter>,
}

impl PostQuery {
    /// Parses `input` (see the module rules).
    pub fn parse(input: &str) -> Self {
        parse_query(input)
    }

    /// Whether the query asks for nothing (blank input).
    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }

    /// The free text to match: words and phrases, in that order, joined by spaces.
    pub fn text(&self) -> String {
        let parts: Vec<&str> = self.words.iter().chain(&self.phrases).map(String::as_str).collect();
        parts.join(" ")
    }
}
