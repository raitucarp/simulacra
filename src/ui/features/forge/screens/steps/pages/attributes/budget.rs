//! Pure numbers behind the "balanced build" meter, and which score sliders go in which card.

use crate::domain::agent_config::AgentConfig;
use crate::state::forge::attributes::{budget_spent, PERSONALITY_POINT_BUDGET};
use crate::state::forge::ScoreField;

/// The Big Five personality scores, in card order.
pub const BIG_FIVE: [ScoreField; 5] = [
    ScoreField::Openness,
    ScoreField::Conscientiousness,
    ScoreField::Extraversion,
    ScoreField::Agreeableness,
    ScoreField::Neuroticism,
];

/// Bounds of the meter's fill fraction.
const EMPTY: f32 = 0.0;
const FULL: f32 = 1.0;

/// The HEXACO addition to the Big Five.
pub const HEXACO: [ScoreField; 1] = [ScoreField::HonestyHumility];

/// Points spent against the balanced-build budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BudgetMeter {
    pub spent: u16,
    pub budget: u16,
}

impl BudgetMeter {
    pub fn of(cfg: &AgentConfig) -> Self {
        Self { spent: budget_spent(cfg), budget: PERSONALITY_POINT_BUDGET }
    }

    /// Points left to spend (zero when over budget).
    pub fn remaining(self) -> u16 {
        self.budget.saturating_sub(self.spent)
    }

    /// How far the build is over budget, if it is.
    pub fn over_by(self) -> Option<u16> {
        self.spent.checked_sub(self.budget).filter(|over| *over > 0)
    }

    /// Spent share of the budget in `0.0..=1.0`, for the meter bar.
    pub fn fill(self) -> f32 {
        if self.budget == 0 {
            return FULL;
        }
        (f32::from(self.spent) / f32::from(self.budget)).clamp(EMPTY, FULL)
    }
}
