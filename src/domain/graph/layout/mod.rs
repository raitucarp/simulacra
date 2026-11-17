//! Deterministic force-directed layout of a [`GraphModel`], normalized to the unit square.

mod force;

use super::GraphModel;

/// Smallest bounding-box span used when normalizing, so single-node graphs stay finite.
const MIN_SPAN: f64 = 1e-9;
/// Center of the unit square.
pub const LAYOUT_CENTER: f32 = 0.5;

/// A node position with both coordinates in `0.0..=1.0`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutPoint {
    pub x: f32,
    pub y: f32,
}

/// Node positions, index-aligned with [`GraphModel::nodes`].
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GraphLayout {
    positions: Vec<LayoutPoint>,
}

impl GraphLayout {
    /// Runs a seeded Fruchterman–Reingold relaxation with a fixed, size-dependent
    /// iteration count and fits the result (aspect preserved) into the unit square.
    pub fn compute(model: &GraphModel) -> Self {
        let count = model.node_count();
        if count == 0 {
            return Self::default();
        }
        let edges: Vec<(usize, usize)> = model.edges().iter().map(|e| (e.source, e.target)).collect();
        let mut points = force::initial_positions(count);
        force::relax(&mut points, &edges, force::iterations_for(count));
        Self { positions: normalize(&points) }
    }

    pub fn positions(&self) -> &[LayoutPoint] {
        &self.positions
    }

    pub fn position(&self, index: usize) -> Option<LayoutPoint> {
        self.positions.get(index).copied()
    }

    /// Width and height of the box around every position (zero when empty).
    pub fn extent(&self) -> (f32, f32) {
        let span = |axis: fn(&LayoutPoint) -> f32| {
            let values = self.positions.iter().map(axis);
            let (low, high) = values.fold((f32::INFINITY, f32::NEG_INFINITY), |(low, high), v| (low.min(v), high.max(v)));
            if high >= low { high - low } else { 0.0 }
        };
        (span(|p| p.x), span(|p| p.y))
    }
}

/// Uniformly scales `points` into the unit square and centers them.
fn normalize(points: &[[f64; 2]]) -> Vec<LayoutPoint> {
    let mut min = [f64::INFINITY; 2];
    let mut max = [f64::NEG_INFINITY; 2];
    for p in points {
        for axis in 0..2 {
            min[axis] = min[axis].min(p[axis]);
            max[axis] = max[axis].max(p[axis]);
        }
    }
    let extent = [max[0] - min[0], max[1] - min[1]];
    let span = extent[0].max(extent[1]).max(MIN_SPAN);
    let center = f64::from(LAYOUT_CENTER);
    let offset = [center - extent[0] / span * center, center - extent[1] / span * center];
    points
        .iter()
        .map(|p| LayoutPoint {
            x: ((p[0] - min[0]) / span + offset[0]) as f32,
            y: ((p[1] - min[1]) / span + offset[1]) as f32,
        })
        .collect()
}
