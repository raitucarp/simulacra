//! N12: the drawing pad of the ink sheet, its own entity so only the pad repaints while a
//! stroke is drawn. The pointer, pressed, draws with the chosen pen; Undo takes the last
//! stroke back and Clear all of them. Positions are kept in canvas units.

use crate::domain::post::ink::{Ink, InkDrawing, Stroke, PEN_WIDTHS};
use crate::theme::ThemeTokens;
use crate::ui::components::ink_figure::{ink_figure, placement};
use gpui_kit::*;
use std::cell::Cell;
use std::rc::Rc;

/// The pad: the strokes so far, the one being drawn, and the pen.
pub struct InkPad {
    drawing: InkDrawing,
    current: Option<Stroke>,
    pub ink: Ink,
    pub width: u8,
    bounds: Rc<Cell<Bounds<Pixels>>>,
    theme: ThemeTokens,
}

impl InkPad {
    pub fn new(theme: ThemeTokens) -> Self {
        Self { drawing: InkDrawing::blank(), current: None, ink: Ink::Text, width: PEN_WIDTHS[1], bounds: Rc::default(), theme }
    }

    /// The drawing as it stands (a stroke still being drawn included).
    pub fn drawing(&self) -> InkDrawing {
        let mut drawing = self.drawing.clone();
        drawing.strokes.extend(self.current.clone());
        drawing
    }

    pub fn undo(&mut self) {
        self.current = None;
        self.drawing.strokes.pop();
    }

    pub fn clear(&mut self) {
        self.current = None;
        self.drawing.strokes.clear();
    }

    /// `position` (in the window) in canvas units.
    fn to_canvas(&self, position: Point<Pixels>) -> (i16, i16) {
        let (scale, left, top) = placement(self.bounds.get(), self.drawing.width, self.drawing.height);
        let scale = scale.max(f32::EPSILON);
        (((position.x.as_f32() - left) / scale).round() as i16, ((position.y.as_f32() - top) / scale).round() as i16)
    }

    fn begin(&mut self, position: Point<Pixels>) {
        let (x, y) = self.to_canvas(position);
        self.current = Some(Stroke { points: vec![x, y], width: self.width, ink: self.ink });
    }

    fn extend(&mut self, position: Point<Pixels>) {
        let (x, y) = self.to_canvas(position);
        if let Some(stroke) = self.current.as_mut() {
            stroke.points.extend([x, y]);
        }
    }

    fn end(&mut self) {
        if let Some(stroke) = self.current.take() {
            self.drawing.strokes.push(stroke);
        }
    }
}

impl Render for InkPad {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let cell = self.bounds.clone();
        let probe = canvas(move |bounds, _, _| cell.set(bounds), |_, _, _, _| {}).absolute().top_0().left_0().size_full();
        div()
            .id("ink-pad")
            .relative()
            .w(theme.ink_canvas_width)
            .h(theme.ink_canvas_height)
            .rounded(theme.radius_md)
            .border(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .bg(theme.bg_card)
            .cursor_crosshair()
            .on_mouse_down(MouseButton::Left, cx.listener(|pad, event: &MouseDownEvent, _, cx| {
                pad.begin(event.position);
                cx.notify();
            }))
            .on_mouse_move(cx.listener(|pad, event: &MouseMoveEvent, _, cx| match event.pressed_button {
                Some(MouseButton::Left) if pad.current.is_some() => {
                    pad.extend(event.position);
                    cx.notify();
                }
                _ if pad.current.is_some() => {
                    pad.end();
                    cx.notify();
                }
                _ => {}
            }))
            .on_mouse_up(MouseButton::Left, cx.listener(|pad, _: &MouseUpEvent, _, cx| {
                pad.end();
                cx.notify();
            }))
            .child(probe)
            .child(ink_figure(self.drawing(), theme))
    }
}
