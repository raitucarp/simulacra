//! The bundled palette catalogue: every `assets/themes/*.json`, parsed once on first use.

mod embedded;

use crate::theme::palette::{PaletteMode, ThemePalette};
use embedded::EMBEDDED_PALETTES;
use std::sync::OnceLock;

/// Id of the palette used when nothing (or an unknown id) is selected.
pub const DEFAULT_PALETTE_ID: &str = "simulacra-midnight";
/// Ids of the former hard-coded presets, now palettes.
pub const OBSIDIAN_ZINC_ID: &str = "obsidian-zinc";
pub const MIDNIGHT_SLATE_ID: &str = "midnight-slate";

/// Ordered, read-only catalogue of the bundled palettes.
#[derive(Debug)]
pub struct ThemeRegistry {
    palettes: Vec<ThemePalette>,
}

impl ThemeRegistry {
    /// The process-wide catalogue.
    pub fn global() -> &'static ThemeRegistry {
        static REGISTRY: OnceLock<ThemeRegistry> = OnceLock::new();
        REGISTRY.get_or_init(ThemeRegistry::load)
    }

    /// Parses every embedded palette, skipping (and reporting) any that fail.
    fn load() -> Self {
        let palettes = EMBEDDED_PALETTES
            .iter()
            .filter_map(|(id, json)| match ThemePalette::from_json(json) {
                Ok(palette) => Some(palette),
                Err(err) => {
                    eprintln!("simulacra: bundled palette `{id}` is invalid: {err}");
                    None
                }
            })
            .collect();
        Self { palettes }
    }

    /// Every palette in gallery order.
    pub fn all(&self) -> &[ThemePalette] {
        &self.palettes
    }

    /// The palette with `id`.
    pub fn get(&self, id: &str) -> Option<&ThemePalette> {
        self.palettes.iter().find(|palette| palette.id == id)
    }

    /// Palettes of one mode, in gallery order.
    pub fn by_mode(&self, mode: PaletteMode) -> impl Iterator<Item = &ThemePalette> {
        self.palettes.iter().filter(move |palette| palette.mode == mode)
    }

    /// The default palette (Simulacra Midnight).
    pub fn default_palette(&self) -> &ThemePalette {
        self.get(DEFAULT_PALETTE_ID)
            .or_else(|| self.palettes.first())
            .expect("the bundled palette catalogue is never empty (guarded by tests)")
    }

    /// Number of bundled palettes.
    pub fn len(&self) -> usize {
        self.palettes.len()
    }

    /// True when no palette parsed (a build defect; tests forbid it).
    pub fn is_empty(&self) -> bool {
        self.palettes.is_empty()
    }
}

#[cfg(test)]
mod tests;
