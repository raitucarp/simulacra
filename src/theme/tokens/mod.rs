//! Core design tokens for typography, surfaces, borders, spacing, and radii.
//! Strictly tokenized to eliminate magic strings and hardcoded values.

#[macro_use] mod macros;
mod serde_helpers;

use gpui_kit::{px, rgba, Pixels, Rgba};
use crate::theme::palette::PaletteMode;
use serde::{Deserialize, Serialize}; use serde_helpers::{px_serde, rgba_serde};

theme_tokens! {
    // Light/dark appearance the colors were derived for
    typed: [mode: PaletteMode],
    // Font Family Tokens (Epistemic Quad-Typeface System)
    strings: [font_family_ui, font_family_display, font_family_body, font_family_mono],
    colors: [
        // Surface & Background Colors
        bg_app, bg_card, bg_elevated, bg_sidebar, bg_callout, hover_bg,
        // Border Colors
        border, border_subtle, border_strong, border_focus,
        // Keyboard focus ring (focus-visible outline of custom controls and kit widgets)
        focus_ring,
        // Text & Content Colors
        text_primary, text_secondary, text_muted,
        // Accents & Semantic Indicators
        accent_user, accent_alt, accent_agent, accent_brand, accent_dialectic, accent_success,
        accent_danger, warning, info, selection,
        // Readable foregrounds on filled accent, danger and scrim surfaces
        on_accent, on_danger, on_overlay,
        // Interaction overlays, inputs, scrollbars, modal overlays and drop shadows
        pressed_bg, input_bg, scrollbar, scrollbar_hover, overlay, shadow,
        // Window Controls & Modal Interactivity
        bg_window_control_hover, bg_window_control_close_hover, bg_transparent, bg_tab_active,
        bg_scrim,
        // Tinted Accent Badges & Borders
        accent_brand_subtle, accent_brand_border, accent_success_subtle, accent_success_border,
        accent_dialectic_subtle, accent_dialectic_border,
        // Mind Graph Topological Colors
        graph_edge,
        // Feed Surface Specialization (R2 Seamless Flat OLED)
        bg_feed_post,
    ],
    pixels: [
        // Typography Scales
        font_size_2xs, font_size_xs, font_size_sm, font_size_base, font_size_md, font_size_lg,
        font_size_xl,
        // Root (rem) size handed to gpui-component, whose widgets size their text in rems
        font_size_root,
        // Micro-Spacing Tokens
        spacing_3xs, spacing_2xs, spacing_xs, spacing_sm, spacing_md, spacing_lg,
        spacing_xl, spacing_2xl,
        // Semantic layout spacing: roles mapped onto the spacing grid, so they follow density
        #[doc = "Content inset of a panel: Forge stage, options and gallery, settings, vault, roster overview, Mind Graph, dialogs."] panel_padding,
        #[doc = "Inset of narrow rails and sidebars whose rows carry their own padding (shell sidebars, Forge rail, roster list)."] panel_padding_compact,
        #[doc = "Gap between sections or cards stacked in a panel; pairs with `panel_padding`."] section_gap,
        #[doc = "Gap between blocks stacked inside a section, card or sidebar (heading, hint, controls)."] stack_gap,
        #[doc = "Gap between an icon and its label and other tight inline pairs."] inline_gap,
        #[doc = "Inset of a card or tile surface; pairs with `stack_gap` inside it."] card_padding,
        #[doc = "Gap between adjacent controls: buttons in a toolbar, chips in a strip, field and button."] control_gap,
        #[doc = "Height of the blank line between paragraphs of post prose."] paragraph_break_height,
        // Indicator dots, the chip dismiss button and the cadence heatmap month row
        dot_size_xs, dot_size_sm, chip_dismiss_size, heatmap_month_row_height,
        // Radii Tokens
        radius_xs, radius_sm, radius_md, radius_lg, radius_xl, radius_full,
        // Border Widths
        border_width_hairline, border_width_threadline, focus_ring_width,
        // Elevation shadow geometry
        shadow_offset_y, shadow_blur,
        // Sizing & Layout Tokens
        titlebar_height, feed_header_height, post_avatar_col_width, threadline_min_height,
        win_control_width, win_control_height, avatar_size_sm, avatar_size_md, avatar_size_lg,
        modal_width, modal_height, modal_rail_width, modal_footer_height, cadence_cell_size,
        graph_canvas_height,
        // Cadence Heatmap Grid & Mind Graph Canvas
        heatmap_cell_height, heatmap_cell_gap, heatmap_cell_radius, heatmap_label_width, graph_view_min_height,
        graph_node_radius_min, graph_node_radius_max, graph_mini_node_radius, graph_edge_width,
        // Composer Modal & Attachment Tokens (Milestone 2)
        modal_composer_width, modal_composer_max_height, post_image_max_height,
        post_image_preview_height, attachment_btn_size,
        // Agent & Persona Lab Two-Pane Layout Tokens
        contact_list_pane_width, agent_category_rail_width, contact_avatar_size, hero_avatar_size,
        config_tab_height, trait_bar_height, prompt_preview_height,
        // Dense chrome ramps (type-scaled text and icons, density-scaled half steps)
        font_size_micro, font_size_hint, font_size_label, icon_size_xs, icon_size_sm, icon_size_md,
        icon_size_lg, icon_size_xl, icon_size_2xl, spacing_xs_plus, spacing_sm_plus,
        // Shell columns, sidebars, popovers, indicators and window-control corners
        left_sidebar_collapsed_width, left_sidebar_collapsed_min, left_sidebar_collapsed_max,
        left_sidebar_expanded_width, left_sidebar_expanded_min, left_sidebar_expanded_max,
        right_sidebar_width, right_sidebar_min, right_sidebar_max, stream_pane_width,
        stream_pane_min_width, stream_pane_max_width, feed_tab_indicator_width, nav_icon_slot_width,
        nav_active_dot_size, workspace_dot_size, workspace_badge_size, workspace_trigger_height,
        workspace_popover_width, language_popover_right, language_popover_width, win_control_radius,
        avatar_initial_small_max, avatar_initial_medium_max,
        // Window restore size, resizable panel ranges, splitter hit band and the splash screen
        window_default_width, window_default_height, splitter_hit_padding, split_flex_min,
        forge_rail_min, forge_rail_max, forge_options_min, forge_options_max, forge_preview_min,
        forge_preview_max, contact_list_pane_min, contact_list_pane_max, modal_rail_min,
        modal_rail_max, graph_details_height, graph_details_min, graph_details_max,
        splash_brand_width, splash_guide_size, splash_progress_width, splash_progress_height,
        // Settings window (header, inputs, size caps and inset, rail, rows, art) and its appearance editor
        settings_header_height, settings_input_height, settings_provider_button_height, settings_brand_badge_size, settings_tab_indicator_width,
        settings_tab_indicator_height, theme_preview_card_width, theme_preview_avatar_size, color_field_width, hex_readout_width,
        settings_modal_max_width, settings_modal_max_height, settings_modal_inset, settings_rail_item_height, settings_rail_compact_width,
        settings_row_label_min_width, settings_text_control_width, settings_number_readout_width, settings_header_art_width, settings_empty_art_width,
        // Agent Forge character creator
        forge_rail_width, forge_rail_thumb_width, forge_options_width, forge_preview_width,
        forge_topbar_height, forge_portrait_size, forge_nameplate_avatar_size, forge_class_card_width,
        forge_option_card_width, forge_part_thumb_size, forge_swatch_size, forge_guide_size,
        forge_radar_size, forge_skill_bar_height, forge_trading_card_width, forge_explainer_width,
        forge_empty_art_width, forge_tile_min_height, forge_bubble_max_width, forge_preset_card_width,
        forge_compact_card_width,
        // Agent Forge Birthsign: the Enneagram wheel and its clickable points
        forge_wheel_size, forge_wheel_dot_size,
        // Agent Forge: Daily Life clock ring and week strip, Relationships heart pips
        forge_clock_size, forge_clock_ring_width, forge_week_cell_height, forge_heart_size,
        // Agent Forge: hour labels inside the clock ring, keyboard-hint key caps
        forge_clock_label_size, forge_key_cap_min_width,
        // Agent Forge: palette gallery category tabs and the step palette strip
        forge_palette_tab_icon_width, forge_palette_chip_width, forge_palette_search_width,
        onboarding_width, onboarding_art_size, // First-run welcome overlay
        // Startup windows, responsive layout (breakpoints, sidebar, drawer, stacked inspector), heatmap cells, graph labels
        splash_window_width, splash_window_height, window_min_width, window_min_height, layout_medium_min_width,
        layout_wide_min_width, right_sidebar_medium_width, right_sidebar_medium_max, right_drawer_width, inspector_stacked_height,
        inspector_stacked_min, inspector_stacked_max, heatmap_cell_min, heatmap_cell_max, heatmap_month_label_width, graph_label_max_width, graph_mini_node_radius_max,
        forge_probe_image_size, forge_sample_height, // Agent Forge Engine step: "Test image" result, sample-answer editor
        nav_row_height, // sb A2: compact left navigation (rows, rail, active bar, badge, New menu), palette, shortcut sheet
        nav_rail_button_size, nav_active_bar_width, nav_active_bar_height, sidebar_badge_size, new_menu_width, palette_width, palette_results_max_height, palette_top_offset, key_cap_min_width, shortcut_sheet_width, shortcut_sheet_max_height, shortcut_sheet_section_width,
        vault_facet_rail_width, vault_facet_drawer_width, vault_cover_width, vault_cover_height, vault_cover_thumb_width, vault_entry_cover_width, reader_measure_width, reader_measure_narrow_width, reader_measure_wide_width, vault_entry_side_width, vault_search_width, vault_editor_width, vault_meta_label_width, vault_column_creator_width, vault_column_kind_width, vault_column_year_width, vault_column_added_width, vault_column_links_width, vault_progress_height, vault_empty_art_width, vault_banner_width, vault_kind_art_width, vault_letter_cell_size, vault_tree_indent, // sb B1: Knowledge Vault catalogue
        import_sheet_width, import_sheet_max_height, import_preview_height, import_preview_row_height, import_art_width, import_meta_label_width, capture_box_width, capture_box_top, drop_art_width, // sb B2: import sheet and preview, quick capture, drop hint
    ],
    // Typography Hierarchy & Line Height; heatmap level opacities; graph dimming & curvature
    floats: [
        line_height_body, cadence_alpha_1, cadence_alpha_2, cadence_alpha_3, cadence_alpha_4,
        graph_dim_opacity, graph_curve_bend,
        // Splash: shortest time on screen, fade, longest avatar warm-up (ms); mini graph width/height
        splash_min_duration_ms, splash_fade_duration_ms, splash_warm_up_limit_ms, graph_mini_aspect,
        // Forge disabled option cards and stage backdrop art; feed failed tools; settings rows switched off
        forge_disabled_opacity, forge_backdrop_opacity, post_failed_tool_opacity, settings_disabled_opacity,
        line_height_reading, vault_motion_duration_ms, vault_muted_opacity, // sb B1: reader leading, calm entry fade, read-state dimming
    ],
}

impl Default for ThemeTokens {
    fn default() -> Self {
        crate::theme::minimalist_dark()
    }
}

#[cfg(test)]
mod tests;
