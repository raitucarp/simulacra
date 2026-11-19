//! What the shell hands the Arena page: the network's agents, the saved debates and
//! experiments, what is running, the last failure, whether a key is set, today's AI budget
//! (for the lab's preview) and the thread open in the inspector (a motion source).

use crate::domain::arena::debate::Debate;
use crate::domain::arena::lab::Experiment;
use crate::domain::arena::ArenaId;
use crate::domain::embedding::{BudgetLimits, DailyUsage};
use crate::domain::{PostId, ProfileId};

/// An agent the page can pick.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentChoice {
    pub id: ProfileId,
    pub name: String,
    pub handle: String,
}

/// What is running.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RunState {
    #[default]
    Idle,
    /// A debate turn by this speaker.
    Speaking(ArenaId, ProfileId),
    /// The judge (and then the argument map) of this debate.
    Judging(ArenaId),
    /// An experiment's cells.
    Answering(ArenaId),
}

impl RunState {
    pub fn is_running(&self) -> bool {
        *self != Self::Idle
    }

    /// Whether the record `id` is the one running.
    pub fn runs(&self, id: &ArenaId) -> bool {
        match self {
            Self::Idle => false,
            Self::Speaking(running, _) | Self::Judging(running) | Self::Answering(running) => running == id,
        }
    }
}

/// The thread open in the inspector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenThread {
    pub post: PostId,
    pub text: String,
}

/// Everything the page shows.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ArenaData {
    pub agents: Vec<AgentChoice>,
    /// Newest first.
    pub debates: Vec<Debate>,
    pub experiments: Vec<Experiment>,
    pub run: RunState,
    /// Why the last request failed, as the user reads it.
    pub notice: Option<String>,
    pub has_key: bool,
    pub usage: DailyUsage,
    pub limits: Option<BudgetLimits>,
    pub today: String,
    pub open_thread: Option<OpenThread>,
}

impl ArenaData {
    pub fn agent_name(&self, id: &ProfileId) -> String {
        self.agents.iter().find(|agent| &agent.id == id).map(|agent| agent.name.clone()).unwrap_or_else(|| id.to_string())
    }

    pub fn debate(&self, id: &ArenaId) -> Option<&Debate> {
        self.debates.iter().find(|debate| &debate.id == id)
    }

    pub fn experiment(&self, id: &ArenaId) -> Option<&Experiment> {
        self.experiments.iter().find(|experiment| &experiment.id == id)
    }
}
