//! Theme palettes for customizable network appearance.
//! Defines tokenized presets (Threads OLED Black, Obsidian Deep Zinc, Midnight Slate).

use super::tokens::ThemeTokens;
use gpui_kit::{px, rgb, rgba};

/// Common typography scale, spacing, layout sizing, and default semantic tokens.
macro_rules! common_tokens {
    ($theme:ident) => {
        $theme.font_family_ui = "Segoe UI".to_string();
        $theme.font_family_display = "Segoe UI".to_string();
        $theme.font_family_body = "Segoe UI".to_string();
        $theme.font_family_mono = "Consolas".to_string();

        $theme.font_size_2xs = px(10.0);
        $theme.font_size_xs = px(11.5);
        $theme.font_size_sm = px(13.0);
        $theme.font_size_base = px(14.0);
        $theme.font_size_md = px(15.0);
        $theme.font_size_lg = px(18.0);
        $theme.font_size_xl = px(22.0);

        $theme.spacing_3xs = px(1.0);
        $theme.spacing_2xs = px(2.0);
        $theme.spacing_px3 = px(3.0);
        $theme.spacing_xs = px(4.0);
        $theme.spacing_sm = px(8.0);
        $theme.spacing_md = px(12.0);
        $theme.spacing_lg = px(16.0);
        $theme.spacing_xl = px(20.0);
        $theme.spacing_2xl = px(24.0);

        $theme.radius_xs = px(3.0);
        $theme.radius_sm = px(6.0);
        $theme.radius_md = px(8.0);
        $theme.radius_lg = px(12.0);
        $theme.radius_xl = px(16.0);
        $theme.radius_full = px(9999.0);

        $theme.border_width_hairline = px(1.0);
        $theme.border_width_threadline = px(2.0);

        $theme.titlebar_height = px(38.0);
        $theme.feed_header_height = px(44.0);
        $theme.post_avatar_col_width = px(38.0);
        $theme.threadline_min_height = px(24.0);
        $theme.win_control_width = px(28.0);
        $theme.win_control_height = px(24.0);
        $theme.avatar_size_sm = px(20.0);
        $theme.avatar_size_md = px(32.0);
        $theme.avatar_size_lg = px(34.0);
        $theme.modal_width = px(840.0);
        $theme.modal_height = px(580.0);
        $theme.modal_rail_width = px(220.0);
        $theme.modal_footer_height = px(54.0);
        $theme.cadence_cell_size = px(10.0);
        $theme.graph_canvas_height = px(140.0);

        $theme.modal_composer_width = px(580.0);
        $theme.modal_composer_max_height = px(620.0);
        $theme.post_image_max_height = px(280.0);
        $theme.post_image_preview_height = px(140.0);
        $theme.attachment_btn_size = px(28.0);

        $theme.contact_list_pane_width = px(236.0);
        $theme.agent_category_rail_width = px(204.0);
        $theme.contact_avatar_size = px(36.0);
        $theme.hero_avatar_size = px(48.0);
        $theme.config_tab_height = px(36.0);
        $theme.trait_bar_height = px(6.0);
        $theme.prompt_preview_height = px(220.0);

        $theme.line_height_body = 1.45;

        $theme.accent_danger = rgb(0xef4444);
        $theme.text_white = rgb(0xffffff);
        $theme.bg_window_control_hover = rgb(0x27272a);
        $theme.bg_window_control_close_hover = rgb(0xdc2626);
        $theme.bg_transparent = rgba(0x00000000);
        $theme.bg_tab_active = rgba(0xffffff0d);
        $theme.bg_scrim = rgba(0x000000dd);

        $theme.accent_brand_subtle = rgba(0x8b5cf620);
        $theme.accent_brand_border = rgba(0x8b5cf62a);
        $theme.accent_success_subtle = rgba(0x10b98115);
        $theme.accent_success_border = rgba(0x10b98140);
        $theme.accent_dialectic_subtle = rgba(0xeab30820);
        $theme.accent_dialectic_border = rgba(0xeab30840);

        $theme.cadence_level_1 = rgb(0x064e3b);
        $theme.cadence_level_2 = rgb(0x059669);
        $theme.cadence_level_3 = rgb(0x10b981);
        $theme.cadence_level_4 = rgb(0x34d399);

        $theme.graph_edge = rgb(0x2d3246);
        $theme.graph_node_neutral = rgb(0x6b7280);
        $theme.bg_feed_post = rgb(0x000000);
    };
}

