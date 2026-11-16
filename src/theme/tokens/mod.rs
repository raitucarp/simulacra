//! Core design tokens for typography, surfaces, borders, spacing, and radii.
//! Strictly tokenized to eliminate magic strings and hardcoded values.

mod serde_helpers;

use gpui_kit::{px, rgba, Pixels, Rgba};
use crate::theme::palette::PaletteMode;
use serde::{Deserialize, Serialize};
use serde_helpers::{px_serde, rgba_serde};

/// Generates [`ThemeTokens`] from field lists grouped by value kind.
///
/// Field order (and therefore the serialized JSON layout) follows the lists in order:
/// typed enums, strings, colors (`0xRRGGBBAA` numbers), pixels (`f32` numbers), then floats.
macro_rules! theme_tokens {
    (
        typed: [$($typed:ident: $ty:ty),* $(,)?],
        strings: [$($string:ident),* $(,)?],
        colors: [$($color:ident),* $(,)?],
        pixels: [$($pixel:ident),* $(,)?],
        floats: [$($float:ident),* $(,)?] $(,)?
    ) => {
        /// Structured theme tokens for the application interface.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(default)]
        pub struct ThemeTokens {
            $(pub $typed: $ty,)*
            $(pub $string: String,)*
            $(#[serde(with = "rgba_serde")] pub $color: Rgba,)*
            $(#[serde(with = "px_serde")] pub $pixel: Pixels,)*
            $(pub $float: f32,)*
        }

        impl ThemeTokens {
            /// Zero-valued tokens; palettes fill every field on top of this.
            pub(crate) fn blank() -> Self {
                Self {
                    $($typed: <$ty>::default(),)*
                    $($string: String::new(),)*
                    $($color: rgba(0),)*
                    $($pixel: px(0.0),)*
                    $($float: 0.0,)*
                }
            }

            /// Snaps every color to 8-bit channels, so derived tokens equal their persisted form.
            pub(crate) fn quantize_colors(&mut self) {
                $(self.$color = crate::theme::color::quantized(self.$color);)*
            }
        }
    };
}

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
        // Micro-Spacing Tokens
        spacing_3xs, spacing_2xs, spacing_px3, spacing_xs, spacing_sm, spacing_md, spacing_lg,
        spacing_xl, spacing_2xl,
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
        // First-run welcome overlay
        onboarding_width, onboarding_art_size,
    ],
    // Typography Hierarchy & Line Height; heatmap level opacities; graph dimming & curvature
    floats: [
        line_height_body, cadence_alpha_1, cadence_alpha_2, cadence_alpha_3, cadence_alpha_4,
        graph_dim_opacity, graph_curve_bend,
        // Agent Forge: opacity of disabled option cards and of the stage backdrop art
        forge_disabled_opacity, forge_backdrop_opacity,
    ],
}

impl Default for ThemeTokens {
    fn default() -> Self {
        crate::theme::minimalist_dark()
    }
}

#[cfg(test)]
mod tests;
