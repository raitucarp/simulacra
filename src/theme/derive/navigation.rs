//! Palette-independent sizes of the left navigation, the command palette and the keyboard
//! shortcut sheet.

use crate::theme::ThemeTokens;
use gpui_kit::px;

/// Compact nav rows, the icon rail, the auto-rail breakpoint, the "New" menu, the palette
/// overlay and the shortcut sheet.
pub(super) fn apply_navigation_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        nav_row_height: px(30.0), nav_rail_button_size: px(36.0),
        nav_active_bar_width: px(3.0), nav_active_bar_height: px(16.0), sidebar_badge_size: px(22.0),
        // Below this window width the left sidebar folds into its icon rail
        left_sidebar_rail_breakpoint: px(1100.0),
        new_menu_width: px(232.0),
        palette_width: px(640.0), palette_results_max_height: px(420.0), palette_top_offset: px(88.0),
        key_cap_min_width: px(20.0),
        shortcut_sheet_width: px(680.0), shortcut_sheet_max_height: px(560.0),
        shortcut_sheet_section_width: px(300.0),
    });
}
