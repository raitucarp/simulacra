//! Normalized `(label, 0..1)` axes for the UI's radar charts.

use super::{score, BUDGET_FIELDS};
use crate::domain::agent_config::AgentConfig;
use crate::state::forge::fields::{field_label_key, ScoreField, MAX_SCORE};

/// The five intelligence quotients, in radar order.
pub const INTELLIGENCE_FIELDS: [ScoreField; 5] = [
    ScoreField::FluidIntelligence,
    ScoreField::CrystallizedIntelligence,
    ScoreField::EmotionalIntelligence,
    ScoreField::SocialIntelligence,
    ScoreField::ExistentialIntelligence,
];

/// One radar axis: the i18n key of its label and a value in `0.0..=1.0`.
#[derive(Debug, Clone, PartialEq)]
pub struct RadarPoint {
    pub field: ScoreField,
    pub label_key: String,
    pub value: f32,
}

fn axes(cfg: &AgentConfig, fields: &[ScoreField]) -> Vec<RadarPoint> {
    fields
        .iter()
        .map(|&field| RadarPoint {
            field,
            label_key: field_label_key(field.path()),
            value: f32::from(score(cfg, field).min(MAX_SCORE)) / f32::from(MAX_SCORE),
        })
        .collect()
}

/// Big Five + HEXACO axes.
pub fn personality_radar(cfg: &AgentConfig) -> Vec<RadarPoint> {
    axes(cfg, &BUDGET_FIELDS)
}

/// Intelligence-quotient axes.
pub fn intelligence_radar(cfg: &AgentConfig) -> Vec<RadarPoint> {
    axes(cfg, &INTELLIGENCE_FIELDS)
}
