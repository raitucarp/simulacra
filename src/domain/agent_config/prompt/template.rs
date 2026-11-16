//! `{{placeholder}}` expansion for the custom-template assembly strategy.

use super::PromptBlocks;
use crate::domain::agent_config::AgentConfig;
use crate::domain::profile::Profile;

/// Placeholder that injects the full human dossier; appended automatically when absent.
pub const ALL_AGENT_INFO_PLACEHOLDER: &str = "{{all_agent_info}}";
/// Number of supported template placeholders.
const PLACEHOLDER_COUNT: usize = 22;
/// Every placeholder a custom template may use (same order as the expansion table).
pub const TEMPLATE_PLACEHOLDERS: [&str; PLACEHOLDER_COUNT] = [
    "{{name}}", "{{handle}}", "{{description}}", "{{gender}}", "{{pronouns}}", "{{bio}}",
    "{{hobbies}}", "{{mbti}}", "{{enneagram}}", "{{identity}}", "{{psychology}}",
    "{{intelligence}}", "{{linguistics}}", "{{diction}}", "{{grammar}}", "{{language}}",
    "{{typing_style}}", "{{schedule}}", "{{mention_attitude}}", "{{network_context}}",
    "{{directive}}", ALL_AGENT_INFO_PLACEHOLDER,
];

impl AgentConfig {
    /// Expands the custom prompt template and appends the dossier and reasoning instruction.
    pub(super) fn custom_template_body(&self, profile: &Profile, blocks: &PromptBlocks) -> String {
        let mut expanded = self.prompt_template.clone();
        for (placeholder, value) in self.template_replacements(profile, blocks) {
            expanded = expanded.replace(placeholder, value);
        }
        if !self.prompt_template.contains(ALL_AGENT_INFO_PLACEHOLDER) && !blocks.full_dossier.is_empty() {
            format!("{}\n\n{}\n\n{}", expanded, blocks.full_dossier, blocks.reasoning)
        } else {
            format!("{}\n\n{}", expanded, blocks.reasoning)
        }
    }

    /// Placeholder → value pairs, applied in order (later values are not re-expanded by earlier keys).
    fn template_replacements<'a>(
        &'a self,
        profile: &'a Profile,
        blocks: &'a PromptBlocks,
    ) -> [(&'static str, &'a str); PLACEHOLDER_COUNT] {
        [
            ("{{name}}", profile.name.as_str()),
            ("{{handle}}", profile.handle.as_str()),
            ("{{description}}", self.description.as_str()),
            ("{{gender}}", self.gender.label()),
            ("{{pronouns}}", self.pronouns.as_str()),
            ("{{bio}}", self.bio.as_str()),
            ("{{hobbies}}", self.hobbies.as_str()),
            ("{{mbti}}", self.mbti.code()),
            ("{{enneagram}}", self.enneagram.code()),
            ("{{identity}}", blocks.identity.as_str()),
            ("{{psychology}}", blocks.psychology.as_str()),
            ("{{intelligence}}", blocks.intelligence.as_str()),
            ("{{linguistics}}", blocks.linguistics.as_str()),
            ("{{diction}}", self.diction_register.label()),
            ("{{grammar}}", self.grammar_style.label()),
            ("{{language}}", self.primary_language.as_str()),
            ("{{typing_style}}", self.typing_quirks.as_str()),
            ("{{schedule}}", blocks.schedule.as_str()),
            ("{{mention_attitude}}", self.mention_attitude.label()),
            ("{{network_context}}", blocks.grounding_summary.as_str()),
            ("{{directive}}", self.system_prompt.as_str()),
            (ALL_AGENT_INFO_PLACEHOLDER, blocks.full_dossier.as_str()),
        ]
    }
}
