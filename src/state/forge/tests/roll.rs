use super::*;
use crate::domain::agent_config::AgentConfig;
use crate::domain::{ArchetypeClass, ClassicTemperament, IdentityVariant, MbtiType};
use crate::state::forge::roll::{drift_identity, drift_mbti, drift_temperament, PERSONALITY_JITTER};
use std::collections::HashSet;

const SEEDS: std::ops::Range<u64> = 0..48;

/// The rolled profile without its random id.
fn rolled(seed: u64, existing: &[Profile]) -> Profile {
    let mut profile = roll_random(seed, ForgeKind::Agent, NET, existing).profile().clone();
    profile.id = crate::domain::ProfileId::new("fixed");
    profile
}

#[test]
fn the_same_seed_rolls_the_same_thinker() {
    assert_eq!(rolled(7, &[]), rolled(7, &[]));
    assert_ne!(rolled(7, &[]), rolled(8, &[]));
}

#[test]
fn rolls_are_valid_balanced_and_varied() {
    let mut classes = HashSet::new();
    for seed in SEEDS {
        let mut draft = AgentDraft::new(ForgeKind::Agent, NET);
        classes.insert(roll_into(&mut draft, seed, &[]));
        let issues = validate(&draft, &[]);
        assert!(can_save(&issues), "seed {seed}: {issues:?}");
        assert!(is_balanced_build(&draft.profile().config), "seed {seed}");
        let age = draft.profile().config.age;
        assert!((crate::state::forge::roll::ROLL_AGE_RANGE.0..=crate::state::forge::roll::ROLL_AGE_RANGE.1).contains(&age));
    }
    assert!(classes.len() >= ArchetypeClass::ALL.len() / 2, "only {classes:?}");
}

#[test]
fn rolls_stay_close_to_their_class() {
    for seed in SEEDS {
        let mut draft = agent();
        let class = roll_into(&mut draft, seed, &[]);
        let (cfg, preset) = (&draft.profile().config, &class.preset().config);
        assert!(cfg.openness.abs_diff(preset.openness) <= PERSONALITY_JITTER, "seed {seed}");
        assert!(class.preset().enneagram_options.contains(&cfg.enneagram));
        assert_eq!(cfg.description, preset.description);
        let suggestion = class.name_suggestions().iter().find(|n| n.name == draft.profile().name);
        let suggestion = suggestion.expect("name comes from the class list");
        assert_eq!(cfg.pronouns, suggestion.pronouns);
        assert_eq!(cfg.gender, suggestion.gender);
    }
}

#[test]
fn rolled_names_and_handles_avoid_existing_profiles() {
    let first = rolled(3, &[]);
    let others = [existing("agent_a", &first.name, &first.handle)];
    let second = rolled(3, &others);
    assert_ne!(second.name, first.name, "a free name from the class list is preferred");
    assert_ne!(second.handle.to_lowercase(), first.handle.to_lowercase());
    let draft = roll_random(3, ForgeKind::Agent, NET, &others);
    assert!(validate(&draft, &others).iter().all(|i| i.kind != IssueKind::HandleTaken));
}

#[test]
fn a_reroll_is_one_undo_step() {
    let mut draft = named(agent(), "Theo Vale", "@theo");
    roll_into(&mut draft, 11, &[]);
    assert!(draft.undo());
    assert_eq!(draft.profile().name, "Theo Vale");
}

#[test]
fn personas_can_be_rolled_too() {
    let draft = roll_random(5, ForgeKind::Persona, NET, &[]);
    assert_eq!(draft.kind(), ForgeKind::Persona);
    assert!(can_save(&validate(&draft, &[])));
}

#[test]
fn typology_follows_scores_only_when_they_cross_the_midpoint() {
    let preset = AgentConfig { mbti: MbtiType::Intj, extraversion: 30, openness: 80, agreeableness: 40, conscientiousness: 80, neuroticism: 20, identity_variant: IdentityVariant::Assertive, temperament: ClassicTemperament::Melancholic, ..AgentConfig::default() };
    let nudged = AgentConfig { extraversion: 45, ..preset.clone() };
    assert_eq!(drift_mbti(&preset, &nudged), MbtiType::Intj);
    assert_eq!(drift_temperament(&preset, &nudged), ClassicTemperament::Melancholic, "no axis crossed");
    let crossed = AgentConfig { extraversion: 55, agreeableness: 60, neuroticism: 60, ..preset.clone() };
    assert_eq!(drift_mbti(&preset, &crossed), MbtiType::Enfj);
    assert_eq!(drift_identity(&preset, &crossed), IdentityVariant::Turbulent);
    assert_eq!(drift_temperament(&preset, &crossed), ClassicTemperament::Choleric);
    let calm = AgentConfig { extraversion: 55, ..preset.clone() };
    assert_eq!(drift_temperament(&preset, &calm), ClassicTemperament::Sanguine);
    assert_eq!(drift_identity(&preset, &calm), IdentityVariant::Assertive);
}
