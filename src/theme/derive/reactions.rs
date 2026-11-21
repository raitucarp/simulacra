//! Palette-independent sizes of reactions (epic R).

use crate::theme::ThemeTokens;
use gpui_kit::px;

/// The emoji picker: its width, the scrolling grid's height and one emoji cell.
pub(super) fn apply_reactions_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        emoji_picker_width: px(332.0), emoji_picker_grid_height: px(224.0), emoji_cell_size: px(36.0),
    });
}
