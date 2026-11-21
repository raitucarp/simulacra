mod job;
mod draft;
mod native;
mod requests;
mod root_job;

use crate::agent::credentials::{Credential, CredentialKind, CredentialStore};
use crate::agent::llm::{LlmProvider, LlmSettings};
use crate::agent::runtime::EnginePlan;
use crate::agent::tools::ToolContext;
use crate::domain::engine::{EngineConfig, OwnModel, TextModel};
use serde_json::{json, Value};
use std::path::PathBuf;

/// Secret of the test credential (must never appear in failures or tool output).
pub(super) const SECRET: &str = "sk-test-secret-0042";
pub(super) const CRED_ID: &str = "cred_test";

/// A fresh, unique media folder under the system temp dir.
pub(super) fn media_dir() -> PathBuf {
    std::env::temp_dir().join(format!("simulacra-test-media-{}", uuid::Uuid::new_v4().simple()))
}

/// A store holding one credential of `provider`'s kind pointing at `endpoint`.
pub(super) fn store(provider: LlmProvider, endpoint: &str) -> CredentialStore {
    let mut store = CredentialStore::default();
    store.upsert(Credential {
        id: CRED_ID.into(),
        kind: CredentialKind::from_provider(provider),
        label: "Test".into(),
        secret: SECRET.into(),
        endpoint: endpoint.into(),
    });
    store
}

/// `engine` on its own `provider` model, resolved against a mock endpoint.
pub(super) fn plan(provider: LlmProvider, endpoint: &str, mut engine: EngineConfig) -> EnginePlan {
    let own = OwnModel { model: "test-model".into(), credential: Some(CRED_ID.into()), ..OwnModel::for_provider(provider) };
    engine.text = TextModel::Own(own);
    EnginePlan::resolve(&engine, &LlmSettings::default(), &store(provider, endpoint), media_dir())
}

/// The tool snapshot shared by runtime tests (vault documents included).
pub(super) fn tool_context() -> ToolContext {
    let doc = |id: &str, title: &str, content: &str| crate::agent::tools::DocumentSnapshot { id: id.into(), title: title.into(), content: content.into() };
    ToolContext {
        documents: vec![
            doc("doc_ethics", "Ethics", "Substance is one; monism unifies mind and body."),
            doc("doc_bread", "Bread", "Knead the dough and bake it."),
        ],
        ..ToolContext::default()
    }
}

/// An OpenAI chat-completions reply with text.
pub(super) fn openai_text(text: &str) -> Value {
    json!({ "choices": [{ "message": { "role": "assistant", "content": text } }] })
}

/// An OpenAI chat-completions reply requesting one tool call.
pub(super) fn openai_tool_call(id: &str, name: &str, arguments: Value) -> Value {
    json!({ "choices": [{ "message": { "role": "assistant", "content": null, "tool_calls": [
        { "id": id, "type": "function", "function": { "name": name, "arguments": arguments.to_string() } } ] } }] })
}

/// A tiny valid PNG (1×1) as base64.
pub(super) const PNG_1X1_BASE64: &str =
    "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==";
