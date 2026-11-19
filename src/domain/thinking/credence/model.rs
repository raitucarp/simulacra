//! A credence: the chance (1–99 %) the author gives a post's main claim, or one sentence of
//! it, with an optional date by which it resolves, and later its outcome.

use crate::domain::document::keyed::keyed_enum;
use crate::domain::PostId;
use jiff::Timestamp;
use jiff::civil::Date;
use serde::{Deserialize, Serialize};

keyed_enum! {
    /// How a prediction turned out; the key is stored in SQLite and names its locale string.
    Outcome {
        /// The claim held.
        True => "true",
        /// The claim failed.
        False => "false",
        /// It could not be decided; closes the prediction without scoring it.
        Ambiguous => "ambiguous",
    }
}

/// Lowest and highest stated chance: certainty (0 or 100 %) is never a credence.
pub const MIN_PERCENT: u8 = 1;
pub const MAX_PERCENT: u8 = 99;
/// Percent per unit of probability.
const PERCENT_PER_UNIT: f64 = 100.0;

/// `value` clamped into [`MIN_PERCENT`]..=[`MAX_PERCENT`].
pub fn clamp_percent(value: i32) -> u8 {
    value.clamp(i32::from(MIN_PERCENT), i32::from(MAX_PERCENT)) as u8
}

/// A stated probability on one post.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Credence {
    pub post_id: PostId,
    /// Chance (1–99) that the main claim, or `sentence`, is true.
    pub percent: u8,
    /// The sentence the credence is about (`None`: the post's main claim).
    pub sentence: Option<String>,
    pub resolves_by: Option<Date>,
    /// What the author wrote when resolving (or empty).
    pub note: String,
    /// `None` while the prediction is open.
    pub outcome: Option<Outcome>,
    pub created_at: Timestamp,
    pub resolved_at: Option<Timestamp>,
}

impl Credence {
    /// An open credence of `percent` (clamped) on the main claim of `post_id`.
    pub fn new(post_id: PostId, percent: u8, now: Timestamp) -> Self {
        Self { post_id, percent: clamp_percent(i32::from(percent)), sentence: None, resolves_by: None, note: String::new(), outcome: None, created_at: now, resolved_at: None }
    }

    /// The stated chance as a probability in (0, 1).
    pub fn probability(&self) -> f64 {
        f64::from(self.percent) / PERCENT_PER_UNIT
    }

    pub fn is_open(&self) -> bool {
        self.outcome.is_none()
    }

    /// Open and its resolution date has come.
    pub fn is_due(&self, today: Date) -> bool {
        self.is_open() && self.resolves_by.is_some_and(|due| due <= today)
    }

    pub fn with_percent(self, percent: u8) -> Self {
        Self { percent: clamp_percent(i32::from(percent)), ..self }
    }

    pub fn with_due(self, due: Option<Date>) -> Self {
        Self { resolves_by: due, ..self }
    }

    /// About `sentence` (a blank one means the main claim).
    pub fn with_sentence(self, sentence: Option<String>) -> Self {
        let sentence = sentence.map(|text| text.trim().to_string()).filter(|text| !text.is_empty());
        Self { sentence, ..self }
    }

    /// Closed as `outcome` with `note` (trimmed) at `now`.
    pub fn resolved(self, outcome: Outcome, note: &str, now: Timestamp) -> Self {
        Self { outcome: Some(outcome), note: note.trim().to_string(), resolved_at: Some(now), ..self }
    }

    /// Open again (the note is kept so it can be edited).
    pub fn reopened(self) -> Self {
        Self { outcome: None, resolved_at: None, ..self }
    }
}
