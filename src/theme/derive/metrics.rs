//! Palette-independent metrics: fonts, type scale, spacing grid, radii and layout sizes.

use super::scales::{Density, RadiusScale, TypeScale};
use crate::theme::ThemeTokens;
use gpui_kit::px;

/// Default interface typeface (UI, display and body roles).
pub const DEFAULT_FONT_FAMILY: &str = "Segoe UI";
/// Default monospace typeface.
pub const DEFAULT_MONO_FONT_FAMILY: &str = "Consolas";
/// Scaled metrics snap to this many steps per pixel (half-pixel grid).
const SNAP_STEPS_PER_PX: f32 = 2.0;

/// `base * factor`, snapped to the half-pixel grid.
fn scaled(base: f32, factor: f32) -> gpui_kit::Pixels {
    px((base * factor * SNAP_STEPS_PER_PX).round() / SNAP_STEPS_PER_PX)
}

/// Font families and the type scale.
pub(super) fn apply_typography(t: &mut ThemeTokens, scale: TypeScale) {
    t.font_family_ui = DEFAULT_FONT_FAMILY.to_string();
    t.font_family_display = DEFAULT_FONT_FAMILY.to_string();
    t.font_family_body = DEFAULT_FONT_FAMILY.to_string();
    t.font_family_mono = DEFAULT_MONO_FONT_FAMILY.to_string();
    let f = scale.factor();
    set_tokens!(t {
        font_size_2xs: scaled(10.0, f), font_size_xs: scaled(11.5, f), font_size_sm: scaled(13.0, f),
        font_size_base: scaled(14.0, f), font_size_md: scaled(15.0, f), font_size_lg: scaled(18.0, f),
        font_size_xl: scaled(22.0, f),
    });
}

/// The 4px spacing grid, stretched or compressed by `density`.
pub(super) fn apply_spacing(t: &mut ThemeTokens, density: Density) {
    let f = density.factor();
    set_tokens!(t {
        spacing_3xs: scaled(1.0, f), spacing_2xs: scaled(2.0, f), spacing_px3: scaled(3.0, f),
        spacing_xs: scaled(4.0, f), spacing_sm: scaled(8.0, f), spacing_md: scaled(12.0, f),
        spacing_lg: scaled(16.0, f), spacing_xl: scaled(20.0, f), spacing_2xl: scaled(24.0, f),
    });
}

/// Corner radii (the pill radius stays fully round), border widths and shadow geometry.
pub(super) fn apply_radii(t: &mut ThemeTokens, radius: RadiusScale) {
    let f = radius.factor();
    set_tokens!(t {
        radius_xs: scaled(3.0, f), radius_sm: scaled(6.0, f), radius_md: scaled(8.0, f),
        radius_lg: scaled(12.0, f), radius_xl: scaled(16.0, f), radius_full: px(9999.0),
        border_width_hairline: px(1.0), border_width_threadline: px(2.0), focus_ring_width: px(2.0),
        shadow_offset_y: px(12.0), shadow_blur: px(32.0),
    });
}

/// Window chrome, panel, modal, composer and lab layout sizes plus body line height.
pub(super) fn apply_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        titlebar_height: px(38.0), feed_header_height: px(44.0), post_avatar_col_width: px(38.0),
        threadline_min_height: px(24.0), win_control_width: px(28.0),
        win_control_height: px(24.0), avatar_size_sm: px(20.0), avatar_size_md: px(32.0),
        avatar_size_lg: px(34.0), modal_width: px(840.0), modal_height: px(580.0),
        modal_rail_width: px(220.0), modal_footer_height: px(54.0), cadence_cell_size: px(10.0),
        graph_canvas_height: px(140.0),

        heatmap_cell_height: px(5.0), heatmap_cell_gap: px(1.0), heatmap_cell_radius: px(1.0),
        heatmap_label_width: px(24.0),
        graph_view_min_height: px(420.0), graph_node_radius_min: px(3.5),
        graph_node_radius_max: px(11.0), graph_mini_node_radius: px(2.0), graph_edge_width: px(1.0),

        modal_composer_width: px(580.0), modal_composer_max_height: px(620.0),
        post_image_max_height: px(280.0), post_image_preview_height: px(140.0),
        attachment_btn_size: px(28.0),

        contact_list_pane_width: px(236.0), agent_category_rail_width: px(204.0),
        contact_avatar_size: px(36.0), hero_avatar_size: px(48.0), config_tab_height: px(36.0),
        trait_bar_height: px(6.0), prompt_preview_height: px(220.0),

        line_height_body: 1.45,
        cadence_alpha_1: 0.28, cadence_alpha_2: 0.48, cadence_alpha_3: 0.72, cadence_alpha_4: 1.0,
        graph_dim_opacity: 0.22, graph_curve_bend: 0.12,
    });
}
