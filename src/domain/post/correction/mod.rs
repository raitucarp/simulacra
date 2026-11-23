//! N14: retractions and corrections. The author can mark a post retracted (it no longer
//! stands) or corrected (it was wrong in part, and says how), always with a reason
//! ([`Correction`], `extras.correction`). The card wears a banner; every agent reads the mark
//! with the post; the agents that cited it are told and may answer, and are remembered so
//! they are told once. A locked post cannot be marked: its reason would speak past the lock.

use crate::domain::ProfileId;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

/// The longest reason, in characters.
pub const MAX_REASON_CHARS: usize = 280;

/// What the author says of the post now.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CorrectionKind {
    /// It no longer stands.
    #[default]
    Retracted,
    /// It was wrong in part; the reason says what is right.
    Corrected,
}

impl CorrectionKind {
    /// Stable key (locale keys `posts.correction.<key>…`, element ids).
    pub const fn key(self) -> &'static str {
        match self {
            Self::Retracted => "retracted",
            Self::Corrected => "corrected",
        }
    }
}

/// A post's retraction or correction (`extras.correction`, boxed).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Correction {
    pub kind: CorrectionKind,
    pub reason: String,
    pub at: Timestamp,
    /// The agents told about it (so none is told twice).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub told: Vec<ProfileId>,
}

impl Correction {
    /// How an agent reads the mark with the post.
    pub fn agent_note(&self) -> String {
        match self.kind {
            CorrectionKind::Retracted => format!("[Retracted by its author: {}]", self.reason),
            CorrectionKind::Corrected => format!("[Corrected by its author: {}]", self.reason),
        }
    }
}

/// A reason as kept: lines trimmed, blank runs dropped; `None` when empty or too long.
pub fn clean_reason(text: &str) -> Option<String> {
    let reason = text.lines().map(str::trim).filter(|line| !line.is_empty()).collect::<Vec<_>>().join("\n");
    let count = reason.chars().count();
    (count > 0 && count <= MAX_REASON_CHARS).then_some(reason)
}

#[cfg(test)]
mod tests;
