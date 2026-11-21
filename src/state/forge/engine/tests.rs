use super::mutate::{self, A1111_DEFAULT_URL, COMFY_DEFAULT_URL};
use super::*;
use crate::agent::credentials::{Credential, CredentialKind, CredentialStore};
use crate::agent::llm::LlmProvider;
use crate::domain::engine::{
    EngineConfig, ImageEngine, ImageEngineKind, ImageTrigger, OutputKind, OwnModel, RequestKind, SchemaPreset, SdFlavor, TextModel,
    TextSource, ToolGrant, ToolKind, TriggerKind, REQUEST_PLACEHOLDERS,
};
use crate::domain::traits::OptionSet;

fn store_with(kind: CredentialKind, secret: &str) -> (CredentialStore, String) {
    let cred = Credential::new(kind, "Work key", secret, "");
    let id = cred.id.clone();
    (CredentialStore { items: vec![cred] }, id)
}

fn texts() -> SampleTexts {
    SampleTexts {
        content: "Hello".into(),
        image_prompt: "a lamp".into(),
        mood: "calm".into(),
        hashtags: vec!["Light".into()],
        thread_parts: vec!["More".into()],
        other: "x".into(),
    }
}

#[test]
fn sections_round_trip_their_issue_fields() {
    for section in EngineSection::ALL {
        assert_eq!(EngineSection::of_field(section.issue_field()), Some(section));
        assert_eq!(section.label_key(), format!("forge.ui.engine.section.{}", section.key()));
    }
    assert_eq!(EngineSection::of_field("engine"), None);
    assert_eq!(EngineSection::of_field("name"), None);
}

#[test]
fn switching_variants_keeps_what_still_applies() {
    let mut engine = EngineConfig::default();
    mutate::set_text_source(&mut engine, TextSource::Own);
    let TextModel::Own(own) = &mut engine.text else { panic!("own model") };
    own.temperature = 1.5;
    mutate::set_provider(&mut engine, LlmProvider::Anthropic);
    let TextModel::Own(own) = &engine.text else { panic!("own model") };
    assert_eq!((own.provider, own.temperature), (LlmProvider::Anthropic, 1.5));
    assert_eq!(own.model, LlmProvider::Anthropic.default_model());
    mutate::set_text_source(&mut engine, TextSource::UseGlobal);
    assert_eq!(engine.text, TextModel::UseGlobal);

    mutate::set_request_kind(&mut engine, RequestKind::Custom);
    assert_eq!(RequestKind::of(&engine.request), RequestKind::Custom);
    mutate::set_output_kind(&mut engine, OutputKind::JsonSchema);
    mutate::apply_preset(&mut engine, SchemaPreset::Thread);
    assert_eq!(engine.structured().map(|s| s.preset), Some(SchemaPreset::Thread));
    mutate::edit_schema(&mut engine, "{}".into());
    assert_eq!(engine.structured().map(|s| s.preset), Some(SchemaPreset::Custom));
}

#[test]
fn image_engine_defaults_and_trigger_percent() {
    let mut engine = EngineConfig::default();
    mutate::set_image_kind(&mut engine, ImageEngineKind::LocalSd);
    assert_eq!(engine.image.engine, ImageEngine::LocalSd { flavor: SdFlavor::ComfyUi, url: COMFY_DEFAULT_URL.into() });
    mutate::set_sd_flavor(&mut engine, SdFlavor::A1111);
    assert_eq!(engine.image.engine, ImageEngine::LocalSd { flavor: SdFlavor::A1111, url: A1111_DEFAULT_URL.into() });
    if let ImageEngine::LocalSd { url, .. } = &mut engine.image.engine {
        *url = "http://gpu-box:9000".into();
    }
    mutate::set_sd_flavor(&mut engine, SdFlavor::ComfyUi);
    assert!(matches!(&engine.image.engine, ImageEngine::LocalSd { url, .. } if url == "http://gpu-box:9000"), "a typed URL is kept");
    engine.image.trigger = ImageTrigger::Chance { percent: 70 };
    mutate::set_trigger(&mut engine, TriggerKind::Always);
    mutate::set_trigger(&mut engine, TriggerKind::Chance);
    assert_eq!(engine.image.trigger, ImageTrigger::Chance { percent: crate::domain::engine::DEFAULT_IMAGE_CHANCE });
}

#[test]
fn tool_grants_follow_the_chosen_set() {
    let mut engine = EngineConfig::default();
    mutate::set_tools(&mut engine, &OptionSet::new([ToolKind::WebSearch, ToolKind::VaultSearch]));
    assert_eq!(mutate::granted(&engine), OptionSet::new([ToolKind::VaultSearch, ToolKind::WebSearch]));
    mutate::web_search_mut(&mut engine).expect("search").backend = crate::domain::engine::SearchBackend::Tavily;
    mutate::set_tools(&mut engine, &OptionSet::new([ToolKind::WebSearch, ToolKind::CustomHttp]));
    assert!(matches!(&engine.tools[0], ToolGrant::WebSearch(s) if s.backend == crate::domain::engine::SearchBackend::Tavily));
    mutate::add_custom_tool(&mut engine);
    mutate::custom_tool_mut(&mut engine, 1).expect("second").name = "second".into();
    mutate::remove_custom_tool(&mut engine, 0);
    assert_eq!(engine.tools.len(), 2);
    assert_eq!(mutate::custom_tool_mut(&mut engine, 0).map(|t| t.name.clone()), Some("second".into()));
}

