//! Header, metric rows and agent pills of the network telemetry roster card.

use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::StyledExt;
use gpui_kit::component::Icon;
use gpui_kit::*;

const TELEMETRY_TITLE: &str = "NETWORK MEMBER TELEMETRY";

fn render_title(theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_sm)
        .child(
            div()
                .w(theme.font_size_base)
                .h(theme.font_size_base)
                .text_color(theme.accent_agent)
                .child(Icon::new(LucideIcon::Cpu)),
        )
        .child(
            div()
                .text_size(theme.font_size_xs)
                .font_bold()
                .text_color(theme.text_primary)
                .child(TELEMETRY_TITLE),
        )
}

pub(super) fn render_header(theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .child(render_title(theme))
}

/// One label/value telemetry metric row.
pub(super) fn render_metric_row(label: &'static str, value: String, value_color: Rgba, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .child(div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(label))
        .child(
            div()
                .text_size(theme.font_size_xs)
                .font_medium()
                .text_color(value_color)
                .child(value),
        )
}

/// Pill showing an agent name and its MBTI code.
pub(super) fn render_agent_pill(name: String, mbti: String, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .px(theme.spacing_sm)
        .py(theme.spacing_2xs)
        .rounded(theme.radius_sm)
        .bg(theme.hover_bg)
        .text_size(theme.font_size_2xs)
        .font_medium()
        .text_color(theme.accent_agent)
        .child(
            div()
                .w(theme.font_size_2xs)
                .h(theme.font_size_2xs)
                .child(Icon::new(LucideIcon::User)),
        )
        .child(format!("{} · {}", name, mbti))
}
