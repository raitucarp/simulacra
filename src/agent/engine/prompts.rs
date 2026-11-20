//! Persona prompt assembly (system + user prompts).

use super::DialecticEngine;
use crate::domain::post::ResolvedSeed;
use crate::domain::{AuthoredPost, Profile};

/// Network name used when no explicit network context is supplied.
const DEFAULT_NETWORK_NAME: &str = "Active Epistemic Network";

/// The network an agent speaks in and the grounding sources it may draw on.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PromptContext {
    pub network_name: String,
    /// Titles of the network's grounding documents.
    pub grounding_docs: Vec<String>,
    /// sb F1: the agent's "Your current beliefs" section (empty when it holds none).
    pub beliefs: String,
}

impl DialecticEngine {
    /// System prompt for `agent_profile` in `context` (the default network when unnamed).
    pub fn system_prompt_in(agent_profile: &Profile, context: &PromptContext) -> String {
        let base = if context.network_name.is_empty() {
            Self::persona_system_prompt(agent_profile)
        } else {
            Self::persona_system_prompt_with_context(agent_profile, &context.network_name, &context.grounding_docs)
        };
        match context.beliefs.trim() {
            "" => base,
            beliefs => format!("{base}

{beliefs}"),
        }
    }

    /// Constructs the full human psychological & linguistic system prompt for a given agent.
    pub fn persona_system_prompt(agent_profile: &Profile) -> String {
        Self::persona_system_prompt_with_context(agent_profile, DEFAULT_NETWORK_NAME, &[])
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
    pub fn persona_user_prompt(parent_post: &AuthoredPost, agent_profile: &Profile) -> String {
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

    /// The user prompt asking `agent_profile` to start a new top-level conversation from
    /// `seed` (or, when the seed is empty, from its own interests).
    pub fn root_user_prompt(agent_profile: &Profile, seed: &ResolvedSeed) -> String {
        agent_profile.config.assemble_root_prompt(agent_profile, seed)
    }
}
