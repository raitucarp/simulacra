//! Fruchterman–Reingold relaxation with center gravity and linear cooling.
//!
//! Seeding is a golden-angle (phyllotaxis) disk, so no randomness is involved and no two
//! nodes start on the same spot; every run with the same input yields the same output.

/// Layout area; the ideal edge length is `sqrt(AREA / n)`.
const AREA: f64 = 1.0;
/// Radius of the seeding disk.
const SEED_RADIUS: f64 = 0.5;
/// Golden angle in radians, spacing the seeded nodes.
const GOLDEN_ANGLE: f64 = 2.399_963_229_728_653;
/// Half step placing seed `i` at the middle of its ring.
const SEED_HALF_STEP: f64 = 0.5;
/// Maximum displacement in the first iteration.
const INITIAL_TEMPERATURE: f64 = 0.1;
/// Pull of every node towards the center, keeping disconnected parts together.
const GRAVITY: f64 = 1.0;
/// Distances below this are clamped to avoid division by zero.
const MIN_DISTANCE: f64 = 1e-4;
/// Upper bound of pair evaluations across all iterations.
const PAIR_BUDGET: usize = 12_000_000;
const MIN_ITERATIONS: usize = 30;
const MAX_ITERATIONS: usize = 300;

/// Number of iterations for `count` nodes: as many as the pair budget allows, clamped.
pub(super) fn iterations_for(count: usize) -> usize {
    let pairs = (count * count.saturating_sub(1) / 2).max(1);
    (PAIR_BUDGET / pairs).clamp(MIN_ITERATIONS, MAX_ITERATIONS)
}

/// Golden-angle disk seeding.
pub(super) fn initial_positions(count: usize) -> Vec<[f64; 2]> {
    (0..count)
        .map(|i| {
            let radius = SEED_RADIUS * ((i as f64 + SEED_HALF_STEP) / count as f64).sqrt();
            let angle = i as f64 * GOLDEN_ANGLE;
            [radius * angle.cos(), radius * angle.sin()]
        })
        .collect()
}

/// Direction and clamped length from `b` to `a`; coincident points get a fixed direction.
fn separation(a: [f64; 2], b: [f64; 2], salt: usize) -> ([f64; 2], f64) {
    let delta = [a[0] - b[0], a[1] - b[1]];
    let length = (delta[0] * delta[0] + delta[1] * delta[1]).sqrt();
    if length < MIN_DISTANCE {
        let angle = salt as f64 * GOLDEN_ANGLE;
        return ([angle.cos(), angle.sin()], MIN_DISTANCE);
    }
    ([delta[0] / length, delta[1] / length], length)
}

fn repulse(points: &[[f64; 2]], k: f64, disp: &mut [[f64; 2]]) {
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let (dir, dist) = separation(points[i], points[j], i + j);
            let force = k * k / dist;
            for axis in 0..2 {
                disp[i][axis] += dir[axis] * force;
                disp[j][axis] -= dir[axis] * force;
            }
        }
    }
}

fn attract(points: &[[f64; 2]], edges: &[(usize, usize)], k: f64, disp: &mut [[f64; 2]]) {
    for &(source, target) in edges {
        let (dir, dist) = separation(points[source], points[target], source + target);
        let force = dist * dist / k;
        for axis in 0..2 {
            disp[source][axis] -= dir[axis] * force;
            disp[target][axis] += dir[axis] * force;
        }
    }
}

/// Moves every point along its displacement, limited to `temperature`, with center gravity.
fn apply(points: &mut [[f64; 2]], disp: &[[f64; 2]], temperature: f64) {
    for (point, d) in points.iter_mut().zip(disp) {
        let step = [d[0] - point[0] * GRAVITY, d[1] - point[1] * GRAVITY];
        let length = (step[0] * step[0] + step[1] * step[1]).sqrt();
        if !length.is_finite() || length < MIN_DISTANCE {
            continue;
        }
        let scale = length.min(temperature) / length;
        point[0] += step[0] * scale;
        point[1] += step[1] * scale;
    }
}

/// Runs `iterations` rounds of repulsion, attraction and cooling.
pub(super) fn relax(points: &mut [[f64; 2]], edges: &[(usize, usize)], iterations: usize) {
    let k = (AREA / points.len().max(1) as f64).sqrt();
    let cooling = INITIAL_TEMPERATURE / iterations.max(1) as f64;
    let mut temperature = INITIAL_TEMPERATURE;
    for _ in 0..iterations {
        let mut disp = vec![[0.0; 2]; points.len()];
        repulse(points, k, &mut disp);
        attract(points, edges, k, &mut disp);
        apply(points, &disp, temperature);
        temperature = (temperature - cooling).max(cooling);
    }
}
