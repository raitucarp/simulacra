//! N20: bilingual posts. A post can keep its text in a language the user is learning
//! ([`Translation`], `extras.translation`: the language's locale code, the text, when); the
//! card shows it in place of the original until toggled back, and words of it can be sent to
//! review. The translation comes from the AI engine; a locked post is never translated.

pub mod prompt;

use jiff::Timestamp;
use serde::{Deserialize, Serialize};

/// The longest translation kept, in characters.
pub const MAX_TRANSLATION_CHARS: usize = 2000;
/// The most words offered for review from one translation.
pub const MAX_STUDY_WORDS: usize = 30;

/// A post's text in another language (`extras.translation`, boxed).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Translation {
    /// The locale code of its language (`"es"`, `"zh-CN"`).
    pub language: String,
    pub text: String,
    pub at: Timestamp,
}

/// The distinct words of `text` worth studying, in order: letters only, two or more of them
/// (one for scripts written without spaces), case folded for the comparison.
pub fn study_words(text: &str) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    for word in text.split(|c: char| !c.is_alphabetic() && c != '\'' && c != '-').map(|word| word.trim_matches(|c| c == '\'' || c == '-')) {
        let wide = word.chars().any(|c| c as u32 >= 0x2E80);
        if word.chars().count() < if wide { 1 } else { 2 } || words.iter().any(|seen| seen.to_lowercase() == word.to_lowercase()) {
            continue;
        }
        words.push(word.to_string());
        if words.len() == MAX_STUDY_WORDS {
            break;
        }
    }
    words
}

#[cfg(test)]
mod tests;
