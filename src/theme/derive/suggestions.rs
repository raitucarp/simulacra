//! Palette-independent sizes of the parameter suggestions (sb D1): example chips, the typical
//! band under sliders and the "Why?" card.

use crate::theme::ThemeTokens;
use gpui_kit::px;

/// Chip width cap and collapsed-row height, band strip and recommended tick, Why card width.
pub(super) fn apply_suggestions_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        suggest_chip_max_width: px(220.0), suggest_chip_row_height: px(26.0),
        suggest_band_height: px(6.0), suggest_tick_width: px(2.0), suggest_card_width: px(320.0),
    });
}
