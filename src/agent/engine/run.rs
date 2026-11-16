//! Reply generation entry points: LLM (async, blocking, background thread) and heuristic.
//!
//! Every LLM path keeps the offline heuristic reply as a fallback, but never hides the
//! failure: it is returned alongside the reply in a [`SimulationOutcome`].

use super::outcome::SimulationOutcome;
use super::reply::build_reply;
use super::DialecticEngine;
use crate::agent::llm::{LlmClient, LlmFailure};
use crate::domain::{AuthoredPost, NetworkId, Post, Profile};
use tokio::sync::oneshot;

/// Name of the dedicated OS thread that runs background agent simulations.
const AGENT_SIM_THREAD_NAME: &str = "simulacra-agent-sim";

/// Generated text and the LLM failure that forced the heuristic fallback, if any.
type TextOutcome = (String, Option<LlmFailure>);

impl DialecticEngine {
    /// Asynchronously generates a persona reply using the configured LLM provider,
    /// falling back to heuristic text (and reporting why) when the request fails.
    pub async fn generate_llm_response(
        parent_post: &AuthoredPost,
        agent_profile: &Profile,
        network_id: &NetworkId,
        client: &LlmClient,
    ) -> SimulationOutcome {
        let system_prompt = Self::persona_system_prompt(agent_profile);
        let user_prompt = Self::persona_user_prompt(parent_post, agent_profile);
        let result = client.generate(&system_prompt, &user_prompt).await;
        let text = Self::text_or_heuristic(result, client, parent_post, agent_profile);
        Self::outcome(parent_post, agent_profile.clone(), network_id.clone(), text)
    }

    /// Spawns an asynchronous background simulation on a dedicated worker thread,
    /// returning a oneshot receiver. This completely decouples LLM network latency
    /// from the UI event loop. The sender is dropped (the receiver errors) only if
    /// the worker thread cannot be spawned.
    pub fn spawn_background_simulation(
        parent_post: AuthoredPost,
        agent_profile: Profile,
        network_id: NetworkId,
        client: LlmClient,
    ) -> oneshot::Receiver<SimulationOutcome> {
        let (tx, rx) = oneshot::channel();
        let _ = std::thread::Builder::new()
            .name(AGENT_SIM_THREAD_NAME.into())
            .spawn(move || {
                let text = Self::generate_text_blocking(&parent_post, &agent_profile, &client);
                let _ = tx.send(Self::outcome(&parent_post, agent_profile, network_id, text));
            });

        rx
    }

    /// Synchronously generates a persona reply using the configured LLM provider,
    /// falling back to heuristic text (and reporting why) when the request fails.
    pub fn generate_llm_response_sync(
        parent_post: &AuthoredPost,
        agent_profile: &Profile,
        network_id: &NetworkId,
        client: &LlmClient,
    ) -> SimulationOutcome {
        let text = Self::generate_text_blocking(parent_post, agent_profile, client);
        Self::outcome(parent_post, agent_profile.clone(), network_id.clone(), text)
    }

    /// Generates an autonomous persona reply to a newly published thought (synchronous heuristic mode).
    pub fn generate_response(
        parent_post: &AuthoredPost,
        agent_profile: &Profile,
        network_id: &NetworkId,
    ) -> Post {
        let response_text = Self::generate_heuristic_text(parent_post, agent_profile);
        build_reply(parent_post, agent_profile.clone(), network_id.clone(), response_text)
    }

    /// Runs the LLM request on a fresh current-thread Tokio runtime, falling back to the
    /// heuristic text if the runtime cannot be built or the request fails.
    fn generate_text_blocking(parent_post: &AuthoredPost, agent_profile: &Profile, client: &LlmClient) -> TextOutcome {
        let system_prompt = Self::persona_system_prompt(agent_profile);
        let user_prompt = Self::persona_user_prompt(parent_post, agent_profile);
        let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build();
        let result = match runtime {
            Ok(rt) => rt.block_on(client.generate(&system_prompt, &user_prompt)),
            Err(e) => Err(e.to_string()),
        };
        Self::text_or_heuristic(result, client, parent_post, agent_profile)
    }

    /// The LLM text on success; otherwise the heuristic text plus the described failure.
    fn text_or_heuristic(
        result: Result<String, String>,
        client: &LlmClient,
        parent_post: &AuthoredPost,
        agent_profile: &Profile,
    ) -> TextOutcome {
        match result {
            Ok(text) => (text, None),
            Err(raw) => {
                let failure = client.describe_failure(&raw);
                (Self::generate_heuristic_text(parent_post, agent_profile), Some(failure))
            }
        }
    }

    /// Wraps generated text into a reply outcome threaded under `parent_post`.
    fn outcome(parent_post: &AuthoredPost, agent: Profile, network_id: NetworkId, text: TextOutcome) -> SimulationOutcome {
        let (content, llm_error) = text;
        SimulationOutcome { reply: build_reply(parent_post, agent, network_id, content), llm_error }
    }
}
