//! Progress replies (D12): the delegated item in the prompt, and the offline fallback.

use super::kind::JobKind;
use super::*;
use crate::agent::runtime::EnginePlan;
use crate::agent::{LlmClient, LlmSettings};
use crate::domain::{AuthoredPost, CapitalizationStyle, NetworkId, Post, Profile};

fn agent() -> Profile {
    Profile::new_agent("agent_ada", "Ada", "@ada", "Analyst")
}

fn parent() -> AuthoredPost {
    let post = Post::create("post_launch", "net_work", "user_main", "Launch\n[ ] write the notes @ada").unwrap();
    AuthoredPost::new(post, Profile::new_primary("user_main", "Sam", "@sam"))
}

fn kind() -> JobKind {
    JobKind::Progress { parent: parent(), item: "write the notes".into() }
}

#[test]
fn the_prompt_names_the_delegated_item_on_top_of_the_reply_prompt() {
    let prompt = kind().user_prompt(&agent());
    assert!(prompt.contains(&DialecticEngine::persona_user_prompt(&parent(), &agent())));
    assert!(prompt.contains("DELEGATED TO-DO") && prompt.contains("\"write the notes\"") && prompt.contains("Sam handed you"));
}

#[test]
fn without_a_model_the_agent_still_replies_with_a_progress_update() {
    let job = EngineJob { kind: kind(), agent: agent(), network: NetworkId::new("net_work"), context: PromptContext::default(), plan: EnginePlan::from_client(LlmClient::new(LlmSettings::default())), tools: None, roll: 0 };
    let outcome = DialecticEngine::run_engine_job(job);
    assert!(outcome.llm_error.is_some());
    assert_eq!(outcome.reply.reply_to_id.as_ref().map(|id| id.as_str()), Some("post_launch"));
    assert_eq!(outcome.reply.author_id.as_str(), "agent_ada");
    assert!(outcome.reply.content.contains("write the notes"), "{}", outcome.reply.content);
    let mut quiet = agent();
    quiet.config.capitalization = CapitalizationStyle::AllLowercase;
    assert!(kind().heuristic_text(&quiet).starts_with("on it: write the notes"));
}
