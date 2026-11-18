//! The update step, pure: what the extraction proposed, merged into an agent's beliefs.
//!
//! - A proposal stating an existing belief (see `matching`) revises its credence, unless the
//!   belief is pinned (pinned beliefs are the user's) or the move is below
//!   [`MIN_CREDENCE_MOVE`] points (noise, not news).
//! - Any other proposal becomes a new belief.
//! - An agent keeps at most [`MAX_BELIEFS_PER_AGENT`]; the least salient unpinned beliefs
//!   that this step did not touch make room, each with a "displaced" revision.

use super::book::{BeliefBook, BeliefChange};
use super::matching::{same_belief, tidy};
use super::model::{Belief, BeliefId, BeliefSource, Credence, Revision, RevisionCause};
use super::salience::by_salience;
use crate::domain::ProfileId;
use chrono::{DateTime, Utc};

/// Beliefs one agent keeps.
pub const MAX_BELIEFS_PER_AGENT: usize = 16;
/// Smallest credence move (percent points) that counts as a revision.
pub const MIN_CREDENCE_MOVE: u8 = 3;
/// Characters kept of a proposition and of a reason.
pub const MAX_PROPOSITION_CHARS: usize = 180;
pub const MAX_REASON_CHARS: usize = 240;

/// One belief the extraction proposes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proposal {
    pub proposition: String,
    pub credence: Credence,
    pub reason: String,
}

fn clipped(text: &str, max: usize) -> String {
    tidy(text).chars().take(max).collect()
}

/// A revision of `belief` from `old` to `new`.
fn revision(belief: &Belief, old: Option<Credence>, new: Option<Credence>, cause: RevisionCause, reason: &str, now: DateTime<Utc>) -> Revision {
    Revision {
        belief: belief.id.clone(),
        agent: belief.agent.clone(),
        proposition: belief.proposition.clone(),
        old,
        new,
        cause,
        reason: clipped(reason, MAX_REASON_CHARS),
        source: belief.source.clone(),
        at: now,
    }
}

/// The working set while proposals are merged.
struct Merge {
    beliefs: Vec<Belief>,
    touched: Vec<BeliefId>,
    revisions: Vec<Revision>,
}

impl Merge {
    fn take(mut self, proposal: &Proposal, source: &BeliefSource, agent: &ProfileId, now: DateTime<Utc>, new_id: &mut impl FnMut() -> BeliefId) -> Self {
        let proposition = clipped(&proposal.proposition, MAX_PROPOSITION_CHARS);
        if proposition.is_empty() {
            return self;
        }
        match self.beliefs.iter().position(|belief| same_belief(&belief.proposition, &proposition)) {
            Some(index) => {
                let old = self.beliefs[index].clone();
                let moved = old.credence.percent().abs_diff(proposal.credence.percent()) >= MIN_CREDENCE_MOVE;
                if old.pinned || !moved || self.touched.contains(&old.id) {
                    return self;
                }
                let updated = Belief { credence: proposal.credence, source: source.clone(), updated_at: now, ..old.clone() };
                self.revisions.push(revision(&updated, Some(old.credence), Some(proposal.credence), RevisionCause::Evidence, &proposal.reason, now));
                self.touched.push(updated.id.clone());
                self.beliefs[index] = updated;
            }
            None => {
                let belief = Belief { id: new_id(), agent: agent.clone(), proposition, credence: proposal.credence, source: source.clone(), created_at: now, updated_at: now, pinned: false };
                self.revisions.push(revision(&belief, None, Some(belief.credence), RevisionCause::Evidence, &proposal.reason, now));
                self.touched.push(belief.id.clone());
                self.beliefs.push(belief);
            }
        }
        self
    }

    /// Ids of the beliefs to displace so at most the cap remain.
    fn overflow(&self, book: &BeliefBook, now: DateTime<Utc>) -> Vec<BeliefId> {
        let excess = self.beliefs.len().saturating_sub(MAX_BELIEFS_PER_AGENT);
        let candidates = self.beliefs.iter().filter(|belief| !belief.pinned && !self.touched.contains(&belief.id));
        let ranked = by_salience(candidates.map(|belief| (belief, book.history(&belief.id).count())), now);
        ranked.into_iter().rev().take(excess).map(|belief| belief.id.clone()).collect()
    }
}

/// The change that merges `proposals` (read from `source`) into `agent`'s beliefs in `book`.
pub fn apply_proposals(book: &BeliefBook, agent: &ProfileId, proposals: &[Proposal], source: &BeliefSource, now: DateTime<Utc>, mut new_id: impl FnMut() -> BeliefId) -> BeliefChange {
    let start = Merge { beliefs: book.of_agent(agent).cloned().collect(), touched: Vec::new(), revisions: Vec::new() };
    let mut merged = proposals.iter().fold(start, |merge, proposal| merge.take(proposal, source, agent, now, &mut new_id));
    let removed = merged.overflow(book, now);
    for belief in merged.beliefs.iter().filter(|belief| removed.contains(&belief.id)) {
        merged.revisions.push(revision(belief, Some(belief.credence), None, RevisionCause::Displaced, "", now));
    }
    let upserted = merged.beliefs.into_iter().filter(|belief| merged.touched.contains(&belief.id) && !removed.contains(&belief.id)).collect();
    BeliefChange { upserted, removed, revisions: merged.revisions }
}

/// A revision written by the user's own action on `belief`.
pub fn user_revision(belief: &Belief, old: Option<Credence>, new: Option<Credence>, cause: RevisionCause, now: DateTime<Utc>) -> Revision {
    Revision { source: BeliefSource::user(), ..revision(belief, old, new, cause, "", now) }
}
