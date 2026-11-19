//! Appearance-mode operations of [`ThemeSettings`].
//!
//! The active palette (`preset_id`) is the preferred palette of the effective mode:
//! [`ThemeSettings::apply_appearance`] selects it, and selecting a palette records it as the
//! preferred one of its own mode.

use super::appearance_mode::{default_palette_for, AppearanceMode, PreferredThemes};
use super::ThemeSettings;
use crate::theme::palette::PaletteMode;

impl ThemeSettings {
    /// The palette mode shown while the operating system is in `system` mode.
    pub fn effective_mode(&self, system: PaletteMode) -> PaletteMode {
        self.appearance.resolve(system)
    }

    /// The preferred palette id of `mode`, or that mode's default when it no longer exists.
    pub fn preferred_id(&self, mode: PaletteMode) -> &str {
        let id = self.preferred.of(mode);
        if self.palette(id).is_some_and(|palette| palette.mode == mode) { id } else { default_palette_for(mode) }
    }

    /// Activates the preferred palette of the effective mode (nothing when it is active).
    pub fn apply_appearance(&mut self, system: PaletteMode) {
        let id = self.preferred_id(self.effective_mode(system)).to_string();
        if id != self.preset_id {
            self.select(&id);
        }
    }

    /// Switches the appearance mode and activates its preferred palette.
    pub fn set_appearance(&mut self, mode: AppearanceMode, system: PaletteMode) {
        self.appearance = mode;
        self.apply_appearance(system);
    }

    /// Makes `id` the preferred palette of `mode`, activating it when `mode` is showing.
    pub fn set_preferred(&mut self, mode: PaletteMode, id: &str, system: PaletteMode) {
        self.preferred.set(mode, id);
        self.apply_appearance(system);
    }

    /// After picking a palette of the other mode, switches to that mode explicitly, so the
    /// pick stays visible instead of being replaced by the other mode's preferred palette.
    pub fn follow_active_palette(&mut self, system: PaletteMode) {
        let mode = self.base_palette().mode;
        if mode != self.effective_mode(system) {
            self.appearance = AppearanceMode::fixed(mode);
        }
    }

    /// Settings saved before appearance modes existed: the active palette's own mode, with
    /// that palette preferred for it and the default palette preferred for the other mode.
    pub fn adopt_active_as_preferred(&mut self) {
        let mode = self.base_palette().mode;
        self.appearance = AppearanceMode::fixed(mode);
        self.preferred = PreferredThemes::default();
        let id = self.base_palette().id.clone();
        self.preferred.set(mode, &id);
    }
}
