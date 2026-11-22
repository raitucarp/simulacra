//! Palette-independent sizes of the Search page (epic Q) and of a code block's output panel
//! (P6).

use crate::theme::ThemeTokens;
use gpui_kit::px;

/// The Search page column, and the tallest a run's output shows before it scrolls.
pub(super) fn apply_search_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        search_page_max_width: px(760.0),
        code_output_max_height: px(220.0),
    });
}
