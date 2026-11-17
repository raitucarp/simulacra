use super::*;
use crate::domain::{MbtiType, ProfileRole};

#[test]
fn new_drafts_are_clean_blank_and_scoped() {
    let draft = agent();
    assert!(!draft.is_dirty());
    assert!(draft.profile().name.is_empty());
    assert!(draft.id().as_str().starts_with("agent_"));
    assert_eq!(draft.profile().network_id.as_ref().map(|n| n.as_str()), Some(NET));
    assert_eq!(draft.step(), ForgeStep::Origin);
    assert!(draft.progress(ForgeStep::Origin).visited);
    let persona = persona();
    assert!(persona.id().as_str().starts_with("user_"));
    assert_eq!(persona.profile().role, ProfileRole::UserAlt);
    assert_eq!(persona.kind(), ForgeKind::Persona);
}

#[test]
fn edits_mark_dirty_and_clamp_to_field_ranges() {
    let mut draft = agent();
    assert!(draft.edit(DraftEdit::Score(ScoreField::Openness, 250)));
    assert!(draft.is_dirty());
    assert_eq!(draft.profile().config.openness, 100);
    draft.edit(DraftEdit::Hour(HourField::ActiveHoursEnd, 30));
    assert_eq!(draft.profile().config.active_hours_end, 23);
    draft.edit(DraftEdit::Count(CountField::Age, u16::MAX));
    assert_eq!(draft.profile().config.age, fields::MAX_AGE);
    draft.edit(DraftEdit::Trait(TraitValue::Mbti(MbtiType::Enfp)));
    assert_eq!(draft.profile().config.mbti, MbtiType::Enfp);
    draft.edit(DraftEdit::OptionalText(OptionalTextField::AvatarPath, Some("a.png".into())));
    assert_eq!(draft.profile().avatar_path.as_deref(), Some("a.png"));
    let portrait = Some(crate::domain::portrait::AvatarMode::Pcg(crate::domain::portrait::PortraitSpec::random(1)));
    assert!(draft.edit(DraftEdit::Avatar(AvatarField::Avatar, portrait.clone())));
    assert_eq!(draft.profile().config.avatar, portrait);
    draft.edit(DraftEdit::Toggle(ToggleField::InjectNetworkGrounding, false));
    assert!(!draft.profile().config.inject_network_grounding);
}

#[test]
fn edits_that_change_nothing_leave_no_history() {
    let mut draft = agent();
    let same = draft.profile().config.openness;
    assert!(!draft.edit(DraftEdit::Score(ScoreField::Openness, same)));
    assert!(!draft.can_undo());
    assert!(!draft.is_dirty());
}

#[test]
fn edit_reports_its_path_and_step() {
    let edit = DraftEdit::Trait(TraitValue::EmojiUsage(crate::domain::EmojiUsage::Never));
    assert_eq!(edit.path(), "emoji_usage");
    assert_eq!(edit.step(), ForgeStep::Voice);
    assert_eq!(DraftEdit::Text(TextField::Bio, String::new()).step(), ForgeStep::Origin);
}

#[test]
fn discard_restores_the_original() {
    let mut draft = named(agent(), "Theo", "theo");
    assert!(draft.is_dirty());
    draft.discard();
    assert!(!draft.is_dirty());
    assert!(!draft.can_undo());
    assert!(draft.profile().name.is_empty());
}

#[test]
fn commit_normalizes_and_becomes_the_new_original() {
    let mut draft = named(agent(), "  theo Vale ", " theo_v ");
    draft.edit(DraftEdit::Text(TextField::Description, " Gadfly  ".into()));
    let profile = draft.commit();
    assert_eq!(profile.name, "theo Vale");
    assert_eq!(profile.handle, "@theo_v");
    assert_eq!(profile.avatar_initial, "T");
    assert_eq!(profile.config.description, "Gadfly");
    assert_eq!(profile.role, ProfileRole::Agent { persona_type: "Gadfly".into() });
    assert!(!draft.is_dirty());
    assert!(!draft.can_undo());
    assert_eq!(draft.profile(), &profile);
}

#[test]
fn commit_keeps_a_customized_initial_and_existing_handle_prefix() {
    let mut draft = named(agent(), "Theo", "@theo");
    draft.edit(DraftEdit::Text(TextField::AvatarInitial, "Θ".into()));
    let profile = draft.commit();
    assert_eq!(profile.avatar_initial, "Θ");
    assert_eq!(profile.handle, "@theo");
}

#[test]
fn edit_existing_tracks_kind_from_the_role() {
    let profile = existing("agent_x", "Ada", "@ada");
    let draft = AgentDraft::edit_existing(profile.clone());
    assert_eq!(draft.kind(), ForgeKind::Agent);
    assert_eq!(draft.original(), &profile);
    assert!(!draft.is_dirty());
}

#[test]
fn navigation_respects_the_kind_and_tracks_progress() {
    let mut draft = persona();
    assert!(!draft.go_to(ForgeStep::Mind), "personas have no Mind step");
    assert_eq!(draft.step(), ForgeStep::Origin);
    assert!(draft.advance());
    assert_eq!(draft.step(), ForgeStep::Embodiment);
    assert!(draft.progress(ForgeStep::Origin).complete);
    assert!(draft.progress(ForgeStep::Embodiment).visited);
    assert!(draft.back());
    assert_eq!(draft.step(), ForgeStep::Origin);
    assert!(!draft.back());
    assert!(draft.go_to(ForgeStep::Review));
    assert!(!draft.advance());
}

#[test]
fn sync_completion_follows_validation_errors() {
    let mut draft = agent();
    draft.go_to(ForgeStep::Voice);
    let issues = validate(&draft, &[]);
    draft.sync_completion(&issues);
    assert!(!draft.progress(ForgeStep::Origin).complete, "unnamed draft has Origin errors");
    assert!(draft.progress(ForgeStep::Voice).complete);
    assert!(!draft.progress(ForgeStep::Mind).complete, "never visited");
}

#[test]
fn autosave_round_trips_the_draft() {
    let mut draft = named(agent(), "Theo", "@theo");
    draft.edit(DraftEdit::Score(ScoreField::Extraversion, 12));
    draft.go_to(ForgeStep::Attributes);
    let json = draft.to_autosave();
    let restored = AgentDraft::from_autosave(&json).expect("restore autosave");
    assert_eq!(restored.profile(), draft.profile());
    assert_eq!(restored.original(), draft.original());
    assert_eq!(restored.kind(), ForgeKind::Agent);
    assert_eq!(restored.step(), ForgeStep::Attributes);
    assert_eq!(restored.progress(ForgeStep::Attributes), draft.progress(ForgeStep::Attributes));
    assert!(restored.is_dirty());
    assert!(!restored.can_undo(), "history is not persisted");
}

#[test]
fn autosave_rejects_garbage_and_future_versions() {
    assert!(matches!(AgentDraft::from_autosave("not json"), Err(AutosaveError::Malformed(_))));
    let json = agent().to_autosave().replacen(&format!("\"version\":{AUTOSAVE_VERSION}"), "\"version\":999", 1);
    assert_eq!(AgentDraft::from_autosave(&json).err(), Some(AutosaveError::UnsupportedVersion(999)));
}

#[test]
fn autosave_repairs_navigation_that_no_longer_fits() {
    let json = persona().to_autosave().replace("\"step\":\"Origin\"", "\"step\":\"Mind\"");
    let restored = AgentDraft::from_autosave(&json).expect("restore");
    assert_eq!(restored.step(), ForgeStep::Origin);
}
