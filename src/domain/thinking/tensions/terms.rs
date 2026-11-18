//! Words of a text as the tension finder compares them: lower case, apostrophes inside words
//! kept (so "don't" stays one negating word), and the content terms that mark a topic
//! (long enough, not a stopword, not a polarity word, a light plural trim).

use super::lexicon::{is_polarity_word, STOPWORDS};
use std::collections::BTreeSet;

/// Shortest content term, in characters.
pub const MIN_TERM_CHARS: usize = 4;
/// Apostrophes kept inside a word; the typographic one is read as the plain one.
const APOSTROPHES: [char; 2] = ['\'', '\u{2019}'];
const PLAIN_APOSTROPHE: &str = "'";
const TYPOGRAPHIC_APOSTROPHE: &str = "\u{2019}";
/// Possessive ending dropped from a term ("Spinoza's" is about "Spinoza").
const POSSESSIVE: &str = "'s";
/// Plural ending trimmed from longer words, unless doubled ("process").
const PLURAL_MARK: &str = "s";
const DOUBLE_S: &str = "ss";

/// The words of `text`, lower case, in order.
pub fn words(text: &str) -> Vec<String> {
    text.split(|c: char| !(c.is_alphanumeric() || APOSTROPHES.contains(&c)))
        .map(|word| word.trim_matches(|c: char| APOSTROPHES.contains(&c)).replace(TYPOGRAPHIC_APOSTROPHE, PLAIN_APOSTROPHE).to_lowercase())
        .filter(|word| !word.is_empty())
        .collect()
}

/// `word` without a plural "s" when it is longer than a content term's minimum.
pub fn singular(word: &str) -> String {
    let long = word.chars().count() > MIN_TERM_CHARS;
    match word.strip_suffix(PLURAL_MARK) {
        Some(stem) if long && !word.ends_with(DOUBLE_S) => stem.to_string(),
        _ => word.to_string(),
    }
}

/// Whether a word marks a topic.
fn is_content(word: &str) -> bool {
    word.chars().count() >= MIN_TERM_CHARS
        && !STOPWORDS.contains(&word)
        && !is_polarity_word(word)
        && !word.chars().all(|c| c.is_ascii_digit())
}

/// The content terms of `text`.
pub fn content_terms(text: &str) -> BTreeSet<String> {
    words(text)
        .into_iter()
        .map(|word| word.strip_suffix(POSSESSIVE).unwrap_or(&word).to_string())
        .filter(|word| is_content(word))
        .map(|word| singular(&word))
        .collect()
}

/// Terms shared by both sets over all terms of either (0 when both are empty).
pub fn jaccard(left: &BTreeSet<String>, right: &BTreeSet<String>) -> f32 {
    let shared = left.intersection(right).count();
    let all = left.union(right).count();
    if all == 0 { 0.0 } else { shared as f32 / all as f32 }
}
