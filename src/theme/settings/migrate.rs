//! Migration of the pre-palette `theme_tokens` setting into [`ThemeSettings`].
//!
//! Tokens that still equal one of the three former hard-coded presets select that preset's
//! palette; anything else is kept as a "Custom (migrated)" palette so the user's look survives.

use super::ThemeSettings;
use crate::theme::color::{mix, quantized, relative_luminance};
use crate::theme::derive::DEFAULT_FONT_FAMILY;
use crate::theme::palette::{PaletteColors, PaletteMode, ThemePalette};
use crate::theme::registry::{DEFAULT_PALETTE_ID, MIDNIGHT_SLATE_ID, OBSIDIAN_ZINC_ID};
use crate::theme::ThemeTokens;
use gpui_kit::rgb;

/// Id and display name of the palette rebuilt from customized legacy tokens.
pub const MIGRATED_PALETTE_ID: &str = "custom-migrated";
pub const MIGRATED_PALETTE_NAME: &str = "Custom (migrated)";
/// Attribution of the migrated palette.
const MIGRATED_CREDIT: &str = "Migrated from an earlier Simulacra theme";
/// How far the selection color moves from the background toward the accent.
const MIGRATED_SELECTION_MIX: f32 = 0.35;

/// `(bg_app, accent_brand, palette id)` of each former hard-coded preset.
const LEGACY_PRESETS: [(u32, u32, &str); 3] = [
    (0x000000, 0x8b5cf6, DEFAULT_PALETTE_ID),
    (0x09090b, 0xa855f7, OBSIDIAN_ZINC_ID),
    (0x0a0e17, 0x38bdf8, MIDNIGHT_SLATE_ID),
];

/// Where a legacy token set lands.
#[derive(Debug, Clone, PartialEq)]
pub enum LegacyTheme {
    /// The tokens were an untouched preset: select this bundled palette.
    Preset(&'static str),
    /// The tokens were customized: keep them as this custom palette.
    Custom(ThemePalette),
}

/// Classifies legacy tokens as an untouched preset or a custom palette.
pub fn migrate_legacy_tokens(tokens: &ThemeTokens) -> LegacyTheme {
    let (bg, accent) = (quantized(tokens.bg_app), quantized(tokens.accent_brand));
    LEGACY_PRESETS
        .iter()
        .find(|(legacy_bg, legacy_accent, _)| bg == rgb(*legacy_bg) && accent == rgb(*legacy_accent))
        .map(|(_, _, id)| LegacyTheme::Preset(id))
        .unwrap_or_else(|| LegacyTheme::Custom(palette_from_tokens(tokens)))
}

/// Rebuilds the fourteen base colors from a full legacy token set.
fn palette_from_tokens(t: &ThemeTokens) -> ThemePalette {
    let is_dark = relative_luminance(t.bg_app) < relative_luminance(t.text_primary);
    let colors = PaletteColors {
        bg: t.bg_app, surface: t.bg_card, elevated: t.bg_elevated, border: t.border,
        fg: t.text_primary, fg_muted: t.text_muted, accent: t.accent_brand,
        accent_2: t.accent_user, agent: t.accent_agent, success: t.accent_success,
        warning: t.accent_dialectic, danger: t.accent_danger, info: t.accent_user,
        selection: quantized(mix(t.bg_app, t.accent_brand, MIGRATED_SELECTION_MIX)),
    };
    ThemePalette {
        id: MIGRATED_PALETTE_ID.to_string(),
        name: MIGRATED_PALETTE_NAME.to_string(),
        mode: if is_dark { PaletteMode::Dark } else { PaletteMode::Light },
        credit: MIGRATED_CREDIT.to_string(),
        based_on: None,
        colors,
    }
}

impl ThemeSettings {
    /// Applies legacy tokens: selects the matching preset or adds and selects the migrated
    /// custom palette, and keeps a non-default interface font.
    pub fn adopt_legacy_tokens(&mut self, tokens: &ThemeTokens) {
        match migrate_legacy_tokens(tokens) {
            LegacyTheme::Preset(id) => self.select(id),
            LegacyTheme::Custom(palette) => {
                self.custom.retain(|p| p.id != palette.id);
                let id = palette.id.clone();
                self.custom.push(palette);
                self.select(&id);
            }
        }
        let font = tokens.font_family_ui.trim();
        if !font.is_empty() && font != DEFAULT_FONT_FAMILY {
            self.font_family = font.to_string();
        }
    }
}
