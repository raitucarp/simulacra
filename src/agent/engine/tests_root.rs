//! New top-level posts: job kinds, seeded prompts, the heuristic fallback and thread chaining.

use super::kind::JobKind;
use super::reply::{build_followup, build_root_post};
use super::*;
use crate::agent::runtime::EnginePlan;
use crate::agent::{LlmClient, LlmSettings};
use crate::domain::post::{PostSeed, ResolvedSeed, SeedDocument, SourceKind};
use crate::domain::{CapitalizationStyle, GroundingDocument, NetworkId, Profile};

fn baker() -> Profile {
    let mut agent = Profile::new_agent("agent_baker", "Rina", "@rina", "Baker");
    agent.config.signature_phrases = String::new();
    agent.config.occupation = "Baker".to_string();
    agent.config.primary_language = "English".to_string();
    agent.config.hobbies = "Sourdough starters, night walks".to_string();
    agent
}

fn seeded() -> ResolvedSeed {
    let doc = SeedDocument { id: "doc_bread".into(), title: "Bread".into(), excerpt: "Knead the dough and bake it.".into() };
    ResolvedSeed { topic: "why crust matters".into(), document: Some(doc) }
}

fn network() -> NetworkId {
    NetworkId::new("net_kitchen")
}

#[test]
fn a_new_post_job_asks_for_an_opening_post_grounded_in_the_seed() {
    let prompt = JobKind::NewPost(seeded()).user_prompt(&baker());
    assert!(prompt.contains("NEW CONVERSATION"));
    assert!(prompt.contains("\"why crust matters\""));
    assert!(prompt.contains("\"Bread\"") && prompt.contains("Knead the dough and bake it.") && prompt.contains("[[Bread]]"));
    assert!(prompt.contains("opening post as Rina (@rina)"));
    assert!(!prompt.contains("Thought in network by") && !prompt.contains("No topic was given"));
    let open = JobKind::NewPost(ResolvedSeed::default()).user_prompt(&baker());
    assert!(open.contains("No topic was given"));
}

#[test]
fn the_heuristic_root_post_has_no_parent_and_cites_its_seed() {
    let post = DialecticEngine::generate_root_post(&baker(), &network(), &seeded());
    assert!(post.id.as_str().starts_with("post_"));
    assert_eq!((post.network_id.clone(), post.author_id.as_str()), (network(), "agent_baker"));
    assert!(post.reply_to_id.is_none() && post.mentions.is_empty() && post.linked_posts.is_empty());
    assert!(post.quote_id.is_none() && post.repost_id.is_none());
    let mbti = baker().config.mbti.code();
    let expected = format!("Reflecting on this — as a Baker ({mbti}), I keep coming back to why crust matters ([[Bread]]). What would it take to see it clearly? #Dialectic");
    assert_eq!(post.content, expected);
    let source = &post.extras.sources[0];
    assert_eq!((source.kind, source.id.as_str(), source.title.as_str()), (SourceKind::VaultDoc, "doc_bread", "Bread"));
}

#[test]
fn an_unseeded_heuristic_root_post_talks_about_a_hobby_in_the_agents_style() {
    let mut agent = baker();
    let text = DialecticEngine::generate_heuristic_root_text(&agent, &ResolvedSeed::default());
    assert!(text.contains("I keep coming back to sourdough starters."), "{text}");
    agent.config.primary_language = "Bahasa Indonesia".to_string();
    agent.config.capitalization = CapitalizationStyle::AllLowercase;
    agent.config.hobbies = String::new();
    let text = DialecticEngine::generate_heuristic_root_text(&agent, &ResolvedSeed { topic: "Roti".into(), document: None });
    assert!(text.starts_with("reflecting on this — sebagai baker"), "{text}");
    assert!(text.contains("memikirkan roti.") && text.ends_with("#dialektika"), "{text}");
}

#[test]
fn a_new_post_without_a_model_falls_back_to_a_root_post() {
    let client = LlmClient::new(LlmSettings::default());
    let job = EngineJob {
        kind: JobKind::NewPost(seeded()),
        agent: baker(),
        network: network(),
        context: PromptContext::default(),
        plan: EnginePlan::from_client(client),
        tools: None,
        roll: 0,
    };
    let outcome = DialecticEngine::run_engine_job(job);
    assert!(outcome.llm_error.is_some(), "the heuristic fallback carries the LLM failure");
    assert!(outcome.reply.reply_to_id.is_none() && outcome.thread.is_empty());
    assert!(outcome.reply.content.contains("why crust matters"));
}

#[test]
fn thread_parts_chain_under_the_root_post() {
    let root = build_root_post(baker(), network(), "first".into());
    let second = build_followup(&root, root.author_id.clone(), "second".into());
    let third = build_followup(&second, root.author_id.clone(), "third".into());
    assert_eq!(second.reply_to_id.as_ref(), Some(&root.id));
    assert_eq!(third.reply_to_id.as_ref(), Some(&second.id));
    assert!(root.created_at < second.created_at && second.created_at < third.created_at);
    assert!(second.mentions.is_empty() && second.network_id == root.network_id);
}

#[test]
fn a_post_seed_resolves_its_document_and_drops_unknown_ids() {
    let docs = vec![GroundingDocument::new("doc_bread", "net_kitchen", " Bread ", "Knead   the\ndough.")];
    let seed = PostSeed { topic: "  crust ".into(), ..PostSeed::document("doc_bread") }.resolve(&docs);
    let doc = seed.document.expect("found");
    assert_eq!((seed.topic.as_str(), doc.title.as_str(), doc.excerpt.as_str()), ("crust", "Bread", "Knead the dough."));
    let missing = PostSeed::document("doc_gone").resolve(&docs);
    assert!(missing.is_empty() && PostSeed::topic("x").resolve(&docs).document.is_none());
}
