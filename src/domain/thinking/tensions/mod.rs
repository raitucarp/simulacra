//! Tensions (sb E1): pairs of posts or vault passages that may contradict each other.
//!
//! - [`side`]: one side of a pair (a post or a passage of a vault document).
//! - [`terms`] / [`lexicon`]: content words and the polarity vocabulary (negations, antonym
//!   pairs, opposite quantifiers) for English, Indonesian, German, French, Spanish and Portuguese.
//! - [`lexical`]: the polarity signals between two texts.
//! - [`candidates`]: which pairs are worth a look (same topic by meaning, or by shared words).
//! - [`rank`]: the documented score that orders them.
//! - [`pair`]: the content hash a classified pair is cached by.
//! - [`classify`]: the AI prompt for a small batch and the lenient reader of its verdicts.
//! - [`status`]: what the user did with a pair (open, resolved, dismissed).
//! - [`record`]: what is remembered about a pair (its verdict, reason and status).
//!
//! Without embeddings or an AI key the finder still works: candidates come from shared words
//! and are ranked by the lexical signals alone; classification is simply skipped.

pub mod candidates;
pub mod classify;
pub mod lexical;
pub mod lexicon;
pub mod pair;
pub mod rank;
pub mod record;
pub mod side;
pub mod status;
pub mod terms;

#[cfg(test)]
mod tests;

pub use candidates::{CandidateInput, MIN_SHARED_TERMS, NEIGHBOURS};
pub use classify::{
    classification_prompt, parse_classifications, Classification, Verdict, BATCH_SIZE, CLASSIFY_ANSWER_TOKENS, MAX_REASON_CHARS,
    MAX_SIDE_CHARS,
};
pub use lexical::{lexical_signals, LexicalSignals};
pub use pair::pair_key;
pub use record::TensionRecord;
pub use rank::{candidate_score, rank_candidates, Candidate, RankOptions, DEFAULT_LIMIT, MIN_SIMILARITY, STRONG_SIMILARITY};
pub use side::{SideKind, TensionSide};
pub use status::TensionStatus;
