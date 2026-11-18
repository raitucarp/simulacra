//! State of one pan/zoom canvas: its view and drag, the frame painted last and the cached
//! screen geometry.

use crate::ui::features::insights::chart::cache::GeometryCache;
use crate::ui::features::insights::chart::pan_zoom::PanZoom;
use crate::ui::features::insights::chart::projection::{Projected, ProjectionKey};
use crate::ui::features::mind_graph::viewport::Frame;
use std::cell::Cell;
use std::rc::Rc;

#[derive(Default)]
pub struct CanvasState {
    pub pan: PanZoom,
    pub frame: Rc<Cell<Option<Frame>>>,
    pub cache: Rc<GeometryCache<ProjectionKey, Projected>>,
    /// Neighbours in meaning of the hovered point, by (report generation, point).
    pub neighbours: Rc<GeometryCache<(u64, usize), Vec<(usize, f32)>>>,
}

impl CanvasState {
    /// Back to the fitted view, forgetting cached geometry.
    pub fn reset(&mut self) {
        self.pan.reset();
        self.cache.clear();
    }

    /// The projection painted last.
    pub fn projected(&self) -> Option<Rc<Projected>> {
        self.cache.last()
    }
}
