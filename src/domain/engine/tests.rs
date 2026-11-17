use super::*;
use crate::agent::LlmProvider;
use crate::domain::AgentConfig;
use serde_json::json;

#[test]
fn the_default_engine_is_the_classic_one() {
    let engine = EngineConfig::default();
    assert!(engine.is_classic());
    assert_eq!(engine.text, TextModel::UseGlobal);
    assert_eq!(engine.request, RequestMode::Native);
    assert_eq!(engine.output, OutputMode::FreeText);
    assert_eq!(engine.image.engine, ImageEngine::None);
    assert!(!engine.image.is_enabled());
    assert!(engine.tools.is_empty());
    assert_eq!(engine.tool_steps(), DEFAULT_MAX_TOOL_STEPS);
    assert!(engine.structured().is_none() && engine.credential_ids().is_empty());
}

fn rich_engine() -> EngineConfig {
    EngineConfig {
        text: TextModel::Own(OwnModel { credential: Some("cred_a".into()), ..OwnModel::for_provider(LlmProvider::Anthropic) }),
        request: RequestMode::Custom(CustomRequest { url: "http://x.test/chat".into(), credential: Some("cred_b".into()), ..CustomRequest::default() }),
        output: OutputMode::JsonSchema(StructuredOutput::from_preset(SchemaPreset::Thread)),
        image: ImageEngineConfig {
            engine: ImageEngine::LocalSd { flavor: SdFlavor::A1111, url: "http://127.0.0.1:7860".into() },
            trigger: ImageTrigger::Chance { percent: 30 },
            credential: Some("cred_c".into()),
            ..ImageEngineConfig::default()
        },
        tools: vec![
            ToolGrant::VaultSearch,
            ToolGrant::ReadNetwork,
            ToolGrant::WebSearch(WebSearchConfig { backend: SearchBackend::Brave, credential: Some("cred_d".into()) }),
            ToolGrant::WebFetch,
            ToolGrant::CustomHttp(HttpToolDef { name: "weather".into(), credential: Some("cred_e".into()), ..HttpToolDef::default() }),
        ],
        max_tool_steps: 99,
    }
}

#[test]
fn engines_round_trip_through_json_with_stable_tags() {
    let engine = rich_engine();
    let value = serde_json::to_value(&engine).expect("serialize");
    assert_eq!(value["text"]["source"], "own");
    assert_eq!(value["text"]["provider"], "anthropic");
    assert_eq!(value["request"]["mode"], "custom");
    assert_eq!(value["request"]["method"], "POST");
    assert_eq!(value["output"]["format"], "json_schema");
    assert_eq!(value["output"]["preset"], "thread");
    assert_eq!(value["image"]["engine"]["engine"], "local_sd");
    assert_eq!(value["image"]["engine"]["flavor"], "a1111");
    assert_eq!(value["image"]["trigger"], json!({ "when": "chance", "percent": 30 }));
    assert_eq!(value["tools"][2], json!({ "tool": "web_search", "backend": "brave", "credential": "cred_d" }));
    assert_eq!(value["tools"][4]["tool"], "custom_http");
    let back: EngineConfig = serde_json::from_value(value).expect("deserialize");
    assert_eq!(back, engine);
    assert_eq!(back.tool_steps(), MAX_TOOL_STEPS_LIMIT, "clamped");
    assert_eq!(back.credential_ids(), vec!["cred_a", "cred_c", "cred_b", "cred_d", "cred_e"]);
}

#[test]
fn partial_engine_json_fills_defaults() {
    let engine: EngineConfig = serde_json::from_value(json!({ "text": { "source": "own", "provider": "groq" }, "tools": [{ "tool": "vault_search" }] }))
        .expect("partial engine");
    let TextModel::Own(own) = &engine.text else { panic!("own model") };
    assert_eq!(own.provider, LlmProvider::Groq);
    assert_eq!(own.model, "", "a missing model stays blank");
    assert_eq!(own.model_or_default(), LlmProvider::Groq.default_model());
    assert_eq!(engine.tools, vec![ToolGrant::VaultSearch]);
    assert_eq!(engine.max_tool_steps, DEFAULT_MAX_TOOL_STEPS);
}

#[test]
fn classic_engines_are_left_out_of_agent_json() {
    let json = serde_json::to_value(AgentConfig::default()).expect("serialize");
    assert!(json.get("engine").is_none());
    let custom = AgentConfig { engine: EngineConfig { max_tool_steps: 2, ..EngineConfig::default() }, ..AgentConfig::default() };
    assert_eq!(serde_json::to_value(custom).expect("serialize")["engine"]["max_tool_steps"], 2);
}

