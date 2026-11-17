//! Title, subtitle and node/link counters of the Mind Graph view.

use crate::theme::ThemeTokens;
use gpui_kit::{div, Div, FontWeight, ParentElement, Rgba, Styled};
use crate::i18n::t_count;
use rust_i18n::t;

fn render_titles(theme: &ThemeTokens) -> Div {
    div()
        .min_w_0()
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .child(
            div()
                .font_family(theme.font_family_display.clone())
                .text_size(theme.font_size_lg)
                .font_weight(FontWeight::BOLD)
                .text_color(theme.text_primary)
                .child(t!("graph.title").to_string()),
        )
        .child(
            div()
                .font_family(theme.font_family_ui.clone())
                .text_size(theme.font_size_xs)
                .text_color(theme.text_secondary)
                .child(t!("graph.subtitle").to_string()),
        )
}

fn render_counter(label: String, color: Rgba, theme: &ThemeTokens) -> Div {
    div()
        .flex_none()
        .whitespace_nowrap()
        .px(theme.spacing_md)
        .py(theme.spacing_xs)
        .rounded(theme.radius_full)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border)
        .font_family(theme.font_family_mono.clone())
        .text_size(theme.font_size_xs)
        .font_weight(FontWeight::MEDIUM)
        .text_color(color)
        .child(label)
}

/// Header row with titles on the left and node/link counters on the right (wrapping under
/// the titles when the panel is narrow).
pub(super) fn render_header(total_nodes: usize, total_links: usize, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .flex_wrap()
        .items_center()
        .justify_between()
        .gap(theme.stack_gap)
        .child(render_titles(theme))
        .child(
            div()
                .flex()
                .items_center()
                .gap(theme.spacing_sm)
                .child(render_counter(
                    t_count("graph.nodes_count", total_nodes),
                    theme.accent_agent,
                    theme,
                ))
                .child(render_counter(
                    t_count("graph.links_count", total_links),
                    theme.accent_user,
                    theme,
                )),
        )
}
