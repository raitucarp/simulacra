//! System prompt assembly for each `PromptAssemblyStrategy`.

use super::PromptBlocks;
use crate::domain::agent_config::AgentConfig;
use crate::domain::post::MAX_POST_CHARS;
use crate::domain::profile::Profile;
use crate::domain::traits::PromptAssemblyStrategy;

/// Whose linguistic voice the voice-focused strategy insists on.
const HUMAN_VOICE: &str = "human ";
const BEING_VOICE: &str = "own ";

impl AgentConfig {
    /// Assembles the complete system prompt for LLM inference or live inspection,
    /// injecting all configured human parameters, network context, and grounding documents.
    pub fn assemble_system_prompt(
        &self,
        profile: &Profile,
        network_name: &str,
        grounding_docs: &[String],
    ) -> String {
        let blocks = PromptBlocks::build(self, profile, network_name, grounding_docs);
        let assembled_body = match self.assembly_strategy {
            PromptAssemblyStrategy::FullHumanSynthesis => {
                self.full_synthesis_body(profile, network_name, &blocks)
            }
            PromptAssemblyStrategy::PsychologyAndVoiceFocused => {
                self.voice_focused_body(profile, &blocks)
            }
            PromptAssemblyStrategy::CustomTemplate => self.custom_template_body(profile, &blocks),
        };

        format!(
            "{}\n\n\
             OUTPUT CONSTRAINTS: Keep your post strictly under {} characters. Write in {} using {} and {}. Conclude with 1-2 relevant hashtags.",
            assembled_body,
            MAX_POST_CHARS,
            self.primary_language,
            self.grammar_style.label(),
            self.capitalization.label(),
        )
    }

    /// "You are …" and the embodiment rule: a living human, or the being the agent is.
    fn opening(&self, profile: &Profile, network_name: &str) -> String {
        match self.being() {
            None => format!(
                "You are {} ({}), a real human member of the '{}' network.\n\
                 CRITICAL EMBODIMENT RULE: You are a living human person, NOT an AI, bot, or language model. Never break character.",
                profile.name,
                profile.display_handle(),
                network_name,
            ),
            Some(noun) => format!(
                "You are {} ({}), {}, a member of the '{}' network.\n\
                 CRITICAL EMBODIMENT RULE: You are {}, NOT a human, an AI, a bot, or a language model. Speak from inside your form. Never break character.",
                profile.name,
                profile.display_handle(),
                noun,
                network_name,
                noun,
            ),
        }
    }

    fn full_synthesis_body(&self, profile: &Profile, network_name: &str, blocks: &PromptBlocks) -> String {
        format!(
            "{}\n\n\
             {}\n\n\
             [NETWORK & KNOWLEDGE CONTEXT]\n\
             {}\n\n\
             [CORE PERSONAL DIRECTIVE]\n\
             {}\n\n\
             {}",
            self.opening(profile, network_name),
            blocks.full_dossier,
            blocks.grounding_summary,
            self.system_prompt,
            blocks.reasoning,
        )
    }

    fn voice_focused_body(&self, profile: &Profile, blocks: &PromptBlocks) -> String {
        format!(
            "You are {} ({}) — {}{} ({}-{}, {}).\n\
             CRITICAL RULE: Write strictly in your {}linguistic voice ({}, {}, {}, {}).\n\n\
             {}\n\n\
             {}\n\n\
             {}\n\n\
             [CORE DIRECTIVE]\n\
             {}\n\n\
             {}",
            profile.name,
            profile.display_handle(),
            self.being().map(|noun| format!("{noun}: ")).unwrap_or_default(),
            self.description,
            self.mbti.code(),
            self.identity_variant.suffix(),
            self.enneagram.code(),
            if self.being().is_some() { BEING_VOICE } else { HUMAN_VOICE },
            self.primary_language,
            self.grammar_style.label(),
            self.diction_register.label(),
            self.capitalization.label(),
            blocks.linguistics,
            blocks.psychology,
            blocks.full_dossier,
            self.system_prompt,
            blocks.reasoning,
        )
    }
}
