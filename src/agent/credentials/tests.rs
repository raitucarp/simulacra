use super::*;
use crate::agent::llm::{LlmProvider, LlmSettings};
use crate::storage::{Database, SettingKey};

fn global(provider: LlmProvider, key: &str) -> LlmSettings {
    LlmSettings { provider, api_key: key.into(), custom_endpoint: "http://local.test/v1".into(), ..LlmSettings::default() }
}

#[test]
fn the_legacy_global_key_migrates_into_the_default_credential_once() {
    let db = Database::in_memory().expect("db");
    let legacy = global(LlmProvider::OpenAi, " sk-legacy-123 ");
    legacy.save_to_db(&db).expect("legacy settings");
    let store = CredentialStore::load_from_db(&db, &legacy);
    let default = store.get(DEFAULT_CREDENTIAL_ID).expect("migrated");
    assert_eq!(default.kind, CredentialKind::OpenAi);
    assert_eq!(default.secret, "sk-legacy-123");
    assert_eq!(default.endpoint, "http://local.test/v1");
    assert!(db.get_setting(SettingKey::Credentials).expect("read").is_some(), "the migration is saved");

    let mut edited = store.clone();
    edited.upsert(Credential::new(CredentialKind::Brave, "Search", "brv-1", ""));
    edited.save_to_db(&db).expect("save");
    assert_eq!(CredentialStore::load_from_db(&db, &legacy), edited, "later loads read the row, no re-migration");
}

#[test]
fn a_fresh_vault_without_a_key_gets_an_empty_store() {
    let db = Database::in_memory().expect("db");
    let store = CredentialStore::load_from_db(&db, &LlmSettings::default());
    assert!(store.items.is_empty() && !store.any_secret());
    assert_eq!(db.get_setting(SettingKey::Credentials).expect("read").as_deref(), Some("[]"));
}

#[test]
fn a_malformed_row_loads_as_empty() {
    assert!(CredentialStore::from_json("{oops").items.is_empty());
    let parsed = CredentialStore::from_json(r#"[{"id":"c1","kind":"tavily","secret":"tv"}]"#);
    assert_eq!(parsed.items[0].kind, CredentialKind::Tavily);
    assert_eq!(parsed.items[0].label, "", "missing fields default");
}

#[test]
fn sync_default_mirrors_the_global_settings() {
    let mut store = CredentialStore::default();
    store.sync_default(&LlmSettings::default());
    assert!(store.items.is_empty(), "no key, nothing to mirror");
    store.sync_default(&global(LlmProvider::Anthropic, "sk-ant"));
    store.sync_default(&global(LlmProvider::Groq, "gsk"));
    assert_eq!(store.items.len(), 1);
    assert_eq!((store.items[0].kind, store.items[0].secret.as_str()), (CredentialKind::Groq, "gsk"));
    store.sync_default(&LlmSettings::default());
    assert_eq!(store.items[0].secret, "", "an existing default follows a cleared key");
}

#[test]
fn resolve_prefers_the_id_then_the_first_keyed_credential_of_the_kind() {
    let mut store = CredentialStore::default();
    let blank = Credential::new(CredentialKind::OpenAi, "Blank", "", "");
    let keyed = Credential::new(CredentialKind::OpenAi, "Keyed", "sk-2", "");
    let other = Credential::new(CredentialKind::Gemini, "G", "g-1", "");
    for c in [&blank, &keyed, &other] {
        store.upsert(c.clone());
    }
    assert_eq!(store.resolve(Some(&blank.id), CredentialKind::OpenAi), Some(&blank));
    assert_eq!(store.resolve(None, CredentialKind::OpenAi), Some(&keyed));
    assert_eq!(store.resolve(Some("missing"), CredentialKind::Gemini), Some(&other));
    assert_eq!(store.resolve(None, CredentialKind::Brave), None);
    assert_eq!(store.secrets(), vec!["sk-2".to_string(), "g-1".to_string()]);
    assert!(store.remove(&keyed.id) && !store.remove(&keyed.id));
    assert_eq!(store.resolve(None, CredentialKind::OpenAi), Some(&blank), "falls back to an unkeyed one");
}

#[test]
fn secrets_never_show_in_debug_or_masks() {
    let cred = Credential::new(CredentialKind::Http, "Tool", "super-secret-token-9876", "");
    let debug = format!("{cred:?}");
    assert!(!debug.contains("super-secret"), "{debug}");
    assert_eq!(cred.masked_secret(), "••••9876");
    assert_eq!(Credential::new(CredentialKind::Http, "", "short", "").masked_secret(), "••••");
    assert!(cred.id.starts_with("cred_"));
}

#[test]
fn kinds_map_to_providers_and_back() {
    for provider in LlmProvider::ALL {
        assert_eq!(CredentialKind::from_provider(provider).provider(), Some(provider));
    }
    assert_eq!(CredentialKind::Brave.provider(), None);
    assert_eq!(CredentialKind::OpenAi.label(), "OpenAI");
    let keys: std::collections::HashSet<&str> = CredentialKind::ALL.iter().map(|k| k.key()).collect();
    assert_eq!(keys.len(), CredentialKind::ALL.len());
}
