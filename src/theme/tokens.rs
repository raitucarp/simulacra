//! Core design tokens for typography, surfaces, borders, spacing, and radii.
//! Strictly tokenized to eliminate magic strings and hardcoded values.

use gpui_kit::{px, rgba, Pixels, Rgba};
use serde::{Deserialize, Serialize};

/// Custom serde helper for gpui_kit::Rgba
mod rgba_serde {
    use super::*;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(color: &Rgba, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize as u32 hex 0xRRGGBBAA
        let r = (color.r * 255.0).round() as u32;
        let g = (color.g * 255.0).round() as u32;
        let b = (color.b * 255.0).round() as u32;
        let a = (color.a * 255.0).round() as u32;
        let val = (r << 24) | (g << 16) | (b << 8) | a;
        serializer.serialize_u32(val)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<Rgba, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = u32::deserialize(deserializer)?;
        Ok(rgba(val))
    }
}

/// Helper for Pixels serialization
mod px_serde {
    use super::*;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(pixels: &Pixels, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f32(pixels.as_f32())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<Pixels, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = f32::deserialize(deserializer)?;
        Ok(px(val))
    }
}

/// Structured theme tokens for the application interface.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ThemeTokens {
    // Font Family Tokens (Epistemic Quad-Typeface System)
    pub font_family_ui: String,
    pub font_family_display: String,
    pub font_family_body: String,
    pub font_family_mono: String,

    // Surface & Background Colors
    #[serde(with = "rgba_serde")]
    pub bg_app: Rgba,
    #[serde(with = "rgba_serde")]
    pub bg_card: Rgba,
    #[serde(with = "rgba_serde")]
    pub bg_elevated: Rgba,
    #[serde(with = "rgba_serde")]
    pub bg_sidebar: Rgba,
    #[serde(with = "rgba_serde")]
    pub bg_callout: Rgba,
    #[serde(with = "rgba_serde")]
    pub hover_bg: Rgba,

    // Border Colors
    #[serde(with = "rgba_serde")]
    pub border: Rgba,
    #[serde(with = "rgba_serde")]
    pub border_subtle: Rgba,
    #[serde(with = "rgba_serde")]
    pub border_strong: Rgba,
    #[serde(with = "rgba_serde")]
    pub border_focus: Rgba,

    // Text & Content Colors
    #[serde(with = "rgba_serde")]
    pub text_primary: Rgba,
    #[serde(with = "rgba_serde")]
    pub text_secondary: Rgba,
    #[serde(with = "rgba_serde")]
    pub text_muted: Rgba,

    // Accents & Semantic Indicators
    #[serde(with = "rgba_serde")]
    pub accent_user: Rgba,
    #[serde(with = "rgba_serde")]
    pub accent_alt: Rgba,
    #[serde(with = "rgba_serde")]
    pub accent_agent: Rgba,
    #[serde(with = "rgba_serde")]
    pub accent_brand: Rgba,
    #[serde(with = "rgba_serde")]
    pub accent_dialectic: Rgba,
    #[serde(with = "rgba_serde")]
    pub accent_success: Rgba,
    #[serde(with = "rgba_serde")]
    pub accent_danger: Rgba,
    #[serde(with = "rgba_serde")]
    pub text_white: Rgba,

    // Window Controls & Modal Interactivity
    #[serde(with = "rgba_serde")]
    pub bg_window_control_hover: Rgba,
    #[serde(with = "rgba_serde")]
    pub bg_window_control_close_hover: Rgba,
    #[serde(with = "rgba_serde")]
    pub bg_transparent: Rgba,
    #[serde(with = "rgba_serde")]
    pub bg_tab_active: Rgba,
    #[serde(with = "rgba_serde")]
    pub bg_scrim: Rgba,

    // Tinted Accent Badges & Borders
    #[serde(with = "rgba_serde")]
    pub accent_brand_subtle: Rgba,
    #[serde(with = "rgba_serde")]
    pub accent_brand_border: Rgba,
    #[serde(with = "rgba_serde")]
    pub accent_success_subtle: Rgba,
    #[serde(with = "rgba_serde")]
    pub accent_success_border: Rgba,
    #[serde(with = "rgba_serde")]
    pub accent_dialectic_subtle: Rgba,
    #[serde(with = "rgba_serde")]
    pub accent_dialectic_border: Rgba,

    // Activity Cadence Heatmap Scale (4 Levels)
    #[serde(with = "rgba_serde")]
    pub cadence_level_1: Rgba,
    #[serde(with = "rgba_serde")]
    pub cadence_level_2: Rgba,
    #[serde(with = "rgba_serde")]
    pub cadence_level_3: Rgba,
    #[serde(with = "rgba_serde")]
    pub cadence_level_4: Rgba,

    // Mind Graph Topological Colors
    #[serde(with = "rgba_serde")]
    pub graph_edge: Rgba,
    #[serde(with = "rgba_serde")]
    pub graph_node_neutral: Rgba,

    // Feed Surface Specialization (R2 Seamless Flat OLED)
    #[serde(with = "rgba_serde")]
    pub bg_feed_post: Rgba,

    // Typography Scales
    #[serde(with = "px_serde")]
    pub font_size_2xs: Pixels,
    #[serde(with = "px_serde")]
    pub font_size_xs: Pixels,
    #[serde(with = "px_serde")]
    pub font_size_sm: Pixels,
    #[serde(with = "px_serde")]
    pub font_size_base: Pixels,
    #[serde(with = "px_serde")]
    pub font_size_md: Pixels,
    #[serde(with = "px_serde")]
    pub font_size_lg: Pixels,
    #[serde(with = "px_serde")]
    pub font_size_xl: Pixels,

    // Micro-Spacing Tokens
    #[serde(with = "px_serde")]
    pub spacing_3xs: Pixels,
    #[serde(with = "px_serde")]
    pub spacing_2xs: Pixels,
    #[serde(with = "px_serde")]
    pub spacing_px3: Pixels,
    #[serde(with = "px_serde")]
    pub spacing_xs: Pixels,
    #[serde(with = "px_serde")]
    pub spacing_sm: Pixels,
    #[serde(with = "px_serde")]
    pub spacing_md: Pixels,
    #[serde(with = "px_serde")]
    pub spacing_lg: Pixels,
    #[serde(with = "px_serde")]
    pub spacing_xl: Pixels,
    #[serde(with = "px_serde")]
    pub spacing_2xl: Pixels,

    // Radii Tokens
    #[serde(with = "px_serde")]
    pub radius_xs: Pixels,
    #[serde(with = "px_serde")]
    pub radius_sm: Pixels,
    #[serde(with = "px_serde")]
    pub radius_md: Pixels,
    #[serde(with = "px_serde")]
    pub radius_lg: Pixels,
    #[serde(with = "px_serde")]
    pub radius_xl: Pixels,
    #[serde(with = "px_serde")]
    pub radius_full: Pixels,

    // Border Widths
    #[serde(with = "px_serde")]
    pub border_width_hairline: Pixels,
    #[serde(with = "px_serde")]
    pub border_width_threadline: Pixels,

    // Sizing & Layout Tokens
    #[serde(with = "px_serde")]
    pub titlebar_height: Pixels,
    #[serde(with = "px_serde")]
    pub feed_header_height: Pixels,
    #[serde(with = "px_serde")]
    pub post_avatar_col_width: Pixels,
    #[serde(with = "px_serde")]
    pub threadline_min_height: Pixels,
    #[serde(with = "px_serde")]
    pub win_control_width: Pixels,
    #[serde(with = "px_serde")]
    pub win_control_height: Pixels,
    #[serde(with = "px_serde")]
    pub avatar_size_sm: Pixels,
    #[serde(with = "px_serde")]
    pub avatar_size_md: Pixels,
    #[serde(with = "px_serde")]
    pub avatar_size_lg: Pixels,
    #[serde(with = "px_serde")]
    pub modal_width: Pixels,
    #[serde(with = "px_serde")]
    pub modal_height: Pixels,
    #[serde(with = "px_serde")]
    pub modal_rail_width: Pixels,
    #[serde(with = "px_serde")]
    pub modal_footer_height: Pixels,
    #[serde(with = "px_serde")]
    pub cadence_cell_size: Pixels,
    #[serde(with = "px_serde")]
    pub graph_canvas_height: Pixels,

    // Composer Modal & Attachment Tokens (Milestone 2)
    #[serde(with = "px_serde")]
    pub modal_composer_width: Pixels,
    #[serde(with = "px_serde")]
    pub modal_composer_max_height: Pixels,
    #[serde(with = "px_serde")]
    pub post_image_max_height: Pixels,
    #[serde(with = "px_serde")]
    pub post_image_preview_height: Pixels,
    #[serde(with = "px_serde")]
    pub attachment_btn_size: Pixels,

    // Agent & Persona Lab Two-Pane Layout Tokens
    #[serde(with = "px_serde")]
    pub contact_list_pane_width: Pixels,
    #[serde(with = "px_serde")]
    pub agent_category_rail_width: Pixels,
    #[serde(with = "px_serde")]
    pub contact_avatar_size: Pixels,
    #[serde(with = "px_serde")]
    pub hero_avatar_size: Pixels,
    #[serde(with = "px_serde")]
    pub config_tab_height: Pixels,
    #[serde(with = "px_serde")]
    pub trait_bar_height: Pixels,
    #[serde(with = "px_serde")]
    pub prompt_preview_height: Pixels,

    // Typography Hierarchy & Line Height
    pub line_height_body: f32,
}

