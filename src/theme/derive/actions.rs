//! Palette-independent sizes of the post actions sheet and the inspector's outline view (sb C2).

use crate::theme::ThemeTokens;
use gpui_kit::px;

/// Actions sheet (card, action rail, confidence bars, concept hit lists) and outline rows.
pub(super) fn apply_actions_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        post_actions_sheet_width: px(760.0), post_actions_sheet_max_height: px(680.0), post_actions_rail_width: px(212.0),
        post_actions_score_bar_width: px(96.0), post_actions_hits_max_height: px(148.0),
        outline_indent: px(18.0), outline_chevron_slot: px(16.0), outline_author_max_width: px(96.0),
    });
}
