//! Spawned AI agent persona status card molecule.

use crate::theme::ThemeTokens;
use gpui_kit::base::StyledExt;
use gpui_kit::*;

fn render_initial_badge(initial: &str, theme: &ThemeTokens) -> Div {
    div()
        .w_7()
        .h_7()
        .rounded_full()
        .bg(theme.bg_card)
        .border_1()
        .border_color(theme.accent_agent)
        .text_color(theme.accent_agent)
        .flex()
        .items_center()
        .justify_center()
        .text_xs()
        .font_bold()
        .child(initial.to_string())
}

fn render_status_text(name: &str, status_desc: &str, theme: &ThemeTokens) -> Div {
    div()
        .flex_1()
        .flex()
        .flex_col()
        .child(
            div()
                .flex()
                .items_center()
                .gap_1p5()
                .child(div().w_1p5().h_1p5().rounded_full().bg(theme.accent_agent))
                .child(
                    div()
                        .text_xs()
                        .font_bold()
                        .text_color(theme.text_primary)
                        .child(name.to_string()),
                ),
        )
        .child(div().text_xs().text_color(theme.text_secondary).child(status_desc.to_string()))
}

pub fn render_agent_status_row(
    name: &str,
    initial: &str,
    status_desc: &str,
    theme: &ThemeTokens,
) -> impl IntoElement {
    let hover_bg = theme.hover_bg;

    div()
        .w_full()
        .p_2()
        .rounded_md()
        .hover(move |s| s.bg(hover_bg))
        .cursor_pointer()
        .flex()
        .items_center()
        .gap_2p5()
        .child(render_initial_badge(initial, theme))
        .child(render_status_text(name, status_desc, theme))
}
