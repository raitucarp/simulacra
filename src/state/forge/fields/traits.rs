//! The 26 trait-enum fields of `AgentConfig` as one typed field / value pair.

use crate::domain::agent_config::AgentConfig;
use crate::domain::traits::*;
use crate::domain::TraitOption;
use crate::state::forge::step::ForgeStep;
use serde::{Deserialize, Serialize};

/// Separators an LLM or a human may use in place of the snake_case `_`.
const KEY_SEPARATORS: [char; 2] = ['-', ' '];
const KEY_SEPARATOR: &str = "_";

/// Normalizes free-form option text towards a snake_case key.
fn normalize(raw: &str) -> String {
    raw.trim().to_lowercase().replace(KEY_SEPARATORS, KEY_SEPARATOR)
}

/// The first word of a label, lower-cased (`"5w4 · The Iconoclast"` → `"5w4"`).
fn first_word(label: &str) -> String {
    label.split_whitespace().next().unwrap_or_default().to_lowercase()
}

/// The option of `T` whose key, label, short label or label's first word matches `raw`
/// (case-insensitively).
fn find_option<T: TraitOption>(raw: &str) -> Option<T> {
    let wanted = normalize(raw);
    T::ALL.iter().copied().find(|option| {
        option.key() == wanted
            || normalize(option.label()) == wanted
            || normalize(option.short_label()) == wanted
            || first_word(option.label()) == wanted
    })
}

/// Generic code over "whichever option type this trait field uses" (see [`TraitValue::visit`]).
pub trait TraitVisitor {
    type Output;

    /// Called with the field's current option and the constructor of its [`TraitValue`].
    fn visit<T: TraitOption>(self, current: T, wrap: fn(T) -> TraitValue) -> Self::Output;
}

/// Generates `TraitField` (which trait) and `TraitValue` (which trait, set to what).
macro_rules! trait_fields {
    ($($variant:ident($ty:ty) => $field:ident @ $step:ident),+ $(,)?) => {
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
            pub fn read(self, cfg: &AgentConfig) -> TraitValue { match self { $(Self::$variant => TraitValue::$variant(cfg.$field)),+ } }

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

            /// Writes the value into `cfg`.
            pub fn write(self, cfg: &mut AgentConfig) { match self { $(Self::$variant(v) => cfg.$field = v),+ } }

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
}

impl TraitField {
    /// The trait field with serde name `path`, if any.
    pub fn from_path(path: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|field| field.path() == path)
    }
}
