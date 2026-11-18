//! Concept names as comparable keys, the concepts an author marked in a text (wikilinks and
//! hashtags) or that the concept extractor finds, and whether a text mentions a concept.

use crate::domain::post::{HASHTAG_SIGIL, WIKILINK_CLOSE, WIKILINK_OPEN};
use crate::domain::post_actions::extract_concepts;

/// Characters read as spaces when comparing (hashtag word joints and punctuation).
const JOINTS: [char; 2] = ['_', '-'];
/// Concepts the extractor offers per text that count towards a concept's mentions.
pub const EXTRACTED_PER_TEXT: usize = 3;

/// The comparison key of a concept: lower case, joints as spaces, single spaces.
pub fn concept_key(name: &str) -> String {
    let spaced: String = name.trim().trim_start_matches(HASHTAG_SIGIL).chars().map(|c| if JOINTS.contains(&c) { ' ' } else { c }).collect();
    spaced.split_whitespace().collect::<Vec<_>>().join(" ").to_lowercase()
}

/// `text` as space-separated lower-case words (links, tags and punctuation removed).
fn words_of(text: &str) -> String {
    let plain = text.replace(WIKILINK_OPEN, " ").replace(WIKILINK_CLOSE, " ");
    let spaced: String = plain.chars().map(|c| if c.is_alphanumeric() { c } else { ' ' }).collect();
    spaced.split_whitespace().collect::<Vec<_>>().join(" ").to_lowercase()
}

/// `text` prepared for many [`appears_in`] checks: its words between single spaces.
pub fn searchable(text: &str) -> String {
    format!(" {} ", words_of(text))
}

/// Whether the concept `key` appears as whole words in a [`searchable`] text.
pub fn appears_in(searchable: &str, key: &str) -> bool {
    let needle = words_of(key);
    !needle.is_empty() && searchable.contains(&format!(" {needle} "))
}

/// Whether `text` mentions the concept whose key is `key` (as whole words).
pub fn mentions(text: &str, key: &str) -> bool {
    appears_in(&searchable(text), key)
}

/// Wikilinked titles and hashtags of `text`: the concepts its author marked.
pub fn marked_concepts(text: &str) -> Vec<String> {
    let links = text.split(WIKILINK_OPEN).skip(1).filter_map(|part| part.find(WIKILINK_CLOSE).map(|end| part[..end].to_string()));
    let tags = text.split_whitespace().filter_map(|word| word.strip_prefix(HASHTAG_SIGIL)).map(|tag| tag.trim_matches(|c: char| !c.is_alphanumeric()).to_string());
    dedup(links.chain(tags).map(|name| concept_key(&name)).filter(|key| !key.is_empty()))
}

/// The concepts `text` refers to: its marked concepts, then the extractor's first few.
pub fn concepts_of(text: &str) -> Vec<String> {
    let extracted = extract_concepts(text, |_| false).into_iter().take(EXTRACTED_PER_TEXT).map(|name| concept_key(&name));
    dedup(marked_concepts(text).into_iter().chain(extracted).filter(|key| !key.is_empty()))
}

/// `keys` without repeats, first occurrence kept.
pub fn dedup(keys: impl Iterator<Item = String>) -> Vec<String> {
    keys.fold(Vec::new(), |mut seen, key| {
        if !seen.contains(&key) {
            seen.push(key);
        }
        seen
    })
}
