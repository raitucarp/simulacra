//! Search within the journal: every word of the query must appear in an entry (any case,
//! any order). Only the Journal page uses it; journal text never reaches the stream's search.

use super::entry::{newest_first, JournalEntry};

/// Markdown heading and list marks at line starts.
const LINE_MARKS: &[char] = &['#', '-', '*', '>', ' '];

/// The text without its line-start marks, lines joined by spaces.
fn plain_text(body: &str) -> String {
    body.lines().map(|line| line.trim_start_matches(LINE_MARKS)).filter(|line| !line.is_empty()).collect::<Vec<_>>().join(" ")
}

/// The words of `query`, lowercased.
fn terms(query: &str) -> Vec<String> {
    query.split_whitespace().map(str::to_lowercase).collect()
}

/// Whether `entry` contains every term.
fn matches(entry: &JournalEntry, terms: &[String]) -> bool {
    let body = entry.body.to_lowercase();
    terms.iter().all(|term| body.contains(term.as_str()))
}

/// The entries matching `query`, newest first (all of them for an empty query).
pub fn search_entries<'a>(entries: &'a [JournalEntry], query: &str) -> Vec<&'a JournalEntry> {
    let terms = terms(query);
    newest_first(entries).into_iter().filter(|entry| matches(entry, &terms)).collect()
}

/// A short excerpt of `entry` around the first term, for a result row.
pub fn excerpt_around(entry: &JournalEntry, query: &str, radius: usize) -> String {
    let plain = plain_text(&entry.body);
    let body: Vec<char> = plain.chars().collect();
    let lower: String = plain.to_lowercase();
    let first = terms(query).first().and_then(|term| lower.find(term.as_str())).map(|byte| lower[..byte].chars().count()).unwrap_or(0);
    let (start, end) = (first.saturating_sub(radius), (first + radius).min(body.len()));
    let text: String = body[start..end].iter().collect::<String>().split_whitespace().collect::<Vec<_>>().join(" ");
    let (lead, tail) = (if start > 0 { "…" } else { "" }, if end < body.len() { "…" } else { "" });
    format!("{lead}{text}{tail}")
}
