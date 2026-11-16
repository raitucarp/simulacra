//! Gallery filtering: the Dark/Light mode chips and the free-text search.

use crate::theme::{PaletteMode, ThemePalette};

/// Which palettes the gallery shows by mode.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ModeFilter {
    #[default]
    All,
    Dark,
    Light,
}

impl ModeFilter {
    /// Every filter, in chip order.
    pub const ALL: [ModeFilter; 3] = [ModeFilter::All, ModeFilter::Dark, ModeFilter::Light];

    /// True when palettes of `mode` pass this filter.
    pub fn admits(self, mode: PaletteMode) -> bool {
        match self {
            ModeFilter::All => true,
            ModeFilter::Dark => mode == PaletteMode::Dark,
            ModeFilter::Light => mode == PaletteMode::Light,
        }
    }

    /// i18n key of the chip label.
    pub const fn label_key(self) -> &'static str {
        match self {
            ModeFilter::All => "appearance.filter_all",
            ModeFilter::Dark => "appearance.filter_dark",
            ModeFilter::Light => "appearance.filter_light",
        }
    }

    /// Element-id suffix of the chip.
    pub const fn id(self) -> &'static str {
        match self {
            ModeFilter::All => "all",
            ModeFilter::Dark => PaletteMode::Dark.id(),
            ModeFilter::Light => PaletteMode::Light.id(),
        }
    }
}

/// True when the palette's name, id or credit contains every whitespace-separated term of
/// `query` (case-insensitive); an empty query matches everything.
pub fn matches_query(palette: &ThemePalette, query: &str) -> bool {
    let haystack = format!("{} {} {}", palette.name, palette.id, palette.credit).to_lowercase();
    query.split_whitespace().all(|term| haystack.contains(&term.to_lowercase()))
}

/// True when the palette passes both the mode filter and the search query.
pub fn is_visible(palette: &ThemePalette, query: &str, filter: ModeFilter) -> bool {
    filter.admits(palette.mode) && matches_query(palette, query)
}
