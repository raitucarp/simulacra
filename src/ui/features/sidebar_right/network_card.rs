//! Network domain selection card molecule.

use crate::domain::Network;
use crate::theme::ThemeTokens;
use gpui_kit::base::StyledExt;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, px, Div, InteractiveElement, IntoElement, ParentElement, StatefulInteractiveElement, Styled, Window};
use rust_i18n::t;

/// Height of the active indicator bar on the card's leading edge.
const ACTIVE_INDICATOR_HEIGHT: f32 = 36.0;

fn text_line(theme_color: gpui_kit::Rgba) -> Div {
    div().w_full().min_w_0().text_xs().text_color(theme_color)
}

fn render_details(network: &Network, theme: &ThemeTokens) -> Div {
    let meta = t!(
        "sidebar.docs_and_agents",
        docs = network.doc_count,
        agents = network.agent_count
    )
    .to_string();

    div()
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .gap_1()
        .child(text_line(theme.text_primary).font_bold().child(network.name.clone()))
        .child(text_line(theme.accent_user).font_medium().child(meta))
        .child(text_line(theme.text_secondary).child(network.description.clone()))
}

/// Renders a domain card in the left sidebar with active indicator and multi-line wrapping description.
pub fn render_network_card(
    network: &Network,
    is_active: bool,
    theme: &ThemeTokens,
    on_click: impl Fn(&gpui_kit::ClickEvent, &mut Window, &mut gpui_kit::App) + 'static,
) -> impl IntoElement {
    let bg_color = if is_active { theme.hover_bg } else { theme.bg_card };
    let border_color = if is_active { theme.accent_agent } else { theme.border };
    let hover_bg = theme.hover_bg;

    div()
        .id(format!("net-card-{}", network.id))
        .w_full()
        .p_3()
        .rounded_lg()
        .border_1()
        .border_color(border_color)
        .bg(bg_color)
        .cursor_pointer()
        .hover(move |s| s.bg(hover_bg))
        .on_click(on_click)
        .flex()
        .items_start()
        .gap_2p5()
        .when(is_active, |this| {
            this.child(
                div()
                    .w_1()
                    .h(px(ACTIVE_INDICATOR_HEIGHT))
                    .rounded_full()
                    .bg(theme.accent_agent),
            )
        })
        .child(render_details(network, theme))
}
