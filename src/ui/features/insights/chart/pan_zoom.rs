//! Pan and zoom of a canvas: drag to pan, wheel to zoom at the pointer, a press without
//! movement is a click. Pure state transitions over the Mind Graph's [`Viewport`].

use crate::ui::features::mind_graph::viewport::{Fit, Frame, Viewport};

pub const ZOOM_MIN: f32 = 0.5;
pub const ZOOM_MAX: f32 = 12.0;
/// Zoom factor per scrolled pixel (exponential).
const ZOOM_PER_PIXEL: f32 = 0.002;
/// Movement (pixels) after which a press is a drag rather than a click.
const DRAG_THRESHOLD: f32 = 3.0;

/// A press in progress.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Press {
    origin: (f32, f32),
    pan: (f32, f32),
    moved: bool,
}

/// The view of one canvas and its drag.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PanZoom {
    pub view: Viewport,
    press: Option<Press>,
}

impl PanZoom {
    pub fn press(&mut self, at: (f32, f32)) {
        self.press = Some(Press { origin: at, pan: self.view.pan, moved: false });
    }

    /// Pans while pressed; returns whether the view changed.
    pub fn drag(&mut self, at: (f32, f32)) -> bool {
        let Some(press) = self.press.as_mut() else { return false };
        let (dx, dy) = (at.0 - press.origin.0, at.1 - press.origin.1);
        press.moved |= dx.abs() > DRAG_THRESHOLD || dy.abs() > DRAG_THRESHOLD;
        if !press.moved {
            return false;
        }
        self.view.pan = (press.pan.0 + dx, press.pan.1 + dy);
        true
    }

    /// Ends the press; true when it was a click (no drag).
    pub fn release(&mut self) -> bool {
        self.press.take().is_some_and(|press| !press.moved)
    }

    /// Forgets a press whose release happened elsewhere.
    pub fn cancel(&mut self) {
        self.press = None;
    }

    pub fn is_dragging(&self) -> bool {
        self.press.is_some_and(|press| press.moved)
    }

    /// Zooms by a vertical wheel delta (pixels) keeping the point under `cursor` still.
    pub fn wheel(&mut self, frame: &Frame, fit: Fit, cursor: (f32, f32), delta_y: f32) {
        let factor = (delta_y * ZOOM_PER_PIXEL).exp();
        let wanted = (self.view.zoom * factor).clamp(ZOOM_MIN, ZOOM_MAX);
        self.view.zoom_at(frame, fit, cursor, wanted / self.view.zoom);
    }

    /// Back to the fitted view.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn is_reset(&self) -> bool {
        self.view == Viewport::default()
    }
}
