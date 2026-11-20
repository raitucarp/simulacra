//! Generation entry points: LLM (async, blocking, background thread) and heuristic.
//!
//! Every LLM path keeps the offline heuristic reply as a fallback, but never hides the
//! failure: it is returned alongside the reply in a [`SimulationOutcome`]. The background
//! paths run through the per-agent engine runtime ([`super::job`]).

use super::job::EngineJob;
use super::kind::JobKind;
use super::outcome::SimulationOutcome;
use super::prompts::PromptContext;
use super::reply::build_reply;
use super::DialecticEngine;
use crate::agent::llm::LlmClient;
use crate::agent::runtime::{EnginePlan, GLOBAL_GATE_KEY};
use crate::domain::post::ResolvedSeed;
use crate::domain::{AuthoredPost, NetworkId, Post, Profile};
use tokio::sync::oneshot;

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
        let (content, llm_error) = match client.generate(&system_prompt, &user_prompt).await {
            Ok(text) => (text, None),
            Err(raw) => (Self::generate_heuristic_text(parent_post, agent_profile), Some(client.describe_failure(&raw))),
        };
        let reply = build_reply(parent_post, agent_profile.clone(), network_id.clone(), content);
        SimulationOutcome { reply, thread: Vec::new(), llm_error, gate_key: GLOBAL_GATE_KEY.to_string(), image_error: None }
    }

    /// Spawns a classic (global model, free text) simulation on a worker thread.
    pub fn spawn_background_simulation(
        parent_post: AuthoredPost,
        agent_profile: Profile,
        network_id: NetworkId,
        client: LlmClient,
    ) -> oneshot::Receiver<SimulationOutcome> {
        Self::spawn_grounded_simulation(parent_post, agent_profile, network_id, client, PromptContext::default())
    }

    /// [`Self::spawn_background_simulation`] with the network name and grounding sources
    /// passed to the agent's system prompt.
    pub fn spawn_grounded_simulation(
        parent_post: AuthoredPost,
        agent_profile: Profile,
        network_id: NetworkId,
        client: LlmClient,
        context: PromptContext,
    ) -> oneshot::Receiver<SimulationOutcome> {
        Self::spawn_engine_simulation(Self::classic_job(parent_post, agent_profile, network_id, client, context))
    }

    /// Synchronously generates a classic persona reply, falling back to heuristic text
    /// (and reporting why) when the request fails.
    pub fn generate_llm_response_sync(
        parent_post: &AuthoredPost,
        agent_profile: &Profile,
        network_id: &NetworkId,
        client: LlmClient,
    ) -> SimulationOutcome {
        let job = Self::classic_job(parent_post.clone(), agent_profile.clone(), network_id.clone(), client, PromptContext::default());
        Self::run_engine_job(job)
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

    /// Generates a new top-level persona post from `seed` (synchronous heuristic mode, used
    /// when no model is configured).
    pub fn generate_root_post(agent_profile: &Profile, network_id: &NetworkId, seed: &ResolvedSeed) -> Post {
        let kind = JobKind::NewPost(seed.clone());
        let text = kind.heuristic_text(agent_profile);
        kind.post(agent_profile.clone(), network_id.clone(), text, Default::default())
    }

    /// A job for the classic engine: `client`, free text, no tools, no image.
    fn classic_job(parent: AuthoredPost, agent: Profile, network: NetworkId, client: LlmClient, context: PromptContext) -> EngineJob {
        EngineJob { kind: JobKind::Reply(parent), agent, network, context, plan: EnginePlan::from_client(client), tools: None, roll: 0 }
    }
}
