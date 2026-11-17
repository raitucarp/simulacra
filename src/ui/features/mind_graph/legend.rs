//! Node-kind color legend and interaction hint of the Mind Graph.

use super::geometry::node_color;
use crate::domain::graph::{NodeKind, NODE_KINDS};
use crate::theme::ThemeTokens;
use gpui_kit::{div, Div, ParentElement, Styled};
use rust_i18n::t;

/// Translation key of the legend label for each node kind.
pub(super) fn kind_label_key(kind: NodeKind) -> &'static str {
    match kind {
        NodeKind::Post => "graph.legend_thoughts",
        NodeKind::Document => "graph.legend_docs",
        NodeKind::Hashtag => "graph.legend_tags",
        NodeKind::Profile => "graph.legend_profiles",
    }
}

fn render_entry(kind: NodeKind, count: usize, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_sm)
        .child(
            div()
                .w(theme.dot_size_sm)
                .h(theme.dot_size_sm)
                .rounded(theme.radius_full)
                .bg(node_color(kind, theme)),
        )
        .child(t!("graph.legend_entry", label = t!(kind_label_key(kind)), count = count).to_string())
}

/// Legend row (one swatch and count per node kind) with the pan/zoom hint on the right.
pub(super) fn render_legend(counts: [usize; NODE_KINDS.len()], theme: &ThemeTokens) -> Div {
    let entries = NODE_KINDS.iter().zip(counts).map(|(kind, count)| render_entry(*kind, count, theme));
    div()
        .flex()
        .flex_wrap()
        .items_center()
        .justify_between()
        .gap(theme.spacing_md)
        .font_family(theme.font_family_ui.clone())
        .text_size(theme.font_size_xs)
        .text_color(theme.text_muted)
        .child(div().flex().flex_wrap().items_center().gap(theme.spacing_xl).children(entries))
        .child(div().child(t!("graph.hint").to_string()))
}
