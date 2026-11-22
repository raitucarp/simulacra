//! Derivation of the full [`ThemeTokens`] set from a [`ThemePalette`] and the user's scales.
//!
//! Every color token is a pure function of the fourteen palette colors and the palette mode,
//! so the same palette always yields the same tokens, in light and dark mode alike.

mod accents;
mod actions;
mod arena;
mod chrome;
mod code;
mod data;
mod forge;
mod image_prompt;
mod import;
mod insight_more;
mod insights;
mod learning;
mod journal;
mod media;
mod metrics;
mod navigation;
mod panels;
mod responsive;
mod review;
mod scales;
mod reactions; // epic R
mod semantic;
mod spacing;
mod suggestions;
mod surfaces;
mod symbols; // epic S
mod todos; // epic D
mod search; // epic Q
mod text;
mod thinking;
mod tints;
mod vault;

pub use metrics::{DEFAULT_FONT_FAMILY, DEFAULT_MONO_FONT_FAMILY};
pub use scales::{Density, RadiusScale, TypeScale};

use crate::theme::palette::ThemePalette;
use crate::theme::ThemeTokens;

/// Derives every token for `palette` at the given density, type scale and radius scale.
pub fn derive_tokens(
    palette: &ThemePalette,
    density: Density,
    type_scale: TypeScale,
    radius_scale: RadiusScale,
) -> ThemeTokens {
    let mut t = ThemeTokens::blank();
    t.mode = palette.mode;
    metrics::apply_typography(&mut t, type_scale);
    metrics::apply_spacing(&mut t, density);
    spacing::apply_semantic_spacing(&mut t);
    metrics::apply_radii(&mut t, radius_scale);
    metrics::apply_layout(&mut t);
    chrome::apply_shell_layout(&mut t);
    chrome::apply_chrome_layout(&mut t);
    chrome::apply_settings_layout(&mut t);
    forge::apply_forge_layout(&mut t);
    panels::apply_panel_layout(&mut t);
    navigation::apply_navigation_layout(&mut t);
    import::apply_import_layout(&mut t);
    media::apply_media_layout(&mut t);
    semantic::apply_semantic_layout(&mut t);
    review::apply_review_layout(&mut t); review::apply_review_map_layout(&mut t);
    insights::apply_insights_layout(&mut t); insight_more::apply_insight_more_layout(&mut t); // sb F3: lens views
    learning::apply_learning_layout(&mut t);
    journal::apply_journal_layout(&mut t); arena::apply_arena_layout(&mut t); // sb F1: Arena and Beliefs sizes
    responsive::apply_responsive_layout(&mut t);
    vault::apply_vault_layout(&mut t);
    actions::apply_actions_layout(&mut t);
    suggestions::apply_suggestions_layout(&mut t);
    image_prompt::apply_image_prompt_layout(&mut t);
    thinking::apply_thinking_layout(&mut t); data::apply_data_layout(&mut t); reactions::apply_reactions_layout(&mut t); symbols::apply_symbols_layout(&mut t); todos::apply_todos_layout(&mut t); search::apply_search_layout(&mut t); // sb F2; epic R, S, D, Q
    surfaces::apply_surfaces(&mut t, palette);
    text::apply_text(&mut t, palette);
    accents::apply_accents(&mut t, palette); accents::apply_shades(&mut t, palette);
    tints::apply_tints(&mut t, palette); // sb C1: after the text colors it keeps readable
    code::apply_code(&mut t, palette); // posts B: syntax colors, after the surfaces they sit on
    t.quantize_colors();
    t
}

/// [`derive_tokens`] at the default density, type scale and radius scale.
pub fn derive_default_tokens(palette: &ThemePalette) -> ThemeTokens {
    derive_tokens(palette, Density::default(), TypeScale::default(), RadiusScale::default())
}

#[cfg(test)]
mod tests;
