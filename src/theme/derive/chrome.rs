//! Palette-independent sizes of the app chrome: shell columns, popovers, indicators and the
//! settings modal.

use crate::theme::ThemeTokens;
use gpui_kit::px;

/// Resizable shell columns (default, min and max widths) and the feed tab indicator.
pub(super) fn apply_shell_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        left_sidebar_collapsed_width: px(64.0), left_sidebar_collapsed_min: px(60.0),
        left_sidebar_collapsed_max: px(68.0), left_sidebar_expanded_width: px(240.0),
        left_sidebar_expanded_min: px(180.0), left_sidebar_expanded_max: px(380.0),
        right_sidebar_width: px(320.0), right_sidebar_min: px(260.0), right_sidebar_max: px(460.0),
        stream_pane_width: px(520.0), stream_pane_min_width: px(380.0), stream_pane_max_width: px(720.0),
        feed_tab_indicator_width: px(48.0),
    });
}

/// Navigation indicators, workspace and language popovers, avatar initial thresholds and dots.
pub(super) fn apply_chrome_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        nav_icon_slot_width: px(20.0), nav_active_dot_size: px(6.0), workspace_dot_size: px(8.0),
        workspace_badge_size: px(28.0), workspace_trigger_height: px(46.0),
        workspace_popover_width: px(260.0), language_popover_right: px(100.0),
        language_popover_width: px(220.0),
        avatar_initial_small_max: px(24.0), avatar_initial_medium_max: px(36.0),
        // Status and legend dots, and the round dismiss button of a filter chip
        dot_size_xs: px(4.0), dot_size_sm: px(8.0), chip_dismiss_size: px(16.0),
    });
}

/// Settings modal header, inputs, provider buttons, rail and appearance editor sizes.
pub(super) fn apply_settings_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        settings_header_height: px(64.0), settings_input_height: px(34.0),
        settings_provider_button_height: px(40.0), settings_brand_badge_size: px(26.0),
        settings_tab_indicator_width: px(3.0), settings_tab_indicator_height: px(16.0),
        theme_preview_card_width: px(172.0), theme_preview_avatar_size: px(16.0),
        color_field_width: px(262.0), hex_readout_width: px(76.0),
    });
}
