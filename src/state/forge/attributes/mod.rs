//! Attribute helpers: the "balanced build" personality budget, quotient level names, radar data.

mod levels;
mod radar;

pub use levels::{QuotientLevel, LEVEL_THRESHOLDS};
pub use radar::{intelligence_radar, personality_radar, RadarPoint, INTELLIGENCE_FIELDS};

use super::fields::{ScoreField, MAX_SCORE};
use crate::domain::agent_config::AgentConfig;

/// The Big Five + HEXACO scores that share the point budget.
pub const BUDGET_FIELDS: [ScoreField; 6] = [
    ScoreField::Openness,
    ScoreField::Conscientiousness,
    ScoreField::Extraversion,
    ScoreField::Agreeableness,
    ScoreField::Neuroticism,
    ScoreField::HonestyHumility,
];

/// Total points a balanced build may spread over [`BUDGET_FIELDS`] (an average of 70).
pub const PERSONALITY_POINT_BUDGET: u16 = 420;

/// Value of a score field on a bare config.
pub fn score(cfg: &AgentConfig, field: ScoreField) -> u8 {
    *field.of(cfg)
}

/// Points currently spent on the budgeted personality scores.
pub fn budget_spent(cfg: &AgentConfig) -> u16 {
    BUDGET_FIELDS.iter().map(|field| u16::from(score(cfg, *field))).sum()
}

/// Points left in the balanced-build budget (negative when the build is over budget).
pub fn budget_remaining(cfg: &AgentConfig) -> i32 {
    i32::from(PERSONALITY_POINT_BUDGET) - i32::from(budget_spent(cfg))
}

/// Whether `field` can go up by `by` points: within `0..=100` and, for budgeted
/// personality scores, without exceeding the balanced-build budget.
pub fn can_raise(cfg: &AgentConfig, field: ScoreField, by: u8) -> bool {
    let raised = u16::from(score(cfg, field)) + u16::from(by);
    if raised > u16::from(MAX_SCORE) {
        return false;
    }
    !BUDGET_FIELDS.contains(&field) || budget_remaining(cfg) >= i32::from(by)
}

/// Whether the personality scores fit the balanced-build budget.
pub fn is_balanced_build(cfg: &AgentConfig) -> bool {
    budget_remaining(cfg) >= 0
}
