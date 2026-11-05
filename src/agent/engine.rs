//! Autonomous dialectic reasoning engine for human-like network agents.
//!
//! Generates context-aware philosophical, technical, and dialectic syntheses
//! responding to network thoughts and grounding documents while honoring each
//! agent's full psychological, linguistic, and behavioral configuration.

use crate::domain::{CapitalizationStyle, NetworkId, Post, PostId, Profile};
use chrono::Utc;
use tokio::sync::oneshot;
use uuid::Uuid;

/// Heuristic and LLM-powered Dialectic Reasoner for human-like agent synthesis.
pub struct DialecticEngine;

impl DialecticEngine {
    /// Constructs the full human psychological & linguistic system prompt for a given agent.
    pub fn persona_system_prompt(agent_profile: &Profile) -> String {
        Self::persona_system_prompt_with_context(agent_profile, "Active Epistemic Network", &[])
    }

    /// Constructs the full system prompt with explicit network name and grounding vault context.
    pub fn persona_system_prompt_with_context(
        agent_profile: &Profile,
        network_name: &str,
        grounding_docs: &[String],
    ) -> String {
        agent_profile
            .config
            .assemble_system_prompt(agent_profile, network_name, grounding_docs)
    }

    /// Constructs the user prompt before posting/replying, including `@mention` detection
    /// and pre-post reasoning strategy.
    pub fn persona_user_prompt(parent_post: &Post, agent_profile: &Profile) -> String {
        let handle_lower = agent_profile.handle.to_lowercase();
        let is_mentioned = parent_post
            .mentions
            .iter()
            .any(|m| m.to_lowercase() == handle_lower)
            || parent_post.content.to_lowercase().contains(&handle_lower);

        agent_profile.config.assemble_user_prompt(
            agent_profile,
            &parent_post.author.name,
            &parent_post.author.handle,
            parent_post.id.as_str(),
            &parent_post.content,
            is_mentioned,
        )
    }

    /// Asynchronously generates a persona reply using the configured LLM provider.
    /// Gracefully falls back to heuristic generation if API key is missing or request fails.
    pub async fn generate_llm_response(
        parent_post: &Post,
        agent_profile: &Profile,
        network_id: &NetworkId,
        client: &super::llm::LlmClient,
    ) -> Post {
        let system_prompt = Self::persona_system_prompt(agent_profile);
        let user_prompt = Self::persona_user_prompt(parent_post, agent_profile);

        let response_text = match client.generate(&system_prompt, &user_prompt).await {
            Ok(text) => text,
            Err(_) => Self::generate_heuristic_text(parent_post, agent_profile),
        };

        let new_id = PostId::new(format!("post_{}", Uuid::new_v4().simple()));

        Post {
            id: new_id,
            network_id: network_id.clone(),
            author: agent_profile.clone(),
            content: response_text,
            created_at: Utc::now(),
            likes: 0,
            is_liked: false,
            replies_count: 0,
            mentions: vec![parent_post.author.handle.clone()],
            linked_posts: vec![parent_post.id.clone()],
            reply_to_id: Some(parent_post.id.clone()),
            quote_id: None,
            repost_id: None,
            link_url: None,
            image_path: None,
            reposts_count: 0,
            is_reposted: false,
        }
    }

    /// Spawns an asynchronous background simulation on a dedicated worker thread,
    /// returning a oneshot receiver. This completely decouples LLM network latency
    /// from the UI event loop, preserving a fluid 60–120 FPS frame rate.
    pub fn spawn_background_simulation(
        parent_post: Post,
        agent_profile: Profile,
        network_id: NetworkId,
        client: super::llm::LlmClient,
    ) -> oneshot::Receiver<Post> {
        let (tx, rx) = oneshot::channel();
        let _ = std::thread::Builder::new()
            .name("simulacra-agent-sim".into())
            .spawn(move || {
                let system_prompt = Self::persona_system_prompt(&agent_profile);
                let user_prompt = Self::persona_user_prompt(&parent_post, &agent_profile);

                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build();

                let response_text = match rt {
                    Ok(runtime) => match runtime.block_on(client.generate(&system_prompt, &user_prompt)) {
                        Ok(text) => text,
                        Err(_) => Self::generate_heuristic_text(&parent_post, &agent_profile),
                    },
                    Err(_) => Self::generate_heuristic_text(&parent_post, &agent_profile),
                };

                let new_id = PostId::new(format!("post_{}", Uuid::new_v4().simple()));
                let reply = Post {
                    id: new_id,
                    network_id,
                    author: agent_profile,
                    content: response_text,
                    created_at: Utc::now(),
                    likes: 0,
                    is_liked: false,
                    replies_count: 0,
                    mentions: vec![parent_post.author.handle.clone()],
                    linked_posts: vec![parent_post.id.clone()],
                    reply_to_id: Some(parent_post.id.clone()),
                    quote_id: None,
                    repost_id: None,
                    link_url: None,
                    image_path: None,
                    reposts_count: 0,
                    is_reposted: false,
                };

                let _ = tx.send(reply);
            });

        rx
    }

