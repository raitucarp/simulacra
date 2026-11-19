//! A force layout that scales to thousands of nodes: Fruchterman–Reingold with repulsion only
//! between nodes in neighbouring cells of a uniform grid (cell side three ideal edge lengths,
//! the repulsion tapering smoothly to zero there), so each round costs O(n + m) instead of
//! O(n²). The round count is set from a work budget; seeding is a golden-angle disk, so the
//! result is deterministic. Positions are fitted, aspect kept, into the unit square.

use super::Graph;
use std::collections::HashMap;

const GOLDEN_ANGLE: f32 = 2.399_963;
const SEED_RADIUS: f32 = 0.5;
const INITIAL_TEMPERATURE: f32 = 0.08;
const GRAVITY: f32 = 0.6;
const MIN_DISTANCE: f32 = 1e-4;
/// Grid cell side in ideal edge lengths; repulsion fades to zero at this distance, the same
/// in every direction (a hard, cell-shaped cutoff would pile nodes up on the cell borders).
const CELL_SPAN: f32 = 3.0;
/// Node-plus-edge visits allowed over all rounds.
const WORK_BUDGET: usize = 3_000_000;
pub const MIN_ROUNDS: usize = 25;
pub const MAX_ROUNDS: usize = 220;

/// Rounds for a graph of `n` nodes and `m` edges.
pub fn rounds_for(n: usize, m: usize) -> usize {
    (WORK_BUDGET / (n + m).max(1)).clamp(MIN_ROUNDS, MAX_ROUNDS)
}

fn seed(n: usize) -> Vec<[f32; 2]> {
    (0..n)
        .map(|i| {
            let radius = SEED_RADIUS * ((i as f32 + 0.5) / n as f32).sqrt();
            let angle = i as f32 * GOLDEN_ANGLE;
            [radius * angle.cos(), radius * angle.sin()]
        })
        .collect()
}

/// Positions in the unit square, index-aligned with the graph's nodes.
pub fn force_layout(graph: &Graph) -> Vec<[f32; 2]> {
    let n = graph.node_count();
    let mut points = seed(n);
    if n < 2 {
        return fit_unit(&points);
    }
    let k = (1.0 / n as f32).sqrt();
    let rounds = rounds_for(n, graph.edge_count());
    for round in 0..rounds {
        let temperature = INITIAL_TEMPERATURE * (1.0 - round as f32 / rounds as f32);
        let mut shift = repulsion(&points, k);
        attraction(graph, &points, k, &mut shift);
        for (p, d) in points.iter_mut().zip(&shift) {
            let step = [d[0] - p[0] * GRAVITY, d[1] - p[1] * GRAVITY];
            let length = (step[0] * step[0] + step[1] * step[1]).sqrt().max(MIN_DISTANCE);
            let scale = length.min(temperature) / length;
            (p[0], p[1]) = (p[0] + step[0] * scale, p[1] + step[1] * scale);
        }
    }
    fit_unit(&points)
}

/// Repulsion `k² / d` between nodes of the same or adjacent grid cells.
fn repulsion(points: &[[f32; 2]], k: f32) -> Vec<[f32; 2]> {
    let cell = k * CELL_SPAN;
    let key = |p: &[f32; 2]| ((p[0] / cell).floor() as i32, (p[1] / cell).floor() as i32);
    let mut grid: HashMap<(i32, i32), Vec<usize>> = HashMap::new();
    points.iter().enumerate().for_each(|(i, p)| grid.entry(key(p)).or_default().push(i));
    let mut shift = vec![[0.0f32; 2]; points.len()];
    for (i, p) in points.iter().enumerate() {
        let (cx, cy) = key(p);
        for (dx, dy) in (-1..=1).flat_map(|dx| (-1..=1).map(move |dy| (dx, dy))) {
            for &j in grid.get(&(cx + dx, cy + dy)).map_or(&[][..], Vec::as_slice) {
                if j == i {
                    continue;
                }
                let delta = [p[0] - points[j][0], p[1] - points[j][1]];
                let distance = (delta[0] * delta[0] + delta[1] * delta[1]).sqrt().max(MIN_DISTANCE);
                if distance >= cell {
                    continue;
                }
                let taper = (1.0 - distance / cell).powi(2);
                let force = k * k / distance / distance * taper;
                (shift[i][0], shift[i][1]) = (shift[i][0] + delta[0] * force, shift[i][1] + delta[1] * force);
            }
        }
    }
    shift
}

/// Spring attraction `d² / k` along every edge.
fn attraction(graph: &Graph, points: &[[f32; 2]], k: f32, shift: &mut [[f32; 2]]) {
    for (a, b) in graph.edges() {
        let delta = [points[a][0] - points[b][0], points[a][1] - points[b][1]];
        let distance = (delta[0] * delta[0] + delta[1] * delta[1]).sqrt().max(MIN_DISTANCE);
        let force = distance / k;
        for axis in 0..2 {
            shift[a][axis] -= delta[axis] * force;
            shift[b][axis] += delta[axis] * force;
        }
    }
}

/// Uniformly scales `points` into the unit square, centred.
pub fn fit_unit(points: &[[f32; 2]]) -> Vec<[f32; 2]> {
    let (mut min, mut max) = ([f32::INFINITY; 2], [f32::NEG_INFINITY; 2]);
    for p in points {
        for axis in 0..2 {
            (min[axis], max[axis]) = (min[axis].min(p[axis]), max[axis].max(p[axis]));
        }
    }
    let extent = [(max[0] - min[0]).max(0.0), (max[1] - min[1]).max(0.0)];
    let span = extent[0].max(extent[1]).max(MIN_DISTANCE);
    let offset = [(1.0 - extent[0] / span) / 2.0, (1.0 - extent[1] / span) / 2.0];
    points.iter().map(|p| [(p[0] - min[0]) / span + offset[0], (p[1] - min[1]) / span + offset[1]]).collect()
}
