//! Shared contract for every selectable human-trait enum, plus the `trait_enum!` generator.
//!
//! Each trait enum exposes a stable snake_case `key` (used for i18n and illustration
//! lookups) alongside its human-readable `label`. Variant identifiers remain the serde
//! wire format persisted in SQLite and must never change.

/// Root namespace for trait i18n keys (`traits.{group}.{key}`).
pub const TRAIT_I18N_ROOT: &str = "traits";
/// Suffix appended to a trait label key to address its description.
pub const TRAIT_DESCRIPTION_SUFFIX: &str = "desc";
/// Root directory for trait illustrations (`illustrations/{group}/{key}.svg`).
pub const TRAIT_ILLUSTRATION_ROOT: &str = "illustrations";
/// File extension of trait illustrations.
pub const TRAIT_ILLUSTRATION_EXT: &str = "svg";

/// A closed set of selectable options describing one dimension of a human persona.
pub trait TraitOption: Copy + Sized + 'static {
    /// Every option, in display order.
    const ALL: &'static [Self];
    /// Stable snake_case identifier of the option group.
    const GROUP: &'static str;

    /// Stable snake_case identifier of this option within its group.
    fn key(self) -> &'static str;

    /// Full human-readable label.
    fn label(self) -> &'static str;

    /// Compact label for dense UI; defaults to the full label.
    fn short_label(self) -> &'static str {
        self.label()
    }

    /// i18n key of the label: `traits.{GROUP}.{key}`.
    fn label_key(self) -> String {
        format!("{TRAIT_I18N_ROOT}.{}.{}", Self::GROUP, self.key())
    }

    /// i18n key of the description: `traits.{GROUP}.{key}.desc`.
    fn description_key(self) -> String {
        format!("{}.{TRAIT_DESCRIPTION_SUFFIX}", self.label_key())
    }

    /// Asset path of the illustration: `illustrations/{GROUP}/{key}.svg`.
    fn illustration(self) -> String {
        format!(
            "{TRAIT_ILLUSTRATION_ROOT}/{}/{}.{TRAIT_ILLUSTRATION_EXT}",
            Self::GROUP,
            self.key()
        )
    }
}

/// Counts identifiers at compile time (used for `ALL` array lengths).
macro_rules! count_idents {
    () => { 0usize };
    ($head:ident $($tail:ident)*) => { 1usize + $crate::domain::trait_option::count_idents!($($tail)*) };
}

/// Generates a trait enum with `ALL`, `label`, `short_label`, `TraitOption` and optional `Display`.
///
/// Syntax: `pub enum Name in "group" [with display] { #[default] Variant("key", "Label" [, short: "Short"]), … }`
macro_rules! trait_enum {
    (@first $value:literal $($fallback:literal)?) => { $value };
    (@display $name:ident) => {
        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", self.label())
            }
        }
    };
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident in $group:literal $(with $display:ident)? {
            $( $(#[$vmeta:meta])* $variant:ident ( $key:literal, $label:literal $(, short: $short:literal)? ) ),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ::serde::Serialize, ::serde::Deserialize, Default)]
        $vis enum $name { $( $(#[$vmeta])* $variant, )+ }

        impl $name {
            pub const ALL: [Self; $crate::domain::trait_option::count_idents!($($variant)+)] = [$(Self::$variant),+];

            #[inline]
            pub const fn label(self) -> &'static str {
                match self { $(Self::$variant => $label,)+ }
            }

            #[inline]
            pub const fn short_label(self) -> &'static str {
                match self { $(Self::$variant => $crate::domain::trait_option::trait_enum!(@first $($short)? $label),)+ }
            }
        }

        impl $crate::domain::trait_option::TraitOption for $name {
            const ALL: &'static [Self] = &<$name>::ALL;
            const GROUP: &'static str = $group;

            #[inline]
            fn key(self) -> &'static str {
                match self { $(Self::$variant => $key,)+ }
            }

            #[inline]
            fn label(self) -> &'static str { <$name>::label(self) }

            #[inline]
            fn short_label(self) -> &'static str { <$name>::short_label(self) }
        }

        $( $crate::domain::trait_option::trait_enum!(@$display $name); )?
    };
}

/// Generates a `const fn` lookup table returning one `&'static str` per variant.
macro_rules! trait_table {
    ($(#[$meta:meta])* $name:ident :: $method:ident { $($variant:ident => $value:literal),+ $(,)? }) => {
        impl $name {
            $(#[$meta])*
            #[inline]
            pub const fn $method(self) -> &'static str {
                match self { $(Self::$variant => $value,)+ }
            }
        }
    };
}

pub(crate) use {count_idents, trait_enum, trait_table};
