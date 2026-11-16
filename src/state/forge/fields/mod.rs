//! Typed handles on every editable `Profile` / `AgentConfig` field, each owned by one forge step.
//!
//! A field's `path` is its serde name; `Profile` and `AgentConfig` share no field names, so the
//! bare name is unambiguous. The step tables here are the single source of the field → step map.

mod map;
mod traits;

pub use map::{field_label_key, field_step, fields_for_step, mapped_fields, SYSTEM_FIELDS};
pub use traits::{TraitField, TraitValue};

use super::step::ForgeStep;
use crate::domain::agent_config::AgentConfig;
use crate::domain::Profile;
use serde::{Deserialize, Serialize};

/// Generates a field enum over values of one type: `Variant => [config] field @ Step`.
macro_rules! forge_fields {
    ($(#[$meta:meta])* $name:ident: $ty:ty {
        $($variant:ident => $([$scope:ident])? $field:ident @ $step:ident),+ $(,)?
    }) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub enum $name { $($variant),+ }

        impl $name {
            /// Every field of this kind, in declaration order.
            pub const ALL: &'static [Self] = &[$(Self::$variant),+];

            /// Serde name of the field.
            pub const fn path(self) -> &'static str { match self { $(Self::$variant => stringify!($field)),+ } }

            /// The forge step that edits this field.
            pub const fn step(self) -> ForgeStep { match self { $(Self::$variant => ForgeStep::$step),+ } }

            /// Current value of the field on `profile`.
            pub fn get(self, profile: &Profile) -> &$ty { match self { $(Self::$variant => &profile$(.$scope)?.$field),+ } }

            /// Mutable access to the field on `profile`.
            pub fn get_mut(self, profile: &mut Profile) -> &mut $ty { match self { $(Self::$variant => &mut profile$(.$scope)?.$field),+ } }
        }
    };
    (config $(#[$meta:meta])* $name:ident: $ty:ty { $($variant:ident => $field:ident @ $step:ident),+ $(,)? }) => {
        forge_fields! { $(#[$meta])* $name: $ty { $($variant => [config] $field @ $step),+ } }

        impl $name {
            /// Current value of the field on a bare config.
            pub fn of(self, cfg: &AgentConfig) -> &$ty { match self { $(Self::$variant => &cfg.$field),+ } }

            /// Mutable access to the field on a bare config.
            pub fn of_mut(self, cfg: &mut AgentConfig) -> &mut $ty { match self { $(Self::$variant => &mut cfg.$field),+ } }
        }
    };
}

forge_fields! {
    /// Free-text fields (whitespace kept while typing; normalized on commit).
    TextField: String {
        Name => name @ Origin,
        Handle => handle @ Origin,
        Description => [config] description @ Origin,
        Pronouns => [config] pronouns @ Origin,
        LocationCulture => [config] location_culture @ Origin,
        Occupation => [config] occupation @ Origin,
        Bio => [config] bio @ Origin,
        AvatarInitial => avatar_initial @ Appearance,
        Tritype => [config] tritype @ Birthsign,
        CoreValues => [config] core_values @ Disposition,
        CurrentMood => [config] current_mood @ Disposition,
        PrimaryLanguage => [config] primary_language @ Voice,
        SecondaryLanguages => [config] secondary_languages @ Voice,
        TypingQuirks => [config] typing_quirks @ Voice,
        SignaturePhrases => [config] signature_phrases @ Voice,
        AvoidedWords => [config] avoided_words @ Voice,
        Hobbies => [config] hobbies @ DailyLife,
        DailyRoutine => [config] daily_routine @ DailyLife,
        PreferredTopics => [config] preferred_topics @ DailyLife,
        CustomMentionInstruction => [config] custom_mention_instruction @ Relationships,
        SystemPrompt => [config] system_prompt @ Grimoire,
        PromptTemplate => [config] prompt_template @ Grimoire,
    }
}

forge_fields! {
    /// Optional free-text fields.
    OptionalTextField: Option<String> {
        AvatarPath => avatar_path @ Appearance,
    }
}

forge_fields! {
    config
    /// Percent-like scores in `0..=MAX_SCORE`.
    ScoreField: u8 {
        Openness => openness @ Attributes,
        Conscientiousness => conscientiousness @ Attributes,
        Extraversion => extraversion @ Attributes,
        Agreeableness => agreeableness @ Attributes,
        Neuroticism => neuroticism @ Attributes,
        HonestyHumility => honesty_humility @ Attributes,
        FluidIntelligence => fluid_intelligence @ Mind,
        CrystallizedIntelligence => crystallized_intelligence @ Mind,
        EmotionalIntelligence => emotional_intelligence @ Mind,
        SocialIntelligence => social_intelligence @ Mind,
        ExistentialIntelligence => existential_intelligence @ Mind,
        VocabularyComplexity => vocabulary_complexity @ Voice,
        SpontaneityRate => spontaneity_rate @ DailyLife,
    }
}

forge_fields! {
    /// Hours of the day in `0..=MAX_HOUR`.
    HourField: u8 {
        ActiveHoursStart => [config] active_hours_start @ DailyLife,
        ActiveHoursEnd => [config] active_hours_end @ DailyLife,
    }
}

forge_fields! {
    /// Whole-number counts.
    CountField: u16 {
        Age => [config] age @ Origin,
    }
}

forge_fields! {
    /// On/off switches.
    ToggleField: bool {
        IsActive => [config] is_active @ DailyLife,
        InjectIdentityAndBio => [config] inject_identity_and_bio @ Grimoire,
        InjectPsychologicalProfile => [config] inject_psychological_profile @ Grimoire,
        InjectIntelligenceProfile => [config] inject_intelligence_profile @ Grimoire,
        InjectLinguisticStylistics => [config] inject_linguistic_stylistics @ Grimoire,
        InjectSocialAndSchedule => [config] inject_social_and_schedule @ Grimoire,
        InjectNetworkGrounding => [config] inject_network_grounding @ Grimoire,
    }
}

/// Upper bound of every [`ScoreField`].
pub const MAX_SCORE: u8 = 100;
/// Last hour of the day for [`HourField`]s.
pub const MAX_HOUR: u8 = 23;
/// Upper bound of [`CountField::Age`].
pub const MAX_AGE: u16 = 150;