#[test]
fn readiness_reports_chosen_automatic_missing_and_gone_keys() {
    let (store, id) = store_with(CredentialKind::OpenAi, "sk-test-123456789");
    assert_eq!(credential_status(&store, Some(&id), Some(CredentialKind::OpenAi)), CredentialStatus::Ready { label: "Work key".into(), chosen: true });
    assert_eq!(credential_status(&store, None, Some(CredentialKind::OpenAi)), CredentialStatus::Ready { label: "Work key".into(), chosen: false });
    assert_eq!(credential_status(&store, None, Some(CredentialKind::Gemini)), CredentialStatus::Missing);
    assert_eq!(credential_status(&store, Some("cred_gone"), None), CredentialStatus::Gone);
    assert_eq!(credential_status(&store, None, None), CredentialStatus::NotNeeded);
    let (blank, blank_id) = store_with(CredentialKind::OpenAi, " ");
    assert_eq!(credential_status(&blank, Some(&blank_id), Some(CredentialKind::OpenAi)), CredentialStatus::Missing);
    assert!(!CredentialStatus::Missing.is_ready());
}

#[test]
fn json_helpers_fill_placeholders_and_insert_tokens() {
    assert_eq!(body_error(r#"{"m": {{model}}, "x": {{messages}}}"#, &REQUEST_PLACEHOLDERS), None);
    assert!(body_error(r#"{"m": "{{model}}"#, &REQUEST_PLACEHOLDERS).is_some());
    assert!(body_error(r#"{"m": {{nope}}}"#, &REQUEST_PLACEHOLDERS).is_some(), "unknown placeholders stay and break JSON");
    assert_eq!(object_problem(r#"{"a": 1}"#), None);
    assert_eq!(object_problem("[1]"), Some(JsonProblem::NotAnObject));
    assert!(matches!(object_problem("{"), Some(JsonProblem::Syntax(_))));
    assert_eq!(insert_token("", "{{user}}"), "{{user}}");
    assert_eq!(insert_token("a", "{{user}}"), "a {{user}}");
    assert_eq!(insert_token("a\n", "{{user}}"), "a\n{{user}}");
}

#[test]
fn the_sample_answer_renders_through_the_mapping() {
    let structured = crate::domain::engine::StructuredOutput::from_preset(SchemaPreset::Illustrated);
    let sample = sample_answer(&structured, &texts());
    let draft = preview(&structured, &sample).expect("sample maps");
    assert_eq!(draft.content, "Hello");
    assert_eq!(draft.image_prompt.as_deref(), Some("a lamp"));
    assert_eq!(draft.body(), "Hello #Light");
    let thread = crate::domain::engine::StructuredOutput::from_preset(SchemaPreset::Thread);
    assert_eq!(preview(&thread, &sample_answer(&thread, &texts())).expect("thread").thread_parts, vec!["More".to_string()]);
    assert_eq!(preview(&structured, r#"{"other": 1}"#), Err(PreviewError::NothingToPost));
    assert!(matches!(preview(&structured, "{"), Err(PreviewError::InvalidJson(_))));
    let custom = crate::domain::engine::StructuredOutput { schema: r#"{"properties": {"n": {"type": "integer"}, "tags": {"type": "array", "items": {"type": "string"}}}}"#.into(), ..structured };
    assert_eq!(serde_json::from_str::<serde_json::Value>(&sample_answer(&custom, &texts())).expect("json"), serde_json::json!({"n": 1, "tags": ["x"]}));
}

#[test]
fn the_tint_switch_is_off_by_default_and_survives_a_preset_change() {
    let mut engine = EngineConfig::default();
    mutate::set_may_tint(&mut engine, true);
    assert!(engine.structured().is_none(), "free text has no tint switch");
    mutate::set_output_kind(&mut engine, OutputKind::JsonSchema);
    assert_eq!(engine.structured().map(|s| s.may_tint), Some(false));
    mutate::set_may_tint(&mut engine, true);
    mutate::apply_preset(&mut engine, SchemaPreset::Expressive);
    assert_eq!(engine.structured().map(|s| (s.preset, s.may_tint)), Some((SchemaPreset::Expressive, true)));
    mutate::set_may_tint(&mut engine, false);
    assert_eq!(engine.structured().map(|s| s.may_tint), Some(false));
}

#[test]
fn own_model_default_matches_the_provider() {
    let own = OwnModel::for_provider(LlmProvider::Groq);
    assert_eq!(own.model_or_default(), LlmProvider::Groq.default_model());
}
