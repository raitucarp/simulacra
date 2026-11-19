//! Screen geometry of a pan/zoom canvas: unit-square positions mapped through the Mind
//! Graph's viewport math into the frame, with a spatial grid for hit testing. Built once per
//! (data generation, frame, viewport) through [`GeometryCache`](super::cache::GeometryCache).

use super::spatial::SpatialGrid;
use crate::domain::graph::layout::LayoutPoint;
use crate::ui::features::mind_graph::viewport::{Fit, Frame, Viewport};

/// Cache key of a projection.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProjectionKey {
    pub generation: u64,
    pub frame: (f32, f32, f32, f32),
    pub pan: (f32, f32),
    pub zoom: f32,
}

impl ProjectionKey {
    pub fn new(generation: u64, frame: &Frame, view: &Viewport) -> Self {
        Self { generation, frame: (frame.x, frame.y, frame.width, frame.height), pan: view.pan, zoom: view.zoom }
    }
}

/// Points in window pixels and their hit-test grid.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Projected {
    pub points: Vec<(f32, f32)>,
    pub grid: SpatialGrid,
}

/// The fit of unit-square `positions` (their bounding box) with `padding` pixels inside.
pub fn fit_of(positions: &[[f32; 2]], padding: f32) -> Fit {
    let span = |axis: usize| {
        let (low, high) = positions.iter().fold((f32::INFINITY, f32::NEG_INFINITY), |(l, h), p| (l.min(p[axis]), h.max(p[axis])));
        if high >= low { high - low } else { 0.0 }
    };
    Fit::new(padding, (span(0), span(1)))
}

/// Maps `positions` to the window and buckets them in cells of `cell` pixels.
pub fn project(positions: &[[f32; 2]], frame: &Frame, view: &Viewport, fit: Fit, cell: f32) -> Projected {
    let points: Vec<(f32, f32)> = positions.iter().map(|p| view.to_screen(frame, fit, LayoutPoint { x: p[0], y: p[1] })).collect();
    let grid = SpatialGrid::new(&points, cell);
    Projected { points, grid }
}

/// The frame as a culling rectangle.
pub fn rect_of(frame: &Frame) -> (f32, f32, f32, f32) {
    (frame.x, frame.y, frame.width, frame.height)
}
