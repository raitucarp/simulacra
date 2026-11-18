//! Classifying candidate pairs with the AI, a small batch per request: the prompt, the
//! verdict vocabulary and a lenient reader of the reply. Results are cached by pair key.

mod parse;
mod prompt;

pub use parse::{parse_classifications, MAX_REASON_CHARS};
pub use prompt::{classification_prompt, MAX_SIDE_CHARS};

use crate::domain::document::keyed::keyed_enum;

/// Pairs sent in one request.
pub const BATCH_SIZE: usize = 4;
/// The answer budget (output tokens) of one batch.
pub const CLASSIFY_ANSWER_TOKENS: u32 = 600;

keyed_enum! {
    /// What the model said about a pair; the key is stored with the cached verdict.
    Verdict {
        /// Both texts cannot be true as stated.
        Contradicts => "contradicts",
        /// Both can be true together.
        Compatible => "compatible",
        /// They are about different things.
        Unrelated => "unrelated",
    }
}

/// Words read as each verdict (lower case, surrounding punctuation stripped).
const CONTRADICTS_WORDS: &[&str] = &[
    "contradicts", "contradict", "contradiction", "contradictory", "conflict", "conflicts", "conflicting", "inconsistent",
    "incompatible",
];
const COMPATIBLE_WORDS: &[&str] = &["compatible", "consistent", "agree", "agrees", "complementary"];
const UNRELATED_WORDS: &[&str] = &["unrelated", "irrelevant", "none", "neither"];

impl Verdict {
    /// The verdict a model's word names (case and surrounding punctuation ignored).
    pub fn read(word: &str) -> Option<Self> {
        let word = word.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
        let named = |words: &[&str]| words.contains(&word.as_str());
        [(CONTRADICTS_WORDS, Self::Contradicts), (COMPATIBLE_WORDS, Self::Compatible), (UNRELATED_WORDS, Self::Unrelated)]
            .into_iter()
            .find_map(|(words, verdict)| named(words).then_some(verdict))
    }
}

/// The verdict on one pair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Classification {
    /// The pair's key ([`super::pair_key`]).
    pub pair: String,
    pub verdict: Verdict,
    /// One sentence on why.
    pub reason: String,
}
