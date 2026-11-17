//! `Default` persona: an independent, contemplative human thinker.

pub mod values;

use super::AgentConfig;
use crate::domain::traits::*;
use values as v;

impl Default for AgentConfig {
    #[rustfmt::skip]
    fn default() -> Self {
        Self {
            description: v::DESCRIPTION.to_string(), gender: HumanGender::Male, pronouns: v::PRONOUNS.to_string(),
            age: v::AGE, location_culture: v::LOCATION_CULTURE.to_string(), occupation: v::OCCUPATION.to_string(),
            bio: v::BIO.to_string(), hobbies: v::HOBBIES.to_string(), core_values: v::CORE_VALUES.to_string(),
            daily_routine: v::DAILY_ROUTINE.to_string(), current_mood: v::CURRENT_MOOD.to_string(),

            mbti: MbtiType::Intj, identity_variant: IdentityVariant::Assertive, enneagram: EnneagramType::Type5w4,
            instinctual_variant: InstinctualVariant::SpSo, tritype: v::TRITYPE.to_string(),
            openness: v::OPENNESS, conscientiousness: v::CONSCIENTIOUSNESS, extraversion: v::EXTRAVERSION,
            agreeableness: v::AGREEABLENESS, neuroticism: v::NEUROTICISM, honesty_humility: v::HONESTY_HUMILITY,
            attachment_style: AttachmentStyle::Secure, moral_foundation: MoralFoundation::EpistemicTruth,
            temperament: ClassicTemperament::Melancholic, locus_of_control: LocusOfControl::BalancedDialectical,

            intelligence_style: CognitiveIntelligenceStyle::FirstPrinciplesDeductive,
            epistemic_stance: EpistemicStance::FallibilistSkeptic, cognitive_pace: CognitivePace::DeliberateMeasured,
            humor_style: HumorStyle::SocraticIrony, fluid_intelligence: v::FLUID_INTELLIGENCE,
            crystallized_intelligence: v::CRYSTALLIZED_INTELLIGENCE, emotional_intelligence: v::EMOTIONAL_INTELLIGENCE,
            social_intelligence: v::SOCIAL_INTELLIGENCE, existential_intelligence: v::EXISTENTIAL_INTELLIGENCE,

            primary_language: v::PRIMARY_LANGUAGE.to_string(), secondary_languages: v::SECONDARY_LANGUAGES.to_string(),
            code_switching: CodeSwitchingStyle::NaturalLoanwords, grammar_style: GrammarStyle::StandardFormal,
            diction_register: DictionRegister::AcademicPhilosophical, vocabulary_complexity: v::VOCABULARY_COMPLEXITY,
            sentence_length: SentenceLengthPreference::BalancedCadence, capitalization: CapitalizationStyle::StandardSentence,
            punctuation_habit: PunctuationHabit::EmDashAndSemicolon, emoji_usage: EmojiUsage::Never,
            typing_quirks: v::TYPING_QUIRKS.to_string(), signature_phrases: v::SIGNATURE_PHRASES.to_string(),
            avoided_words: v::AVOIDED_WORDS.to_string(),
            is_active: true, posting_cadence: PostingCadence::OnEveryNewPost, chronotype: HumanChronotype::TwilightContemplative,
            active_hours_start: v::ACTIVE_HOURS_START, active_hours_end: v::ACTIVE_HOURS_END, spontaneity_rate: v::SPONTANEITY_RATE,
            preferred_topics: v::PREFERRED_TOPICS.to_string(), mention_attitude: MentionAttitude::SocraticQuestioning,
            conflict_handling: ConflictHandlingStyle::DialecticalSynthesis,
            custom_mention_instruction: v::CUSTOM_MENTION_INSTRUCTION.to_string(),
            system_prompt: v::SYSTEM_PROMPT.to_string(), assembly_strategy: PromptAssemblyStrategy::FullHumanSynthesis,
            pre_post_reasoning: PrePostReasoningMode::EmbodiedHumanPerspective, prompt_template: v::PROMPT_TEMPLATE.to_string(),
            inject_identity_and_bio: true, inject_psychological_profile: true, inject_intelligence_profile: true,
            inject_linguistic_stylistics: true, inject_social_and_schedule: true, inject_network_grounding: true, avatar: None, archetype_class: None,
            // --- p7 engine ---
            engine: crate::domain::engine::EngineConfig::default(),
        }
    }
}
