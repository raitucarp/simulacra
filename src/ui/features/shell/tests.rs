//! Unit tests of the shell's settings draft.

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

mod credential_form {
    use super::super::credential_state::{credential_from_form, CredentialEdit};
    use crate::agent::credentials::{Credential, CredentialKind};

    fn typed(label: &str, secret: &str) -> (String, String, String) {
        (label.to_string(), secret.to_string(), "http://x.test".to_string())
    }

    #[test]
    fn new_credentials_get_an_id_and_a_default_label() {
        let edit = CredentialEdit { id: None, kind: CredentialKind::Tavily };
        let made = credential_from_form(&edit, None, typed("", "tv-1"));
        assert!(made.id.starts_with("cred_"));
        assert_eq!((made.label.as_str(), made.secret.as_str(), made.endpoint.as_str()), ("Tavily", "tv-1", "http://x.test"));
    }

    #[test]
    fn editing_with_a_blank_key_keeps_the_stored_secret() {
        let stored = Credential::new(CredentialKind::OpenAi, "Old", "sk-stored", "");
        let edit = CredentialEdit { id: Some(stored.id.clone()), kind: CredentialKind::OpenRouter };
        let kept = credential_from_form(&edit, Some(&stored), typed("Renamed", ""));
        assert_eq!((kept.id.as_str(), kept.secret.as_str(), kept.kind), (stored.id.as_str(), "sk-stored", CredentialKind::OpenRouter));
        let replaced = credential_from_form(&edit, Some(&stored), typed("Renamed", "sk-new"));
        assert_eq!(replaced.secret, "sk-new");
    }
}
