//! Every agent's beliefs and the whole revision log, as loaded from the vault, with the
//! queries the profile, the prompt and the update step need. A [`BeliefChange`] is applied
//! by value: the book that comes back holds the changed beliefs and the new revisions.

use super::model::{Belief, BeliefId, Revision};
use super::salience::by_salience;
use crate::domain::ProfileId;
use chrono::{DateTime, Utc};

/// All beliefs and revisions.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BeliefBook {
    pub beliefs: Vec<Belief>,
    /// Oldest first.
    pub revisions: Vec<Revision>,
}

/// What one update or user action changes.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BeliefChange {
    /// New or changed beliefs (replacing any with the same id).
    pub upserted: Vec<Belief>,
    pub removed: Vec<BeliefId>,
    pub revisions: Vec<Revision>,
}

impl BeliefChange {
    pub fn is_empty(&self) -> bool {
        self.upserted.is_empty() && self.removed.is_empty() && self.revisions.is_empty()
    }
}

impl BeliefBook {
    /// The beliefs of `agent`, in stored order.
    pub fn of_agent<'a>(&'a self, agent: &'a ProfileId) -> impl Iterator<Item = &'a Belief> + 'a {
        self.beliefs.iter().filter(move |belief| &belief.agent == agent)
    }

    pub fn belief(&self, id: &BeliefId) -> Option<&Belief> {
        self.beliefs.iter().find(|belief| &belief.id == id)
    }

    /// The revisions of belief `id`, oldest first.
    pub fn history<'a>(&'a self, id: &'a BeliefId) -> impl DoubleEndedIterator<Item = &'a Revision> + 'a {
        self.revisions.iter().filter(move |revision| &revision.belief == id)
    }

    /// The revisions of `agent`, newest first.
    pub fn log_of<'a>(&'a self, agent: &'a ProfileId) -> impl Iterator<Item = &'a Revision> + 'a {
        self.revisions.iter().rev().filter(move |revision| &revision.agent == agent)
    }

    /// The beliefs of `agent`, most salient first.
    pub fn ranked<'a>(&'a self, agent: &'a ProfileId, now: DateTime<Utc>) -> Vec<&'a Belief> {
        by_salience(self.of_agent(agent).map(|belief| (belief, self.history(&belief.id).count())), now)
    }

    /// The book with `change` applied.
    pub fn applied(&self, change: &BeliefChange) -> Self {
        let replaced = |belief: &Belief| change.upserted.iter().any(|new| new.id == belief.id) || change.removed.contains(&belief.id);
        let kept = self.beliefs.iter().filter(|belief| !replaced(belief)).cloned();
        Self {
            beliefs: kept.chain(change.upserted.iter().cloned()).collect(),
            revisions: self.revisions.iter().chain(&change.revisions).cloned().collect(),
        }
    }
}
