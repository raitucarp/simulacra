//! Palette-independent sizes of post symbols (epic S).

use crate::theme::ThemeTokens;
use gpui_kit::px;

/// Signifier badges on a symbol face (and how far they overhang its corner), the symbol in
/// front of an outline bullet, and a library icon's share of its disc.
pub(super) fn apply_symbols_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        symbol_badge_size: px(16.0), symbol_badge_offset: px(3.0), symbol_outline_size: px(16.0),
        symbol_glyph_scale: 0.62,
    });
}
