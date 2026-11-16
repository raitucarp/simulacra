//! Editing operations of [`ThemeSettings`]: selecting, duplicating, recoloring, resetting
//! and importing palettes.

use super::{ThemeSettings, CUSTOM_PALETTE_PREFIX};
use crate::theme::palette::{PaletteColorKey, ThemePalette};
use crate::theme::registry::ThemeRegistry;
use gpui_kit::Rgba;

impl ThemeSettings {
    /// First unused `custom-<n>` id.
    fn next_custom_id(&self) -> String {
        (1..)
            .map(|n| format!("{CUSTOM_PALETTE_PREFIX}{n}"))
            .find(|id| self.palette(id).is_none())
            .unwrap_or_else(|| CUSTOM_PALETTE_PREFIX.to_string())
    }

    /// Writes pending overrides into the active custom palette and clears them.
    fn commit_overrides(&mut self) {
        let overrides = std::mem::take(&mut self.overrides);
        let active = self.preset_id.clone();
        if let Some(palette) = self.custom.iter_mut().find(|p| p.id == active) {
            palette.colors = palette.colors.with_overrides(&overrides);
        }
    }

    /// Activates palette `id`, keeping edits made to the previous custom palette.
    pub fn select(&mut self, id: &str) {
        self.commit_overrides();
        self.preset_id = id.to_string();
    }

    /// Copies the effective palette into a new custom palette named `name` and activates it.
    pub fn duplicate_active(&mut self, name: String) -> String {
        let source = self.effective_palette();
        let based_on = if self.active_is_custom() { source.based_on.clone() } else { Some(source.id.clone()) };
        let id = self.next_custom_id();
        self.custom.push(ThemePalette { id: id.clone(), name, based_on, ..source });
        self.select(&id);
        id
    }

    /// Overrides one base color of the active palette; only custom palettes are editable.
    pub fn set_color(&mut self, key: PaletteColorKey, color: Rgba) -> bool {
        if !self.active_is_custom() {
            return false;
        }
        self.overrides.set(key, color);
        true
    }

    /// Drops pending edits and restores a custom palette to the bundled palette it came from.
    pub fn reset_to_preset(&mut self) {
        self.overrides.clear();
        let active = self.preset_id.clone();
        let Some(palette) = self.custom.iter_mut().find(|p| p.id == active) else { return };
        let origin = palette.based_on.as_deref().and_then(|id| ThemeRegistry::global().get(id));
        if let Some(origin) = origin {
            palette.colors = origin.colors;
            palette.mode = origin.mode;
        }
    }

    /// Adds `palette` as a new custom palette (fresh id, no bundled origin) and activates it.
    pub fn import(&mut self, palette: ThemePalette) -> String {
        let id = self.next_custom_id();
        self.custom.push(ThemePalette { id: id.clone(), based_on: None, ..palette });
        self.select(&id);
        id
    }

    /// Removes custom palette `id`, falling back to its bundled origin when it was active.
    pub fn delete_custom(&mut self, id: &str) {
        let Some(index) = self.custom.iter().position(|p| p.id == id) else { return };
        let removed = self.custom.remove(index);
        if self.preset_id == id {
            self.overrides.clear();
            let fallback = removed.based_on.filter(|origin| self.palette(origin).is_some());
            self.preset_id = fallback.unwrap_or_else(|| ThemeRegistry::global().default_palette().id.clone());
        }
    }
}
