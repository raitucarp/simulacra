//! Design system: palettes, the bundled palette registry, token derivation, persisted theme
//! settings and the bridge to gpui-component's own theme.

/// Assigns a table of `field: value` pairs onto a mutable [`ThemeTokens`].
macro_rules! set_tokens {
    ($theme:ident { $($field:ident: $value:expr),* $(,)? }) => {
        $($theme.$field = $value;)*
    };
}

pub mod color;
pub mod derive;
pub mod kit;
pub mod palette;
pub mod palettes;
pub mod registry;
pub mod tokens;

pub use derive::{derive_tokens, Density, RadiusScale, TypeScale};
pub use kit::{kit_theme_config, sync_kit_theme};
pub use palette::{PaletteColorKey, PaletteColors, PaletteMode, PaletteOverrides, ThemePalette};
pub use palettes::{midnight_slate, minimalist_dark, obsidian_zinc};
pub use registry::ThemeRegistry;
pub use tokens::ThemeTokens;
