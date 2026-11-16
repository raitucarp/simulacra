//! Unit tests of the agent editor field rules that guard against draft / data loss.

use super::editor_field::EditorTextField as F;
use crate::domain::Profile;

fn sample() -> Profile {
    let mut profile = Profile::new_agent("agent_test", "Sokrates", "@sokrates", "Philosopher");
    profile.config.primary_language = "Greek".to_string();
    profile.avatar_path = Some("old.png".to_string());
    profile
}

#[test]
fn all_lists_every_field_in_declaration_order() {
    // `AgentEditorFields::slot` indexes its table by discriminant.
    for (idx, field) in F::ALL.into_iter().enumerate() {
        assert_eq!(field as usize, idx, "{field:?} is out of order");
    }
}

#[test]
fn empty_required_fields_keep_the_stored_value() {
    let mut profile = sample();
    for field in [F::Name, F::Handle, F::PrimaryLanguage] {
        field.apply(&mut profile, "   ");
    }
    assert_eq!(profile, sample());
}

#[test]
fn handle_gets_the_at_prefix_once() {
    let mut profile = sample();
    F::Handle.apply(&mut profile, " plato ");
    assert_eq!(profile.handle, "@plato");
    F::Handle.apply(&mut profile, "@aristotle");
    assert_eq!(profile.handle, "@aristotle");
}

#[test]
fn avatar_path_strips_quotes_and_clears_when_empty() {
    let mut profile = sample();
    F::AvatarPath.apply(&mut profile, "\"C:\\img\\me.png\"");
    assert_eq!(profile.avatar_path.as_deref(), Some("C:\\img\\me.png"));
    F::AvatarPath.apply(&mut profile, "  ");
    assert_eq!(profile.avatar_path, None);
}

#[test]
fn short_fields_trim_and_long_fields_keep_whitespace() {
    let mut profile = sample();
    F::Occupation.apply(&mut profile, "  Teacher  ");
    F::Bio.apply(&mut profile, "  line one\n");
    assert_eq!(profile.config.occupation, "Teacher");
    assert_eq!(profile.config.bio, "  line one\n");
}

#[test]
fn display_then_apply_round_trips_every_field() {
    let original = sample();
    let mut copy = original.clone();
    for field in F::ALL {
        field.apply(&mut copy, &field.display_value(&original));
    }
    assert_eq!(copy, original, "unchanged inputs must not modify the profile");
}

mod settings_draft {
    use super::super::settings_draft::SettingsDraft;
    use crate::agent::{LlmProvider, LlmSettings};

    fn saved() -> LlmSettings {
        LlmSettings { api_key: "saved-key".into(), ..LlmSettings::default() }
    }

    fn other_provider(current: LlmProvider) -> LlmProvider {
        *LlmProvider::ALL.iter().find(|p| **p != current).expect("several providers")
    }

    #[test]
    fn picks_stay_in_the_draft_until_applied() {
        let saved = saved();
        let mut draft = SettingsDraft::default();
        let target = other_provider(saved.provider);

        let model = draft.select_provider(&saved, target);

        assert_eq!(model, target.default_model());
        assert!(draft.is_dirty());
        assert_eq!(draft.view(&saved).provider, target, "the modal shows the pick");
        assert_eq!(saved.provider, LlmSettings::default().provider, "persisted settings untouched");
    }

    #[test]
    fn apply_merges_typed_fields_and_clears_the_draft() {
        let saved = saved();
        let mut draft = SettingsDraft::default();
        let target = other_provider(saved.provider);
        draft.select_provider(&saved, target);
        draft.select_model(&saved, "preset-model");

        let typed = ("new-key".to_string(), "typed-model".to_string(), "http://e".to_string());
        let applied = draft.apply(&saved, typed);

        assert_eq!(applied.provider, target);
        assert_eq!((applied.api_key.as_str(), applied.model.as_str()), ("new-key", "typed-model"));
        assert_eq!(applied.custom_endpoint, "http://e");
        assert!(!draft.is_dirty());
    }

    #[test]
    fn empty_typed_model_falls_back_to_the_provider_default() {
        let saved = saved();
        let mut draft = SettingsDraft::default();
        let applied = draft.apply(&saved, ("k".into(), "  ".into(), String::new()));
        assert_eq!(applied.model, saved.provider.default_model());
    }

    #[test]
    fn discard_reverts_to_the_saved_settings() {
        let saved = saved();
        let mut draft = SettingsDraft::default();
        draft.select_provider(&saved, other_provider(saved.provider));
        draft.discard();
        assert!(!draft.is_dirty());
        assert_eq!(draft.view(&saved).provider, saved.provider);
        assert_eq!(draft.view(&saved).model, saved.model);
    }
}
