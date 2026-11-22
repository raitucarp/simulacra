//! Palette-independent sizes of the Search page (epic Q).

use crate::theme::ThemeTokens;
use gpui_kit::px;

/// The Search page column.
pub(super) fn apply_search_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        search_page_max_width: px(760.0),
    });
}
