//! The fourteen base colors of a palette, their typed keys and partial overrides.

use super::hex_serde::{self, HexColor};
use gpui_kit::Rgba;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Generates [`PaletteColors`] and [`PaletteColorKey`] from one `Variant => field` list so
/// the struct, the key enum and its accessors can never drift apart.
macro_rules! palette_colors {
    ($($variant:ident => $field:ident),* $(,)?) => {
        /// Base colors every theme is derived from (JSON `colors` object, `#rrggbb` strings).
        #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
        pub struct PaletteColors {
            $(#[serde(with = "hex_serde")] pub $field: Rgba,)*
        }

        /// Typed name of one base color (serialized as its JSON field name).
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub enum PaletteColorKey {
            $($variant,)*
        }

        impl PaletteColorKey {
            /// Every key, in JSON field order.
            pub const ALL: &'static [PaletteColorKey] = &[$(PaletteColorKey::$variant,)*];

            /// JSON field name, also the i18n suffix of the key's label.
            pub const fn id(self) -> &'static str {
                match self {
                    $(PaletteColorKey::$variant => stringify!($field),)*
                }
            }

            /// This key's color in `colors`.
            pub fn get(self, colors: &PaletteColors) -> Rgba {
                match self {
                    $(PaletteColorKey::$variant => colors.$field,)*
                }
            }

            /// Replaces this key's color in `colors`.
            pub fn set(self, colors: &mut PaletteColors, value: Rgba) {
                match self {
                    $(PaletteColorKey::$variant => colors.$field = value,)*
                }
            }
        }
    };
}

palette_colors! {
    Bg => bg, Surface => surface, Elevated => elevated, Border => border,
    Fg => fg, FgMuted => fg_muted, Accent => accent, Accent2 => accent_2, Agent => agent,
    Success => success, Warning => warning, Danger => danger, Info => info,
    Selection => selection,
}

impl PaletteColors {
    /// These colors with every override applied.
    pub fn with_overrides(mut self, overrides: &PaletteOverrides) -> Self {
        for (key, color) in &overrides.0 {
            key.set(&mut self, color.0);
        }
        self
    }
}

/// A partial palette: only the base colors the user changed (`{"accent":"#ff0000"}`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PaletteOverrides(BTreeMap<PaletteColorKey, HexColor>);

impl PaletteOverrides {
    /// True when no color is overridden.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Overrides `key` with `color`.
    pub fn set(&mut self, key: PaletteColorKey, color: Rgba) {
        self.0.insert(key, HexColor(color));
    }

    /// The override of `key`, if any.
    pub fn get(&self, key: PaletteColorKey) -> Option<Rgba> {
        self.0.get(&key).map(|color| color.0)
    }

    /// Drops every override.
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Number of overridden colors.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
