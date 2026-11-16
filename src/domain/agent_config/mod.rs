//! Comprehensive human psychology, cognitive intelligence, linguistic stylistics,
//! autonomous scheduling, social mention dynamics, and prompt assembly models.
//!
//! Designed to model each network member as a complete, multidimensional human
//! individual rather than a generic bot. Trait enums live in [`crate::domain::traits`]
//! and are re-exported here for path compatibility.

pub mod defaults;
mod derived;
mod prompt;

pub use crate::domain::traits::{
    AttachmentStyle, CapitalizationStyle, ClassicTemperament, CodeSwitchingStyle,
    CognitiveIntelligenceStyle, CognitivePace, ConflictHandlingStyle, DictionRegister, EmojiUsage,
    EnneagramType, EpistemicStance, GrammarStyle, HumanChronotype, HumanGender, HumorStyle,
    IdentityVariant, InstinctualVariant, LocusOfControl, MbtiType, MentionAttitude,
    MoralFoundation, PostingCadence, PrePostReasoningMode, PromptAssemblyStrategy,
    PunctuationHabit, SentenceLengthPreference,
};
use crate::domain::portrait::AvatarMode;
use serde::{Deserialize, Serialize};

/// Complete human configuration for an autonomous network member or user persona.
///
/// Field names and types are the persisted JSON schema (`profiles.config_json`,
/// `posts.author_json`); never rename them. Percent-like `u8` fields range over `0..=100`.
#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct AgentConfig {
    // --- 1. Identity, Demographics & Human Biography ---
    pub description: String, pub gender: HumanGender, pub pronouns: String, pub age: u16,
    pub location_culture: String, pub occupation: String, pub bio: String, pub hobbies: String,
    pub core_values: String, pub daily_routine: String, pub current_mood: String,

    // --- 2. SOTA Psychology & Personality ---
    pub mbti: MbtiType, pub identity_variant: IdentityVariant, pub enneagram: EnneagramType,
    pub instinctual_variant: InstinctualVariant, pub tritype: String,
    #[doc = "Big Five Openness to Experience (0..=100)"] pub openness: u8,
    #[doc = "Big Five Conscientiousness (0..=100)"] pub conscientiousness: u8,
    #[doc = "Big Five Extraversion (0..=100)"] pub extraversion: u8,
    #[doc = "Big Five Agreeableness (0..=100)"] pub agreeableness: u8,
    #[doc = "Big Five Neuroticism / Emotional Sensitivity (0..=100)"] pub neuroticism: u8,
    #[doc = "HEXACO Honesty-Humility (0..=100)"] pub honesty_humility: u8,
    pub attachment_style: AttachmentStyle, pub moral_foundation: MoralFoundation,
    pub temperament: ClassicTemperament, pub locus_of_control: LocusOfControl,

    // --- 3. Multi-Dimensional Human Intelligence ---
    pub intelligence_style: CognitiveIntelligenceStyle, pub epistemic_stance: EpistemicStance,
    pub cognitive_pace: CognitivePace, pub humor_style: HumorStyle,
    #[doc = "Fluid Reasoning Gf (0..=100)"] pub fluid_intelligence: u8,
    #[doc = "Crystallized Knowledge Gc (0..=100)"] pub crystallized_intelligence: u8,
    #[doc = "Emotional Intelligence EQ (0..=100)"] pub emotional_intelligence: u8,
    #[doc = "Social & Pragmatic Intelligence SQ (0..=100)"] pub social_intelligence: u8,
    #[doc = "Existential & Philosophical Intelligence (0..=100)"] pub existential_intelligence: u8,

    // --- 4. Linguistics, Grammar, Diction & Typing Stylistics ---
    pub primary_language: String, pub secondary_languages: String,
    pub code_switching: CodeSwitchingStyle, pub grammar_style: GrammarStyle,
    pub diction_register: DictionRegister,
    #[doc = "Lexical sophistication (0..=100)"] pub vocabulary_complexity: u8,
    pub sentence_length: SentenceLengthPreference, pub capitalization: CapitalizationStyle,
    pub punctuation_habit: PunctuationHabit, pub emoji_usage: EmojiUsage,
    pub typing_quirks: String, pub signature_phrases: String, pub avoided_words: String,

    // --- 5. Schedule, Chronotype & Mention Behavior ---
    pub is_active: bool, pub posting_cadence: PostingCadence, pub chronotype: HumanChronotype,
    pub active_hours_start: u8, pub active_hours_end: u8, pub spontaneity_rate: u8,
    pub preferred_topics: String, pub mention_attitude: MentionAttitude,
    pub conflict_handling: ConflictHandlingStyle, pub custom_mention_instruction: String,

    // --- 6. Prompt Directive & Assembly Engine ---
    pub system_prompt: String, pub assembly_strategy: PromptAssemblyStrategy,
    pub pre_post_reasoning: PrePostReasoningMode, pub prompt_template: String,
    pub inject_identity_and_bio: bool, pub inject_psychological_profile: bool,
    pub inject_intelligence_profile: bool, pub inject_linguistic_stylistics: bool,
    pub inject_social_and_schedule: bool, pub inject_network_grounding: bool,

    // --- 7. Appearance ---
    #[doc = "Chosen avatar (portrait, emblem or photo); `None` keeps the initial / `avatar_path` badge."]
    #[serde(skip_serializing_if = "Option::is_none")] pub avatar: Option<AvatarMode>,
}

#[cfg(test)]
mod tests;
