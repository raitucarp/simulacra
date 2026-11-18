//! A uniform grid over screen points for hit testing: the pointer looks only at its own and
//! the neighbouring cells, so hovering stays O(1) on average however many points there are.

use std::collections::HashMap;

/// Points bucketed by a square cell size.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpatialGrid {
    cell: f32,
    buckets: HashMap<(i32, i32), Vec<u32>>,
}

impl SpatialGrid {
    /// Buckets `points` into cells of side `cell` (at least one pixel).
    pub fn new(points: &[(f32, f32)], cell: f32) -> Self {
        let cell = cell.max(1.0);
        let mut buckets: HashMap<(i32, i32), Vec<u32>> = HashMap::new();
        for (i, p) in points.iter().enumerate() {
            buckets.entry(Self::key_of(*p, cell)).or_default().push(i as u32);
        }
        Self { cell, buckets }
    }

    fn key_of((x, y): (f32, f32), cell: f32) -> (i32, i32) {
        ((x / cell).floor() as i32, (y / cell).floor() as i32)
    }

    /// The point closest to `at` within `reach(i)` of it (each point may have its own reach,
    /// up to one cell), preferring the nearest.
    pub fn nearest(&self, points: &[(f32, f32)], at: (f32, f32), reach: impl Fn(usize) -> f32) -> Option<usize> {
        let (cx, cy) = Self::key_of(at, self.cell);
        let around = (-1..=1).flat_map(|dx| (-1..=1).map(move |dy| (cx + dx, cy + dy)));
        around
            .filter_map(|key| self.buckets.get(&key))
            .flatten()
            .map(|&i| i as usize)
            .filter_map(|i| {
                let (dx, dy) = (points[i].0 - at.0, points[i].1 - at.1);
                let distance = (dx * dx + dy * dy).sqrt();
                (distance <= reach(i)).then_some((i, distance))
            })
            .min_by(|a, b| a.1.total_cmp(&b.1).then(a.0.cmp(&b.0)))
            .map(|(i, _)| i)
    }
}
