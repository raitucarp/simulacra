//! Custom request transports, plan resolution and image generation.

use super::{media_dir, openai_tool_call, store, tool_context, CRED_ID, PNG_1X1_BASE64, SECRET};
use crate::agent::credentials::CredentialStore;
use crate::agent::llm::{LlmProvider, LlmSettings};
use crate::agent::mock_server::{MockReply, MockServer};
use crate::agent::runtime::{run_engine, EnginePlan, Transport, GLOBAL_GATE_KEY};
use crate::domain::engine::{
    CustomRequest, EngineConfig, HeaderPair, ImageEngine, ImageEngineConfig, ImageTrigger, OutputMode, OwnModel, RequestMode,
    SchemaPreset, StructuredOutput, TextModel, ToolGrant,
};
use serde_json::json;

fn custom(url: String, body_template: &str, reply_path: &str) -> CustomRequest {
    CustomRequest {
        url,
        headers: vec![HeaderPair::new("X-Token", "{{api_key}}"), HeaderPair::new("X-Model", "{{model}}")],
        body_template: body_template.into(),
        reply_path: reply_path.into(),
        tool_call_path: "data.calls".into(),
        credential: Some(CRED_ID.into()),
        ..CustomRequest::default()
    }
}

fn custom_plan(request: CustomRequest, output: OutputMode, tools: Vec<ToolGrant>) -> EnginePlan {
    let engine = EngineConfig { request: RequestMode::Custom(request), output, tools, ..EngineConfig::default() };
    let global = LlmSettings { model: "global-model".into(), ..LlmSettings::default() };
    EnginePlan::resolve(&engine, &global, &store(LlmProvider::Custom, ""), media_dir())
}

#[tokio::test]
async fn custom_requests_fill_placeholders_and_read_the_reply_path() {
    let server = MockServer::start(vec![MockReply::json(json!({ "data": { "text": "{\"content\": \"Raw call works.\"}" } }))]);
    let template = r#"{"sys": {{system}}, "prompt": {{user}}, "model": {{model}}, "schema": {{schema}}, "n": 1}"#;
    let output = OutputMode::JsonSchema(StructuredOutput::from_preset(SchemaPreset::Post));
    let plan = custom_plan(custom(format!("{}/gen", server.url), template, "data.text"), output, Vec::new());
    assert!(matches!(plan.transport, Transport::Custom(_)) && plan.is_ready());
    assert_eq!(plan.gate_key, CRED_ID);
    let result = run_engine(&plan, "SYS \"quoted\"", "USER", None, 0).await.expect("custom call");
    assert_eq!(result.draft.content, "Raw call works.");
    let request = &server.requests()[0];
    assert_eq!((request.method.as_str(), request.path.as_str()), ("POST", "/gen"));
    assert_eq!(request.header("x-token"), Some(SECRET));
    assert_eq!(request.header("x-model"), Some("global-model"));
    let body = request.json();
    assert!(body["sys"].as_str().is_some_and(|s| s.starts_with("SYS \"quoted\"") && s.contains("OUTPUT FORMAT")));
    assert_eq!((body["prompt"].as_str(), body["model"].as_str(), body["n"].as_i64()), (Some("USER"), Some("global-model"), Some(1)));
    assert_eq!(body["schema"]["required"], json!(["content"]));
}

#[tokio::test]
async fn custom_requests_carry_tool_calls_and_messages() {
    let call = openai_tool_call("c9", "vault_search", json!({ "query": "monism" }));
    let calls = call["choices"][0]["message"]["tool_calls"].clone();
    let server = MockServer::start(vec![
        MockReply::json(json!({ "data": { "text": "", "calls": calls } })),
        MockReply::json(json!({ "data": { "text": "Answered after the tool." } })),
    ]);
    let template = r#"{"messages": {{messages}}, "tools": {{tools}}}"#;
    let plan = custom_plan(custom(server.url.clone(), template, "data.text"), OutputMode::FreeText, vec![ToolGrant::VaultSearch]);
    let result = run_engine(&plan, "S", "U", Some(tool_context()), 0).await.expect("custom loop");
    assert_eq!(result.draft.content, "Answered after the tool.");
    let requests = server.requests();
    assert_eq!(requests[0].json()["tools"][0]["function"]["name"], "vault_search");
    let messages = requests[1].json()["messages"].clone();
    assert_eq!(messages[3]["role"], "tool");
    assert_eq!(messages[3]["tool_call_id"], "c9");
}

