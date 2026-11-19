//! What the formulas count: sentences (the import sentence splitter, so abbreviations and CJK
//! terminators are handled the same way everywhere), words, syllables, letters, long words
//! and CJK characters.

use super::script::is_cjk;
use super::syllables::{syllables, SyllableRules};
use crate::domain::post::HASHTAG_SIGIL;
use crate::import::segment::split_sentences;

/// Letters a word needs to count as long (LIX: "more than six letters").
pub const LONG_WORD_LETTERS: usize = 6;
/// Marks that make a token a link rather than a word.
const LINK_MARK: &str = "://";
const MENTION_SIGIL: char = '@';

/// The counts of one text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TextCounts {
    pub sentences: usize,
    pub words: usize,
    pub syllables: usize,
    pub letters: usize,
    pub long_words: usize,
    pub cjk_chars: usize,
}

/// The words of `text`: whitespace-separated tokens without surrounding punctuation, links,
/// mentions and hashtags (tags label a post; they are not prose), that hold a letter.
pub fn words(text: &str) -> impl Iterator<Item = &str> {
    text.split_whitespace()
        .filter(|token| !token.contains(LINK_MARK) && !token.starts_with(MENTION_SIGIL) && !token.starts_with(HASHTAG_SIGIL))
        .map(|token| token.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(|word| word.chars().any(char::is_alphabetic))
}

impl TextCounts {
    /// Counts `text`, estimating syllables with `rules`.
    pub fn of(text: &str, rules: SyllableRules) -> Self {
        let sentences = split_sentences(text, rules.sentence_locale()).len();
        let letters_of = |word: &str| word.chars().filter(|c| c.is_alphabetic()).count();
        let (words, syllables, letters, long_words) = words(text).fold((0, 0, 0, 0), |(w, s, l, long), word| {
            let letters = letters_of(word);
            (w + 1, s + syllables(word, rules), l + letters, long + usize::from(letters > LONG_WORD_LETTERS))
        });
        let cjk_chars = text.chars().filter(|c| is_cjk(*c)).count();
        Self { sentences: sentences.max(usize::from(words > 0 || cjk_chars > 0)), words, syllables, letters, long_words, cjk_chars }
    }

    /// Mean words per sentence (ASL).
    pub fn words_per_sentence(&self) -> f64 {
        ratio(self.words, self.sentences)
    }

    /// Mean syllables per word (ASW).
    pub fn syllables_per_word(&self) -> f64 {
        ratio(self.syllables, self.words)
    }

    /// Mean letters per word.
    pub fn letters_per_word(&self) -> f64 {
        ratio(self.letters, self.words)
    }

    /// Share of long words (0–1).
    pub fn long_word_share(&self) -> f64 {
        ratio(self.long_words, self.words)
    }

    /// Mean CJK characters per sentence.
    pub fn cjk_chars_per_sentence(&self) -> f64 {
        ratio(self.cjk_chars, self.sentences)
    }
}

/// `part / whole`, zero for an empty whole.
fn ratio(part: usize, whole: usize) -> f64 {
    if whole == 0 { 0.0 } else { part as f64 / whole as f64 }
}
