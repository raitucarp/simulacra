use super::*;
use crate::domain::{AuthoredPost, NetworkId, Post, PostId, Profile};
use jiff::Timestamp;

fn parent_post(id: &str, author_name: &str, content: &str, mentions: Vec<String>) -> AuthoredPost {
    let author = Profile::new_primary("u1", author_name, "@architect");
    let post = Post {
        id: PostId::new(id),
        network_id: NetworkId::new("net_philosophy"),
        author_id: author.id.clone(),
        content: content.to_string(),
        created_at: Timestamp::now(),
        likes: 0,
        is_liked: false,
        replies_count: 0,
        mentions,
        linked_posts: vec![],
        reply_to_id: None,
        quote_id: None,
        repost_id: None,
        link_url: None,
        image_path: None,
        reposts_count: 0,
        is_reposted: false,
        extras: Default::default(),
    };
    AuthoredPost::new(post, author)
}

fn socrates() -> Profile {
    Profile::new_agent(
        "agent_socrates",
        "Sokrates of Alopece",
        "@sokrates",
        "Classical Dialectician",
    )
}

#[test]
fn test_socratic_response_generation() {
    let parent = parent_post(
        "post_test",
        "Architect",
        "Testing language models and recursive semiotics",
        vec![],
    );

    let reply = DialecticEngine::generate_response(&parent, &socrates(), &parent.network_id);
    assert!(reply.content.contains("[[post_test]]"));
    assert_eq!(reply.linked_posts, vec![PostId::new("post_test")]);
}

#[tokio::test]
async fn test_async_llm_response_generation() {
    let parent = parent_post(
        "post_async_test",
        "Architect",
        "Testing async inference and monism",
        vec![],
    );

    let spinoza = Profile::new_agent(
        "agent_spinoza",
        "Baruch de Spinoza",
        "@baruch_spinoza",
        "Geometric Rationalist",
    );

    let client = crate::agent::LlmClient::new(crate::agent::LlmSettings::default());
    let outcome =
        DialecticEngine::generate_llm_response(&parent, &spinoza, &parent.network_id, &client)
            .await;
    let failure = outcome.llm_error.expect("missing API key is reported, not swallowed");
    assert!(failure.missing_key);
    let reply = outcome.reply;
    assert!(reply.content.contains("[[post_async_test]]"));
    assert_eq!(reply.linked_posts, vec![PostId::new("post_async_test")]);
}

#[test]
fn test_spawn_background_simulation() {
    let parent = parent_post(
        "post_bg_test",
        "Architect",
        "Testing background thread simulation",
        vec![],
    );

    let client = crate::agent::LlmClient::new(crate::agent::LlmSettings::default());
    let rx = DialecticEngine::spawn_background_simulation(
        parent.clone(),
        socrates(),
        parent.network_id.clone(),
        client,
    );
    let outcome = rx.blocking_recv().expect("channel received reply");
    assert!(outcome.llm_error.is_some(), "heuristic fallback carries the LLM failure");
    assert!(outcome.reply.content.contains("[[post_bg_test]]"));
}

#[test]
fn test_persona_prompt_assembly_includes_all_human_parameters_and_mention_attitude() {
    let parent = parent_post(
        "post_mention_test",
        "Arya Wiratama",
        "Hey @sokrates what do you think of epistemic humility?",
        vec!["@sokrates".to_string()],
    );

    let mut socrates = socrates();
    socrates.config.hobbies = "Barefoot walks in the agora".to_string();

    let sys_prompt = DialecticEngine::persona_system_prompt(&socrates);
    assert!(sys_prompt.contains("Sokrates of Alopece (@sokrates)"));
    assert!(sys_prompt.contains("Barefoot walks in the agora"));
    assert!(sys_prompt.contains("PSYCHOLOGICAL & PERSONALITY PROFILE"));

    let user_prompt = DialecticEngine::persona_user_prompt(&parent, &socrates);
    assert!(user_prompt.contains("DIRECT @MENTION ALERT"));
    assert!(user_prompt.contains("[[post_mention_test]]"));
}

#[test]
fn test_reply_post_shape_is_threaded_under_parent() {
    let parent = parent_post("post_shape", "Architect", "anything", vec![]);
    let reply = DialecticEngine::generate_response(&parent, &socrates(), &parent.network_id);

    assert!(reply.id.as_str().starts_with("post_"));
    assert_ne!(reply.id, parent.id);
    assert_eq!(reply.network_id, parent.network_id);
    assert_eq!(reply.author_id.as_str(), "agent_socrates");
    assert_eq!(reply.mentions, vec!["@architect".to_string()]);
    assert_eq!(reply.reply_to_id, Some(PostId::new("post_shape")));
    assert_eq!((reply.likes, reply.replies_count, reply.reposts_count), (0, 0, 0));
    assert!(!reply.is_liked && !reply.is_reposted);
    assert!(reply.quote_id.is_none() && reply.repost_id.is_none());
    assert!(reply.link_url.is_none() && reply.image_path.is_none());
}

#[test]
fn test_canned_voice_keyword_rules_and_fallback() {
    let ai = parent_post("p1", "Architect", "Is this AI thinking?", vec![]);
    let sign = parent_post("p2", "Architect", "the sign and the signified", vec![]);
    let other = parent_post("p3", "Architect", "bread", vec![]);

    let text = |p: &Post| DialecticEngine::generate_heuristic_text(p, &socrates());
    assert_eq!(
        text(&ai),
        "If prediction mimics reasoning without subjective telos, does it illuminate what thinking is, or merely what it appears to be? Let us examine this premise [[p1]]. #SocraticDialectic #Epistemology"
    );
    assert!(text(&sign).starts_with("Can a symbol signify anything"));
    assert!(text(&sign).contains("[[p2]]"));
    assert!(text(&other).starts_with("You assert this [[p3]], my friend"));
}

#[test]
fn test_generic_voice_uses_occupation_and_language() {
    let parent = parent_post("p_generic", "Architect", "bread", vec![]);
    let mut agent = Profile::new_agent("a1", "Someone", "@someone_new", "Baker");
    agent.config.signature_phrases = String::new();
    agent.config.primary_language = "English".to_string();
    let text = DialecticEngine::generate_heuristic_text(&parent, &agent);
    assert!(text.starts_with("Reflecting on this — looking at [[p_generic]]"));
    assert!(text.ends_with("#Dialectic"));

    agent.config.primary_language = "Bahasa Indonesia".to_string();
    agent.config.signature_phrases = "\"Wah\", hmm; lain".to_string();
    let text = DialecticEngine::generate_heuristic_text(&parent, &agent);
    assert!(text.starts_with("Wah — menarik melihat perspektif di [[p_generic]]"));
    assert!(text.ends_with("#Dialektika"));
}

#[test]
fn live_prompts_carry_the_network_and_its_grounding_sources() {
    let mut agent = socrates();
    agent.config.inject_network_grounding = true;
    let context = PromptContext {
        network_name: "Epistemology & Mind".to_string(),
        grounding_docs: vec!["Meno".to_string(), "Theaetetus".to_string()],
        ..PromptContext::default()
    };
    let prompt = DialecticEngine::system_prompt_in(&agent, &context);
    assert!(prompt.contains("Network: Epistemology & Mind"));
    assert!(prompt.contains("Grounding Knowledge Sources: Meno ; Theaetetus"));
    let fallback = DialecticEngine::system_prompt_in(&agent, &PromptContext::default());
    assert_eq!(fallback, DialecticEngine::persona_system_prompt(&agent));
}
