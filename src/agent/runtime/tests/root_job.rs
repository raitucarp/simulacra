//! End to end: an agent opens a new top-level conversation from a seed through a mock
//! OpenAI-compatible server; its thread parts chain under the new root post.

use super::job::job_of;
use super::openai_text;
use crate::agent::engine::{DialecticEngine, JobKind};
use crate::agent::mock_server::{MockReply, MockServer};
use crate::domain::engine::{EngineConfig, OutputMode, SchemaPreset, StructuredOutput};
use crate::domain::post::{PostSeed, ResolvedSeed, SourceKind};
use crate::domain::GroundingDocument;
use serde_json::json;

fn seed() -> ResolvedSeed {
    let docs = vec![GroundingDocument::new("doc_ethics", "net_x", "Ethics", "Substance is one; monism unifies mind and body.")];
    PostSeed { topic: "Is a lens a kind of thought?".into(), ..PostSeed::document("doc_ethics") }.resolve(&docs)
}

#[test]
fn a_seeded_new_post_is_a_root_whose_thread_chains_under_it() {
    let answer = json!({ "content": "Grinding lenses taught me to see.", "thread_parts": ["Glass bends light.", "Mind bends too."] });
    let server = MockServer::start(vec![MockReply::json(openai_text(&answer.to_string()))]);
    let engine = EngineConfig { output: OutputMode::JsonSchema(StructuredOutput::from_preset(SchemaPreset::Thread)), ..EngineConfig::default() };
    let outcome = DialecticEngine::run_engine_job(job_of(JobKind::NewPost(seed()), engine, &server.url));
    assert!(outcome.llm_error.is_none(), "{:?}", outcome.llm_error);
    let root = &outcome.reply;
    assert_eq!(root.content, "Grinding lenses taught me to see.");
    assert!(root.reply_to_id.is_none() && root.mentions.is_empty() && root.linked_posts.is_empty());
    assert_eq!(root.author_id.as_str(), "agent_spinoza");
    assert!(root.extras.sources.iter().any(|s| s.kind == SourceKind::VaultDoc && s.id == "doc_ethics"), "the seed document is cited");
    assert_eq!(outcome.thread.len(), 2);
    assert_eq!(outcome.thread[0].reply_to_id.as_ref(), Some(&root.id));
    assert_eq!(outcome.thread[1].reply_to_id.as_ref(), Some(&outcome.thread[0].id));
    assert_eq!(outcome.thread[1].content, "Mind bends too.");
    let user = server.requests()[0].json()["messages"][1]["content"].as_str().unwrap_or_default().to_string();
    assert!(user.contains("NEW CONVERSATION") && user.contains("Is a lens a kind of thought?"), "{user}");
    assert!(user.contains("[[Ethics]]") && user.contains("monism unifies mind and body"), "{user}");
    assert!(!user.contains("Thought in network by"), "no parent post in a new-post prompt");
}

#[test]
fn a_failing_engine_falls_back_to_a_heuristic_root_post() {
    let server = MockServer::start(vec![MockReply::raw(500, "text/plain", "boom")]);
    let outcome = DialecticEngine::run_engine_job(job_of(JobKind::NewPost(seed()), EngineConfig::default(), &server.url));
    assert!(outcome.llm_error.expect("failure reported").message.contains("500"));
    let root = &outcome.reply;
    assert!(root.reply_to_id.is_none() && outcome.thread.is_empty());
    assert!(root.content.contains("Is a lens a kind of thought? ([[Ethics]])"), "{}", root.content);
    assert_eq!(root.extras.sources.len(), 1, "only the seed document");
}
