//! Stream pane header: Threads-style "For you" / "Dialectic" tabs and loop status pill.

use super::consts::ACTIVE_TAB_INDICATOR_WIDTH;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::{div, px, Div, FontWeight, ParentElement, Styled};
use rust_i18n::t;

/// Header row of the stream pane.
pub(crate) fn render_stream_header(theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .px(theme.spacing_lg)
        .h(theme.feed_header_height)
        .border_b(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .flex()
        .items_center()
        .justify_between()
        .child(
            div()
                .flex()
                .items_center()
                .gap(theme.spacing_xl)
                .h_full()
                .child(render_active_tab(theme))
                .child(render_inactive_tab(theme)),
        )
        .child(render_loop_status(theme))
}

/// Tab label text with the display typeface.
fn tab_label(theme: &ThemeTokens, weight: FontWeight) -> Div {
    div()
        .font_family(theme.font_family_display.clone())
        .text_size(theme.font_size_base)
        .font_weight(weight)
}

fn render_active_tab(theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .flex_col()
        .items_center()
        .h_full()
        .justify_between()
        .pt(theme.spacing_sm)
        .child(
            tab_label(theme, FontWeight::BOLD)
                .text_color(theme.text_primary)
                .child(t!("feed.for_you").to_string()),
        )
        .child(
            div()
                .w(px(ACTIVE_TAB_INDICATOR_WIDTH))
                .h(theme.border_width_threadline)
                .rounded(theme.radius_full)
                .bg(theme.accent_brand),
        )
}

fn render_inactive_tab(theme: &ThemeTokens) -> Div {
    div().pt(theme.spacing_sm).h_full().child(
        tab_label(theme, FontWeight::MEDIUM)
            .text_color(theme.text_muted)
            .child(t!("feed.dialectic").to_string()),
    )
}

/// "Loop active" status pill.
fn render_loop_status(theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .px(theme.spacing_sm)
        .py(theme.spacing_2xs)
        .rounded(theme.radius_full)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .child(
            div()
                .w(theme.font_size_xs)
                .h(theme.font_size_xs)
                .text_color(theme.accent_success)
                .child(Icon::new(LucideIcon::Activity)),
        )
        .child(
            div()
                .font_family(theme.font_family_mono.clone())
                .text_size(theme.font_size_xs)
                .font_weight(FontWeight::MEDIUM)
                .text_color(theme.accent_success)
                .child(t!("feed.loop_active").to_string()),
        )
}
