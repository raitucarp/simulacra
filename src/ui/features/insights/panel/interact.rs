//! Pointer handling of the two pan/zoom canvases (network and embedding map): hover by
//! spatial-grid hit test on the cached projection, drag to pan, wheel to zoom, click to
//! select or open.

use super::{CanvasState, Hover, InsightsPanel};
use crate::ui::features::insights::chart::projection::fit_of;
use crate::ui::features::insights::events::InsightsEvent;
use gpui_kit::*;

/// Extra pixels around a mark that still count as pointing at it.
pub const HIT_SLOP: f32 = 4.0;
/// Pixel height of one wheel "line" for line-based scroll deltas.
const SCROLL_LINE_HEIGHT: f32 = 20.0;

/// Which pan/zoom canvas an event is for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasSurface {
    Network,
    Map,
}

fn position(at: Point<Pixels>) -> (f32, f32) {
    (at.x.as_f32(), at.y.as_f32())
}

impl InsightsPanel {
    fn canvas(&mut self, surface: CanvasSurface) -> &mut CanvasState {
        match surface {
            CanvasSurface::Network => &mut self.network,
            CanvasSurface::Map => &mut self.map,
        }
    }

    /// The mark under `at` on `surface`, if any.
    fn hit(&self, surface: CanvasSurface, at: (f32, f32)) -> Hover {
        let canvas = match surface {
            CanvasSurface::Network => &self.network,
            CanvasSurface::Map => &self.map,
        };
        let Some(projected) = canvas.projected() else { return Hover::None };
        let reach = |i: usize| self.mark_radius(surface, i) + HIT_SLOP;
        match (surface, projected.grid.nearest(&projected.points, at, reach)) {
            (CanvasSurface::Network, Some(i)) => Hover::Node(i),
            (CanvasSurface::Map, Some(i)) => Hover::Point(i),
            (_, None) => Hover::None,
        }
    }

    pub(in crate::ui::features::insights) fn on_canvas_move(&mut self, surface: CanvasSurface, event: &MouseMoveEvent, cx: &mut Context<Self>) {
        let at = position(event.position);
        if event.pressed_button != Some(MouseButton::Left) {
            self.canvas(surface).pan.cancel();
        }
        if self.canvas(surface).pan.drag(at) {
            cx.notify();
            return;
        }
        let hover = self.hit(surface, at);
        if hover != self.hover || (hover != Hover::None && self.pointer != at) {
            (self.hover, self.pointer) = (hover, at);
            cx.notify();
        }
    }

    pub(in crate::ui::features::insights) fn on_canvas_press(&mut self, surface: CanvasSurface, event: &MouseDownEvent, _: &mut Window, _: &mut Context<Self>) {
        self.canvas(surface).pan.press(position(event.position));
    }

    pub(in crate::ui::features::insights) fn on_canvas_release(&mut self, surface: CanvasSurface, event: &MouseUpEvent, cx: &mut Context<Self>) {
        let clicked = self.canvas(surface).pan.release();
        if !clicked {
            cx.notify();
            return;
        }
        match (surface, self.hit(surface, position(event.position))) {
            (CanvasSurface::Network, Hover::Node(i)) if event.click_count > 1 || self.selected == Some(i) => self.open_node(i, cx),
            (CanvasSurface::Network, Hover::Node(i)) => self.selected = Some(i),
            (CanvasSurface::Network, _) => self.selected = None,
            (CanvasSurface::Map, Hover::Point(i)) => self.open_point(i, cx),
            _ => {}
        }
        cx.notify();
    }

    pub(in crate::ui::features::insights) fn on_canvas_scroll(&mut self, surface: CanvasSurface, event: &ScrollEvent, _: &mut Window, cx: &mut Context<Self>) {
        let positions = self.positions(surface);
        let Some(frame) = self.canvas(surface).frame.get() else { return };
        let fit = fit_of(&positions, self.canvas_padding());
        let delta = event.delta.pixel_delta(px(SCROLL_LINE_HEIGHT)).y.as_f32();
        self.canvas(surface).pan.wheel(&frame, fit, position(event.position), delta);
        cx.stop_propagation();
        cx.notify();
    }

    pub(in crate::ui::features::insights) fn on_canvas_leave(&mut self, cx: &mut Context<Self>) {
        if self.hover != Hover::None {
            self.hover = Hover::None;
            cx.notify();
        }
    }

    /// Opens what network node `i` stands for.
    pub(in crate::ui::features::insights) fn open_node(&mut self, i: usize, cx: &mut Context<Self>) {
        if let Some(node) = self.report.as_ref().and_then(|r| r.network.nodes.get(i).cloned()) {
            cx.emit(InsightsEvent::OpenNode(node));
        }
    }

    /// Opens the post of embedding point `i`.
    fn open_point(&mut self, i: usize, cx: &mut Context<Self>) {
        let post = self.report.as_ref().and_then(|r| r.embedding_posts.get(i).and_then(|&p| r.posts.get(p)).map(|p| p.id.clone()));
        if let Some(id) = post {
            cx.emit(InsightsEvent::OpenPost(id));
        }
    }

    /// Opens post `index` of the report.
    pub(in crate::ui::features::insights) fn open_post(&mut self, index: usize, cx: &mut Context<Self>) {
        if let Some(id) = self.report.as_ref().and_then(|r| r.posts.get(index)).map(|p| p.id.clone()) {
            cx.emit(InsightsEvent::OpenPost(id));
        }
    }

    /// Unit-square positions drawn on `surface`.
    pub(in crate::ui::features::insights) fn positions(&self, surface: CanvasSurface) -> Vec<[f32; 2]> {
        let Some(report) = &self.report else { return Vec::new() };
        match surface {
            CanvasSurface::Network => report.network.positions.clone(),
            CanvasSurface::Map => report.embedding.points.clone(),
        }
    }

    /// Inner padding of the pan/zoom canvases.
    pub(in crate::ui::features::insights) fn canvas_padding(&self) -> f32 {
        self.theme.spacing_xl.as_f32()
    }
}
