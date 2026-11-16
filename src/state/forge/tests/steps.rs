use super::*;
use crate::domain::agent_config::AgentConfig;
use crate::state::forge::fields::{mapped_fields, SYSTEM_FIELDS};
use std::collections::BTreeMap;
use std::path::Path;

const ASSETS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets");
/// Profile keys that are neither editable nor shown: the id, and the nested config itself.
const UNMAPPED_PROFILE_KEYS: [&str; 2] = ["id", "config"];

fn json_keys<T: serde::Serialize>(value: &T) -> Vec<String> {
    match serde_json::to_value(value).expect("serialize") {
        serde_json::Value::Object(map) => map.keys().cloned().collect(),
        other => panic!("expected an object, got {other}"),
    }
}

#[test]
fn personas_walk_four_steps_and_agents_walk_all() {
    use ForgeStep as S;
    assert_eq!(steps_for(ForgeKind::Persona), &[S::Origin, S::Appearance, S::Voice, S::Review]);
    assert_eq!(steps_for(ForgeKind::Agent), &ForgeStep::ALL);
    assert_eq!(steps_for(ForgeKind::Agent).first(), Some(&S::Origin));
    assert_eq!(steps_for(ForgeKind::Agent).last(), Some(&S::Review));
}

#[test]
fn step_keys_match_banner_files_and_i18n_namespace() {
    for step in ForgeStep::ALL {
        assert!(Path::new(ASSETS_DIR).join(step.banner()).is_file(), "{step:?} banner");
        assert_eq!(ForgeStep::from_key(step.key()), Some(step));
        assert_eq!(ForgeStep::ALL[step.index()], step);
        assert_eq!(step.label_key(), format!("forge.step.{}.label", step.key()));
        assert_eq!(step.tagline_key(), format!("forge.step.{}.tagline", step.key()));
        assert_eq!(step.guide_tip_key(), format!("forge.step.{}.tip", step.key()));
    }
    assert_eq!(ForgeStep::DailyLife.banner(), "illustrations/steps/daily_life.svg");
}

#[test]
fn next_and_previous_follow_the_kind() {
    use ForgeStep as S;
    assert_eq!(S::Appearance.next(ForgeKind::Persona), Some(S::Voice));
    assert_eq!(S::Appearance.next(ForgeKind::Agent), Some(S::Birthsign));
    assert_eq!(S::Voice.previous(ForgeKind::Persona), Some(S::Appearance));
    assert_eq!(S::Review.next(ForgeKind::Agent), None);
    assert_eq!(S::Origin.previous(ForgeKind::Agent), None);
    assert_eq!(S::Mind.next(ForgeKind::Persona), None, "personas never visit Mind");
}

#[test]
fn every_profile_and_config_field_belongs_to_exactly_one_step() {
    let mut expected: Vec<String> = json_keys(&Profile::new_alt("p", "P", "@p"))
        .into_iter()
        .filter(|key| !UNMAPPED_PROFILE_KEYS.contains(&key.as_str()))
        .collect();
    expected.extend(json_keys(&AgentConfig::default()));
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for (path, _) in mapped_fields() {
        *counts.entry(path).or_default() += 1;
    }
    for key in &expected {
        assert_eq!(counts.get(key.as_str()), Some(&1), "field `{key}` must map to exactly one step");
    }
    assert_eq!(counts.len(), expected.len(), "mapped fields that no longer exist: {counts:?}");
    assert_eq!(counts.values().sum::<usize>(), expected.len());
}

#[test]
fn field_lookup_and_step_contents() {
    assert_eq!(field_step("name"), Some(ForgeStep::Origin));
    assert_eq!(field_step("mbti"), Some(ForgeStep::Birthsign));
    assert_eq!(field_step("prompt_template"), Some(ForgeStep::Grimoire));
    assert_eq!(field_step("inject_network_grounding"), Some(ForgeStep::Grimoire));
    assert_eq!(field_step("emoji_usage"), Some(ForgeStep::Voice));
    assert_eq!(field_step("no_such_field"), None);
    assert!(fields_for_step(ForgeStep::Review).is_empty());
    for step in ForgeStep::ALL.into_iter().filter(|s| *s != ForgeStep::Review) {
        assert!(!fields_for_step(step).is_empty(), "{step:?} has no fields");
    }
    assert_eq!(SYSTEM_FIELDS.len(), 2);
    assert_eq!(field_label_key("bio"), "forge.field.bio");
}

#[test]
fn grimoire_holds_the_prompt_engineering_fields() {
    let grimoire = fields_for_step(ForgeStep::Grimoire);
    for field in ["system_prompt", "assembly_strategy", "pre_post_reasoning", "prompt_template", "inject_identity_and_bio"] {
        assert!(grimoire.contains(&field), "{field}");
    }
}
