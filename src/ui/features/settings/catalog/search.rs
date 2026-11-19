//! Settings search (pure): every whitespace-separated term must appear, case-insensitively,
//! in a row's title, description, keywords or its category's title / keywords; matches in
//! the title and description are returned as byte ranges to highlight.

use super::rows::SettingRow;
use crate::ui::features::settings::tabs::SettingsTab;
use std::ops::Range;

/// The translated, searchable texts of one row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchEntry {
    pub tab: SettingsTab,
    pub row: SettingRow,
    pub title: String,
    pub description: String,
    pub keywords: String,
    /// The category's title (a row matches its category's name too).
    pub context: String,
}

/// A row that matched, with the ranges to highlight.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchHit {
    pub tab: SettingsTab,
    pub row: SettingRow,
    pub title_ranges: Vec<Range<usize>>,
    pub description_ranges: Vec<Range<usize>>,
}

/// One character, case-folded.
fn fold(c: char) -> char {
    c.to_lowercase().next().unwrap_or(c)
}

/// The lowercase terms of `query` (empty for a blank query).
pub fn query_terms(query: &str) -> Vec<String> {
    query.split_whitespace().map(|term| term.chars().map(fold).collect()).collect()
}

/// Byte ranges of every non-overlapping, case-insensitive occurrence of `term` in `text`.
pub fn find_ranges(text: &str, term: &str) -> Vec<Range<usize>> {
    let needle: Vec<char> = term.chars().map(fold).collect();
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    if needle.is_empty() || needle.len() > chars.len() {
        return Vec::new();
    }
    let matches_at = |start: usize| chars[start..start + needle.len()].iter().zip(&needle).all(|((_, c), n)| fold(*c) == *n);
    let end_of = |index: usize| chars.get(index).map_or(text.len(), |(byte, _)| *byte);
    (0..=chars.len() - needle.len()).filter(|start| matches_at(*start)).fold(Vec::new(), |mut found: Vec<Range<usize>>, start| {
        let range = chars[start].0..end_of(start + needle.len());
        if found.last().is_none_or(|last| last.end <= range.start) {
            found.push(range);
        }
        found
    })
}

/// `ranges` sorted with overlapping or touching ranges merged.
pub fn merge_ranges(mut ranges: Vec<Range<usize>>) -> Vec<Range<usize>> {
    ranges.sort_by_key(|range| range.start);
    ranges.into_iter().fold(Vec::new(), |mut merged: Vec<Range<usize>>, range| {
        match merged.last_mut() {
            Some(last) if range.start <= last.end => last.end = last.end.max(range.end),
            _ => merged.push(range),
        }
        merged
    })
}

/// Highlight ranges of every term in `text`.
fn highlights(text: &str, terms: &[String]) -> Vec<Range<usize>> {
    merge_ranges(terms.iter().flat_map(|term| find_ranges(text, term)).collect())
}

/// The hit for `entry`, when every term appears in one of its texts.
pub fn match_entry(entry: &SearchEntry, terms: &[String]) -> Option<SearchHit> {
    let texts = [&entry.title, &entry.description, &entry.keywords, &entry.context];
    let all_found = terms.iter().all(|term| texts.iter().any(|text| !find_ranges(text, term).is_empty()));
    (!terms.is_empty() && all_found).then(|| SearchHit {
        tab: entry.tab,
        row: entry.row,
        title_ranges: highlights(&entry.title, terms),
        description_ranges: highlights(&entry.description, terms),
    })
}

/// Every entry matching `query`, in window order (empty for a blank query).
pub fn search(query: &str, entries: &[SearchEntry]) -> Vec<SearchHit> {
    let terms = query_terms(query);
    entries.iter().filter_map(|entry| match_entry(entry, &terms)).collect()
}

/// How many of `hits` belong to `tab`.
pub fn hits_in(hits: &[SearchHit], tab: SettingsTab) -> usize {
    hits.iter().filter(|hit| hit.tab == tab).count()
}
