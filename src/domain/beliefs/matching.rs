//! Whether two propositions say the same thing, by word overlap: both are reduced to their
//! lower-case words (each ideograph and kana counts as a word, since those scripts write no
//! spaces) and compared with the Jaccard index over the word sets.

use std::collections::BTreeSet;

/// Overlap at which two propositions count as the same belief.
pub const SAME_BELIEF_OVERLAP: f32 = 0.6;
/// Words shorter than this (in characters) carry little meaning and are ignored, unless
/// the script writes whole words in one character.
const MIN_WORD_CHARS: usize = 3;

/// Whether `ch` belongs to a script without spaces between words.
fn is_unspaced_script(ch: char) -> bool {
    matches!(ch as u32, 0x3040..=0x30FF | 0x3400..=0x4DBF | 0x4E00..=0x9FFF | 0xAC00..=0xD7AF)
}

/// The meaningful lower-case words of `text`.
pub fn words(text: &str) -> BTreeSet<String> {
    let lower = text.to_lowercase();
    let spaced = lower.split(|ch: char| !ch.is_alphanumeric() || is_unspaced_script(ch)).filter(|word| word.chars().count() >= MIN_WORD_CHARS).map(str::to_string);
    let unspaced = lower.chars().filter(|ch| is_unspaced_script(*ch)).map(String::from);
    spaced.chain(unspaced).collect()
}

/// The Jaccard index of the two word sets (0 when either is empty).
pub fn overlap(left: &str, right: &str) -> f32 {
    let (a, b) = (words(left), words(right));
    if a.is_empty() || b.is_empty() {
        return 0.0;
    }
    let shared = a.intersection(&b).count();
    let union = a.union(&b).count();
    shared as f32 / union as f32
}

/// Whether `left` and `right` state the same belief.
pub fn same_belief(left: &str, right: &str) -> bool {
    overlap(left, right) >= SAME_BELIEF_OVERLAP
}

/// `text` with runs of whitespace collapsed and the ends trimmed.
pub fn tidy(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}
