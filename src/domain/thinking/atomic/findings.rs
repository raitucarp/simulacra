//! What the atomic-note check found, each finding with its locale key and the points it
//! takes off the score.

/// One reason a note is not atomic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Finding {
    /// The note makes `count` distinct claims (one is atomic).
    MultipleClaims { count: usize },
    /// Shorter than a claim with its context usually is.
    TooShort { chars: usize },
    /// Longer than one idea usually needs.
    TooLong { chars: usize },
    /// Concepts that never meet in one claim: separate subjects.
    MultipleTopics { topics: Vec<String> },
    /// No wikilink, URL or hashtag ties the note to anything.
    NoLink,
    /// A claim about evidence or numbers without a source.
    NoSource,
    /// The note opens with `word`, which points back at something it never says.
    DanglingPronoun { word: String },
}

/// Points off per claim beyond the first, and the most the claim count can take off.
const CLAIM_PENALTY: u8 = 15;
const MAX_CLAIM_PENALTY: u8 = 60;
const LENGTH_PENALTY: u8 = 15;
const TOPICS_PENALTY: u8 = 15;
const LINK_PENALTY: u8 = 10;
const SOURCE_PENALTY: u8 = 15;
const PRONOUN_PENALTY: u8 = 10;

impl Finding {
    /// Stable key (locale strings `…atomic.finding.<key>`).
    pub fn key(&self) -> &'static str {
        match self {
            Self::MultipleClaims { .. } => "multiple_claims",
            Self::TooShort { .. } => "too_short",
            Self::TooLong { .. } => "too_long",
            Self::MultipleTopics { .. } => "multiple_topics",
            Self::NoLink => "no_link",
            Self::NoSource => "no_source",
            Self::DanglingPronoun { .. } => "dangling_pronoun",
        }
    }

    /// Points this finding takes off the score; each extra claim costs more.
    pub fn penalty(&self) -> u8 {
        match self {
            Self::MultipleClaims { count } => {
                let extra = u8::try_from(count.saturating_sub(1)).unwrap_or(u8::MAX);
                extra.saturating_mul(CLAIM_PENALTY).min(MAX_CLAIM_PENALTY)
            }
            Self::TooShort { .. } | Self::TooLong { .. } => LENGTH_PENALTY,
            Self::MultipleTopics { .. } => TOPICS_PENALTY,
            Self::NoLink => LINK_PENALTY,
            Self::NoSource => SOURCE_PENALTY,
            Self::DanglingPronoun { .. } => PRONOUN_PENALTY,
        }
    }
}

/// The best score.
pub const FULL_SCORE: u8 = 100;

/// 100 minus every penalty, never below zero.
pub fn score_of(findings: &[Finding]) -> u8 {
    findings.iter().fold(FULL_SCORE, |score, finding| score.saturating_sub(finding.penalty()))
}
