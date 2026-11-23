//! Palette-independent sizes of the views of a list of posts (epic W).

use crate::theme::ThemeTokens;
use gpui_kit::px;

/// The rapid log's bullet gutter (signifiers and the bullet) and the shortest calendar day.
pub(super) fn apply_views_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        view_gutter_width: px(44.0), calendar_cell_min_height: px(72.0),
    });
}
