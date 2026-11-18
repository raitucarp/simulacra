//! How many AI requests an action will send, as the confirm states it before anything runs.

use crate::domain::arena::debate::{Debate, Phase};
use crate::domain::arena::lab::Experiment;

/// A judged debate asks the judge, then maps the transcript's arguments.
const JUDGING_REQUESTS: u32 = 2;

/// What one action is about to send.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpendEstimate {
    pub requests: u32,
}

impl SpendEstimate {
    /// An action of `requests` requests (at least one: asking is for something that sends).
    pub const fn requests(requests: u32) -> Self {
        Self { requests: if requests == 0 { 1 } else { requests } }
    }

    /// A single request.
    pub const fn one() -> Self {
        Self::requests(1)
    }
}

fn count(n: usize) -> u32 {
    u32::try_from(n).unwrap_or(u32::MAX)
}

/// The requests a debate still needs: its unspoken slots, the judge and the argument map, and
/// one belief update per debater once it is judged (zero once it is done).
pub fn debate_requests(debate: &Debate) -> u32 {
    let updates = count(debate.setup.debaters.len());
    match debate.phase() {
        Phase::Done => 0,
        Phase::Judging => JUDGING_REQUESTS + updates,
        Phase::Speaking(_) => count(debate.total_turns().saturating_sub(debate.turns.len())) + JUDGING_REQUESTS + updates,
    }
}

/// The requests an experiment still needs: one per unanswered cell and agent, then one belief
/// update per agent.
pub fn experiment_requests(experiment: &Experiment) -> u32 {
    let pending = count(experiment.pending().len());
    if pending == 0 {
        return 0;
    }
    pending + count(experiment.agents.len())
}
