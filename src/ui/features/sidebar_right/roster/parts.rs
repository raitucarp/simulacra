//! Header, metric rows and agent pills of the network telemetry roster card.

use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::StyledExt;
use gpui_kit::component::Icon;
use gpui_kit::*;

const TELEMETRY_TITLE: &str = "NETWORK MEMBER TELEMETRY";
const ACTIVE_LABEL: &str = "Active";

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

fn render_active_badge(theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .px(theme.spacing_sm)
        .py(theme.spacing_2xs)
        .rounded(theme.radius_full)
        .bg(theme.hover_bg)
        .border(theme.border_width_hairline)
        .border_color(theme.border)
        .child(
            div()
                .w(theme.font_size_xs)
                .h(theme.font_size_xs)
                .text_color(theme.accent_success)
                .child(Icon::new(LucideIcon::Activity)),
        )
        .child(
            div()
                .text_size(theme.font_size_xs)
                .font_medium()
                .text_color(theme.accent_success)
                .child(ACTIVE_LABEL),
        )
}

pub(super) fn render_header(theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .child(render_title(theme))
        .child(render_active_badge(theme))
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
