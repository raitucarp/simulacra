//! End to end: an agent with a structured thread schema and vault search posts through a
//! mock OpenAI-compatible server; the outcome carries the reply, its thread and its sources.

use super::{openai_text, openai_tool_call, plan, tool_context};
use crate::agent::engine::{DialecticEngine, EngineJob, PromptContext};
use crate::agent::llm::LlmProvider;
use crate::agent::mock_server::{MockReply, MockServer};
use crate::domain::engine::{EngineConfig, OutputMode, SchemaPreset, StructuredOutput, ToolGrant};
use crate::domain::post::SourceKind;
use crate::domain::{AuthoredPost, NetworkId, Post, PostId, Profile};
use serde_json::json;

fn parent() -> AuthoredPost {
    let author = Profile::new_primary("u1", "Arya", "@arya");
    let post = Post::create("post_root", "net_x", author.id.clone(), "Is the mind the body?").expect("post");
    AuthoredPost::new(post, author)
}

fn job(engine: EngineConfig, endpoint: &str) -> EngineJob {
    let agent = Profile::new_agent("agent_spinoza", "Baruch", "@baruch", "Lens grinder");
    EngineJob {
        parent: parent(),
        agent,
        network: NetworkId::new("net_x"),
        context: PromptContext { network_name: "Mind".into(), grounding_docs: vec!["Ethics".into()] },
        plan: plan(LlmProvider::OpenAi, endpoint, engine),
        tools: Some(tool_context()),
        roll: 0,
    }
}

#[test]
fn a_structured_tool_using_agent_posts_a_thread_with_sources() {
    let answer = json!({ "content": "Mind and body are one thing.", "thread_parts": ["Seen two ways.", "Like a coin."], "hashtags": ["Monism"] });
    let server = MockServer::start(vec![
        MockReply::json(openai_tool_call("v1", "vault_search", json!({ "query": "mind body monism" }))),
        MockReply::json(openai_text(&answer.to_string())),
    ]);
    let engine = EngineConfig {
        output: OutputMode::JsonSchema(StructuredOutput::from_preset(SchemaPreset::Thread)),
        tools: vec![ToolGrant::VaultSearch],
        ..EngineConfig::default()
    };
    let outcome = DialecticEngine::run_engine_job(job(engine, &server.url));
    assert!(outcome.llm_error.is_none(), "{:?}", outcome.llm_error);
    let reply = &outcome.reply;
    assert_eq!(reply.content, "Mind and body are one thing. #Monism");
    assert_eq!(reply.reply_to_id, Some(PostId::new("post_root")));
    assert_eq!(reply.extras.tools[0].name, "vault_search");
    assert_eq!((reply.extras.sources[0].kind, reply.extras.sources[0].id.as_str()), (SourceKind::VaultDoc, "doc_ethics"));
    assert_eq!(outcome.thread.len(), 2);
    assert_eq!(outcome.thread[0].reply_to_id.as_ref(), Some(&reply.id));
    assert_eq!(outcome.thread[1].reply_to_id.as_ref(), Some(&outcome.thread[0].id));
    assert_eq!(outcome.thread[1].content, "Like a coin.");
    assert!(outcome.thread[0].created_at > reply.created_at && outcome.thread[0].mentions.is_empty());
    assert_eq!(outcome.gate_key, super::CRED_ID);
    let system = server.requests()[0].json()["messages"][0]["content"].as_str().unwrap_or_default().to_string();
    assert!(system.contains("Network: Mind"), "the persona prompt is used");
    let round_trip: Post = serde_json::from_str(&serde_json::to_string(reply).expect("ser")).expect("de");
    assert_eq!(&round_trip, reply);
}

#[test]
fn a_failing_engine_falls_back_to_the_heuristic_reply() {
    let server = MockServer::start(vec![MockReply::raw(500, "text/plain", "boom")]);
    let outcome = DialecticEngine::run_engine_job(job(EngineConfig::default(), &server.url));
    let failure = outcome.llm_error.expect("failure reported");
    assert!(failure.message.contains("500"));
    assert!(outcome.reply.content.contains("[[post_root]]"), "heuristic text");
    assert!(outcome.thread.is_empty() && outcome.reply.extras.is_empty());
}
