//! The colors of a post card's container: the plain feed surface, the elevated focused one,
//! or the post's tint (sb C2), whose fill stays on hover and whose border marks the edge.

use crate::domain::post::Tint;
use crate::theme::color::composite;
use crate::theme::{tint_colors, ThemeTokens};
use gpui_kit::Rgba;

/// A card's fill at rest and on hover, its bottom rule and, when tinted, its edge color.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct CardSurface {
    pub background: Rgba,
    pub hover: Rgba,
    pub rule: Rgba,
    /// The tint's border, drawn down the leading edge.
    pub edge: Option<Rgba>,
}

/// The surface of a card tinted `tint` (or not), focused or not.
pub(crate) fn card_surface(tint: Option<Tint>, is_focused: bool, theme: &ThemeTokens) -> CardSurface {
    let Some(tint) = tint else {
        let (background, hover) = if is_focused { (theme.bg_elevated, theme.bg_elevated) } else { (theme.bg_feed_post, theme.hover_bg) };
        return CardSurface { background, hover, rule: theme.border_subtle, edge: None };
    };
    let colors = tint_colors(tint, theme);
    let hover = if is_focused { colors.background } else { composite(theme.hover_bg, colors.background) };
    CardSurface { background: colors.background, hover, rule: colors.border, edge: Some(colors.border) }
}

/// The fill and edge of a tinted outline row, `None` for an untinted one.
pub(crate) fn row_tint(tint: Option<Tint>, theme: &ThemeTokens) -> Option<(Rgba, Rgba)> {
    tint.map(|tint| tint_colors(tint, theme)).map(|colors| (colors.background, colors.border))
}

#[cfg(test)]
mod tests;
