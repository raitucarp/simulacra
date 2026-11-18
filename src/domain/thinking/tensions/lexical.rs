//! Lexical signals of opposition between two texts: one side negates where the other does
//! not, the sides use the two poles of an antonym pair, or of an opposite quantifier. Shared
//! content terms are reported too, but they describe the topic, not the opposition.

use super::lexicon::{ANTONYMS, NEGATIONS, NEGATION_SUFFIX, QUANTIFIERS};
use super::terms::{content_terms, singular, words};

/// What a negation mismatch adds to the strength.
const NEGATION_WEIGHT: f32 = 0.4;
/// What each antonym pair adds.
const ANTONYM_WEIGHT: f32 = 0.35;
/// What each opposite quantifier adds.
const QUANTIFIER_WEIGHT: f32 = 0.35;
/// The strength is capped here.
const MAX_STRENGTH: f32 = 1.0;
const EVEN: usize = 2;

/// The opposition two texts show on the surface.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LexicalSignals {
    /// The sides share a topic and exactly one of them is negated (negation counts differ in
    /// parity, so "not untrue" reads as affirmed).
    pub negation: bool,
    /// Antonym pairs, the left text's word first.
    pub antonyms: Vec<(String, String)>,
    /// Opposite quantifiers or modalities, the left text's pole first.
    pub quantifiers: Vec<(String, String)>,
    /// Content terms both texts use, sorted.
    pub shared_terms: Vec<String>,
}

impl LexicalSignals {
    /// How strongly the surface suggests opposition, in `[0, 1]`: 0.4 for a negation
    /// mismatch plus 0.35 per antonym pair and per opposite quantifier, capped at 1.
    pub fn strength(&self) -> f32 {
        let negation = if self.negation { NEGATION_WEIGHT } else { 0.0 };
        let antonyms = self.antonyms.len() as f32 * ANTONYM_WEIGHT;
        let quantifiers = self.quantifiers.len() as f32 * QUANTIFIER_WEIGHT;
        (negation + antonyms + quantifiers).min(MAX_STRENGTH)
    }

    /// Whether no opposition was seen (shared terms alone are a topic, not a signal).
    pub fn is_empty(&self) -> bool {
        !self.negation && self.antonyms.is_empty() && self.quantifiers.is_empty()
    }
}

/// The signals between `left` and `right`.
pub fn lexical_signals(left: &str, right: &str) -> LexicalSignals {
    let (left_words, right_words) = (words(left), words(right));
    let shared_terms: Vec<String> = content_terms(left).intersection(&content_terms(right)).cloned().collect();
    let negation = !shared_terms.is_empty() && negations(&left_words) % EVEN != negations(&right_words) % EVEN;
    let stem = |words: &[String]| words.iter().map(|word| singular(word)).collect::<Vec<_>>();
    LexicalSignals {
        negation,
        antonyms: opposed(&stem(&left_words), &stem(&right_words), ANTONYMS),
        quantifiers: opposed(&left_words, &right_words, QUANTIFIERS),
        shared_terms,
    }
}

/// How many negating words `words` holds.
fn negations(words: &[String]) -> usize {
    words.iter().filter(|word| NEGATIONS.contains(&word.as_str()) || word.ends_with(NEGATION_SUFFIX)).count()
}

/// The pairs whose first pole one side takes and whose second pole the other takes.
fn opposed(left: &[String], right: &[String], pairs: &[(&str, &str)]) -> Vec<(String, String)> {
    pairs
        .iter()
        .filter_map(|(a, b)| match (pole(left, a, b), pole(right, a, b)) {
            (Some(true), Some(false)) => Some((a.to_string(), b.to_string())),
            (Some(false), Some(true)) => Some((b.to_string(), a.to_string())),
            _ => None,
        })
        .collect()
}

/// `Some(true)` when `words` take pole `a` only, `Some(false)` for `b` only, else `None`.
/// A pole nested in the other ("must" in "must not") only counts outside of it.
fn pole(words: &[String], a: &str, b: &str) -> Option<bool> {
    let (a_words, b_words): (Vec<&str>, Vec<&str>) = (a.split_whitespace().collect(), b.split_whitespace().collect());
    let (a_count, b_count) = (occurrences(words, &a_words), occurrences(words, &b_words));
    let a_only = if contains(&b_words, &a_words) { a_count.saturating_sub(b_count) } else { a_count };
    let b_only = if contains(&a_words, &b_words) { b_count.saturating_sub(a_count) } else { b_count };
    match (a_only > 0, b_only > 0) {
        (true, false) => Some(true),
        (false, true) => Some(false),
        _ => None,
    }
}

/// How often the word sequence `phrase` occurs in `words`.
fn occurrences(words: &[String], phrase: &[&str]) -> usize {
    if phrase.is_empty() || phrase.len() > words.len() {
        return 0;
    }
    words.windows(phrase.len()).filter(|window| window.iter().zip(phrase).all(|(w, p)| w == p)).count()
}

/// Whether the word sequence `outer` contains `inner`.
fn contains(outer: &[&str], inner: &[&str]) -> bool {
    !inner.is_empty() && inner.len() <= outer.len() && outer.windows(inner.len()).any(|window| window == inner)
}
