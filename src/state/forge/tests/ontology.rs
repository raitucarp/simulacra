//! Flat ontology and rhetoric in the forge model: coherent bodies, multi-selects, describe,
//! roll, validation and class previews.

use super::*;
use crate::domain::traits::*;
use crate::domain::ArchetypeClass;
use crate::state::forge::roll::rolled_embodiment;

const SEEDS: std::ops::Range<u64> = 0..64;

fn body(draft: &AgentDraft) -> Embodiment {
    draft.profile().config.embodiment
}

#[test]
fn kind_and_form_edits_keep_the_body_coherent() {
    let mut draft = agent();
    assert!(draft.edit(DraftEdit::Trait(TraitValue::Form(BodyForm::Octopus))));
    assert_eq!(body(&draft).body_kind, BodyKind::Creature, "the kind follows the form");
    assert!(draft.edit(DraftEdit::Trait(TraitValue::BodyKind(BodyKind::Object))));
    assert_eq!(body(&draft).form, BodyForm::TinCan, "a foreign form gives way to the kind's first");
    assert!(body(&draft).is_coherent());
}

#[test]
fn picking_a_form_is_one_undo_step_to_its_nature() {
    let mut draft = agent();
    let can = body_for_form(body(&draft), BodyForm::TinCan);
    assert!(draft.edit_all(&body_edits(can)));
    assert_eq!(body(&draft), Embodiment::natural(BodyForm::TinCan));
    let mut stance = body(&draft);
    stance.withdrawal = Withdrawal::FullyRevealed;
    draft.edit_all(&body_edits(stance));
    let book = body_for_form(body(&draft), BodyForm::Book);
    assert_eq!(book.withdrawal, Withdrawal::FullyRevealed, "same kind keeps the stance");
    assert_eq!(body_for_kind(book, BodyKind::Object), book);
    assert!(draft.undo() && draft.undo());
    assert!(body(&draft).is_human());
}

#[test]
fn rolled_beings_are_coherent_and_people_stay_people() {
    let rolled: Vec<Embodiment> = SEEDS.map(|seed| rolled_embodiment(seed, BodyKind::Human, BeingScope::AnyBeing)).collect();
    assert!(rolled.iter().all(Embodiment::is_coherent));
    let kinds: std::collections::HashSet<BodyKind> = rolled.iter().map(|b| b.body_kind).collect();
    assert!(kinds.len() >= BodyKind::ALL.len() / 2, "varied kinds: {kinds:?}");
    assert!(SEEDS.clone().all(|seed| rolled_embodiment(seed, BodyKind::Human, BeingScope::SameKind).is_human()));
    assert!(SEEDS.clone().all(|seed| rolled_embodiment(seed, BodyKind::Plant, BeingScope::SameKind).body_kind == BodyKind::Plant));
}

#[test]
fn a_class_or_a_reroll_never_changes_the_body() {
    let mut draft = agent();
    draft.edit_all(&body_edits(Embodiment::natural(BodyForm::Whale)));
    apply_class(&mut draft, ArchetypeClass::StoicSage);
    assert_eq!(body(&draft), Embodiment::natural(BodyForm::Whale));
    assert_eq!(draft.profile().config.rhetoric, ArchetypeClass::StoicSage.config().rhetoric, "a class sets rhetoric");
    roll_into(&mut draft, 3, &[]);
    assert_eq!(body(&draft).form, BodyForm::Whale);
}

#[test]
fn set_edits_toggle_and_preview_as_sets() {
    let mut draft = agent();
    let devices = OptionSet::new([RhetoricalDevice::Irony, RhetoricalDevice::Metaphor]);
    assert!(draft.edit(DraftEdit::TraitSet(TraitSetValue::RhetoricalDevices(devices.clone()))));
    assert_eq!(draft.profile().config.rhetoric.rhetorical_devices.keys(), ["metaphor", "irony"]);
    assert!(!draft.edit(DraftEdit::TraitSet(TraitSetValue::RhetoricalDevices(devices))), "no-op");
    let changes = preview_diff(&draft, ArchetypeClass::ExistentialPoet);
    let change = changes.iter().find(|c| c.field == "rhetorical_devices").expect("devices change");
    assert_eq!(change.step, ForgeStep::Rhetoric);
    assert!(matches!(change.after, ChangeValue::Set(TraitSetValue::RhetoricalDevices(_))));
}

