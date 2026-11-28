//! Palette-independent sizes of post symbols (epic S).

use crate::theme::ThemeTokens;
use gpui_kit::px;

/// Signifier badges on a symbol face (and how far they overhang its corner), the symbol in
/// front of an outline bullet, a library icon's share of its disc, and the bar of an icon set download;
/// also the illustration pack cards (WS5), downloaded like the icon sets.
pub(super) fn apply_symbols_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        symbol_badge_size: px(16.0), symbol_badge_offset: px(3.0), symbol_outline_size: px(16.0), symbol_progress_width: px(96.0), symbol_progress_height: px(6.0),
        symbol_glyph_scale: 0.62,
        pack_card_width: px(236.0), pack_sample_height: px(48.0), // WS5: illustration pack cards
    });
}
