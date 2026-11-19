//! The debate setup form as a pure value: which agents debate on which side, the format,
//! rounds, moderator and judge. Every edit returns a new draft; switching to the Socratic
//! format clears the sides, and leaving it gives everyone a side again.

use crate::domain::arena::debate::{DebateFormat, DebateSetup, Debater, Judge, Side, DEFAULT_ROUNDS, MAX_DEBATERS, MAX_ROUNDS, MIN_ROUNDS};
use crate::domain::{PostId, ProfileId};

/// The form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebateDraft {
    pub debaters: Vec<Debater>,
    pub format: DebateFormat,
    pub rounds: u8,
    pub moderator: Option<ProfileId>,
    pub judge: Judge,
    /// The post the motion came from.
    pub source_post: Option<PostId>,
}

impl Default for DebateDraft {
    fn default() -> Self {
        Self { debaters: Vec::new(), format: DebateFormat::Oxford, rounds: DEFAULT_ROUNDS, moderator: None, judge: Judge::GlobalModel, source_post: None }
    }
}

impl DebateDraft {
    pub fn side_of(&self, agent: &ProfileId) -> Option<Side> {
        self.debaters.iter().find(|debater| &debater.agent == agent).map(|debater| debater.side)
    }

    /// The side a newly added debater takes: open in a Socratic round, else the side with
    /// fewer speakers (For on a tie).
    fn next_side(&self) -> Side {
        if self.format == DebateFormat::Socratic {
            return Side::Open;
        }
        let count = |side: Side| self.debaters.iter().filter(|debater| debater.side == side).count();
        if count(Side::Against) < count(Side::For) {
            Side::Against
        } else {
            Side::For
        }
    }

    /// `agent` set on `side`, or taken out with `None` (also out of the moderator's chair).
    pub fn with_side(&self, agent: &ProfileId, side: Option<Side>) -> Self {
        let mut next = self.clone();
        next.debaters.retain(|debater| &debater.agent != agent);
        if let Some(side) = side.filter(|_| next.debaters.len() < MAX_DEBATERS) {
            next.debaters.push(Debater { agent: agent.clone(), side });
            if next.moderator.as_ref() == Some(agent) {
                next.moderator = None;
            }
        }
        next
    }

    /// `agent` added on the side that needs one, or removed when already in.
    pub fn toggled(&self, agent: &ProfileId) -> Self {
        match self.side_of(agent) {
            Some(_) => self.with_side(agent, None),
            None => self.with_side(agent, Some(self.next_side())),
        }
    }

    pub fn with_format(&self, format: DebateFormat) -> Self {
        let mut next = Self { format, ..self.clone() };
        let agents: Vec<ProfileId> = next.debaters.drain(..).map(|debater| debater.agent).collect();
        agents.iter().fold(next, |draft, agent| draft.with_side(agent, Some(draft.next_side())))
    }

    pub fn with_rounds(&self, delta: i32) -> Self {
        let rounds = (i32::from(self.rounds) + delta).clamp(i32::from(MIN_ROUNDS), i32::from(MAX_ROUNDS)) as u8;
        Self { rounds, ..self.clone() }
    }

    /// `moderator` in the chair (out of the debaters), or nobody.
    pub fn with_moderator(&self, moderator: Option<ProfileId>) -> Self {
        let mut next = match &moderator {
            Some(agent) => self.with_side(agent, None),
            None => self.clone(),
        };
        next.moderator = moderator;
        next
    }

    pub fn with_judge(&self, judge: Judge) -> Self {
        Self { judge, ..self.clone() }
    }

    /// The setup of `motion` (validated by the domain before it starts).
    pub fn setup(&self, motion: &str) -> DebateSetup {
        DebateSetup {
            motion: motion.trim().to_string(),
            source_post: self.source_post.clone(),
            debaters: self.debaters.clone(),
            format: self.format,
            rounds: self.rounds,
            moderator: self.moderator.clone(),
            judge: self.judge.clone(),
        }
    }
}
