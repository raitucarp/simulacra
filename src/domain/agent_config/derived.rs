//! Values derived from an `AgentConfig` without any prompt formatting.

use super::AgentConfig;

impl AgentConfig {
    /// Returns the formatted MBTI string with Assertive/Turbulent suffix (e.g., `"INTJ-A"`).
    #[inline]
    pub fn mbti_full_code(&self) -> String {
        format!("{}-{}", self.mbti.code(), self.identity_variant.suffix())
    }

    /// Returns the count of active dossier sections injected into the compiled prompt.
    #[inline]
    pub const fn injected_section_count(&self) -> usize {
        (self.inject_identity_and_bio as usize)
            + (self.inject_psychological_profile as usize)
            + (self.inject_intelligence_profile as usize)
            + (self.inject_linguistic_stylistics as usize)
            + (self.inject_social_and_schedule as usize)
            + (self.inject_network_grounding as usize)
    }
}
