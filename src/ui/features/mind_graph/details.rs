//! Bottom bar of the Mind Graph: the selected node, an "open" action and a view reset.

use super::geometry::node_color;
use super::legend::kind_label_key;
use crate::domain::graph::{GraphNode, NodeKind};
use crate::theme::ThemeTokens;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, App, ClickEvent, Div, FontWeight, InteractiveElement, ParentElement, StatefulInteractiveElement, Styled, Window};
use crate::i18n::t_count;
use rust_i18n::t;

/// Click handler of a details-bar button.
pub(super) type ClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

const OPEN_BUTTON_ID: &str = "mind-graph-open-node";
const RESET_BUTTON_ID: &str = "mind-graph-reset-view";

/// Translation key of the "open" action for a node kind.
fn open_label_key(kind: NodeKind) -> &'static str {
    match kind {
        NodeKind::Post => "graph.open_post",
        NodeKind::Document => "graph.open_doc",
        NodeKind::Hashtag => "graph.open_tag",
        NodeKind::Profile => "graph.open_profile",
    }
}

fn render_button(id: &'static str, label: String, on_click: ClickHandler, theme: &ThemeTokens) -> impl gpui_kit::IntoElement {
    div()
        .id(id)
        .px(theme.spacing_md)
        .py(theme.spacing_xs)
        .rounded(theme.radius_full)
        .border(theme.border_width_hairline)
        .border_color(theme.border)
        .bg(theme.bg_elevated)
        .text_color(theme.text_primary)
        .cursor_pointer()
        .hover(move |s| s.bg(theme.hover_bg))
        .on_click(on_click)
        .child(label)
}

fn render_selection(node: Option<(&GraphNode, usize)>, theme: &ThemeTokens) -> Div {
    let Some((node, degree)) = node else {
        return div().text_color(theme.text_muted).child(t!("graph.nothing_selected").to_string());
    };
    let kind = node.node.kind();
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_sm)
        .min_w_0()
        .child(div().w(theme.dot_size_sm).h(theme.dot_size_sm).rounded(theme.radius_full).bg(node_color(kind, theme)))
        .child(div().text_color(theme.text_muted).child(t!(kind_label_key(kind)).to_string()))
        .child(div().font_weight(FontWeight::MEDIUM).text_color(theme.text_primary).truncate().child(node.label.clone()))
        .child(div().text_color(theme.text_muted).child(t_count("graph.links_count", degree)))
}

/// Selected-node summary on the left; open and reset buttons on the right.
pub(super) fn render_details(
    node: Option<(&GraphNode, usize)>,
    on_open: ClickHandler,
    on_reset: ClickHandler,
    theme: &ThemeTokens,
) -> Div {
    let open_key = node.map(|(n, _)| open_label_key(n.node.kind()));
    div()
        .flex()
        .items_center()
        .justify_between()
        .gap(theme.spacing_md)
        .font_family(theme.font_family_ui.clone())
        .text_size(theme.font_size_xs)
        .child(render_selection(node, theme))
        .child(
            div()
                .flex()
                .items_center()
                .gap(theme.control_gap)
                .when_some(open_key, |row, key| row.child(render_button(OPEN_BUTTON_ID, t!(key).to_string(), on_open, theme)))
                .child(render_button(RESET_BUTTON_ID, t!("graph.reset_view").to_string(), on_reset, theme)),
        )
}
