//! Layout of the Mind Graph view: header, legend, interactive canvas and details bar.

use super::details::render_details;
use super::header::render_header;
use super::legend::render_legend;
use super::paint::ScenePainter;
use super::panel::MindGraphPanel;
use super::viewport::Frame;
use crate::domain::graph::NODE_KINDS;
use crate::ui::components::{persisted_v_resizable, SplitPane};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const ROOT_ID: &str = "mind-graph-view";
const CANVAS_ID: &str = "mind-graph-canvas";
/// Layout id of the canvas | details split (sizes persist).
const SPLIT_ID: &str = "mind-graph-split";

impl MindGraphPanel {
    /// Canvas container wired for drag, wheel, hover and click, painting the scene.
    fn render_canvas(&self, cx: &mut Context<Self>) -> Stateful<Div> {
        let (scene, theme) = (Rc::clone(&self.scene), self.theme.clone());
        let (view, focus, options) = (self.view, self.focus(), self.options());
        let frame_slot = Rc::clone(&self.frame);
        let surface = canvas(
            move |bounds, _, _| frame_slot.set(Some(Frame::from(bounds))),
            move |bounds, (), window, cx| {
                let frame = Frame::from(bounds);
                ScenePainter::new(&scene, (view, frame), &options, focus, &theme).paint(window, cx);
            },
        )
        .absolute()
        .top_0()
        .left_0()
        .size_full();
        let is_empty = self.scene.model.node_count() == 0;
        let (dragging, pointing) = (self.drag.is_some_and(|d| d.moved), self.hovered.is_some());
        div()
            .id(CANVAS_ID)
            .relative()
            .flex_1()
            .min_h_0()
            .w_full()
            .overflow_hidden()
            .rounded(self.theme.radius_md)
            .border(self.theme.border_width_hairline)
            .border_color(self.theme.border)
            .bg(self.theme.bg_app)
            .map(|d| if dragging { d.cursor_grabbing() } else if pointing { d.cursor_pointer() } else { d.cursor_grab() })
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_press))
            .on_mouse_move(cx.listener(Self::on_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::on_release))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::on_release_out))
            .on_scroll(cx.listener(Self::on_scroll))
            .child(surface)
            .when(is_empty, |d| d.flex().items_center().justify_center().child(self.render_empty()))
    }

    fn render_empty(&self) -> Div {
        div()
            .text_size(self.theme.font_size_sm)
            .text_color(self.theme.text_muted)
            .child(t!("graph.empty").to_string())
    }

    fn render_card(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let model = &self.scene.model;
        let counts = NODE_KINDS.map(|kind| model.count_of(kind));
        let selection = self.selected_index().map(|idx| (&model.nodes()[idx], self.scene.degrees[idx]));
        let on_open = Box::new(cx.listener(|this, _: &ClickEvent, _, cx| this.open_selected(cx)));
        let on_reset = Box::new(cx.listener(|this, _: &ClickEvent, _, cx| this.reset_view(cx)));
        let details = div().size_full().pt(theme.stack_gap).child(render_details(selection, on_open, on_reset, theme).w_full());
        let split = persisted_v_resizable(SPLIT_ID, theme)
            .pane(SplitPane::flex(theme.graph_canvas_height).child(self.render_canvas(cx)))
            .pane(SplitPane::sized(theme.graph_details_height, theme.graph_details_min..theme.graph_details_max).child(details));
        div()
            .flex_1()
            .min_h_0()
            .w_full()
            .p(theme.card_padding)
            .rounded(theme.radius_lg)
            .border(theme.border_width_hairline)
            .border_color(theme.border)
            .bg(theme.bg_card)
            .flex()
            .flex_col()
            .gap(theme.stack_gap)
            .child(render_legend(counts, theme))
            .child(div().flex_1().min_h_0().w_full().child(split))
    }
}

/// Name of the graph view in the dev render counter.
const STATS_NAME: &str = "MindGraph";

impl Render for MindGraphPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        crate::ui::render_stats::record(STATS_NAME);
        let model = &self.scene.model;
        let header = render_header(model.node_count(), model.edge_count(), &self.theme);
        div()
            .id(ROOT_ID)
            .size_full()
            .flex()
            .flex_col()
            .gap(self.theme.section_gap)
            .p(self.theme.panel_padding)
            .bg(self.theme.bg_app)
            .child(header)
            .child(self.render_card(cx))
    }
}
