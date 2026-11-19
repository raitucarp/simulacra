//! One exchange of a dialogue: the tutor's question (its level and move), the learner's
//! answer, and the assessment of that answer (level reached, verdict, what was right, the
//! misconceptions).

use super::moves::SocraticMove;
use crate::domain::document::keyed::keyed_enum;
use crate::domain::learning::bloom::BloomLevel;
use serde::{Deserialize, Serialize};

keyed_enum! {
    /// How well an answer met the question's level.
    Verdict {
        /// Met it: the tutor climbs.
        Solid => "solid",
        /// Partly: the tutor stays and probes.
        Partial => "partial",
        /// Not yet: the tutor probes, then steps down.
        Missing => "missing",
    }
}

/// One question of the tutor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TutorQuestion {
    pub text: String,
    /// The level it aims at.
    pub level: BloomLevel,
    #[serde(rename = "move")]
    pub kind: SocraticMove,
}

/// The tutor's reading of one answer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Assessment {
    /// The highest level the answer showed, if any.
    pub reached: Option<BloomLevel>,
    pub verdict: Verdict,
    /// What was right.
    #[serde(default)]
    pub right: Vec<String>,
    #[serde(default)]
    pub misconceptions: Vec<String>,
}

impl Assessment {
    /// The reading of a reply that assessed nothing: a partial answer, nothing named.
    pub fn unclear() -> Self {
        Self { reached: None, verdict: Verdict::Partial, right: Vec::new(), misconceptions: Vec::new() }
    }
}

/// A question, the learner's answer and its assessment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exchange {
    pub question: TutorQuestion,
    #[serde(default)]
    pub answer: Option<String>,
    #[serde(default)]
    pub assessment: Option<Assessment>,
}