#[test]
fn describe_fills_bodies_and_lists() {
    let prompt = describe_prompt("A rusty tin can.");
    assert!(prompt.contains("- form: person | tin_can") && prompt.contains("- rhetorical_devices: metaphor"));
    let reply = r#"{"form": "tin can", "rhetorical_devices": ["irony", "chiasmus", "sarcasm"], "lexical_fields": "seafaring"}"#;
    let patch = parse_described(reply).expect("parses");
    let mut draft = agent();
    draft.apply_patch(&patch);
    let cfg = &draft.profile().config;
    assert_eq!((cfg.embodiment.body_kind, cfg.embodiment.form), (BodyKind::Object, BodyForm::TinCan));
    assert_eq!(cfg.rhetoric.rhetorical_devices.keys(), ["chiasmus", "irony"]);
    assert_eq!(cfg.rhetoric.lexical_fields.keys(), ["seafaring"]);
    let invalid = PatchWarning::InvalidSetOption { field: TraitSet::RhetoricalDevices, value: "sarcasm".into() };
    assert_eq!(patch.warnings, vec![invalid]);
}

#[test]
fn an_agent_without_active_days_is_flagged_on_daily_life() {
    let mut draft = named(agent(), "Clock", "@clock");
    draft.edit(DraftEdit::TraitSet(TraitSetValue::ActiveDays(OptionSet::default())));
    let issues = validate(&draft, &[]);
    let issue = issues.iter().find(|i| i.kind == IssueKind::NoActiveDays).expect("flagged");
    assert_eq!((issue.step, issue.field, issue.severity), (ForgeStep::DailyLife, "active_days", Severity::Warning));
    assert!(can_save(&issues));
}

#[test]
fn old_configs_load_as_humans_posting_every_day() {
    let old: crate::domain::AgentConfig = serde_json::from_str(r#"{"bio": "From before."}"#).expect("old config");
    assert!(old.embodiment.is_human());
    assert_eq!(old.active_days.len(), 7);
    let odd = r#"{"form": "Cat", "rhetorical_devices": ["Irony", "Retired"], "active_days": ["Monday", 3]}"#;
    let cfg: crate::domain::AgentConfig = serde_json::from_str(odd).expect("unknown entries are skipped");
    assert_eq!((cfg.embodiment.form, cfg.rhetoric.rhetorical_devices.len(), cfg.active_days.len()), (BodyForm::Cat, 1, 1));
    assert_eq!(cfg.embodiment.body_kind, BodyKind::Creature, "a stored form brings its kind");
    let wire = serde_json::to_value(&cfg).expect("serializes");
    assert_eq!(wire["form"], "Cat", "embodiment fields are flat top-level keys");
}

#[test]
fn the_avatar_follows_the_body_unless_chosen_by_hand() {
    use crate::domain::portrait::{AvatarMode, EmblemSpec, FormKind};
    use crate::state::forge::avatar_for_body;
    let can = Embodiment::natural(BodyForm::TinCan);
    match avatar_for_body(can, None, 9) {
        Some(AvatarMode::Form(spec)) => assert_eq!((spec.kind, spec.form()), (FormKind::Object, "tin_can")),
        other => panic!("expected a tin-can form body, got {other:?}"),
    }
    let shown = avatar_for_body(can, None, 9);
    assert_eq!(avatar_for_body(can, shown.as_ref(), 10), None, "the matching body stays");
    assert!(matches!(avatar_for_body(Embodiment::default(), shown.as_ref(), 9), Some(AvatarMode::Pcg(_))));
    let emblem = AvatarMode::Emblem(EmblemSpec::default());
    assert_eq!(avatar_for_body(can, Some(&emblem), 9), None, "an emblem is the user's choice");
    let mut draft = agent();
    roll_being(&mut draft, 5, BeingScope::AnyBeing);
    let body = draft.profile().config.embodiment;
    assert_eq!(body.is_human(), !matches!(draft.profile().config.avatar, Some(AvatarMode::Form(_))));
}
