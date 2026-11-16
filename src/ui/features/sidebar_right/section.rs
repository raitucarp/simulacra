//! Shared section chrome (icon + title + trailing badge) for right sidebar panels.

use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::StyledExt;
use gpui_kit::component::Icon;
use gpui_kit::*;

/// Icon + bold display-font title used by the heatmap, dot graph and spark feed headers.
pub(super) fn render_section_title(
    icon: LucideIcon,
    icon_color: Rgba,
    title: String,
    theme: &ThemeTokens,
) -> Div {
    div()
        .flex()
        .items_center()
        .gap_2()
        .child(div().w_4().h_4().text_color(icon_color).child(Icon::new(icon)))
        .child(
            div()
                .font_family(theme.font_family_display.clone())
                .text_xs()
                .font_bold()
                .text_color(theme.text_primary)
                .child(title),
        )
}

/// Header row placing a title on the left and a badge on the right.
pub(super) fn render_section_header(title: Div, badge: impl IntoElement) -> Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .child(title)
        .child(badge)
}

/// Bordered panel section with a bottom divider used by the graph and heatmap cards.
pub(super) fn render_divided_section(theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .pb_4()
        .border_b_1()
        .border_color(theme.border)
        .flex()
        .flex_col()
        .gap_2p5()
}

/// Small hover-tinted mono badge (padding `px_2 py_0p5`) with an accent text color.
pub(super) fn render_mono_badge(text: String, color: Rgba, theme: &ThemeTokens) -> Div {
    div()
        .px_2()
        .py_0p5()
        .bg(theme.hover_bg)
        .border_1()
        .border_color(theme.border)
        .font_family(theme.font_family_mono.clone())
        .text_xs()
        .font_medium()
        .text_color(color)
        .child(text)
}