#[test]
fn old_agent_json_without_an_engine_still_loads() {
    let cfg: AgentConfig = serde_json::from_str(r#"{"age": 44, "occupation": "Potter"}"#).expect("legacy config");
    assert_eq!(cfg.age, 44);
    assert!(cfg.engine.is_classic());
}

#[test]
fn a_malformed_engine_resets_only_the_engine() {
    let raw = json!({ "age": 61, "occupation": "Cartographer", "engine": { "text": { "source": "teleport" }, "tools": 7 } });
    let cfg: AgentConfig = serde_json::from_value(raw).expect("rest of the config survives");
    assert_eq!((cfg.age, cfg.occupation.as_str()), (61, "Cartographer"));
    assert!(cfg.engine.is_classic());
    let unknown_provider = json!({ "engine": { "text": { "source": "own", "provider": "skynet" } } });
    let cfg: AgentConfig = serde_json::from_value(unknown_provider).expect("config");
    assert!(cfg.engine.is_classic());
}

#[test]
fn agent_configs_with_engines_round_trip() {
    let cfg = AgentConfig { engine: rich_engine(), ..AgentConfig::default() };
    let json = serde_json::to_string(&cfg).expect("serialize");
    assert_eq!(serde_json::from_str::<AgentConfig>(&json).expect("deserialize"), cfg);
}

#[test]
fn dotted_paths_walk_objects_and_arrays() {
    let reply = json!({ "choices": [{ "message": { "content": "hi", "n": 3, "flag": true, "none": null } }] });
    assert_eq!(value_at(&reply, "choices.0.message.content"), Some(&json!("hi")));
    assert_eq!(value_text(&reply, "choices.0.message.content").as_deref(), Some("hi"));
    assert_eq!(value_text(&reply, "choices.0.message.n").as_deref(), Some("3"));
    assert_eq!(value_text(&reply, "choices.0.message.flag").as_deref(), Some("true"));
    assert_eq!(value_text(&reply, "choices.0.message.none"), None);
    assert_eq!(value_at(&reply, "choices.1.message"), None);
    assert_eq!(value_at(&reply, "choices.x"), None);
    assert_eq!(value_at(&reply, "choices.0.message.content.deeper"), None);
    assert_eq!(value_at(&reply, " "), Some(&reply));
    let object: serde_json::Value = serde_json::from_str(&value_text(&reply, "choices.0").expect("object text")).expect("json");
    assert_eq!(object, reply["choices"][0], "objects come back as compact JSON");
}

#[test]
fn every_preset_schema_is_an_object_schema_requiring_content() {
    for preset in SchemaPreset::ALL {
        let structured = StructuredOutput::from_preset(preset);
        let schema = structured.schema_value().unwrap_or_else(|| panic!("{preset:?} schema parses"));
        assert_eq!(schema["type"], "object", "{preset:?}");
        assert!(schema["required"].as_array().is_some_and(|r| r.contains(&json!("content"))), "{preset:?}");
        assert_eq!(structured.mapping, OutputMapping::top_level());
        assert!(!preset.key().is_empty());
    }
    let broken = StructuredOutput { schema: "{not json".into(), ..StructuredOutput::default() };
    assert!(broken.schema_value().is_none());
}

#[test]
fn image_settings_parse_sizes_and_triggers() {
    let config = ImageEngineConfig { engine: ImageEngine::OpenAi, size: "512 x 768".into(), ..ImageEngineConfig::default() };
    assert_eq!(config.dimensions(), Some((512, 768)));
    assert!(config.is_enabled());
    let never = ImageEngineConfig { trigger: ImageTrigger::Never, ..config.clone() };
    assert!(!never.is_enabled());
    assert_eq!(ImageEngineConfig { size: "large".into(), ..config }.dimensions(), None);
    assert_eq!(ImageTrigger::default(), ImageTrigger::WhenModelAsks);
}

#[test]
fn picker_keys_are_unique_and_match_variants() {
    let keys = |list: Vec<&str>| {
        let mut sorted = list.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), list.len(), "{list:?}");
    };
    keys(ToolKind::ALL.iter().map(|k| k.key()).collect());
    keys(ImageEngineKind::ALL.iter().map(|k| k.key()).collect());
    keys(SearchBackend::ALL.iter().map(|k| k.key()).collect());
    keys(SdFlavor::ALL.iter().map(|k| k.key()).collect());
    keys(SchemaPreset::ALL.iter().map(|k| k.key()).collect());
    assert_eq!(ToolGrant::WebFetch.kind(), ToolKind::WebFetch);
    assert_eq!(ImageEngine::Custom(CustomRequest::default()).kind(), ImageEngineKind::Custom);
    assert_eq!(REQUEST_PLACEHOLDERS.len(), 6);
    assert!(HttpMethod::Post.has_body() && !HttpMethod::Get.has_body());
}
