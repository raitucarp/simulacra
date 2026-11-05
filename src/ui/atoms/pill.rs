//! Atomic pill components for Obsidian links and hashtags.
//! Strictly tokenized via ThemeTokens.

use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::*;

/// Renders an Obsidian `[[post_id]]` clickable reference pill with tokenized styles.
pub fn render_obsidian_pill(target_id: &str, theme: &ThemeTokens) -> impl IntoElement {
    let hover_bg = theme.hover_bg;
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_px3)
        .px(theme.spacing_xs)
        .py(theme.spacing_3xs)
        .rounded(theme.radius_xs)
        .bg(theme.bg_elevated)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .font_family(theme.font_family_mono.clone())
        .text_size(theme.font_size_xs)
        .font_weight(gpui_kit::FontWeight::MEDIUM)
        .text_color(theme.accent_user)
        .cursor_pointer()
        .hover(move |s| s.bg(hover_bg))
        .child(Icon::new(LucideIcon::Link2).size(theme.font_size_xs))
        .child(target_id.trim().to_string())
}

/// Renders a hashtag span with tokenized styling.
pub fn render_hashtag_span(tag: &str, theme: &ThemeTokens) -> impl IntoElement {
    let hover_bg = theme.hover_bg;
    let clean_tag = tag.trim_start_matches('#');
    div()
        .px(theme.spacing_xs)
        .py(theme.spacing_2xs)
        .rounded(theme.radius_sm)
        .bg(theme.hover_bg)
        .font_family(theme.font_family_ui.clone())
        .text_size(theme.font_size_xs)
        .font_weight(gpui_kit::FontWeight::MEDIUM)
        .text_color(theme.accent_agent)
        .cursor_pointer()
        .hover(move |s| s.bg(hover_bg))
        .child(format!("#{}", clean_tag))
}
