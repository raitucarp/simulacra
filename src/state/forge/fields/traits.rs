//! The single-choice trait-enum fields of `AgentConfig` as one typed field / value pair
//! (`field in group` reaches into a flattened sub-struct such as `embodiment`).

use crate::domain::agent_config::AgentConfig;
use crate::domain::traits::*;
use crate::domain::TraitOption;
use super::lenient::find_option;
use crate::state::forge::step::ForgeStep;
use serde::{Deserialize, Serialize};

/// Generic code over "whichever option type this trait field uses" (see [`TraitValue::visit`]).
pub trait TraitVisitor {
    type Output;

    /// Called with the field's current option and the constructor of its [`TraitValue`].
    fn visit<T: TraitOption>(self, current: T, wrap: fn(T) -> TraitValue) -> Self::Output;
}

/// Generates `TraitField` (which trait) and `TraitValue` (which trait, set to what).
macro_rules! trait_fields {
    ($($variant:ident($ty:ty) => $field:ident $(in $group:ident)? @ $step:ident),+ $(,)?) => {
        /// Every trait-enum field of `AgentConfig`.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub enum TraitField { $($variant),+ }

        /// A trait-enum field together with its chosen option.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub enum TraitValue { $($variant($ty)),+ }

        impl TraitField {
            /// Every trait field, in declaration order.
            pub const ALL: &'static [Self] = &[$(Self::$variant),+];

            /// Serde name of the field.
            pub const fn path(self) -> &'static str { match self { $(Self::$variant => stringify!($field)),+ } }

            /// The forge step that edits this field.
            pub const fn step(self) -> ForgeStep { match self { $(Self::$variant => ForgeStep::$step),+ } }

            /// The value currently set on `cfg`.
            pub fn read(self, cfg: &AgentConfig) -> TraitValue { match self { $(Self::$variant => TraitValue::$variant(cfg$(.$group)?.$field)),+ } }

            /// Stable keys of every allowed option, in display order.
            pub fn option_keys(self) -> Vec<&'static str> {
                match self { $(Self::$variant => <$ty as TraitOption>::ALL.iter().map(|o| o.key()).collect()),+ }
            }

            /// Parses an option key (or label) leniently; `None` when nothing matches.
            pub fn parse(self, raw: &str) -> Option<TraitValue> {
                match self { $(Self::$variant => find_option::<$ty>(raw).map(TraitValue::$variant)),+ }
            }
        }

        impl TraitValue {
            /// The field this value belongs to.
            pub const fn field(self) -> TraitField { match self { $(Self::$variant(_) => TraitField::$variant),+ } }

            /// Stable key of the chosen option.
            pub fn key(self) -> &'static str { match self { $(Self::$variant(v) => TraitOption::key(v)),+ } }

            /// i18n key of the chosen option's label.
            pub fn label_key(self) -> String { match self { $(Self::$variant(v) => TraitOption::label_key(v)),+ } }

            /// Writes the value into `cfg` as is (see [`TraitValue::write`]).
            fn write_raw(self, cfg: &mut AgentConfig) { match self { $(Self::$variant(v) => cfg$(.$group)?.$field = v),+ } }

            /// Hands the chosen option to `visitor` at its concrete type, with the constructor
            /// that turns any option of that type back into a value of this field.
            pub fn visit<V: TraitVisitor>(self, visitor: V) -> V::Output {
                match self { $(Self::$variant(v) => visitor.visit(v, TraitValue::$variant)),+ }
            }
        }
    };
}

trait_fields! {
    Gender(HumanGender) => gender @ Origin,
    Mbti(MbtiType) => mbti @ Birthsign,
    IdentityVariant(IdentityVariant) => identity_variant @ Birthsign,
    Enneagram(EnneagramType) => enneagram @ Birthsign,
    InstinctualVariant(InstinctualVariant) => instinctual_variant @ Birthsign,
    AttachmentStyle(AttachmentStyle) => attachment_style @ Disposition,
    MoralFoundation(MoralFoundation) => moral_foundation @ Disposition,
    Temperament(ClassicTemperament) => temperament @ Disposition,
    LocusOfControl(LocusOfControl) => locus_of_control @ Disposition,
    HumorStyle(HumorStyle) => humor_style @ Disposition,
    IntelligenceStyle(CognitiveIntelligenceStyle) => intelligence_style @ Mind,
    EpistemicStance(EpistemicStance) => epistemic_stance @ Mind,
    CognitivePace(CognitivePace) => cognitive_pace @ Mind,
    CodeSwitching(CodeSwitchingStyle) => code_switching @ Voice,
    GrammarStyle(GrammarStyle) => grammar_style @ Voice,
    DictionRegister(DictionRegister) => diction_register @ Voice,
    SentenceLength(SentenceLengthPreference) => sentence_length @ Voice,
    Capitalization(CapitalizationStyle) => capitalization @ Voice,
    PunctuationHabit(PunctuationHabit) => punctuation_habit @ Voice,
    EmojiUsage(EmojiUsage) => emoji_usage @ Voice,
    PostingCadence(PostingCadence) => posting_cadence @ DailyLife,
    Chronotype(HumanChronotype) => chronotype @ DailyLife,
    MentionAttitude(MentionAttitude) => mention_attitude @ Relationships,
    ConflictHandling(ConflictHandlingStyle) => conflict_handling @ Relationships,
    AssemblyStrategy(PromptAssemblyStrategy) => assembly_strategy @ Grimoire,
    PrePostReasoning(PrePostReasoningMode) => pre_post_reasoning @ Grimoire,
    // --- p7 ontology ---
    BodyKind(BodyKind) => body_kind in embodiment @ Embodiment,
    Form(BodyForm) => form in embodiment @ Embodiment,
    Material(Material) => material in embodiment @ Embodiment,
    Scale(Scale) => scale in embodiment @ Embodiment,
    Temporality(Temporality) => temporality in embodiment @ Embodiment,
    Perception(Perception) => perception in embodiment @ Embodiment,
    Mobility(Mobility) => mobility in embodiment @ Embodiment,
    Anthropomorphism(Anthropomorphism) => anthropomorphism in embodiment @ Embodiment,
    RelationToHumans(RelationToHumans) => relation_to_humans in embodiment @ Embodiment,
    Withdrawal(Withdrawal) => withdrawal in embodiment @ Embodiment,
    RhetoricalMode(RhetoricalMode) => rhetorical_mode in rhetoric @ Rhetoric,
    StylisticSchool(StylisticSchool) => stylistic_school in rhetoric @ Rhetoric,
    SemioticMode(SemioticMode) => semiotic_mode in rhetoric @ Rhetoric,
    SentenceStructure(SentenceStructure) => sentence_structure in rhetoric @ Rhetoric,
    PostFormat(PostFormat) => post_format in rhetoric @ Rhetoric,
}

impl TraitValue {
    /// Writes the value into `cfg`. A body kind and its form stay coherent: setting the form
    /// sets its kind, and a new kind replaces a form of another kind with its own first form.
    pub fn write(self, cfg: &mut AgentConfig) {
        match self {
            Self::BodyKind(kind) => cfg.embodiment.set_kind(kind),
            Self::Form(form) => cfg.embodiment.set_form(form),
            other => other.write_raw(cfg),
        }
    }
}

impl TraitField {
    /// The trait field with serde name `path`, if any.
    pub fn from_path(path: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|field| field.path() == path)
    }
}
