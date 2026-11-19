//! The atomic-note check of one text: its claims, its findings and a 0–100 score
//! (100 minus the findings' penalties, see [`super::findings`]).
//!
//! Length bands: under [`SHORT_CHARS`] characters a note rarely carries a claim with enough
//! context to be read alone; over [`LONG_CHARS`] (about two posts) it rarely holds a single
//! idea. A missing link is only flagged from [`LINK_MIN_CHARS`] on, so a quick one-liner is
//! not nagged.

use super::claims::claims_of;
use super::context::{dangling_pronoun, has_link, has_source, sounds_empirical};
use super::findings::{score_of, Finding};
use super::topics::topics_of;
use crate::import::segment::SentenceLocale;

/// Score from which a single-claim note counts as atomic.
pub const ATOMIC_SCORE: u8 = 80;
pub const SHORT_CHARS: usize = 40;
pub const LONG_CHARS: usize = 560;
pub const LINK_MIN_CHARS: usize = 80;
/// Distinct topics from which a note is flagged.
pub const MANY_TOPICS: usize = 2;

/// What the check says about one text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtomicReport {
    /// 0–100; higher is more atomic.
    pub score: u8,
    pub claims: Vec<String>,
    /// Characters of the trimmed text.
    pub chars: usize,
    /// Every finding, most important first.
    pub findings: Vec<Finding>,
}

impl AtomicReport {
    /// One claim at most, and a score of at least [`ATOMIC_SCORE`].
    pub fn is_atomic(&self) -> bool {
        self.claims.len() <= 1 && self.score >= ATOMIC_SCORE
    }
}

/// Findings about the claims and topics.
fn structure_findings(text: &str, claims: &[String]) -> Vec<Finding> {
    let topics = topics_of(text, claims);
    let many_claims = (claims.len() > 1).then(|| Finding::MultipleClaims { count: claims.len() });
    let many_topics = (topics.len() >= MANY_TOPICS).then_some(Finding::MultipleTopics { topics });
    many_claims.into_iter().chain(many_topics).collect()
}

/// Findings about length and context.
fn context_findings(text: &str, chars: usize) -> Vec<Finding> {
    let length = match chars {
        0 => None,
        n if n < SHORT_CHARS => Some(Finding::TooShort { chars: n }),
        n if n > LONG_CHARS => Some(Finding::TooLong { chars: n }),
        _ => None,
    };
    let pronoun = dangling_pronoun(text).map(|word| Finding::DanglingPronoun { word });
    let no_link = (chars >= LINK_MIN_CHARS && !has_link(text)).then_some(Finding::NoLink);
    let no_source = (sounds_empirical(text) && !has_source(text)).then_some(Finding::NoSource);
    [length, pronoun, no_link, no_source].into_iter().flatten().collect()
}

/// Checks `text` (a draft, a post or a vault note) split by the rules of `locale`.
pub fn check_atomicity(text: &str, locale: SentenceLocale) -> AtomicReport {
    let text = text.trim();
    let chars = text.chars().count();
    let claims = claims_of(text, locale);
    let findings: Vec<Finding> = structure_findings(text, &claims).into_iter().chain(context_findings(text, chars)).collect();
    AtomicReport { score: score_of(&findings), claims, chars, findings }
}
