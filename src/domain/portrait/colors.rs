//! Curated skin-tone and hair-color palettes: the only non-theme colors a portrait uses
//! (they fill the `{{skin}}` and `{{hair}}` template placeholders).

use serde::{Deserialize, Serialize};

/// Generates a named color enum (serialized as its snake_case key) with `ALL`, `key()` and `hex()`.
macro_rules! named_colors {
    ($(#[$meta:meta])* $name:ident { $first:ident($first_key:literal, $first_hex:literal) $(, $variant:ident($key:literal, $hex:literal))* $(,)? }) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub enum $name {
            #[default]
            $first,
            $($variant,)*
        }

        impl $name {
            /// Every color in swatch order.
            pub const ALL: &'static [Self] = &[Self::$first, $(Self::$variant,)*];

            /// Stable snake_case identifier (serde name and i18n suffix).
            pub const fn key(self) -> &'static str {
                match self {
                    Self::$first => $first_key,
                    $(Self::$variant => $key,)*
                }
            }

            /// `#rrggbb` fill substituted into portrait templates.
            pub const fn hex(self) -> &'static str {
                match self {
                    Self::$first => $first_hex,
                    $(Self::$variant => $hex,)*
                }
            }
        }
    };
}

named_colors! {
    /// Eight skin tones spanning very light to very deep, warm and neutral undertones.
    SkinTone {
        Sand("sand", "#e0ac7e"),
        Porcelain("porcelain", "#f7dcc8"),
        Ivory("ivory", "#efc6a2"),
        Honey("honey", "#c98d5d"),
        Caramel("caramel", "#a96f44"),
        Bronze("bronze", "#8a5532"),
        Umber("umber", "#683e24"),
        Ebony("ebony", "#46291a"),
    }
}

named_colors! {
    /// Ten hair colors: eight natural shades plus two dyed ones.
    HairColor {
        Chestnut("chestnut", "#5b3a29"),
        Jet("jet", "#1d1a1a"),
        Espresso("espresso", "#3a2a20"),
        Auburn("auburn", "#7c3524"),
        Copper("copper", "#b0582c"),
        Honey("honey", "#c99a5b"),
        Platinum("platinum", "#e6dcc2"),
        Silver("silver", "#b5b8bf"),
        Lilac("lilac", "#9d7cc9"),
        Teal("teal", "#2f8f8a"),
    }
}
