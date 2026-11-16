use super::provider::PROVIDER_META;
use super::*;

#[test]
fn provider_meta_table_is_indexed_by_discriminant() {
    for (idx, provider) in LlmProvider::ALL.iter().enumerate() {
        assert_eq!(PROVIDER_META[idx].provider, *provider);
        assert_eq!(provider.meta().provider, *provider);
    }
}

#[test]
fn provider_ids_round_trip_case_insensitively() {
    for provider in LlmProvider::ALL {
        assert_eq!(LlmProvider::from_id(provider.id_str()), provider);
        assert_eq!(
            LlmProvider::from_id(&provider.id_str().to_uppercase()),
            provider
        );
    }
    assert_eq!(LlmProvider::from_id("unknown"), LlmProvider::Gemini);
}

#[test]
fn provider_metadata_matches_published_values() {
    let expected = [
        ("gemini", "Google Gemini", "gemini-1.5-flash", 3),
        ("openai", "OpenAI", "gpt-4o-mini", 3),
        ("anthropic", "Anthropic", "claude-3-5-sonnet-20241022", 2),
        ("openrouter", "OpenRouter", "google/gemini-flash-1.5", 3),
        ("groq", "Groq", "llama-3.3-70b-versatile", 2),
        ("custom", "Custom Endpoint", "default-model", 1),
    ];
    for (provider, (id, label, model, presets)) in LlmProvider::ALL.iter().zip(expected) {
        assert_eq!(provider.id_str(), id);
        assert_eq!(provider.to_string(), label);
        assert_eq!(provider.default_model(), model);
        assert_eq!(provider.preset_models().len(), presets);
        assert_eq!(provider.preset_models()[0], model);
    }
}

#[test]
fn default_settings_values() {
    let settings = LlmSettings::default();
    assert_eq!(settings.provider, LlmProvider::Gemini);
    assert_eq!(settings.model, "gemini-1.5-flash");
    assert!(settings.api_key.is_empty() && settings.custom_endpoint.is_empty());
    assert_eq!(settings.temperature, 0.7);
    assert_eq!(settings.max_tokens, 300);
}

#[tokio::test]
async fn generate_without_api_key_fails_fast() {
    let client = LlmClient::new(LlmSettings {
        api_key: "   ".to_string(),
        ..LlmSettings::default()
    });
    assert!(!client.has_api_key());
    let err = client.generate("sys", "user").await.unwrap_err();
    assert_eq!(
        err,
        "API key is not configured. Please open Settings to set your LLM API Key."
    );
}

#[test]
fn settings_persist_round_trip_and_blank_model_falls_back() {
    use super::settings::KEY_MODEL;
    let db = crate::storage::Database::in_memory().expect("in-memory db");
    let saved = LlmSettings {
        provider: LlmProvider::Groq,
        api_key: "test-key".to_string(),
        model: "mixtral-8x7b-32768".to_string(),
        custom_endpoint: "http://localhost:9000".to_string(),
        temperature: 0.25,
        max_tokens: 512,
    };
    saved.save_to_db(&db).expect("save settings");

    let loaded = LlmSettings::load_from_db(&db);
    assert_eq!(loaded.provider, LlmProvider::Groq);
    assert_eq!(loaded.api_key, "test-key");
    assert_eq!(loaded.model, "mixtral-8x7b-32768");
    assert_eq!(loaded.custom_endpoint, "http://localhost:9000");
    assert_eq!(loaded.temperature, 0.25);
    assert_eq!(loaded.max_tokens, 512);

    db.set_setting(KEY_MODEL, "  ").expect("blank model");
    assert_eq!(LlmSettings::load_from_db(&db).model, "llama-3.3-70b-versatile");
}
