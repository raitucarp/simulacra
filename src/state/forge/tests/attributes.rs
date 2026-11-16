use super::*;
use crate::domain::agent_config::AgentConfig;
use crate::domain::ArchetypeClass;
use crate::state::forge::attributes::{BUDGET_FIELDS, LEVEL_THRESHOLDS};

fn flat(score: u8) -> AgentConfig {
    let mut cfg = AgentConfig::default();
    for field in BUDGET_FIELDS {
        *field.of_mut(&mut cfg) = score;
    }
    cfg
}

#[test]
fn budget_counts_the_six_personality_scores() {
    let cfg = flat(50);
    assert_eq!(budget_spent(&cfg), 300);
    assert_eq!(budget_remaining(&cfg), i32::from(PERSONALITY_POINT_BUDGET) - 300);
    assert!(is_balanced_build(&cfg));
    assert!(!is_balanced_build(&flat(90)));
    assert!(budget_remaining(&flat(90)) < 0);
}

#[test]
fn can_raise_respects_budget_and_range() {
    let cfg = flat(70);
    assert_eq!(budget_remaining(&cfg), 0);
    assert!(!can_raise(&cfg, ScoreField::Openness, 1), "budget exhausted");
    assert!(can_raise(&cfg, ScoreField::FluidIntelligence, 1), "quotients are not budgeted");
    let roomy = flat(40);
    assert!(can_raise(&roomy, ScoreField::Openness, 60));
    assert!(!can_raise(&roomy, ScoreField::Openness, 61), "never above 100");
    assert!(can_raise(&roomy, ScoreField::Openness, 0));
}

#[test]
fn every_class_preset_is_a_balanced_build() {
    for class in ArchetypeClass::ALL {
        assert!(is_balanced_build(&class.config()), "{class:?} spends {}", budget_spent(&class.config()));
    }
    assert!(is_balanced_build(&AgentConfig::default()));
}

#[test]
fn quotient_levels_use_the_thresholds() {
    assert_eq!(QuotientLevel::from_score(0), QuotientLevel::Novice);
    assert_eq!(QuotientLevel::from_score(19), QuotientLevel::Novice);
    assert_eq!(QuotientLevel::from_score(20), QuotientLevel::Apprentice);
    assert_eq!(QuotientLevel::from_score(59), QuotientLevel::Adept);
    assert_eq!(QuotientLevel::from_score(60), QuotientLevel::Expert);
    assert_eq!(QuotientLevel::from_score(94), QuotientLevel::Master);
    assert_eq!(QuotientLevel::from_score(95), QuotientLevel::Legendary);
    assert_eq!(QuotientLevel::from_score(u8::MAX), QuotientLevel::Legendary);
    for (level, threshold) in QuotientLevel::ALL.into_iter().zip(LEVEL_THRESHOLDS) {
        assert_eq!(level.threshold(), threshold);
        assert_eq!(QuotientLevel::from_score(threshold), level);
        assert_eq!(level.label_key(), format!("forge.level.{}", level.key()));
    }
    assert!(LEVEL_THRESHOLDS.windows(2).all(|w| w[0] < w[1]));
}

#[test]
fn radar_axes_are_normalized_and_labelled() {
    let cfg = AgentConfig { openness: 100, neuroticism: 0, fluid_intelligence: 25, ..AgentConfig::default() };
    let personality = personality_radar(&cfg);
    assert_eq!(personality.len(), 6);
    assert_eq!(personality[0].label_key, "forge.field.openness");
    assert_eq!(personality[0].value, 1.0);
    assert_eq!(personality[4].value, 0.0);
    let minds = intelligence_radar(&cfg);
    assert_eq!(minds.len(), 5);
    assert_eq!(minds[0].field, ScoreField::FluidIntelligence);
    assert!((minds[0].value - 0.25).abs() < f32::EPSILON);
    assert!(minds.iter().chain(&personality).all(|p| (0.0..=1.0).contains(&p.value)));
}
