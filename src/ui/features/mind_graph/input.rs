//! Pointer input of the Mind Graph canvas: drag to pan, wheel to zoom, hover and click.

use super::geometry::hit_test;
use super::panel::{Drag, MindGraphEvent, MindGraphPanel};
use super::viewport::wheel_factor;
use gpui_kit::{
    px, Context, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Point,
    ScrollEvent, Window,
};

/// Pointer travel (pixels) after which a press becomes a drag instead of a click.
const CLICK_SLOP: f32 = 3.0;
/// Pixel height of one wheel "line" for line-based scroll deltas.
const SCROLL_LINE_HEIGHT: f32 = 20.0;
/// Click count of a double click.
const DOUBLE_CLICK: usize = 2;

pub(super) fn xy(p: Point<Pixels>) -> (f32, f32) {
    (p.x.as_f32(), p.y.as_f32())
}

impl MindGraphPanel {
    /// Node under `pos` in the last painted frame.
    pub(super) fn hit(&self, pos: (f32, f32)) -> Option<usize> {
        let frame = self.frame.get()?;
        hit_test(&self.scene, &self.view, &frame, self.fit(), self.radius_range(), pos)
    }

    pub(super) fn on_press(&mut self, event: &MouseDownEvent, _: &mut Window, _: &mut Context<Self>) {
        self.drag = Some(Drag { origin: xy(event.position), pan: self.view.pan, moved: false });
    }

    pub(super) fn on_move(&mut self, event: &MouseMoveEvent, _: &mut Window, cx: &mut Context<Self>) {
        let pos = xy(event.position);
        let pressed = event.pressed_button == Some(MouseButton::Left);
        if let Some(drag) = self.drag.as_mut().filter(|_| pressed) {
            let (dx, dy) = (pos.0 - drag.origin.0, pos.1 - drag.origin.1);
            drag.moved |= dx.hypot(dy) > CLICK_SLOP;
            if drag.moved {
                self.view.pan = (drag.pan.0 + dx, drag.pan.1 + dy);
                cx.notify();
            }
            return;
        }
        self.drag = None;
        let hovered = self.hit(pos);
        if hovered != self.hovered {
            self.hovered = hovered;
            cx.notify();
        }
    }

    pub(super) fn on_release(&mut self, event: &MouseUpEvent, _: &mut Window, cx: &mut Context<Self>) {
        if self.drag.take().is_some_and(|drag| drag.moved) {
            return;
        }
        let hit = self.hit(xy(event.position)).map(|idx| self.scene.model.nodes()[idx].node.clone());
        if let Some(node) = hit.clone() {
            cx.emit(MindGraphEvent::Select(node.clone()));
            if event.click_count >= DOUBLE_CLICK {
                cx.emit(MindGraphEvent::Open(node));
            }
        }
        self.selected = hit;
        cx.notify();
    }

    pub(super) fn on_release_out(&mut self, _: &MouseUpEvent, _: &mut Window, _: &mut Context<Self>) {
        self.drag = None;
    }

    pub(super) fn on_scroll(&mut self, event: &ScrollEvent, _: &mut Window, cx: &mut Context<Self>) {
        let Some(frame) = self.frame.get() else {
            return;
        };
        let delta = event.delta.pixel_delta(px(SCROLL_LINE_HEIGHT)).y.as_f32();
        self.view.zoom_at(&frame, self.fit(), xy(event.position), wheel_factor(delta));
        cx.notify();
    }
}
