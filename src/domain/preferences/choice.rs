//! One-of-many preferences: the [`Choice`] trait and the `choice_enum!` generator.
//!
//! Every choice is stored by its stable wire key (`"day_month_year"`). Reading tolerates
//! keys it does not know (a newer or hand-edited vault): they fall back to the default
//! instead of discarding the whole category.

/// A preference with a fixed set of options, each with a stable wire key.
pub trait Choice: Copy + PartialEq + Default + 'static {
    /// Every option's wire key, in display order.
    const KEYS: &'static [&'static str];

    /// The wire key of this option.
    fn key(self) -> &'static str;

    /// The option stored under `key`.
    fn from_key(key: &str) -> Option<Self>;
}

/// Generates a [`Choice`] enum with lenient (fallback-to-default) serde by wire key.
macro_rules! choice_enum {
    (
        $(#[$doc:meta])* $name:ident, default: $default:ident,
        [$($(#[$variant_doc:meta])* $variant:ident => $key:literal),+ $(,)?]
    ) => {
        $(#[$doc])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {
            $($(#[$variant_doc])* $variant,)+
        }

        impl $name {
            /// Every option, in display order.
            pub const ALL: &'static [$name] = &[$($name::$variant,)+];
        }

        impl Default for $name {
            fn default() -> Self {
                $name::$default
            }
        }

        impl $crate::domain::preferences::Choice for $name {
            const KEYS: &'static [&'static str] = &[$($key,)+];

            fn key(self) -> &'static str {
                match self {
                    $($name::$variant => $key,)+
                }
            }

            fn from_key(key: &str) -> Option<Self> {
                Self::ALL.iter().copied().find(|option| $crate::domain::preferences::Choice::key(*option) == key)
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str($crate::domain::preferences::Choice::key(*self))
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let raw = <String as serde::Deserialize>::deserialize(deserializer)?;
                Ok(<Self as $crate::domain::preferences::Choice>::from_key(&raw).unwrap_or_default())
            }
        }
    };
}

pub(crate) use choice_enum;
