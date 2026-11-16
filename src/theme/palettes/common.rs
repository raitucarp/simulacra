//! Typography scale, spacing, layout sizing, and semantic tokens shared by every palette.

use crate::theme::ThemeTokens;
use gpui_kit::{px, rgb, rgba};

/// Font families, type scale, spacing grid, radii and border widths.
pub(super) fn apply_typography(t: &mut ThemeTokens) {
    t.font_family_ui = "Segoe UI".to_string();
    t.font_family_display = "Segoe UI".to_string();
    t.font_family_body = "Segoe UI".to_string();
    t.font_family_mono = "Consolas".to_string();
    set_tokens!(t {
        font_size_2xs: px(10.0), font_size_xs: px(11.5), font_size_sm: px(13.0),
        font_size_base: px(14.0), font_size_md: px(15.0), font_size_lg: px(18.0),
        font_size_xl: px(22.0),

        spacing_3xs: px(1.0), spacing_2xs: px(2.0), spacing_px3: px(3.0), spacing_xs: px(4.0),
        spacing_sm: px(8.0), spacing_md: px(12.0), spacing_lg: px(16.0), spacing_xl: px(20.0),
        spacing_2xl: px(24.0),

        radius_xs: px(3.0), radius_sm: px(6.0), radius_md: px(8.0), radius_lg: px(12.0),
        radius_xl: px(16.0), radius_full: px(9999.0),

        border_width_hairline: px(1.0), border_width_threadline: px(2.0),
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

/// Danger/white, window controls, tinted accents, cadence heatmap and graph colors.
pub(super) fn apply_semantic_colors(t: &mut ThemeTokens) {
    set_tokens!(t {
        accent_danger: rgb(0xef4444),
        text_white: rgb(0xffffff),
        bg_window_control_hover: rgb(0x27272a),
        bg_window_control_close_hover: rgb(0xdc2626),
        bg_transparent: rgba(0x00000000),
        bg_tab_active: rgba(0xffffff0d),
        bg_scrim: rgba(0x000000dd),

        accent_brand_subtle: rgba(0x8b5cf620),
        accent_brand_border: rgba(0x8b5cf62a),
        accent_success_subtle: rgba(0x10b98115),
        accent_success_border: rgba(0x10b98140),
        accent_dialectic_subtle: rgba(0xeab30820),
        accent_dialectic_border: rgba(0xeab30840),

        graph_edge: rgb(0x2d3246),
        bg_feed_post: rgb(0x000000),
    });
}
