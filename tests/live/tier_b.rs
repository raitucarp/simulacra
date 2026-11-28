//! Tier B, tools and transports: a function-tool loop (thought signatures round trip), the
//! emit tool carrying structured output while tools are offered, native Google Search
//! grounding, a custom raw request, and Gemini's OpenAI-compatible endpoint.

use super::fixtures::{plan, tool_context, ETHICS_DOC, GEMINI_CRED, OPENAI_COMPAT_CRED};
use super::harness::{block_on, ensure, pass, quote, run, Key, Live, Model, Tier, GEMINI_MODELS_URL, GEMINI_OPENAI_URL};
use simulacra::agent::llm::{LlmClient, LlmProvider, LlmSettings};
use simulacra::agent::runtime::{run_engine, EngineResult};
use simulacra::domain::engine::{
    CustomRequest, EngineConfig, HeaderPair, OutputMode, OwnModel, RequestMode, SchemaPreset, SearchBackend, StructuredOutput, TextModel, ToolGrant, WebSearchConfig,
};
use simulacra::domain::post::SourceKind;

const SYSTEM: &str = "You are Baruch, a philosopher agent on a small social network. Posts are one or two sentences.";
const ASK_VAULT: &str = "Call the vault_search tool to find what the Knowledge Vault says about substance, then post one sentence citing it.";
/// Answer budget of the OpenAI-compatible path, where thinking counts against `max_tokens`
/// with no headroom added by the app.
const OPENAI_COMPAT_MAX_TOKENS: u32 = 4096;

fn engine(tools: Vec<ToolGrant>) -> EngineConfig {
    EngineConfig { tools, max_tool_steps: 3, ..EngineConfig::default() }
}

fn generate(live: &Live, engine: &EngineConfig, user: &str) -> Result<EngineResult, String> {
    block_on(run_engine(&plan(live, engine), SYSTEM, user, Some(tool_context()), 0))
}

/// The result used `vault_search` and cites the Ethics document.
fn searched_the_vault(result: &EngineResult) -> Result<String, String> {
    ensure(result.tools.iter().any(|tool| tool.name == "vault_search"), || "the model never called vault_search".to_string())?;
    ensure(result.sources.iter().any(|s| s.kind == SourceKind::VaultDoc && s.id == ETHICS_DOC), || "the Ethics document is not a source".to_string())?;
    Ok(format!("{} tool calls, post {}", result.tools.len(), quote(&result.draft.content)))
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn b01_tool_loop_with_vault_search() {
    run(Tier::B, "tool loop: vault_search (thought signatures)", Model::Text, Key::Needed, |live| {
        let result = generate(live, &engine(vec![ToolGrant::VaultSearch]), ASK_VAULT)?;
        pass(searched_the_vault(&result)?)
    });
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn b02_emit_tool_structured_output_with_tools() {
    run(Tier::B, "emit tool: thread schema while tools are offered", Model::Text, Key::Needed, |live| {
        let structured = OutputMode::JsonSchema(StructuredOutput::from_preset(SchemaPreset::Thread));
        let config = EngineConfig { output: structured, ..engine(vec![ToolGrant::VaultSearch]) };
        let result = generate(live, &config, ASK_VAULT)?;
        ensure(!result.draft.content.trim().is_empty(), || "empty post".to_string())?;
        pass(format!("{} thread parts, {} tool calls, post {}", result.draft.thread_parts.len(), result.tools.len(), quote(&result.draft.content)))
    });
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn b03_native_google_search_grounding() {
    run(Tier::B, "native google_search: grounding sources", Model::Text, Key::Needed, |live| {
        let search = ToolGrant::WebSearch(WebSearchConfig { backend: SearchBackend::ProviderNative, credential: None });
        let result = generate(live, &engine(vec![search]), "Search the web: what is the latest stable release of the Rust programming language? Answer in one sentence.")?;
        let web = result.sources.iter().filter(|s| s.kind == SourceKind::Web).count();
        ensure(web > 0, || format!("no grounding sources; post {}", quote(&result.draft.content)))?;
        pass(format!("{web} web sources (first: {}), post {}", result.sources[0].title, quote(&result.draft.content)))
    });
}

/// A raw `generateContent` request built from the custom-request template.
fn custom_request() -> CustomRequest {
    CustomRequest {
        url: format!("{GEMINI_MODELS_URL}/{{{{model}}}}:generateContent"),
        headers: vec![HeaderPair::new("x-goog-api-key", "{{api_key}}"), HeaderPair::new("Content-Type", "application/json")],
        body_template: r#"{"systemInstruction": {"parts": [{"text": {{system}}}]}, "contents": [{"role": "user", "parts": [{"text": {{user}}}]}], "generationConfig": {"maxOutputTokens": 4096}}"#.into(),
        reply_path: "candidates.0.content.parts.0.text".into(),
        tool_call_path: String::new(),
        credential: Some(GEMINI_CRED.into()),
        ..CustomRequest::default()
    }
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn b04_custom_raw_request_to_gemini() {
    run(Tier::B, "custom raw request (generateContent template)", Model::Text, Key::Needed, |live| {
        let config = EngineConfig { request: RequestMode::Custom(custom_request()), ..EngineConfig::default() };
        let result = block_on(run_engine(&plan(live, &config), SYSTEM, "Post one sentence about light.", None, 0))?;
        pass(format!("post {}", quote(&result.draft.content)))
    });
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn b05_openai_compatible_endpoint_chat() {
    run(Tier::B, "OpenAI-compatible endpoint: chat", Model::Text, Key::Needed, |live| {
        let settings = LlmSettings {
            provider: LlmProvider::Custom,
            custom_endpoint: GEMINI_OPENAI_URL.into(),
            max_tokens: OPENAI_COMPAT_MAX_TOKENS,
            ..live.settings()
        };
        let text = block_on(LlmClient::new(settings).generate("You answer with one word.", "Reply with exactly the word: pong"))?;
        ensure(text.to_lowercase().contains("pong"), || format!("unexpected reply {}", quote(&text)))?;
        pass(format!("answered {}", quote(&text)))
    });
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn b06_openai_compatible_endpoint_tool_loop() {
    run(Tier::B, "OpenAI-compatible endpoint: tool loop (extra_content signatures)", Model::Text, Key::Needed, |live| {
        let own = OwnModel {
            provider: LlmProvider::Custom,
            model: live.model.clone(),
            max_tokens: OPENAI_COMPAT_MAX_TOKENS,
            endpoint: GEMINI_OPENAI_URL.into(),
            credential: Some(OPENAI_COMPAT_CRED.into()),
            ..OwnModel::for_provider(LlmProvider::Custom)
        };
        let config = EngineConfig { text: TextModel::Own(own), ..engine(vec![ToolGrant::VaultSearch]) };
        let result = generate(live, &config, ASK_VAULT)?;
        pass(searched_the_vault(&result)?)
    });
}