/// Baseline tokens initialization helper.
fn base_tokens() -> ThemeTokens {
    ThemeTokens {
        font_family_ui: "Segoe UI".to_string(),
        font_family_display: "Segoe UI".to_string(),
        font_family_body: "Segoe UI".to_string(),
        font_family_mono: "Consolas".to_string(),

        bg_app: rgb(0x000000),          // Pure OLED black background
        bg_card: rgb(0x08080a),         // Layered L1 card surface (AGENTS.md #08080a)
        bg_elevated: rgb(0x111115),     // Surface L2 elevated/flyout (AGENTS.md #111115)
        bg_sidebar: rgb(0x000000),      // Pure OLED black sidebar
        bg_callout: rgba(0x8b5cf612),   // Subtle violet tinted quote box
        hover_bg: rgba(0xffffff0a),     // Clean ghost hover
        border: rgb(0x1e1e22),          // Precision hairline neutral border
        border_subtle: rgb(0x18181b),   // Subtle threadline and card divider
        border_strong: rgb(0x27272a),   // Focused inputs and card outlines
        border_focus: rgb(0x8b5cf6),    // Dialectic brand focus ring
        text_primary: rgb(0xf4f4f5),    // Crisp high-legibility pure white
        text_secondary: rgb(0xa1a1aa),  // Balanced neutral zinc secondary
        text_muted: rgb(0x71717a),      // Threads neutral muted metadata
        accent_user: rgb(0x38bdf8),     // Clean sky accent
        accent_alt: rgb(0xeab308),      // Dialectic grounding gold
        accent_agent: rgb(0xa855f7),    // Sophisticated dialectic purple
        accent_brand: rgb(0x8b5cf6),    // Brand violet
        accent_dialectic: rgb(0xeab308),// Grounding highlight
        accent_success: rgb(0x10b981),  // Active loop emerald
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

        cadence_level_1: rgb(0x064e3b),
        cadence_level_2: rgb(0x059669),
        cadence_level_3: rgb(0x10b981),
        cadence_level_4: rgb(0x34d399),

        graph_edge: rgb(0x2d3246),
        graph_node_neutral: rgb(0x6b7280),
        bg_feed_post: rgb(0x000000),

        font_size_2xs: px(10.0),
        font_size_xs: px(11.0),
        font_size_sm: px(12.5),
        font_size_base: px(14.0),
        font_size_md: px(15.5),
        font_size_lg: px(18.0),
        font_size_xl: px(22.0),

        spacing_3xs: px(1.0),
        spacing_2xs: px(2.0),
        spacing_px3: px(3.0),
        spacing_xs: px(4.0),
        spacing_sm: px(8.0),
        spacing_md: px(12.0),
        spacing_lg: px(16.0),
        spacing_xl: px(20.0),
        spacing_2xl: px(24.0),

        radius_xs: px(3.0),
        radius_sm: px(6.0),
        radius_md: px(8.0),
        radius_lg: px(12.0),
        radius_xl: px(16.0),
        radius_full: px(9999.0),

        border_width_hairline: px(1.0),
        border_width_threadline: px(2.0),

        titlebar_height: px(38.0),
        feed_header_height: px(44.0),
        post_avatar_col_width: px(38.0),
        threadline_min_height: px(24.0),
        win_control_width: px(28.0),
        win_control_height: px(24.0),
        avatar_size_sm: px(20.0),
        avatar_size_md: px(32.0),
        avatar_size_lg: px(34.0),
        modal_width: px(840.0),
        modal_height: px(580.0),
        modal_rail_width: px(220.0),
        modal_footer_height: px(54.0),
        cadence_cell_size: px(10.0),
        graph_canvas_height: px(140.0),

        modal_composer_width: px(580.0),
        modal_composer_max_height: px(620.0),
        post_image_max_height: px(280.0),
        post_image_preview_height: px(140.0),
        attachment_btn_size: px(28.0),

        contact_list_pane_width: px(236.0),
        agent_category_rail_width: px(204.0),
        contact_avatar_size: px(36.0),
        hero_avatar_size: px(48.0),
        config_tab_height: px(36.0),
        trait_bar_height: px(6.0),
        prompt_preview_height: px(220.0),

        line_height_body: 1.45,
    }
}

