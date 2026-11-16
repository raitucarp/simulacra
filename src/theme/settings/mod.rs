//! The user's persisted theme choice: active palette, color overrides, custom palettes,
//! scales and font family. Tokens are always derived from this, never stored.

mod edit;
mod migrate;

pub use migrate::{migrate_legacy_tokens, LegacyTheme, MIGRATED_PALETTE_ID, MIGRATED_PALETTE_NAME};

use crate::theme::derive::{derive_tokens, Density, RadiusScale, TypeScale, DEFAULT_FONT_FAMILY};
use crate::theme::palette::{PaletteOverrides, ThemePalette};
use crate::theme::registry::{ThemeRegistry, DEFAULT_PALETTE_ID};
use crate::theme::ThemeTokens;

/// Prefix of every custom palette id.
pub const CUSTOM_PALETTE_PREFIX: &str = "custom-";

/// Everything that determines the app's look.
#[derive(Debug, Clone, PartialEq)]
pub struct ThemeSettings {
    /// Active palette id: a bundled palette or one of `custom`.
    pub preset_id: String,
    /// Unsaved-to-palette color edits applied over the active palette.
    pub overrides: PaletteOverrides,
    /// User-created palettes (duplicates, imports, migrated themes).
    pub custom: Vec<ThemePalette>,
    pub density: Density,
    pub type_scale: TypeScale,
    pub radius_scale: RadiusScale,
    /// Interface font family (UI, display and body roles).
    pub font_family: String,
}

impl Default for ThemeSettings {
    fn default() -> Self {
        Self {
            preset_id: DEFAULT_PALETTE_ID.to_string(),
            overrides: PaletteOverrides::default(),
            custom: Vec::new(),
            density: Density::default(),
            type_scale: TypeScale::default(),
            radius_scale: RadiusScale::default(),
            font_family: DEFAULT_FONT_FAMILY.to_string(),
        }
    }
}

impl ThemeSettings {
    /// The custom or bundled palette with `id` (custom palettes shadow bundled ones).
    pub fn palette(&self, id: &str) -> Option<&ThemePalette> {
        self.custom.iter().find(|p| p.id == id).or_else(|| ThemeRegistry::global().get(id))
    }

    /// True when `id` names one of the user's custom palettes.
    pub fn is_custom(&self, id: &str) -> bool {
        self.custom.iter().any(|p| p.id == id)
    }

    /// True when the active palette is a custom one (and therefore editable).
    pub fn active_is_custom(&self) -> bool {
        self.is_custom(&self.preset_id)
    }

    /// The active palette without overrides (the default palette when the id is unknown).
    pub fn base_palette(&self) -> &ThemePalette {
        self.palette(&self.preset_id).unwrap_or_else(|| ThemeRegistry::global().default_palette())
    }

    /// The active palette with overrides applied.
    pub fn effective_palette(&self) -> ThemePalette {
        self.base_palette().with_overrides(&self.overrides)
    }

    /// Tokens for `palette` at these settings' scales and font family.
    pub fn tokens_for(&self, palette: &ThemePalette) -> ThemeTokens {
        let mut tokens = derive_tokens(palette, self.density, self.type_scale, self.radius_scale);
        tokens.font_family_ui = self.font_family.clone();
        tokens.font_family_display = self.font_family.clone();
        tokens.font_family_body = self.font_family.clone();
        tokens
    }

    /// Tokens of the effective palette.
    pub fn tokens(&self) -> ThemeTokens {
        self.tokens_for(&self.effective_palette())
    }
}

#[cfg(test)]
mod tests;