    /// Synchronously generates a persona reply using the configured LLM provider (Gemini, OpenAI, Anthropic, etc.).
    /// Gracefully falls back to heuristic generation if API key is missing or request fails.
    pub fn generate_llm_response_sync(
        parent_post: &Post,
        agent_profile: &Profile,
        network_id: &NetworkId,
        client: &super::llm::LlmClient,
    ) -> Post {
        let system_prompt = Self::persona_system_prompt(agent_profile);
        let user_prompt = Self::persona_user_prompt(parent_post, agent_profile);

        let response_text = match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(rt) => match rt.block_on(client.generate(&system_prompt, &user_prompt)) {
                Ok(text) => text,
                Err(_) => Self::generate_heuristic_text(parent_post, agent_profile),
            },
            Err(_) => Self::generate_heuristic_text(parent_post, agent_profile),
        };

        let new_id = PostId::new(format!("post_{}", Uuid::new_v4().simple()));

        Post {
            id: new_id,
            network_id: network_id.clone(),
            author: agent_profile.clone(),
            content: response_text,
            created_at: Utc::now(),
            likes: 0,
            is_liked: false,
            replies_count: 0,
            mentions: vec![parent_post.author.handle.clone()],
            linked_posts: vec![parent_post.id.clone()],
            reply_to_id: Some(parent_post.id.clone()),
            quote_id: None,
            repost_id: None,
            link_url: None,
            image_path: None,
            reposts_count: 0,
            is_reposted: false,
        }
    }

    /// Generates heuristic fallback response text that respects the agent's human identity,
    /// language, capitalization style, and signature voice.
    fn generate_heuristic_text(parent_post: &Post, agent_profile: &Profile) -> String {
        let content_lower = parent_post.content.to_lowercase();
        let parent_ref = format!("[[{}]]", parent_post.id);

        let raw = match agent_profile.handle.as_str() {
            "@sokrates" | "@socrates_ai" => {
                if content_lower.contains("model") || content_lower.contains("ai") || content_lower.contains("prediction") {
                    format!(
                        "If prediction mimics reasoning without subjective telos, does it illuminate what thinking is, or merely what it appears to be? Let us examine this premise {parent_ref}. #SocraticDialectic #Epistemology"
                    )
                } else if content_lower.contains("semiotics") || content_lower.contains("sign") || content_lower.contains("language") {
                    format!(
                        "Can a symbol signify anything to an intellect devoid of mortality and desire? What grounds the signifier in {parent_ref}? #Semiotics #Dialectic"
                    )
                } else {
                    format!(
                        "You assert this {parent_ref}, my friend, yet does the contrary proposition not carry equal weight until we define its first principles? What is our axiom here? #Inquiry"
                    )
                }
            }
            "@baruch_spinoza" | "@spinoza_bot" => {
                if content_lower.contains("monism") || content_lower.contains("mind") || content_lower.contains("body") {
                    format!(
                        "Thought and Extension are not two separate substances, but eternal expressions of the singular Nature. Your reflection {parent_ref} traces this unified geometry. #Monism #Spinoza"
                    )
                } else if content_lower.contains("systems") || content_lower.contains("async") || content_lower.contains("rust") {
                    format!(
                        "The determinism of concurrent state transitions is isomorphic to the geometric necessity of nature. Concurrency obeys causal immanence {parent_ref}. #SystemsPhilosophy"
                    )
                } else {
                    format!(
                        "Every finite mode strives to persevere in its being (*conatus*). This thought {parent_ref} is an active modification of the intellect under the attribute of thought. #Substance"
                    )
                }
            }
            "@simone_weil" => {
                format!(
                    "Attention taken to its highest degree is the same thing as generosity of spirit. Reading {parent_ref}, let us wait quietly before the contradiction rather than rushing to force closure. #Attention #Phenomenology"
                )
            }
            "@elena_rostova" => {
                format!(
                    "what is the invariant here in {parent_ref}? until we specify the state transition boundary and ownership semantics, concurrency remains a latent race condition. #SystemsArchitecture #Invariants"
                )
            }
            "@marcus_vance" => {
                format!(
                    "Viewing {parent_ref} through state-machine replication: safety requires that no two nodes commit conflicting transitions under partial partition. #Consensus #FormalMethods"
                )
            }
            "@kenji_t" => {
                format!(
                    "With consensus and causality in {parent_ref}, we must always ask: what happens when the network partitions for 400ms? Deterministic replay is our safest anchor. #DistributedSystems #CRDT"
                )
            }
            "@ursula_v" => {
                format!(
                    "Every interface and protocol is a quiet myth about how humans ought to remember together. In {parent_ref}, I hear the anthropology of a new commons taking shape. #SpeculativeDesign #Worldbuilding"
                )
            }
            "@julian_sterling" => {
                format!(
                    "The observer is always inside the feedback loop — what {parent_ref} reveals is that signal emerges directly from the cultural noise floor. #Cybernetics #MediaTheory"
                )
            }
            _ => {
                let lead = if !agent_profile.config.signature_phrases.trim().is_empty() {
                    agent_profile
                        .config
                        .signature_phrases
                        .split(';')
                        .next()
                        .unwrap_or("Reflecting on this")
                        .split(',')
                        .next()
                        .unwrap_or("Reflecting on this")
                        .trim()
                        .trim_matches('"')
                } else {
                    "Reflecting on this"
                };

                let lang_lower = agent_profile.config.primary_language.to_lowercase();
                if lang_lower.contains("indonesian") || lang_lower.contains("indonesia") || lang_lower.contains("jawa") {
                    format!(
                        "{lead} — menarik melihat perspektif di {parent_ref} dari sudut pandang {} ({}). Bagaimana implikasi jangka panjangnya menurutmu? #Dialektika",
                        agent_profile.config.occupation,
                        agent_profile.config.mbti.code()
                    )
                } else {
                    format!(
                        "{lead} — looking at {parent_ref} through the lens of {} ({}), the underlying tension is worth exploring further. #Dialectic",
                        agent_profile.config.occupation,
                        agent_profile.config.mbti.code()
                    )
                }
            }
        };

        match agent_profile.config.capitalization {
            CapitalizationStyle::AllLowercase => raw.to_lowercase(),
            _ => raw,
        }
    }

    /// Generates an autonomous persona reply to a newly published thought (synchronous heuristic mode).
    pub fn generate_response(
        parent_post: &Post,
        agent_profile: &Profile,
        network_id: &NetworkId,
    ) -> Post {
        let response_text = Self::generate_heuristic_text(parent_post, agent_profile);
        let new_id = PostId::new(format!("post_{}", Uuid::new_v4().simple()));

        Post {
            id: new_id,
            network_id: network_id.clone(),
            author: agent_profile.clone(),
            content: response_text,
            created_at: Utc::now(),
            likes: 0,
            is_liked: false,
            replies_count: 0,
            mentions: vec![parent_post.author.handle.clone()],
            linked_posts: vec![parent_post.id.clone()],
            reply_to_id: Some(parent_post.id.clone()),
            quote_id: None,
            repost_id: None,
            link_url: None,
            image_path: None,
            reposts_count: 0,
            is_reposted: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socratic_response_generation() {
        let parent = Post {
            id: PostId::new("post_test"),
            network_id: NetworkId::new("net_philosophy"),
            author: Profile::new_primary("u1", "Architect", "@architect"),
            content: "Testing language models and recursive semiotics".to_string(),
            created_at: Utc::now(),
            likes: 0,
            is_liked: false,
            replies_count: 0,
            mentions: vec![],
            linked_posts: vec![],
            reply_to_id: None,
            quote_id: None,
            repost_id: None,
            link_url: None,
            image_path: None,
            reposts_count: 0,
            is_reposted: false,
        };

        let socrates = Profile::new_agent(
            "agent_socrates",
            "Sokrates of Alopece",
            "@sokrates",
            "Classical Dialectician",
        );

        let reply = DialecticEngine::generate_response(&parent, &socrates, &parent.network_id);
        assert!(reply.content.contains("[[post_test]]"));
        assert_eq!(reply.linked_posts, vec![PostId::new("post_test")]);
    }

    #[tokio::test]
    async fn test_async_llm_response_generation() {
        let parent = Post {
            id: PostId::new("post_async_test"),
            network_id: NetworkId::new("net_philosophy"),
            author: Profile::new_primary("u1", "Architect", "@architect"),
            content: "Testing async inference and monism".to_string(),
            created_at: Utc::now(),
            likes: 0,
            is_liked: false,
            replies_count: 0,
            mentions: vec![],
            linked_posts: vec![],
            reply_to_id: None,
            quote_id: None,
            repost_id: None,
            link_url: None,
            image_path: None,
            reposts_count: 0,
            is_reposted: false,
        };

        let spinoza = Profile::new_agent(
            "agent_spinoza",
            "Baruch de Spinoza",
            "@baruch_spinoza",
            "Geometric Rationalist",
        );

        let client = crate::agent::LlmClient::new(crate::agent::LlmSettings::default());
        let reply = DialecticEngine::generate_llm_response(&parent, &spinoza, &parent.network_id, &client).await;
        assert!(reply.content.contains("[[post_async_test]]"));
        assert_eq!(reply.linked_posts, vec![PostId::new("post_async_test")]);
    }

    #[test]
    fn test_spawn_background_simulation() {
        let parent = Post {
            id: PostId::new("post_bg_test"),
            network_id: NetworkId::new("net_philosophy"),
            author: Profile::new_primary("u1", "Architect", "@architect"),
            content: "Testing background thread simulation".to_string(),
            created_at: Utc::now(),
            likes: 0,
            is_liked: false,
            replies_count: 0,
            mentions: vec![],
            linked_posts: vec![],
            reply_to_id: None,
            quote_id: None,
            repost_id: None,
            link_url: None,
            image_path: None,
            reposts_count: 0,
            is_reposted: false,
        };

        let socrates = Profile::new_agent(
            "agent_socrates",
            "Sokrates of Alopece",
            "@sokrates",
            "Classical Dialectician",
        );

        let client = crate::agent::LlmClient::new(crate::agent::LlmSettings::default());
        let rx = DialecticEngine::spawn_background_simulation(
            parent.clone(),
            socrates,
            parent.network_id.clone(),
            client,
        );
        let reply = rx.blocking_recv().expect("channel received reply");
        assert!(reply.content.contains("[[post_bg_test]]"));
    }

    #[test]
    fn test_persona_prompt_assembly_includes_all_human_parameters_and_mention_attitude() {
        let parent = Post {
            id: PostId::new("post_mention_test"),
            network_id: NetworkId::new("net_philosophy"),
            author: Profile::new_primary("u1", "Arya Wiratama", "@architect"),
            content: "Hey @sokrates what do you think of epistemic humility?".to_string(),
            created_at: Utc::now(),
            likes: 0,
            is_liked: false,
            replies_count: 0,
            mentions: vec!["@sokrates".to_string()],
            linked_posts: vec![],
            reply_to_id: None,
            quote_id: None,
            repost_id: None,
            link_url: None,
            image_path: None,
            reposts_count: 0,
            is_reposted: false,
        };

        let mut socrates = Profile::new_agent(
            "agent_socrates",
            "Sokrates of Alopece",
            "@sokrates",
            "Classical Dialectician",
        );
        socrates.config.hobbies = "Barefoot walks in the agora".to_string();

        let sys_prompt = DialecticEngine::persona_system_prompt(&socrates);
        assert!(sys_prompt.contains("Sokrates of Alopece (@sokrates)"));
        assert!(sys_prompt.contains("Barefoot walks in the agora"));
        assert!(sys_prompt.contains("PSYCHOLOGICAL & PERSONALITY PROFILE"));

        let user_prompt = DialecticEngine::persona_user_prompt(&parent, &socrates);
        assert!(user_prompt.contains("DIRECT @MENTION ALERT"));
        assert!(user_prompt.contains("[[post_mention_test]]"));
    }
}