#[tokio::test]
async fn invalid_custom_templates_and_empty_replies_fail_clearly() {
    let plan = custom_plan(custom("http://127.0.0.1:9/x".into(), "{\"a\": {{unknown}}}", "data.text"), OutputMode::FreeText, Vec::new());
    let error = run_engine(&plan, "S", "U", None, 0).await.expect_err("bad template");
    assert!(error.contains("not valid JSON"), "{error}");
    let server = MockServer::start(vec![MockReply::json(json!({ "other": 1 }))]);
    let plan = custom_plan(custom(server.url.clone(), "{}", "data.text"), OutputMode::FreeText, Vec::new());
    let error = run_engine(&plan, "S", "U", None, 0).await.expect_err("nothing at the path");
    assert!(error.contains("reply path"), "{error}");
    let unready = custom_plan(CustomRequest::default(), OutputMode::FreeText, Vec::new());
    assert!(!unready.is_ready());
}

#[test]
fn plans_resolve_credentials_gates_and_fallbacks() {
    let classic = EnginePlan::resolve(&EngineConfig::default(), &LlmSettings::default(), &CredentialStore::default(), media_dir());
    assert_eq!(classic.gate_key, GLOBAL_GATE_KEY);
    assert!(!classic.is_ready() && classic.image.is_none() && classic.structured.is_none());
    let own = EngineConfig { text: TextModel::Own(OwnModel::for_provider(LlmProvider::Anthropic)), ..EngineConfig::default() };
    let keyed = store(LlmProvider::Anthropic, "");
    let plan = EnginePlan::resolve(&own, &LlmSettings::default(), &keyed, media_dir());
    assert_eq!((plan.gate_key.as_str(), plan.is_ready(), plan.transport.provider()), (CRED_ID, true, LlmProvider::Anthropic));
    assert_eq!(EnginePlan::gate_of(&own, &LlmSettings::default(), &keyed), (CRED_ID.to_string(), true));
    let missing = EnginePlan::gate_of(&own, &LlmSettings::default(), &CredentialStore::default());
    assert_eq!(missing, ("provider:anthropic".to_string(), false));
    let global = LlmSettings { api_key: "g".into(), ..LlmSettings::default() };
    assert_eq!(EnginePlan::gate_of(&EngineConfig::default(), &global, &keyed), (GLOBAL_GATE_KEY.to_string(), true));
}

#[tokio::test]
async fn custom_image_engines_save_the_picture_into_the_media_folder() {
    let server = MockServer::start(vec![
        MockReply::json(super::openai_text(&json!({ "content": "A study in light.", "image_prompt": "lighthouse" }).to_string())),
        MockReply::json(json!({ "image": { "b64": format!("data:image/png;base64,{PNG_1X1_BASE64}") } })),
    ]);
    let image_request = CustomRequest {
        url: format!("{}/image", server.url),
        body_template: r#"{"prompt": {{prompt}}, "size": {{size}}}"#.into(),
        reply_path: "image.b64".into(),
        ..CustomRequest::default()
    };
    let engine = EngineConfig {
        output: OutputMode::JsonSchema(StructuredOutput::from_preset(SchemaPreset::Illustrated)),
        image: ImageEngineConfig { engine: ImageEngine::Custom(image_request), style_prefix: "ink wash".into(), trigger: ImageTrigger::WhenModelAsks, ..ImageEngineConfig::default() },
        ..EngineConfig::default()
    };
    let plan = super::plan(LlmProvider::OpenAi, &server.url, engine);
    let result = run_engine(&plan, "S", "U", None, 0).await.expect("engine runs");
    let path = result.image_path.clone().expect("image saved");
    assert!(path.ends_with(".png"), "{path}");
    assert!(std::fs::read(&path).expect("file").starts_with(&[0x89, b'P', b'N', b'G']));
    assert_eq!(server.requests()[1].json(), json!({ "prompt": "ink wash, lighthouse", "size": "1024x1024" }));
    assert_eq!(result.extras().image_prompt.as_deref(), Some("lighthouse"));
    let _ = std::fs::remove_file(path);
}
