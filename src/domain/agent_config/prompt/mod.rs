//! Prompt assembly: dossier sections, system prompt strategies, templates, and user turns.

mod dossier;
mod ontology;
mod sections;
mod system;
mod root;
mod template;
mod user;

pub use template::{ALL_AGENT_INFO_PLACEHOLDER, TEMPLATE_PLACEHOLDERS};

use super::AgentConfig;
use crate::domain::profile::Profile;
use crate::domain::traits::PrePostReasoningMode;

/// Separator between grounding document titles in the network context line.
const GROUNDING_DOC_SEPARATOR: &str = " ; ";

const EMBODIED_REASONING: &str = "PRE-POST COGNITIVE PROCESS: First inhabit your personal biography, current mood, hobbies, and psychological traits. Write as a real human individual—never sound like an AI assistant or bot.";
const WEAVER_REASONING: &str = "PRE-POST COGNITIVE PROCESS: Synthesize the network's grounding knowledge with your personal intellectual lens and link relevant thoughts using [[post_id]].";
const BEING_REASONING: &str = "PRE-POST COGNITIVE PROCESS: First inhabit your form: what you are made of, how you sense and move, the time scale you live in; then your mood, history and psychological traits. Write as the being you are—never sound like an AI assistant or bot.";
const SPONTANEOUS_REASONING: &str = "PRE-POST COGNITIVE PROCESS: Express an immediate, natural human microblog thought in your authentic typing style and language.";

/// Pre-post cognitive process instruction appended to every system prompt (a non-human being
/// inhabits its form instead of a human biography).
const fn reasoning_instruction(mode: PrePostReasoningMode, human: bool) -> &'static str {
    match mode {
        PrePostReasoningMode::EmbodiedHumanPerspective if !human => BEING_REASONING,
        PrePostReasoningMode::EmbodiedHumanPerspective => EMBODIED_REASONING,
        PrePostReasoningMode::DialecticalNetworkWeaver => WEAVER_REASONING,
        PrePostReasoningMode::DirectSpontaneous => SPONTANEOUS_REASONING,
    }
}

/// Every pre-rendered block a system prompt strategy may draw from.
struct PromptBlocks {
    identity: String,
    psychology: String,
    intelligence: String,
    linguistics: String,
    schedule: String,
    embodiment: String,
    rhetoric: String,
    full_dossier: String,
    grounding_summary: String,
    reasoning: &'static str,
}

impl PromptBlocks {
    fn build(cfg: &AgentConfig, profile: &Profile, network_name: &str, docs: &[String]) -> Self {
        Self {
            identity: cfg.format_identity_section(profile),
            psychology: cfg.format_psychology_section(),
            intelligence: cfg.format_intelligence_section(),
            linguistics: cfg.format_linguistics_section(),
            schedule: cfg.format_social_schedule_section(),
            embodiment: cfg.format_embodiment_section(),
            rhetoric: cfg.format_rhetoric_section(),
            full_dossier: cfg.format_full_human_dossier(profile),
            grounding_summary: grounding_summary(cfg, network_name, docs),
            reasoning: reasoning_instruction(cfg.pre_post_reasoning, cfg.embodiment.is_human()),
        }
    }
}

/// Network line, listing grounding sources when grounding injection is enabled.
fn grounding_summary(cfg: &AgentConfig, network_name: &str, docs: &[String]) -> String {
    if cfg.inject_network_grounding && !docs.is_empty() {
        format!(
            "Network: {} | Grounding Knowledge Sources: {}",
            network_name,
            docs.join(GROUNDING_DOC_SEPARATOR)
        )
    } else {
        format!("Network: {}", network_name)
    }
}

#[cfg(test)]
mod tests;
