//! Core design tokens for typography, surfaces, borders, spacing, and radii.
//! Strictly tokenized to eliminate magic strings and hardcoded values.

mod serde_helpers;

use gpui_kit::{px, rgba, Pixels, Rgba};
use serde::{Deserialize, Serialize};
use serde_helpers::{px_serde, rgba_serde};

/// Generates [`ThemeTokens`] from field lists grouped by value kind.
///
/// Field order (and therefore the serialized JSON layout) follows the lists in order:
/// strings, colors (`0xRRGGBBAA` numbers), pixels (`f32` numbers), then plain floats.
macro_rules! theme_tokens {
    (
        strings: [$($string:ident),* $(,)?],
        colors: [$($color:ident),* $(,)?],
        pixels: [$($pixel:ident),* $(,)?],
        floats: [$($float:ident),* $(,)?] $(,)?
    ) => {
        /// Structured theme tokens for the application interface.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(default)]
        pub struct ThemeTokens {
            $(pub $string: String,)*
            $(#[serde(with = "rgba_serde")] pub $color: Rgba,)*
            $(#[serde(with = "px_serde")] pub $pixel: Pixels,)*
            $(pub $float: f32,)*
        }

        impl ThemeTokens {
            /// Zero-valued tokens; palettes fill every field on top of this.
            pub(crate) fn blank() -> Self {
                Self {
                    $($string: String::new(),)*
                    $($color: rgba(0),)*
                    $($pixel: px(0.0),)*
                    $($float: 0.0,)*
                }
            }
        }
    };
}

theme_tokens! {
    // Font Family Tokens (Epistemic Quad-Typeface System)
    strings: [font_family_ui, font_family_display, font_family_body, font_family_mono],
    colors: [
        // Surface & Background Colors
        bg_app, bg_card, bg_elevated, bg_sidebar, bg_callout, hover_bg,
        // Border Colors
        border, border_subtle, border_strong, border_focus,
        // Text & Content Colors
        text_primary, text_secondary, text_muted,
        // Accents & Semantic Indicators
        accent_user, accent_alt, accent_agent, accent_brand, accent_dialectic, accent_success,
        accent_danger, text_white,
        // Window Controls & Modal Interactivity
        bg_window_control_hover, bg_window_control_close_hover, bg_transparent, bg_tab_active,
        bg_scrim,
        // Tinted Accent Badges & Borders
        accent_brand_subtle, accent_brand_border, accent_success_subtle, accent_success_border,
        accent_dialectic_subtle, accent_dialectic_border,
        // Activity Cadence Heatmap Scale (4 Levels)
        cadence_level_1, cadence_level_2, cadence_level_3, cadence_level_4,
        // Mind Graph Topological Colors
        graph_edge, graph_node_neutral,
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
        border_width_hairline, border_width_threadline,
        // Sizing & Layout Tokens
        titlebar_height, feed_header_height, post_avatar_col_width, threadline_min_height,
        win_control_width, win_control_height, avatar_size_sm, avatar_size_md, avatar_size_lg,
        modal_width, modal_height, modal_rail_width, modal_footer_height, cadence_cell_size,
        graph_canvas_height,
        // Composer Modal & Attachment Tokens (Milestone 2)
        modal_composer_width, modal_composer_max_height, post_image_max_height,
        post_image_preview_height, attachment_btn_size,
        // Agent & Persona Lab Two-Pane Layout Tokens
        contact_list_pane_width, agent_category_rail_width, contact_avatar_size, hero_avatar_size,
        config_tab_height, trait_bar_height, prompt_preview_height,
    ],
    // Typography Hierarchy & Line Height
    floats: [line_height_body],
}

impl Default for ThemeTokens {
    fn default() -> Self {
        crate::theme::minimalist_dark()
    }
}

#[cfg(test)]
mod tests;
