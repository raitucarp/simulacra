//! A saved debate: its setup, the turns spoken so far, the verdict and the argument map once
//! judged, and the thread it was published as. Where it stands follows from these alone, so
//! a debate stopped halfway resumes at its next slot.

use super::judge::Verdict;
use super::model::{DebateSetup, Slot, Turn};
use super::schedule::schedule;
use crate::domain::arena::ArenaId;
use crate::domain::thinking::argument::ArgumentMap;
use crate::domain::PostId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// One debate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Debate {
    pub id: ArenaId,
    pub setup: DebateSetup,
    #[serde(default)]
    pub turns: Vec<Turn>,
    #[serde(default)]
    pub verdict: Option<Verdict>,
    /// The arguments of the transcript, one node source per turn.
    #[serde(default)]
    pub map: Option<ArgumentMap>,
    /// The root post once published as a thread.
    #[serde(default)]
    pub published: Option<PostId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Where a debate stands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Phase {
    /// The next slot to speak.
    Speaking(Slot),
    /// Every slot spoke; the judge has not scored yet.
    Judging,
    Done,
}

impl Debate {
    /// A new debate of `setup`, with no turns yet.
    pub fn new(id: ArenaId, setup: DebateSetup, now: DateTime<Utc>) -> Self {
        Self { id, setup, turns: Vec::new(), verdict: None, map: None, published: None, created_at: now, updated_at: now }
    }

    pub fn phase(&self) -> Phase {
        match schedule(&self.setup).into_iter().nth(self.turns.len()) {
            Some(slot) => Phase::Speaking(slot),
            None if self.verdict.is_none() => Phase::Judging,
            None => Phase::Done,
        }
    }

    /// Slots in all.
    pub fn total_turns(&self) -> usize {
        schedule(&self.setup).len()
    }

    /// The debate with `text` spoken in its next slot (unchanged when no slot is left).
    pub fn with_turn(&self, text: &str, now: DateTime<Utc>) -> Self {
        let Phase::Speaking(slot) = self.phase() else { return self.clone() };
        let mut next = self.clone();
        next.turns.push(Turn { slot, text: text.trim().to_string(), at: now });
        next.updated_at = now;
        next
    }

    /// The debate judged.
    pub fn with_verdict(&self, verdict: Verdict, map: Option<ArgumentMap>, now: DateTime<Utc>) -> Self {
        Self { verdict: Some(verdict), map, updated_at: now, ..self.clone() }
    }

    /// The debate published as the thread rooted at `root`.
    pub fn with_published(&self, root: PostId, now: DateTime<Utc>) -> Self {
        Self { published: Some(root), updated_at: now, ..self.clone() }
    }

    /// The synthetic post id of turn `index`, cited by the argument map's nodes.
    pub fn turn_ref(&self, index: usize) -> PostId {
        PostId::new(format!("{}{TURN_REF_SEPARATOR}{index}", self.id))
    }
}

/// Joins a debate id and a turn index in a turn reference.
pub const TURN_REF_SEPARATOR: &str = "#turn-";
