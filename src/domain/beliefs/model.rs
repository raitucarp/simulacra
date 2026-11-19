//! What an agent believes: propositions with a credence, where each came from, and the log
//! of every change. All values are immutable; changes produce new values and a revision.

use crate::domain::document::keyed::keyed_enum;
use crate::domain::ProfileId;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

/// Percent points of the whole credence scale.
pub const CREDENCE_SCALE: u8 = 100;
/// Credence of a proposition the agent is unsure about.
pub const CREDENCE_UNSURE: u8 = 50;
/// The id of a belief (stable across revisions).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BeliefId(String);

impl BeliefId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// A fresh random id.
    pub fn random() -> Self {
        Self(uuid::Uuid::new_v4().simple().to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// How strongly a proposition is held, from 0 (surely false) to 1 (surely true), kept in
/// whole percent points so values compare exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Credence(u8);

impl Credence {
    /// `percent` clamped to 0–100.
    pub fn from_percent(percent: i32) -> Self {
        Self(percent.clamp(0, i32::from(CREDENCE_SCALE)) as u8)
    }

    /// A unit value (0–1, clamped), rounded to whole percent points.
    pub fn from_unit(unit: f64) -> Self {
        Self::from_percent((unit.clamp(0.0, 1.0) * f64::from(CREDENCE_SCALE)).round() as i32)
    }

    pub const fn percent(self) -> u8 {
        self.0
    }

    /// The credence on the 0–1 scale.
    pub fn unit(self) -> f32 {
        f32::from(self.0) / f32::from(CREDENCE_SCALE)
    }

    /// The credence moved by `delta` percent points (clamped).
    pub fn nudged(self, delta: i32) -> Self {
        Self::from_percent(i32::from(self.0) + delta)
    }

    /// How far from "unsure" (0 at 0.5, 1 at either end).
    pub fn conviction(self) -> f32 {
        (f32::from(self.0) - f32::from(CREDENCE_UNSURE)).abs() / f32::from(CREDENCE_UNSURE)
    }
}

keyed_enum! {
    /// What a belief was formed or revised from.
    BeliefOrigin {
        /// The agent's own post.
        Post => "post",
        /// A reply the agent wrote, having read the post it answers.
        Reply => "reply",
        /// A thread the agent read.
        Thread => "thread",
        /// A debate in the Arena.
        Debate => "debate",
        /// A thought experiment in the Arena's lab.
        Experiment => "experiment",
        /// The user, from the profile's Beliefs tab.
        User => "user",
    }
}

/// The source of a belief or a revision: its kind, the id it links to (a post, a debate or
/// an experiment; empty for the user) and a short label to show.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BeliefSource {
    pub origin: BeliefOrigin,
    pub id: String,
    pub label: String,
}

impl BeliefSource {
    pub fn new(origin: BeliefOrigin, id: impl Into<String>, label: impl Into<String>) -> Self {
        Self { origin, id: id.into(), label: label.into() }
    }

    /// The user's own edit.
    pub fn user() -> Self {
        Self::new(BeliefOrigin::User, String::new(), String::new())
    }
}

/// One proposition an agent holds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Belief {
    pub id: BeliefId,
    pub agent: ProfileId,
    pub proposition: String,
    pub credence: Credence,
    pub source: BeliefSource,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    /// Pinned beliefs stay in the prompt and are changed only by the user.
    #[serde(default)]
    pub pinned: bool,
}

keyed_enum! {
    /// Why a belief changed, when the reason is not the agent's own words.
    RevisionCause {
        /// The extraction step read new material (the reason holds the agent's words).
        Evidence => "evidence",
        /// The user edited it.
        UserEdit => "user_edit",
        /// The user told the agent to forget it.
        UserForget => "user_forget",
        /// It made room for a newer belief (the agent keeps a small set).
        Displaced => "displaced",
    }
}

/// One change of one belief: credence before (`None` for a new belief) and after (`None`
/// once forgotten), why, and where from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Revision {
    pub belief: BeliefId,
    pub agent: ProfileId,
    pub proposition: String,
    pub old: Option<Credence>,
    pub new: Option<Credence>,
    pub cause: RevisionCause,
    pub reason: String,
    pub source: BeliefSource,
    pub at: Timestamp,
}
