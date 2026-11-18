//! The former hard-coded presets, now derived from their bundled palettes.

use super::derive::derive_default_tokens;
use super::registry::{ThemeRegistry, MIDNIGHT_SLATE_ID, OBSIDIAN_ZINC_ID};
use super::tokens::ThemeTokens;

/// Default-scale tokens of the bundled palette `id` (the default palette when unknown).
fn preset(id: &str) -> ThemeTokens {
    let registry = ThemeRegistry::global();
    derive_default_tokens(registry.get(id).unwrap_or_else(|| registry.default_palette()))
}

/// Simulacra Midnight: OLED black surfaces with violet brand accents (the default theme).
pub fn minimalist_dark() -> ThemeTokens {
    derive_default_tokens(ThemeRegistry::global().default_palette())
}

/// Volcanic Zinc palette (id `obsidian-zinc`, kept for saved settings).
pub fn obsidian_zinc() -> ThemeTokens {
    preset(OBSIDIAN_ZINC_ID)
}

/// Midnight Slate palette.
pub fn midnight_slate() -> ThemeTokens {
    preset(MIDNIGHT_SLATE_ID)
}
