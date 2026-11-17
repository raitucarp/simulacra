//! `keyed_enum!`: closed vocabularies whose variants carry a stable wire key.

/// Declares an enum whose variants serialize as fixed keys, with `ALL`, `key()` and
/// `from_key()`. Keys are persisted (SQLite columns, JSON, locale keys): never rename one.
macro_rules! keyed_enum {
    ($(#[$meta:meta])* $name:ident { $($(#[$vmeta:meta])* $variant:ident => $key:literal),+ $(,)? }) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
        pub enum $name {
            $($(#[$vmeta])* #[serde(rename = $key)] $variant,)+
        }

        impl $name {
            /// Every variant, in display order.
            pub const ALL: &'static [Self] = &[$(Self::$variant,)+];

            /// Stable wire key.
            pub const fn key(self) -> &'static str {
                match self {
                    $(Self::$variant => $key,)+
                }
            }

            /// The variant whose key is `key`.
            pub fn from_key(key: &str) -> Option<Self> {
                Self::ALL.iter().copied().find(|variant| variant.key() == key)
            }
        }
    };
}

pub(crate) use keyed_enum;
