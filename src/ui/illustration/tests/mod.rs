//! Illustration pipeline tests: templates, cache, typed keys, composed portraits and photos.

mod avatars;
mod cache;
mod keys;
mod preview;
mod template;

use crate::theme::{PaletteColors, ThemeRegistry};

/// Colors of the bundled default palette.
fn default_colors() -> PaletteColors {
    ThemeRegistry::global().default_palette().colors
}

/// Colors of some other bundled palette (differs from the default).
fn other_colors() -> PaletteColors {
    let registry = ThemeRegistry::global();
    let default = registry.default_palette();
    registry.all().iter().find(|p| p.colors != default.colors).expect("two distinct palettes").colors
}

/// Parses `svg` with resvg's usvg (what gpui uses), panicking with context on failure.
fn parse(svg: &str, context: &str) -> resvg::usvg::Tree {
    resvg::usvg::Tree::from_str(svg, &resvg::usvg::Options::default()).unwrap_or_else(|e| panic!("{context}: {e}"))
}
