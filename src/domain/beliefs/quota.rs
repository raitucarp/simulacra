//! Each agent's belief-update switch and its daily cap: updates are on by default, at most
//! [`DAILY_UPDATES_PER_AGENT`] per agent and local day, and the profile's Beliefs tab turns
//! them off. Every update is one AI request, charged to the shared daily budget as well.

use serde::{Deserialize, Serialize};

/// Belief updates one agent may make per local day.
pub const DAILY_UPDATES_PER_AGENT: u32 = 6;

/// One agent's switch and today's count (`day` is `YYYY-MM-DD`, local).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BeliefSettings {
    pub updates_enabled: bool,
    pub day: String,
    pub used: u32,
}

impl Default for BeliefSettings {
    fn default() -> Self {
        Self { updates_enabled: true, day: String::new(), used: 0 }
    }
}

impl BeliefSettings {
    /// Updates made on `today`.
    pub fn used_on(&self, today: &str) -> u32 {
        if self.day == today {
            self.used
        } else {
            0
        }
    }

    /// Whether one more update may run on `today`.
    pub fn allows(&self, today: &str) -> bool {
        self.updates_enabled && self.used_on(today) < DAILY_UPDATES_PER_AGENT
    }

    /// The settings after one update on `today`.
    pub fn counted(&self, today: &str) -> Self {
        Self { day: today.to_string(), used: self.used_on(today).saturating_add(1), ..self.clone() }
    }

    /// The settings with updates switched `on` or off.
    pub fn switched(&self, on: bool) -> Self {
        Self { updates_enabled: on, ..self.clone() }
    }
}
