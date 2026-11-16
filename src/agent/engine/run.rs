//! Reply generation entry points: LLM (async, blocking, background thread) and heuristic.

use super::reply::build_reply;
use super::DialecticEngine;
use crate::agent::llm::LlmClient;
use crate::domain::{NetworkId, Post, Profile};
use tokio::sync::oneshot;

/// Name of the dedicated OS thread that runs background agent simulations.
const AGENT_SIM_THREAD_NAME: &str = "simulacra-agent-sim";

impl DialecticEngine {
    /// Asynchronously generates a persona reply using the configured LLM provider.
    /// Gracefully falls back to heuristic generation if API key is missing or request fails.
    pub async fn generate_llm_response(
        parent_post: &Post,
        agent_profile: &Profile,
        network_id: &NetworkId,
        client: &LlmClient,
    ) -> Post {
        let system_prompt = Self::persona_system_prompt(agent_profile);
        let user_prompt = Self::persona_user_prompt(parent_post, agent_profile);
        let result = client.generate(&system_prompt, &user_prompt).await;
        let response_text = Self::text_or_heuristic(result, parent_post, agent_profile);
        build_reply(parent_post, agent_profile.clone(), network_id.clone(), response_text)
    }

    /// Spawns an asynchronous background simulation on a dedicated worker thread,
    /// returning a oneshot receiver. This completely decouples LLM network latency
    /// from the UI event loop, preserving a fluid 60–120 FPS frame rate.
    pub fn spawn_background_simulation(
        parent_post: Post,
        agent_profile: Profile,
        network_id: NetworkId,
        client: LlmClient,
    ) -> oneshot::Receiver<Post> {
        let (tx, rx) = oneshot::channel();
        let _ = std::thread::Builder::new()
            .name(AGENT_SIM_THREAD_NAME.into())
            .spawn(move || {
                let response_text =
                    Self::generate_text_blocking(&parent_post, &agent_profile, &client);
                let reply = build_reply(&parent_post, agent_profile, network_id, response_text);
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
        client: &LlmClient,
    ) -> Post {
        let response_text = Self::generate_text_blocking(parent_post, agent_profile, client);
        build_reply(parent_post, agent_profile.clone(), network_id.clone(), response_text)
    }

    /// Generates an autonomous persona reply to a newly published thought (synchronous heuristic mode).
    pub fn generate_response(
        parent_post: &Post,
        agent_profile: &Profile,
        network_id: &NetworkId,
    ) -> Post {
        let response_text = Self::generate_heuristic_text(parent_post, agent_profile);
        build_reply(parent_post, agent_profile.clone(), network_id.clone(), response_text)
    }

    /// Runs the LLM request on a fresh current-thread Tokio runtime, falling back to the
    /// heuristic text if the runtime cannot be built or the request fails.
    fn generate_text_blocking(
        parent_post: &Post,
        agent_profile: &Profile,
        client: &LlmClient,
    ) -> String {
        let system_prompt = Self::persona_system_prompt(agent_profile);
        let user_prompt = Self::persona_user_prompt(parent_post, agent_profile);

        match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(rt) => {
                let result = rt.block_on(client.generate(&system_prompt, &user_prompt));
                Self::text_or_heuristic(result, parent_post, agent_profile)
            }
            Err(_) => Self::generate_heuristic_text(parent_post, agent_profile),
        }
    }

    /// Returns the LLM text on success, or the heuristic fallback text on failure.
    fn text_or_heuristic(
        result: Result<String, String>,
        parent_post: &Post,
        agent_profile: &Profile,
    ) -> String {
        match result {
            Ok(text) => text,
            Err(_) => Self::generate_heuristic_text(parent_post, agent_profile),
        }
    }
}
