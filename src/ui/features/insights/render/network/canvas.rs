//! The network canvas: geometry projected once per (report, frame, viewport) and reused by
//! every repaint and hit test; pointer, wheel and keyboard wiring; the hover tooltip.

use super::paint::NetworkPainter;
use crate::domain::insights::graph::metrics::Measure;
use crate::domain::insights::InsightsReport;
use crate::ui::features::forge::widgets::KeyboardStop;
use crate::ui::features::insights::chart::projection::{fit_of, project, rect_of, ProjectionKey};
use crate::ui::features::insights::chart::ticks::{compact, percent};
use crate::ui::features::insights::chart::tooltip::{floating_tooltip, tooltip_card};
use crate::ui::features::insights::panel::interact::CanvasSurface;
use crate::ui::features::insights::panel::{Hover, InsightsPanel, LegendId};
use crate::ui::features::mind_graph::viewport::Frame;
use super::super::common::tr;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;
use std::collections::HashSet;
use std::rc::Rc;

const CANVAS_ID: &str = "insights-network-canvas";
/// Estimated tooltip height used for flipping.
const TIP_HEIGHT: f32 = 120.0;

impl InsightsPanel {
    /// The node whose neighbourhood is highlighted: the hovered one, else the picked one.
    pub(super) fn network_anchor(&self) -> Option<usize> {
        match self.hover {
            Hover::Node(i) => Some(i),
            _ => self.selected,
        }
    }

    fn network_surface(&self, report: Rc<InsightsReport>) -> AnyElement {
        let (cache, slot, view) = (Rc::clone(&self.network.cache), Rc::clone(&self.network.frame), self.network.pan.view);
        let (state, theme, generation, anchor) = (self.state.clone(), self.theme.clone(), self.generation, self.network_anchor());
        let hidden: HashSet<usize> = self.hidden.iter().filter(|(legend, _)| *legend == LegendId::Network).map(|(_, i)| *i).collect();
        let (padding, cell) = (self.canvas_padding(), self.theme.insights_node_radius_max.as_f32() * 2.0);
        canvas(
            move |bounds, _, _| slot.set(Some(Frame::from(bounds))),
            move |bounds, (), window, cx| {
                let frame = Frame::from(bounds);
                let positions = &report.network.positions;
                let fit = fit_of(positions, padding);
                let projected = cache.get(&ProjectionKey::new(generation, &frame, &view), || project(positions, &frame, &view, fit, cell));
                let painter = NetworkPainter { report: &report, projected: &projected, state: &state, frame: rect_of(&frame), anchor, hidden: &hidden, theme: &theme };
                window.with_content_mask(Some(ContentMask { bounds }), |window| painter.paint(window, cx));
            },
        )
        .absolute()
        .top_0()
        .left_0()
        .size_full()
        .into_any_element()
    }

    /// Tooltip lines of node `i`.
    fn node_tip(&self, report: &InsightsReport, i: usize) -> Div {
        let network = &report.network;
        let m = &network.metrics;
        let degree = m.degrees[i];
        let lines = vec![
            t!("insights.tip.degree", total = degree.total, incoming = degree.incoming, outgoing = degree.outgoing).to_string(),
            t!("insights.tip.influence", value = format!("{:.4}", m.value(Measure::Influence, i))).to_string(),
            t!("insights.tip.betweenness", value = format!("{:.3}", m.value(Measure::Betweenness, i))).to_string(),
            t!("insights.tip.clustering", value = percent(m.value(Measure::Clustering, i))).to_string(),
            t!("insights.tip.community", number = compact((m.communities.of_node[i] + 1) as f64)).to_string(),
            tr("insights.tip.open"),
        ];
        tooltip_card(network.labels[i].clone(), lines, &self.theme)
    }

    pub(super) fn network_canvas(&self, report: &Rc<InsightsReport>, cx: &mut Context<Self>) -> Stateful<Div> {
        let theme = &self.theme;
        let tip = match (self.hover, self.network.frame.get()) {
            (Hover::Node(i), Some(frame)) if !self.network.pan.is_dragging() => Some(floating_tooltip(&frame, self.pointer, self.node_tip(report, i), TIP_HEIGHT, theme)),
            _ => None,
        };
        let dragging = self.network.pan.is_dragging();
        div()
            .id(CANVAS_ID)
            .relative()
            .w_full()
            .h(theme.insights_canvas_height)
            .min_h(theme.insights_canvas_min_height)
            .overflow_hidden()
            .rounded(theme.radius_md)
            .border(theme.border_width_hairline)
            .border_color(theme.border)
            .bg(theme.bg_app)
            .track_focus(&self.focus)
            .tab_stop_ring(theme)
            .map(|d| if dragging { d.cursor_grabbing() } else if matches!(self.hover, Hover::Node(_)) { d.cursor_pointer() } else { d.cursor_grab() })
            .on_key_down(cx.listener(Self::on_network_key))
            .on_mouse_move(cx.listener(|p, e: &MouseMoveEvent, _, cx| p.on_canvas_move(CanvasSurface::Network, e, cx)))
            .on_mouse_down(MouseButton::Left, cx.listener(|p, e: &MouseDownEvent, w, cx| p.on_canvas_press(CanvasSurface::Network, e, w, cx)))
            .on_mouse_up(MouseButton::Left, cx.listener(|p, e: &MouseUpEvent, _, cx| p.on_canvas_release(CanvasSurface::Network, e, cx)))
            .on_scroll(cx.listener(|p, e: &ScrollEvent, w, cx| p.on_canvas_scroll(CanvasSurface::Network, e, w, cx)))
            .on_hover(cx.listener(|p, hovered: &bool, _, cx| if !hovered { p.on_canvas_leave(cx) }))
            .child(self.network_surface(Rc::clone(report)))
            .children(tip)
    }
}
