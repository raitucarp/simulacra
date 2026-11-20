//! The theme colors of a post container's [`Tint`] (sb C1); derived in `derive::tints`.

use crate::domain::post::Tint;
use crate::theme::ThemeTokens;
use gpui_kit::Rgba;

/// A tinted container's fill and outline.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TintColors {
    pub background: Rgba,
    pub border: Rgba,
}

/// The background and border of `tint` in `theme`.
pub fn tint_colors(tint: Tint, theme: &ThemeTokens) -> TintColors {
    let (background, border) = match tint {
        Tint::Amber => (theme.tint_amber_bg, theme.tint_amber_border),
        Tint::Rose => (theme.tint_rose_bg, theme.tint_rose_border),
        Tint::Sky => (theme.tint_sky_bg, theme.tint_sky_border),
        Tint::Mint => (theme.tint_mint_bg, theme.tint_mint_border),
        Tint::Violet => (theme.tint_violet_bg, theme.tint_violet_border),
        Tint::Slate => (theme.tint_slate_bg, theme.tint_slate_border),
    };
    TintColors { background, border }
}

#[cfg(test)]
mod tests;
