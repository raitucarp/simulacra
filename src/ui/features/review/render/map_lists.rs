//! The editor's lists: every reference connection ("A — B", "A → B", with its relation)
//! with direction and remove buttons, and every point with a remove button.

use super::super::panel::{MapEditor, ReviewPanel};
use super::super::text::tr;
use super::common::caption;
use crate::domain::review::connections::MapEdge;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

const DIRECTION_ID_PREFIX: &str = "review-map-direction-";
const REMOVE_EDGE_ID_PREFIX: &str = "review-map-remove-edge-";
const REMOVE_POINT_ID_PREFIX: &str = "review-map-remove-point-";
const UNDIRECTED: &str = "—";
const DIRECTED: &str = "→";

impl ReviewPanel {
    pub(super) fn render_map_lists(&self, editor: &MapEditor, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let titles: Vec<String> = self.map_chips(&editor.map, Some(&editor.posts)).into_iter().map(|chip| chip.title).collect();
        let edges: Vec<AnyElement> = editor.map.edges.iter().enumerate().map(|(index, edge)| self.edge_row(index, edge, &titles, cx).into_any_element()).collect();
        let points: Vec<AnyElement> = titles.iter().enumerate().map(|(index, title)| self.point_row(index, title, cx).into_any_element()).collect();
        let column = |label: String| div().flex_1().min_w(theme.review_count_tile_width).flex().flex_col().gap(theme.spacing_3xs).child(caption(label, theme));
        div()
            .flex()
            .flex_wrap()
            .items_start()
            .gap(theme.section_gap)
            .child(column(t!("review.map.connections", count = editor.map.edges.len()).to_string()).children(edges))
            .child(column(t!("review.map.points", count = editor.map.points.len()).to_string()).children(points))
    }

    fn list_row(&self, text: String) -> Div {
        let theme = &self.theme;
        div().flex().items_center().gap(theme.inline_gap).child(div().flex_1().min_w_0().truncate().text_size(theme.font_size_xs).text_color(theme.text_primary).child(text))
    }

    fn edge_row(&self, index: usize, edge: &MapEdge, titles: &[String], cx: &mut Context<Self>) -> impl IntoElement {
        let name = |i: usize| titles.get(i).cloned().unwrap_or_default();
        let link = if edge.directed { DIRECTED } else { UNDIRECTED };
        let text = match &edge.label {
            Some(label) => format!("{} {link} {} · {label}", name(edge.from), name(edge.to)),
            None => format!("{} {link} {}", name(edge.from), name(edge.to)),
        };
        let direction = Button::new(ElementId::NamedInteger(DIRECTION_ID_PREFIX.into(), index as u64)).ghost().xsmall().icon(Icon::new(LucideIcon::ArrowLeftRight)).tooltip(tr("review.map.direction"))
            .on_click(cx.listener(move |this, _, _, cx| this.cycle_edge(index, cx)));
        let remove = Button::new(ElementId::NamedInteger(REMOVE_EDGE_ID_PREFIX.into(), index as u64)).ghost().xsmall().icon(Icon::new(LucideIcon::X)).tooltip(tr("review.map.remove_connection"))
            .on_click(cx.listener(move |this, _, _, cx| this.remove_edge(index, cx)));
        self.list_row(text).child(direction).child(remove)
    }

    fn point_row(&self, index: usize, title: &str, cx: &mut Context<Self>) -> impl IntoElement {
        let remove = Button::new(ElementId::NamedInteger(REMOVE_POINT_ID_PREFIX.into(), index as u64)).ghost().xsmall().icon(Icon::new(LucideIcon::X)).tooltip(tr("review.map.remove_point"))
            .on_click(cx.listener(move |this, _, _, cx| this.remove_point(index, cx)));
        self.list_row(title.to_string()).child(remove)
    }
}
