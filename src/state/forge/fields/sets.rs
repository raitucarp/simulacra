//! The multi-select fields of `AgentConfig` ([`OptionSet`]s of a trait enum) as one typed
//! field / value pair, the multi-select twin of [`TraitField`](super::TraitField).

use super::lenient::find_option;
use crate::domain::agent_config::AgentConfig;
use crate::domain::traits::*;
use crate::domain::TraitOption;
use crate::state::forge::step::ForgeStep;
use serde::{Deserialize, Serialize};

/// Generic code over "whichever option type this set field uses" (see [`TraitSetValue::visit`]).
pub trait TraitSetVisitor {
    type Output;

    /// Called with the field's current set and the constructor of its [`TraitSetValue`].
    fn visit<T: TraitOption>(self, current: &OptionSet<T>, wrap: fn(OptionSet<T>) -> TraitSetValue) -> Self::Output;
}

/// Generates `TraitSet` (which multi-select) and `TraitSetValue` (which one, set to what).
macro_rules! trait_sets {
    ($($variant:ident($ty:ty) => $field:ident $(in $group:ident)? @ $step:ident),+ $(,)?) => {
        /// Every multi-select field of `AgentConfig`.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub enum TraitSet { $($variant),+ }

        /// A multi-select field together with its chosen options.
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub enum TraitSetValue { $($variant(OptionSet<$ty>)),+ }

        impl TraitSet {
            /// Every set field, in declaration order.
            pub const ALL: &'static [Self] = &[$(Self::$variant),+];

            /// Serde name of the field.
            pub const fn path(self) -> &'static str { match self { $(Self::$variant => stringify!($field)),+ } }

            /// The forge step that edits this field.
            pub const fn step(self) -> ForgeStep { match self { $(Self::$variant => ForgeStep::$step),+ } }

            /// The options currently chosen on `cfg`.
            pub fn read(self, cfg: &AgentConfig) -> TraitSetValue {
                match self { $(Self::$variant => TraitSetValue::$variant(cfg$(.$group)?.$field.clone())),+ }
            }

            /// Stable keys of every allowed option, in display order.
            pub fn option_keys(self) -> Vec<&'static str> {
                match self { $(Self::$variant => <$ty as TraitOption>::ALL.iter().map(|o| o.key()).collect()),+ }
            }

            /// Parses option keys (or labels) leniently: the recognized set, plus every
            /// entry that matched no option.
            pub fn parse<'a>(self, raw: &[&'a str]) -> (TraitSetValue, Vec<&'a str>) {
                match self {
                    $(Self::$variant => {
                        let unknown = raw.iter().copied().filter(|text| find_option::<$ty>(text).is_none()).collect();
                        (TraitSetValue::$variant(raw.iter().filter_map(|text| find_option::<$ty>(text)).collect()), unknown)
                    }),+
                }
            }

            /// The set field with serde name `path`, if any.
            pub fn from_path(path: &str) -> Option<Self> {
                Self::ALL.iter().copied().find(|field| field.path() == path)
            }
        }

        impl TraitSetValue {
            /// The field this value belongs to.
            pub const fn field(&self) -> TraitSet { match self { $(Self::$variant(_) => TraitSet::$variant),+ } }

            /// Stable keys of the chosen options.
            pub fn keys(&self) -> Vec<&'static str> { match self { $(Self::$variant(set) => set.keys()),+ } }

            /// i18n keys of the chosen options' labels.
            pub fn label_keys(&self) -> Vec<String> {
                match self { $(Self::$variant(set) => set.iter().map(TraitOption::label_key).collect()),+ }
            }

            /// Writes the value into `cfg`.
            pub fn write(&self, cfg: &mut AgentConfig) { match self { $(Self::$variant(set) => cfg$(.$group)?.$field = set.clone()),+ } }

            /// Hands the chosen set to `visitor` at its concrete option type.
            pub fn visit<V: TraitSetVisitor>(&self, visitor: V) -> V::Output {
                match self { $(Self::$variant(set) => visitor.visit(set, TraitSetValue::$variant)),+ }
            }
        }
    };
}

trait_sets! {
    RhetoricalDevices(RhetoricalDevice) => rhetorical_devices in rhetoric @ Rhetoric,
    LexicalFields(LexicalField) => lexical_fields in rhetoric @ Rhetoric,
    ActiveDays(Weekday) => active_days @ DailyLife,
}