impl Default for ThemeTokens {
    fn default() -> Self {
        crate::theme::minimalist_dark()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_tokens_default() {
        let default_tokens = ThemeTokens::default();
        assert_eq!(default_tokens.titlebar_height, px(38.0));
        assert_eq!(default_tokens.feed_header_height, px(44.0));
        assert_eq!(default_tokens.line_height_body, 1.45);
        assert_eq!(default_tokens.bg_feed_post, gpui_kit::rgb(0x000000));
    }

    #[test]
    fn test_theme_tokens_serde_roundtrip() {
        let tokens = ThemeTokens::default();
        let json = serde_json::to_string(&tokens).expect("serialize failed");
        let deserialized: ThemeTokens = serde_json::from_str(&json).expect("deserialize failed");
        assert_eq!(tokens, deserialized);
    }

    #[test]
    fn test_theme_tokens_serde_backward_compatibility() {
        // Test that an older JSON representation missing new fields deserializes with defaults
        let old_json = r#"{
            "font_family_ui":"Segoe UI","font_family_display":"Segoe UI","font_family_body":"Segoe UI","font_family_mono":"Consolas",
            "bg_app":0,"bg_card":0,"bg_elevated":789518,"bg_sidebar":0,"bg_callout":2338125330,"hover_bg":4294967048,
            "border":505291266,"border_subtle":404233218,"border_strong":656878338,"border_focus":2338125566,
            "text_primary":4109694463,"text_secondary":2711726847,"text_muted":1903262463,
            "accent_user":951974143,"accent_alt":3937601791,"accent_agent":2824206335,"accent_brand":2338125566,
            "accent_dialectic":3937601791,"accent_success":280592895,
            "font_size_2xs":10.0,"font_size_xs":11.5,"font_size_sm":13.0,"font_size_base":14.0,"font_size_md":15.0,"font_size_lg":18.0,"font_size_xl":22.0,
            "spacing_2xs":2.0,"spacing_xs":4.0,"spacing_sm":8.0,"spacing_md":12.0,"spacing_lg":16.0,"spacing_xl":20.0,"spacing_2xl":24.0,
            "radius_xs":3.0,"radius_sm":6.0,"radius_md":8.0,"radius_lg":12.0,"radius_xl":16.0,"radius_full":9999.0,
            "border_width_hairline":1.0,"border_width_threadline":2.0
        }"#;
        let deserialized: ThemeTokens = serde_json::from_str(old_json).expect("backward-compatible deserialize failed");
        assert_eq!(deserialized.titlebar_height, px(38.0));
        assert_eq!(deserialized.feed_header_height, px(44.0));
        assert_eq!(deserialized.bg_feed_post, gpui_kit::rgb(0x000000));
        assert_eq!(deserialized.line_height_body, 1.45);
    }
}
