//! The lab form beside its text fields, as a pure value: the agents asked, how many variable
//! rows are in use, and the preset the scenario started from.

use crate::domain::arena::lab::{PresetKey, MAX_VARIABLES};
use crate::domain::ProfileId;

/// Variable rows shown on a fresh form.
const DEFAULT_VARIABLES: usize = 2;

/// The form's non-text state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabDraft {
    pub agents: Vec<ProfileId>,
    /// Variable rows in use (1 to the maximum).
    pub variables: usize,
    pub preset: Option<PresetKey>,
}

impl Default for LabDraft {
    fn default() -> Self {
        Self { agents: Vec::new(), variables: DEFAULT_VARIABLES, preset: None }
    }
}

impl LabDraft {
    /// `agent` asked, or no longer.
    pub fn toggled(&self, agent: &ProfileId) -> Self {
        let mut agents = self.agents.clone();
        match agents.iter().position(|chosen| chosen == agent) {
            Some(index) => {
                agents.remove(index);
            }
            None => agents.push(agent.clone()),
        }
        Self { agents, ..self.clone() }
    }

    /// One variable row more or fewer, kept between one and the maximum.
    pub fn with_variables(&self, delta: i32) -> Self {
        let variables = (self.variables as i32 + delta).clamp(1, MAX_VARIABLES as i32) as usize;
        Self { variables, ..self.clone() }
    }

    /// The agents that are still in the network, in `order`.
    pub fn present_agents(&self, order: &[ProfileId]) -> Vec<ProfileId> {
        order.iter().filter(|agent| self.agents.contains(agent)).cloned().collect()
    }
}
