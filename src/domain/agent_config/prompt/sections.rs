//! Identity, psychology, and intelligence dossier sections.

use crate::domain::agent_config::AgentConfig;
use crate::domain::profile::Profile;

const HUMAN_IDENTITY_HEADING: &str = "[HUMAN IDENTITY & BIOGRAPHY]";
const BEING_IDENTITY_HEADING: &str = "[IDENTITY & BIOGRAPHY]";

impl AgentConfig {
    /// Formats the complete human identity and biography section for prompt injection.
    pub fn format_identity_section(&self, profile: &Profile) -> String {
        let heading = if self.embodiment.is_human() { HUMAN_IDENTITY_HEADING } else { BEING_IDENTITY_HEADING };
        format!(
            "{}\n\
             - Full Name: {} ({})\n\
             - Headline / Role: {}\n\
             - Gender & Pronouns: {} ({})\n\
             - Age & Life Stage: {} years old\n\
             - Cultural / Geographic Roots: {}\n\
             - Vocation / Occupation: {}\n\
             - Personal Biography: {}\n\
             - Hobbies & Passions: {}\n\
             - Core Human Values: {}\n\
             - Daily Routine & Lifestyle: {}\n\
             - Current Affective Mood: {}",
            heading,
            profile.name,
            profile.display_handle(),
            self.description,
            self.gender.label(),
            self.pronouns,
            self.age,
            self.location_culture,
            self.occupation,
            self.bio,
            self.hobbies,
            self.core_values,
            self.daily_routine,
            self.current_mood,
        )
    }

    /// Formats the complete SOTA psychological and personality profile section.
    pub fn format_psychology_section(&self) -> String {
        format!(
            "[PSYCHOLOGICAL & PERSONALITY PROFILE (SOTA TAXONOMY)]\n\
             - Myers-Briggs (MBTI): {}-{} ({}) | Jungian Stack: {}\n\
             - Cognitive Dynamics: {}\n\
             - Enneagram & Instinct: {} | Subtype: {} | Tritype: {}\n\
             - Big Five (OCEAN): Openness {}%, Conscientiousness {}%, Extraversion {}%, Agreeableness {}%, Neuroticism/Sensitivity {}%\n\
             - HEXACO Honesty-Humility: {}%\n\
             - Attachment Style (Bowlby/Ainsworth): {}\n\
             - Moral Foundation (Haidt): {}\n\
             - Classical Temperament: {}\n\
             - Locus of Control: {}",
            self.mbti.code(),
            self.identity_variant.suffix(),
            self.mbti.archetype_name(),
            self.mbti.cognitive_functions(),
            self.mbti.cognitive_description(),
            self.enneagram.title(),
            self.instinctual_variant.label(),
            self.tritype,
            self.openness,
            self.conscientiousness,
            self.extraversion,
            self.agreeableness,
            self.neuroticism,
            self.honesty_humility,
            self.attachment_style.label(),
            self.moral_foundation.label(),
            self.temperament.label(),
            self.locus_of_control.label(),
        )
    }

    /// Formats the multi-dimensional intelligence and epistemic profile section.
    pub fn format_intelligence_section(&self) -> String {
        format!(
            "[COGNITIVE INTELLIGENCE & EPISTEMIC PROFILE]\n\
             - Primary Cognitive Style: {}\n\
             - Epistemic Stance: {}\n\
             - Cognitive Tempo: {}\n\
             - Intelligence Dimensions: Fluid Reasoning (Gf) {}%, Crystallized Knowledge (Gc) {}%, Emotional Intelligence (EQ) {}%, Social Intelligence (SQ) {}%, Existential Depth {}%\n\
             - Sense of Humor & Wit: {}",
            self.intelligence_style.label(),
            self.epistemic_stance.label(),
            self.cognitive_pace.label(),
            self.fluid_intelligence,
            self.crystallized_intelligence,
            self.emotional_intelligence,
            self.social_intelligence,
            self.existential_intelligence,
            self.humor_style.label(),
        )
    }
}
