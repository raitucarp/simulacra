use super::*;
use crate::agent::credentials::{Credential, CredentialKind, CredentialStore};
use crate::agent::LlmSettings;
use crate::domain::embedding::FailureKind;
use crate::domain::preferences::{EmbeddingProvider, Preferences};

fn store(kind: CredentialKind, secret: &str, endpoint: &str) -> CredentialStore {
    CredentialStore { items: vec![Credential::new(kind, "test", secret, endpoint)] }
}

#[test]
fn gemini_uses_the_stored_key_and_its_endpoint_override() {
    let prefs = Preferences::default();
    let config = EmbeddingConfig::resolve(&prefs, &store(CredentialKind::Gemini, "k-1234", ""), &LlmSettings::default());
    assert_eq!(config.provider, ProviderKind::Gemini);
    assert_eq!(config.model, "gemini-embedding-001");
    assert_eq!(config.endpoint, "https://generativelanguage.googleapis.com/v1beta/models");
    assert_eq!(config.api_key, "k-1234");
    assert_eq!(config.dimensions, 768);
    assert_eq!(config.readiness(), Ok(()));
    assert_eq!(config.encoding(), TaskEncoding::Field);
    let mocked = EmbeddingConfig::resolve(&prefs, &store(CredentialKind::Gemini, "k-1234", "http://127.0.0.1:9/"), &LlmSettings::default());
    assert_eq!(mocked.endpoint, "http://127.0.0.1:9", "the credential endpoint overrides the base URL");
    assert!(!format!("{mocked:?}").contains("k-1234"), "Debug hides the key");
}

#[test]
fn a_missing_gemini_key_is_reported_before_any_request() {
    let config = EmbeddingConfig::resolve(&Preferences::default(), &CredentialStore::default(), &LlmSettings::default());
    assert_eq!(config.readiness(), Err(FailureKind::MissingKey));
    let legacy = LlmSettings { api_key: "legacy-key".into(), ..LlmSettings::default() };
    let config = EmbeddingConfig::resolve(&Preferences::default(), &CredentialStore::default(), &legacy);
    assert_eq!(config.api_key, "legacy-key", "the global Gemini key is a fallback");
}

#[test]
fn the_local_server_needs_no_key_and_follows_the_endpoint_setting() {
    let mut prefs = Preferences::default();
    prefs.embeddings = prefs.embeddings.with_provider(EmbeddingProvider::LocalServer);
    assert_eq!(prefs.embeddings.model, "embeddinggemma", "the provider default follows the switch");
    let config = EmbeddingConfig::resolve(&prefs, &CredentialStore::default(), &LlmSettings::default());
    assert_eq!((config.provider, config.endpoint.as_str()), (ProviderKind::LocalServer, "http://localhost:11434"));
    assert_eq!(config.readiness(), Ok(()));
    assert_eq!(config.encoding(), TaskEncoding::Prefix);
    prefs.embeddings.endpoint = "http://10.0.0.5:8080/".into();
    let config = EmbeddingConfig::resolve(&prefs.clone().validated(), &CredentialStore::default(), &LlmSettings::default());
    assert_eq!(config.endpoint, "http://10.0.0.5:8080");
    prefs.embeddings.model = "my-model".into();
    assert_eq!(prefs.embeddings.clone().with_provider(EmbeddingProvider::Gemini).model, "my-model", "a typed model stays");
}

#[test]
fn the_signature_changes_with_provider_dimensions_and_encoding() {
    let gemini = config(ProviderKind::Gemini, "gemini-embedding-001", "x");
    let shorter = EmbeddingConfig { dimensions: 256, ..gemini.clone() };
    let newer = EmbeddingConfig { model: "gemini-embedding-2".into(), ..gemini.clone() };
    let local = EmbeddingConfig { provider: ProviderKind::LocalServer, ..gemini.clone() };
    let signatures = [gemini.signature(), shorter.signature(), newer.signature(), local.signature()];
    let unique: std::collections::HashSet<_> = signatures.iter().collect();
    assert_eq!(unique.len(), 4, "{signatures:?}");
    assert_eq!(gemini.signature(), "gemini|768|field");
}

#[test]
fn embedding_prefs_clamp_dimensions_and_default_blank_models_per_provider() {
    use crate::domain::preferences::{EmbeddingPrefs, DIMENSIONS};
    let prefs = EmbeddingPrefs { dimensions: 5, model: " ".into(), provider: EmbeddingProvider::LocalServer, endpoint: "  ".into(), ..EmbeddingPrefs::default() };
    let clean = prefs.validated();
    assert_eq!(clean.dimensions as i64, DIMENSIONS.min);
    assert_eq!(clean.model, "embeddinggemma", "a blank model is the provider's default");
    assert_eq!(clean.endpoint_or_default(), "http://localhost:11434");
    assert_eq!(EmbeddingPrefs::default().dimensions, 768);
}
