//! A saved thought experiment: the scenario, the agents asked, and each agent's answer per
//! cell (a stance, a one-line verdict and short reasoning). Rerunning keeps the scenario and
//! clears the answers.

use super::scenario::{cells, Scenario};
use crate::domain::arena::ArenaId;
use crate::domain::document::keyed::keyed_enum;
use crate::domain::ProfileId;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

keyed_enum! {
    /// An agent's answer to the question, in one word.
    Stance {
        Yes => "yes",
        No => "no",
        /// It depends, or the agent will not commit.
        Undecided => "undecided",
    }
}

/// One agent's answer in one cell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CellAnswer {
    pub agent: ProfileId,
    /// Index into the scenario's cells.
    pub cell: usize,
    pub stance: Stance,
    pub verdict: String,
    #[serde(default)]
    pub reasoning: String,
}

impl CellAnswer {
    /// The answer as one line for a prompt: `yes: verdict (reasoning)`.
    pub fn as_line(&self) -> String {
        match self.reasoning.trim() {
            "" => format!("{}: {}", self.stance.key(), self.verdict),
            reasoning => format!("{}: {} ({reasoning})", self.stance.key(), self.verdict),
        }
    }
}

/// One experiment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Experiment {
    pub id: ArenaId,
    pub scenario: Scenario,
    pub agents: Vec<ProfileId>,
    #[serde(default)]
    pub answers: Vec<CellAnswer>,
    /// The preset it started from, if any (its key).
    #[serde(default)]
    pub preset: Option<String>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl Experiment {
    pub fn new(id: ArenaId, scenario: Scenario, agents: Vec<ProfileId>, preset: Option<String>, now: Timestamp) -> Self {
        Self { id, scenario, agents, answers: Vec::new(), preset, created_at: now, updated_at: now }
    }

    /// `(agent, cell)` pairs still unanswered, cell by cell.
    pub fn pending(&self) -> Vec<(ProfileId, usize)> {
        let count = cells(&self.scenario).len();
        (0..count)
            .flat_map(|cell| self.agents.iter().map(move |agent| (agent.clone(), cell)))
            .filter(|(agent, cell)| self.answer(agent, *cell).is_none())
            .collect()
    }

    pub fn answer(&self, agent: &ProfileId, cell: usize) -> Option<&CellAnswer> {
        self.answers.iter().find(|answer| &answer.agent == agent && answer.cell == cell)
    }

    /// Whether every agent answered every cell.
    pub fn is_complete(&self) -> bool {
        self.pending().is_empty()
    }

    /// The experiment with `answer` recorded (replacing an earlier one for that pair).
    pub fn with_answer(&self, answer: CellAnswer, now: Timestamp) -> Self {
        let same = |old: &&CellAnswer| old.agent == answer.agent && old.cell == answer.cell;
        let mut answers: Vec<CellAnswer> = self.answers.iter().filter(|old| !same(old)).cloned().collect();
        answers.push(answer);
        Self { answers, updated_at: now, ..self.clone() }
    }

    /// The experiment ready to run again: same scenario and agents, no answers.
    pub fn cleared(&self, now: Timestamp) -> Self {
        Self { answers: Vec::new(), updated_at: now, ..self.clone() }
    }
}
