//! Palette-independent sizes of the window, the resizable panel ranges and the splash screen.

use crate::theme::ThemeTokens;
use gpui_kit::px;

/// Window restore size, min/max of every resizable split, the splitter hit band and the
/// splash screen's art, progress line and timing.
pub(super) fn apply_panel_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        window_default_width: px(1440.0), window_default_height: px(900.0),
        // Half the kit's splitter band (its padding on each side of the hairline)
        splitter_hit_padding: px(4.0),
        // Smallest size of the flexible (main content) pane of a split
        split_flex_min: px(320.0),
        forge_rail_min: px(160.0), forge_rail_max: px(320.0),
        forge_options_min: px(340.0), forge_options_max: px(640.0),
        forge_preview_min: px(260.0), forge_preview_max: px(480.0),
        contact_list_pane_min: px(200.0), contact_list_pane_max: px(400.0),
        modal_rail_min: px(180.0), modal_rail_max: px(300.0),
        graph_details_height: px(56.0), graph_details_min: px(48.0), graph_details_max: px(240.0),
        splash_brand_width: px(160.0), splash_guide_size: px(96.0),
        splash_progress_width: px(240.0), splash_progress_height: px(3.0),
        splash_window_width: px(560.0), splash_window_height: px(420.0),
        window_min_width: px(720.0), window_min_height: px(520.0),

        splash_min_duration_ms: 900.0, splash_fade_duration_ms: 280.0, splash_warm_up_limit_ms: 1500.0,
    });
}
