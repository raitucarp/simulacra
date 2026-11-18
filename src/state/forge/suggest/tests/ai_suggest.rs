//! "Suggest from context": targets, prompt, robust parsing and validation, accepted edits.

use super::*;
use crate::domain::engine::TextSource;
use crate::state::forge::engine::mutate::set_text_source;
use crate::state::forge::{ForgeStep, ScoreField, TraitField, TraitSet};
use crate::state::forge::suggest::ai::{parse_suggestions, suggest_prompt, targets, value_edit, SuggestScope, SuggestedValue};

const MOOD: Param = Param::Text(TextField::CurrentMood);
const OPENNESS: Param = Param::Score(ScoreField::Openness);
const FORMAT: Param = Param::Trait(TraitField::PostFormat);
const DEVICES: Param = Param::Set(TraitSet::RhetoricalDevices);
const TEMPERATURE: Param = Param::Engine(EngineParam::Temperature);

fn own_model_agent() -> AgentDraft {
    let mut draft = agent();
    let mut engine = draft.profile().config.engine.clone();
    set_text_source(&mut engine, TextSource::Own);
    draft.edit(DraftEdit::Engine(Box::new(engine), None));
    draft
}

fn parse(draft: &AgentDraft, reply: &str, wanted: &[Param]) -> crate::state::forge::suggest::ai::ParsedSuggestions {
    parse_suggestions(reply, wanted, &SuggestContext::of(draft)).expect("a JSON object")
}

fn values_of(parsed: &crate::state::forge::suggest::ai::ParsedSuggestions, param: Param) -> Vec<SuggestedValue> {
    parsed.values.iter().find(|(p, _)| *p == param).map(|(_, values)| values.clone()).unwrap_or_default()
}

#[test]
fn targets_follow_the_step_kind_and_engine() {
    let draft = agent();
    let cfg = &draft.profile().config;
    assert!(targets(SuggestScope::Step(ForgeStep::Engine), ForgeKind::Agent, cfg).iter().all(|p| *p != TEMPERATURE));
    let own = own_model_agent();
    assert!(targets(SuggestScope::Step(ForgeStep::Engine), ForgeKind::Agent, &own.profile().config).contains(&TEMPERATURE));
    assert!(targets(SuggestScope::Step(ForgeStep::Grimoire), ForgeKind::Persona, cfg).is_empty());
    assert!(targets(SuggestScope::Field(Param::Engine(EngineParam::Credential)), ForgeKind::Agent, cfg).is_empty());
    let voice = targets(SuggestScope::Step(ForgeStep::Voice), ForgeKind::Agent, cfg);
    assert!(voice.iter().all(|param| param.step() == ForgeStep::Voice) && !voice.is_empty());
}

#[test]
fn the_prompt_lists_the_draft_and_the_rules() {
    let mut draft = agent();
    draft.edit(DraftEdit::Text(TextField::Name, "Hypatia".into()));
    let prompt = suggest_prompt(&[FORMAT, OPENNESS], &SuggestContext::of(&draft), "Indonesian");
    assert!(prompt.user.contains("- name: Hypatia"));
    assert!(prompt.user.contains("post_format: exactly one of: single_post"));
    assert!(prompt.user.contains("openness: a whole number from 0 to 100"));
    assert!(prompt.user.contains("Indonesian"));
    assert!(!prompt.user.contains("- post_format: single_post"), "targets are not in the summary");
}

#[test]
fn fenced_wrapped_and_loosely_keyed_replies_are_read() {
    let draft = agent();
    let reply = "Sure!\n```json\n{\"suggestions\": {\"Current Mood\": [\"Wry\", \"  \", \"Wry\"], \"openness\": [\"77\", 140, 61.6]}}\n```";
    let parsed = parse(&draft, reply, &[MOOD, OPENNESS]);
    assert_eq!(values_of(&parsed, MOOD), vec![SuggestedValue::Text("Wry".into())]);
    assert_eq!(values_of(&parsed, OPENNESS), vec![SuggestedValue::Number(77), SuggestedValue::Number(62)]);
    assert_eq!(parsed.discarded, 3, "blank, duplicate and out of range");
}

#[test]
fn options_are_matched_leniently_and_invalid_ones_dropped() {
    let draft = agent();
    let reply = r#"{"post_format": ["Field Note", "sonnet", "thread"], "rhetorical_devices": [["metaphor", "nonsense", "Paradox"], "irony"]}"#;
    let parsed = parse(&draft, reply, &[FORMAT, DEVICES]);
    assert_eq!(values_of(&parsed, FORMAT), vec![SuggestedValue::Option("field_note"), SuggestedValue::Option("thread")]);
    let devices = values_of(&parsed, DEVICES);
    assert_eq!(devices[0], SuggestedValue::Options(vec!["metaphor", "paradox"]));
    assert_eq!(devices[1], SuggestedValue::Options(vec!["irony"]));
}

#[test]
fn the_current_value_is_not_suggested_again() {
    let draft = agent();
    let current = draft.profile().config.rhetoric.post_format.key();
    let parsed = parse(&draft, &format!("{{\"post_format\": [\"{current}\"]}}"), &[FORMAT]);
    assert!(parsed.is_empty());
    assert_eq!(parsed.discarded, 1);
}

#[test]
fn temperatures_may_come_as_fractions() {
    let draft = own_model_agent();
    let parsed = parse(&draft, r#"{"engine_temperature": [0.65, "0.9", 3.5]}"#, &[TEMPERATURE]);
    assert_eq!(values_of(&parsed, TEMPERATURE), vec![SuggestedValue::Number(65), SuggestedValue::Number(90)]);
}

#[test]
fn replies_without_an_object_are_unreadable() {
    let draft = agent();
    assert!(parse_suggestions("no idea", &[MOOD], &SuggestContext::of(&draft)).is_none());
    assert!(parse(&draft, r#"{"unrelated": 1}"#, &[MOOD]).is_empty());
}

#[test]
fn accepted_values_become_edits_that_apply() {
    let mut draft = own_model_agent();
    let cases = [
        (MOOD, SuggestedValue::Text("Wry".into())),
        (OPENNESS, SuggestedValue::Number(40)),
        (FORMAT, SuggestedValue::Option("thread")),
        (DEVICES, SuggestedValue::Options(vec!["paradox"])),
        (TEMPERATURE, SuggestedValue::Number(55)),
        (Param::Engine(EngineParam::Tools), SuggestedValue::Options(vec!["vault_search"])),
    ];
    for (param, value) in cases {
        let edit = value_edit(param, &value, draft.profile()).unwrap_or_else(|| panic!("{param:?}"));
        assert!(draft.edit(edit), "{param:?} changed the draft");
    }
    assert_eq!(value_edit(OPENNESS, &SuggestedValue::Text("x".into()), draft.profile()), None);
}
