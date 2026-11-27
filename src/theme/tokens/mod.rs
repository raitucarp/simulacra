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
        bg_app, bg_card, bg_elevated, bg_sidebar, bg_callout, hover_bg, // Surface & Background Colors
        bg_code, // Inline code and code block fill in post bodies
        code_keyword, code_type, code_string, code_number, code_comment, // Syntax colors of code blocks, readable on `bg_code`
        spoiler_fill, spoiler_revealed_bg, // Hidden spoiler fill (also its text color, so the text keeps its size unseen) and the faint tint of a revealed one
        border, border_subtle, border_strong, border_focus, // Border Colors
        focus_ring, // Keyboard focus ring (focus-visible outline of custom controls and kit widgets)
        text_primary, text_secondary, text_muted, // Text & Content Colors
        link_text, // L1: web addresses in post bodies, readable on every surface
        accent_brand_text, // The brand accent as text or icon: readable on every surface, selected (brand-tinted) ones included
        accent_user, accent_alt, accent_agent, accent_brand, accent_dialectic, accent_success, // Accents & Semantic Indicators
        accent_danger, warning, info, selection,
        on_accent, on_danger, on_overlay, // Readable foregrounds on filled accent, danger and scrim surfaces
        pressed_bg, input_bg, scrollbar, scrollbar_hover, overlay, shadow, // Interaction overlays, inputs, scrollbars, modal overlays and drop shadows
        bg_window_control_hover, bg_window_control_close_hover, bg_transparent, bg_tab_active, // Window Controls & Modal Interactivity
        bg_scrim,
        accent_brand_subtle, accent_brand_border, accent_success_subtle, accent_success_border, // Tinted Accent Badges & Borders
        accent_dialectic_subtle, accent_dialectic_border,
        graph_edge, // Mind Graph Topological Colors
        bg_feed_post, // Feed Surface Specialization (R2 Seamless Flat OLED)
        tint_amber_bg, tint_amber_border, tint_rose_bg, tint_rose_border, tint_sky_bg, tint_sky_border, // sb C1: tinted post containers (read through `theme::tint_colors`)
        tint_mint_bg, tint_mint_border, tint_violet_bg, tint_violet_border, tint_slate_bg, tint_slate_border,
    ],
    pixels: [
        font_size_2xs, font_size_xs, font_size_sm, font_size_base, font_size_md, font_size_lg, // Typography Scales
        font_size_xl,
        font_size_root, // Root (rem) size handed to gpui-component, whose widgets size their text in rems
        spacing_3xs, spacing_2xs, spacing_xs, spacing_sm, spacing_md, spacing_lg, // Micro-Spacing Tokens
        spacing_xl, spacing_2xl,
        #[doc = "Content inset of a panel: Forge stage, options and gallery, settings, vault, roster overview, Mind Graph, dialogs."] panel_padding, // Semantic layout spacing: roles mapped onto the spacing grid, so they follow density
        #[doc = "Inset of narrow rails and sidebars whose rows carry their own padding (shell sidebars, Forge rail, roster list)."] panel_padding_compact,
        #[doc = "Gap between sections or cards stacked in a panel; pairs with `panel_padding`."] section_gap,
        #[doc = "Gap between blocks stacked inside a section, card or sidebar (heading, hint, controls)."] stack_gap,
        #[doc = "Gap between an icon and its label and other tight inline pairs."] inline_gap,
        #[doc = "Inset of a card or tile surface; pairs with `stack_gap` inside it."] card_padding,
        #[doc = "Gap between adjacent controls: buttons in a toolbar, chips in a strip, field and button."] control_gap,
        #[doc = "Height of the blank line between paragraphs of post prose."] paragraph_break_height,
        dot_size_xs, dot_size_sm, chip_dismiss_size, heatmap_month_row_height, // Indicator dots, the chip dismiss button and the cadence heatmap month row
        radius_xs, radius_sm, radius_md, radius_lg, radius_xl, radius_full, // Radii Tokens
        border_width_hairline, border_width_threadline, focus_ring_width, // Border Widths
        shadow_offset_y, shadow_blur, // Elevation shadow geometry
        titlebar_height, feed_header_height, post_avatar_col_width, threadline_min_height, // Sizing & Layout Tokens
        win_control_width, win_control_height, avatar_size_sm, avatar_size_md, avatar_size_lg,
        modal_width, modal_height, modal_rail_width, modal_footer_height, cadence_cell_size,
        graph_canvas_height,
        heatmap_cell_height, heatmap_cell_gap, heatmap_cell_radius, heatmap_label_width, graph_view_min_height, // Cadence Heatmap Grid & Mind Graph Canvas
        graph_node_radius_min, graph_node_radius_max, graph_mini_node_radius, graph_edge_width,
        modal_composer_width, modal_composer_max_height, post_image_max_height, // Composer Modal & Attachment Tokens (Milestone 2)
        post_image_preview_height, attachment_btn_size,
        contact_list_pane_width, agent_category_rail_width, contact_avatar_size, hero_avatar_size, // Agent & Persona Lab Two-Pane Layout Tokens
        config_tab_height, trait_bar_height, prompt_preview_height,
        font_size_micro, font_size_hint, font_size_label, icon_size_xs, icon_size_sm, icon_size_md, // Dense chrome ramps (type-scaled text and icons, density-scaled half steps)
        icon_size_lg, icon_size_xl, icon_size_2xl, spacing_xs_plus, spacing_sm_plus,
        left_sidebar_collapsed_width, left_sidebar_collapsed_min, left_sidebar_collapsed_max, // Shell columns, sidebars, popovers, indicators and window-control corners
        left_sidebar_expanded_width, left_sidebar_expanded_min, left_sidebar_expanded_max,
        right_sidebar_width, right_sidebar_min, right_sidebar_max, stream_pane_width,
        stream_pane_min_width, stream_pane_max_width, feed_tab_indicator_width, nav_icon_slot_width,
        nav_active_dot_size, workspace_dot_size, workspace_badge_size, workspace_trigger_height,
        workspace_popover_width, language_popover_right, language_popover_width, win_control_radius,
        avatar_initial_small_max, avatar_initial_medium_max,
        window_default_width, window_default_height, splitter_hit_padding, split_flex_min, // Window restore size, resizable panel ranges, splitter hit band and the splash screen
        forge_rail_min, forge_rail_max, forge_options_min, forge_options_max, forge_preview_min,
        forge_preview_max, contact_list_pane_min, contact_list_pane_max, modal_rail_min,
        modal_rail_max, graph_details_height, graph_details_min, graph_details_max,
        splash_brand_width, splash_guide_size, splash_progress_width, splash_progress_height,
        settings_header_height, settings_input_height, settings_provider_button_height, settings_brand_badge_size, settings_tab_indicator_width, // Settings window (header, inputs, size caps and inset, rail, rows, art) and its appearance editor
        settings_tab_indicator_height, theme_preview_card_width, theme_preview_avatar_size, hex_readout_width,
        settings_modal_max_width, settings_modal_max_height, settings_modal_inset, settings_rail_item_height, settings_rail_compact_width,
        settings_row_label_min_width, settings_text_control_width, settings_number_readout_width, settings_header_art_width, settings_empty_art_width,
        forge_rail_width, forge_rail_thumb_width, forge_options_width, forge_preview_width, forge_topbar_height, forge_portrait_size, forge_nameplate_avatar_size, forge_class_card_width, forge_option_card_width, forge_part_thumb_size, forge_swatch_size, forge_guide_size, forge_radar_size, forge_skill_bar_height, forge_trading_card_width, forge_explainer_width, forge_empty_art_width, forge_tile_min_height, forge_bubble_max_width, forge_preset_card_width, forge_compact_card_width, // Agent Forge character creator
        forge_wheel_size, forge_wheel_dot_size, // Agent Forge Birthsign: the Enneagram wheel and its clickable points
        forge_clock_size, forge_clock_ring_width, forge_week_cell_height, forge_heart_size, forge_clock_label_size, forge_key_cap_min_width, // Agent Forge: Daily Life clock ring and week strip, Relationships heart pips, clock hour labels, key caps
        forge_palette_tab_icon_width, forge_palette_chip_width, forge_palette_search_width, // Agent Forge: palette gallery category tabs and the step palette strip
        onboarding_width, onboarding_art_size, // First-run welcome overlay
        splash_window_width, splash_window_height, window_min_width, window_min_height, layout_medium_min_width, // Startup windows, responsive layout (breakpoints, sidebar, drawer, stacked inspector), heatmap cells, graph labels
        layout_wide_min_width, right_panel_auto_min_width, right_sidebar_medium_width, right_sidebar_medium_max, right_drawer_width, inspector_stacked_height,
        inspector_stacked_min, inspector_stacked_max, heatmap_cell_min, heatmap_cell_max, heatmap_month_label_width, graph_label_max_width, graph_mini_node_radius_max,
        forge_probe_image_size, forge_sample_height, // Agent Forge Engine step: "Test image" result, sample-answer editor
        nav_row_height, // sb A2: compact left navigation (rows, rail, active bar, badge, New menu), palette, shortcut sheet
        nav_rail_button_size, nav_active_bar_width, nav_active_bar_height, sidebar_badge_size, new_menu_width, palette_width, palette_results_max_height, palette_top_offset, key_cap_min_width, shortcut_sheet_width, shortcut_sheet_max_height, shortcut_sheet_section_width,
        vault_facet_rail_width, vault_facet_drawer_width, vault_cover_width, vault_cover_height, vault_cover_thumb_width, vault_entry_cover_width, reader_measure_width, reader_measure_narrow_width, reader_measure_wide_width, vault_entry_side_width, vault_search_width, vault_editor_width, vault_meta_label_width, vault_column_creator_width, vault_column_kind_width, vault_column_year_width, vault_column_added_width, vault_column_links_width, vault_progress_height, vault_empty_art_width, vault_banner_width, vault_kind_art_width, vault_letter_cell_size, vault_tree_indent, // sb B1: Knowledge Vault catalogue
        import_sheet_width, import_sheet_max_height, import_preview_height, import_preview_row_height, import_art_width, import_meta_label_width, capture_box_width, capture_box_top, drop_art_width, similarity_bar_width, similarity_bar_height, similarity_readout_width, indexing_indicator_width, similar_sheet_width, similar_sheet_max_height, // sb B2: import sheet and preview, quick capture, drop hint; sb B3: semantic search
        post_actions_sheet_width, post_actions_sheet_max_height, post_actions_rail_width, post_actions_score_bar_width, post_actions_hits_max_height, outline_indent, outline_chevron_slot, outline_author_max_width, // sb C2: post actions sheet, outline view
        link_card_thumb_size, link_card_icon_size, media_gallery_gap, video_play_badge_size, lightbox_inset, lightbox_button_size, lightbox_caption_width, composer_attachment_thumb_size, composer_meter_row_height, media_skeleton_line_width, media_single_max_height, file_card_art_width, // sb C3: link and video cards, galleries, lightbox, composer attachments; L4: the file card art
        review_page_max_width, review_card_max_width, review_card_min_height, review_progress_height, review_grade_button_height, review_chart_height, review_bar_min_height, review_bar_max_width, review_heatmap_cell_min, review_heatmap_cell_max, review_heatmap_gap, review_swatch_size, review_empty_art_width, review_maker_max_height, review_sentence_list_height, review_count_tile_width, review_stat_label_width, // sb C1: review page, item card, grade buttons, charts, sheet height
        review_learn_sheet_width, review_map_height, review_map_point_width, review_map_point_height, review_map_label_width, review_map_edge_width, // sb C1b: learn sheet, connection map canvas, point chips, relation labels, strokes
        insights_page_max_width, insights_tile_width, insights_chart_height, insights_spark_height, insights_axis_width, insights_bar_gap, insights_bar_max_width, insights_bar_min_height, insights_swatch_size, insights_tooltip_width, insights_radial_min, insights_radial_max, insights_canvas_height, insights_canvas_min_height, insights_side_width, insights_rank_label_width, insights_rank_value_width, insights_rank_bar_height, insights_node_radius_min, insights_node_radius_max, insights_point_radius, insights_edge_width, insights_label_max_width, insights_upset_cell, insights_upset_dot, insights_upset_label_width, insights_tree_height, insights_venn_height, insights_diagram_height, insights_filter_width, // sb D2: Insights page charts, canvases, ranked lists, UpSet matrix
        suggest_chip_max_width, suggest_chip_row_height, suggest_band_height, suggest_tick_width, suggest_card_width, // sb D1: suggestion chips, typical band on sliders, Why card
        art_empty_width, art_notice_width, art_spot_width, art_compact_width, image_ideas_sheet_width, image_ideas_sheet_height, image_ideas_picks_width, image_ideas_chip_width, image_ideas_chip_art_width, image_ideas_result_size, // sb D3: app-wide art widths, image ideas sheet
        learning_bubble_max_width, learning_answer_min_height, learning_ladder_dot_size, learning_side_width, learning_date_field_width, learning_radar_min, learning_radar_max, learning_rank_bar_width, learning_rank_label_width, learning_scene_width, learning_walk_scene_width, learning_marker_size, learning_room_thumb_width, learning_cue_image_size, learning_list_max_height, // sb E2: Learn tab (tutor bubbles and ladder, goal side column, gap radar and ranks, palace scenes, markers, thumbnails)
        journal_page_max_width, journal_card_max_width, journal_editor_min_height, journal_grid_size, journal_grid_dot_size, journal_grid_label_width, journal_heat_cell, journal_heat_gap, journal_chart_height, journal_timer_text, journal_timer_text_large, journal_progress_height, journal_energy_button_size, journal_focus_mode_width, journal_thumb_size, journal_list_max_height, journal_row_label_width, // sb E3: journal page, affect grid, heat strip, focus timer and mode, capsules
        thinking_chip_width, thinking_chip_height, thinking_row_height, thinking_edge_width, thinking_handle_width, credence_popover_width, calibration_chart_size, calibration_dot_min, calibration_dot_max, atomic_sheet_width, atomic_sheet_max_height, atomic_score_size, // sb E1: argument canvas and list, credence popover, reliability diagram, atomicity sheet
        arena_page_max_width, arena_side_width, arena_score_label_width, arena_score_bar_width, arena_bar_height, arena_grid_label_width, arena_grid_cell_width, arena_saved_max_height, arena_row_label_width, belief_spark_width, belief_spark_height, belief_spark_stroke, belief_spark_dot, belief_credence_width, // sb F1: Arena page, rubric bars, lab grid, belief sparkline and credence bars
        data_sheet_width, data_sheet_max_height, history_list_width, history_diff_min_height, data_label_width, data_count_width, data_date_field_width, export_pick_max_height, strength_meter_height, knowledge_switch_width, saved_page_max_width, saved_chip_max_width, saved_grip_width, saved_note_min_height, history_lane_width, history_lane_dot, compose_preview_indent, // sb F2, V5: version tree lanes; T2: free compose preview;: data sheets, version list and diff, strength meter, Library / Saved switch, Saved view
        insight_lane_height, insight_lane_label_width, insight_axis_height, insight_brush_min_width, insight_cell_gap, insight_list_max_height, insight_meter_width, insight_meter_height, insight_peak_ring_width, // sb F3: concept timeline lanes and brush, detail lists, composer readability meter
        emoji_picker_width, emoji_picker_grid_height, emoji_cell_size, // epic R: emoji picker (width, scrolling grid, one cell)
        symbol_badge_size, symbol_badge_offset, symbol_outline_size, symbol_progress_width, symbol_progress_height, // epic S: signifier badges on a symbol face, the symbol of an outline bullet, an icon set download bar
        todo_help_sample_width, todo_indent, todo_box_size, tasks_page_max_width, tasks_filter_label_width, // epic D: to-dos (the syntax help card, the indent of a nested item, a box, the Tasks page and its filter labels)
        search_page_max_width, code_output_max_height, // epic Q: the Search page column; P6: the output panel under a code block
        view_gutter_width, calendar_cell_min_height, board_column_width, ink_canvas_width, ink_canvas_height, ink_card_height, // epic W: the bullet gutter of the rapid log, a calendar day, a board column; N12: the drawing pad, a drawing on its card
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
        symbol_glyph_scale, // epic S: a library icon's share of its symbol disc
        file_missing_opacity, // L11: the dimmed art of a missing file
        clap_idle_opacity, // Q6: the colour clap emoji, quiet until the persona claps
    ],
}

impl Default for ThemeTokens {
    fn default() -> Self {
        crate::theme::minimalist_dark()
    }
}

#[cfg(test)]
mod tests;
