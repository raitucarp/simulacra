//! The alphabetical subject index: every term under its initial letter, with how many
//! documents carry it.

use super::Subject;
use crate::domain::document::ids::SubjectId;
use std::collections::HashMap;

/// Heading of terms that do not start with a letter.
pub const NON_LETTER_HEADING: char = '#';

/// One term of the index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexEntry {
    pub id: SubjectId,
    pub label: String,
    /// Label of the broader term, for "see under" context.
    pub broader: Option<String>,
    pub count: usize,
}

/// The terms under one initial letter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexLetter {
    pub letter: char,
    pub entries: Vec<IndexEntry>,
}

/// Upper-cased initial letter of `label` (`#` for digits and symbols).
pub fn index_letter(label: &str) -> char {
    label
        .trim()
        .chars()
        .next()
        .filter(|c| c.is_alphabetic())
        .and_then(|c| c.to_uppercase().next())
        .unwrap_or(NON_LETTER_HEADING)
}

/// Groups `subjects` by initial letter (letters in order, `#` last), terms sorted by label.
pub fn subject_index(subjects: &[Subject], counts: &HashMap<SubjectId, usize>) -> Vec<IndexLetter> {
    let label_of = |id: &SubjectId| subjects.iter().find(|s| &s.id == id).map(|s| s.label.clone());
    let mut entries: Vec<IndexEntry> = subjects
        .iter()
        .map(|s| IndexEntry {
            id: s.id.clone(),
            label: s.label.clone(),
            broader: s.broader.as_ref().and_then(label_of),
            count: counts.get(&s.id).copied().unwrap_or_default(),
        })
        .collect();
    entries.sort_by_key(|entry| (index_letter(&entry.label) == NON_LETTER_HEADING, entry.label.to_lowercase()));
    entries.into_iter().fold(Vec::new(), |mut letters: Vec<IndexLetter>, entry| {
        let letter = index_letter(&entry.label);
        match letters.last_mut() {
            Some(group) if group.letter == letter => group.entries.push(entry),
            _ => letters.push(IndexLetter { letter, entries: vec![entry] }),
        }
        letters
    })
}