/// True Threads / Linear Minimalist OLED Black aesthetic (Default).
pub fn minimalist_dark() -> ThemeTokens {
    let mut t = base_tokens();
    common_tokens!(t);
    t
}

/// Obsidian Deep Zinc palette.
pub fn obsidian_zinc() -> ThemeTokens {
    let mut t = base_tokens();
    common_tokens!(t);

    t.bg_app = rgb(0x09090b);
    t.bg_card = rgb(0x121215);
    t.bg_elevated = rgb(0x18181b);
    t.bg_sidebar = rgb(0x09090b);
    t.bg_callout = rgba(0xa855f715);
    t.hover_bg = rgba(0xffffff0a);
    t.border = rgb(0x27272a);
    t.border_subtle = rgb(0x1f1f23);
    t.border_strong = rgb(0x3f3f46);
    t.border_focus = rgb(0xa855f7);
    t.text_primary = rgb(0xf4f4f5);
    t.text_secondary = rgb(0xa1a1aa);
    t.text_muted = rgb(0x71717a);
    t.accent_user = rgb(0x38bdf8);
    t.accent_alt = rgb(0xeab308);
    t.accent_agent = rgb(0xa855f7);
    t.accent_brand = rgb(0xa855f7);
    t.accent_dialectic = rgb(0xeab308);
    t.accent_success = rgb(0x10b981);
    t.accent_brand_subtle = rgba(0xa855f720);
    t.accent_brand_border = rgba(0xa855f72a);
    t.bg_feed_post = rgb(0x000000);
    t
}

/// Midnight Slate oceanic palette.
pub fn midnight_slate() -> ThemeTokens {
    let mut t = base_tokens();
    common_tokens!(t);

    t.bg_app = rgb(0x0a0e17);
    t.bg_card = rgb(0x0f172a);
    t.bg_elevated = rgb(0x1e293b);
    t.bg_sidebar = rgb(0x070a10);
    t.bg_callout = rgba(0x38bdf812);
    t.hover_bg = rgba(0xffffff0a);
    t.border = rgb(0x1e293b);
    t.border_subtle = rgb(0x162032);
    t.border_strong = rgb(0x334155);
    t.border_focus = rgb(0x38bdf8);
    t.text_primary = rgb(0xf8fafc);
    t.text_secondary = rgb(0x94a3b8);
    t.text_muted = rgb(0x64748b);
    t.accent_user = rgb(0x38bdf8);
    t.accent_alt = rgb(0xf59e0b);
    t.accent_agent = rgb(0x818cf8);
    t.accent_brand = rgb(0x38bdf8);
    t.accent_dialectic = rgb(0xf59e0b);
    t.accent_success = rgb(0x10b981);
    t.accent_brand_subtle = rgba(0x38bdf820);
    t.accent_brand_border = rgba(0x38bdf82a);
    t.accent_dialectic_subtle = rgba(0xf59e0b20);
    t.accent_dialectic_border = rgba(0xf59e0b40);
    t.bg_feed_post = rgb(0x000000);
    t
}
