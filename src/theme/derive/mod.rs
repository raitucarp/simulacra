//! Derivation of the full [`ThemeTokens`] set from a [`ThemePalette`] and the user's scales.
//!
//! Every color token is a pure function of the fourteen palette colors and the palette mode,
//! so the same palette always yields the same tokens, in light and dark mode alike.

mod accents;
mod chrome;
mod forge;
mod metrics;
mod panels;
mod scales;
mod surfaces;
mod text;

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
    metrics::apply_radii(&mut t, radius_scale);
    metrics::apply_layout(&mut t);
    chrome::apply_shell_layout(&mut t);
    chrome::apply_chrome_layout(&mut t);
    chrome::apply_settings_layout(&mut t);
    forge::apply_forge_layout(&mut t);
    panels::apply_panel_layout(&mut t);
    surfaces::apply_surfaces(&mut t, palette);
    text::apply_text(&mut t, palette);
    accents::apply_accents(&mut t, palette);
    accents::apply_shades(&mut t, palette);
    t.quantize_colors();
    t
}

/// [`derive_tokens`] at the default density, type scale and radius scale.
pub fn derive_default_tokens(palette: &ThemePalette) -> ThemeTokens {
    derive_tokens(palette, Density::default(), TypeScale::default(), RadiusScale::default())
}

#[cfg(test)]
mod tests;
