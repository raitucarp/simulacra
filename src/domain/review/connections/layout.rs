//! Where the points of a drill sit: slots on one ring (two rings for larger maps), handed out
//! in a seeded order so the layout never gives the structure away and changes from one
//! review to the next, yet stays reproducible. Positions are fractions of the free area
//! (0–1 on each axis), so the view sizes them to its container.

use crate::domain::review::quiz::shuffled;
use std::f32::consts::{FRAC_PI_2, TAU};

/// Most points placed on a single ring.
pub const SINGLE_RING_POINTS: usize = 8;
/// Share of the points on the outer ring of a two-ring layout.
const OUTER_SHARE: f32 = 0.6;
/// Inner ring radius as a share of the outer one.
const INNER_RADIUS: f32 = 0.42;
const HALF: f32 = 0.5;

/// `count` slots on a ring of `radius` (share of the half-extent), starting at `phase`.
fn ring(count: usize, radius: f32, phase: f32) -> Vec<(f32, f32)> {
    (0..count)
        .map(|i| {
            let angle = phase - FRAC_PI_2 + TAU * i as f32 / count.max(1) as f32;
            (HALF + HALF * radius * angle.cos(), HALF + HALF * radius * angle.sin())
        })
        .collect()
}

/// The slots of `count` points, in ring order.
pub fn slots(count: usize) -> Vec<(f32, f32)> {
    if count <= SINGLE_RING_POINTS {
        return ring(count, 1.0, 0.0);
    }
    let outer = (count as f32 * OUTER_SHARE).ceil() as usize;
    let inner = count - outer;
    let offset = TAU / (2 * inner.max(1)) as f32;
    ring(outer, 1.0, 0.0).into_iter().chain(ring(inner, INNER_RADIUS, offset)).collect()
}

/// The position of each of `count` points (by point index), shuffled by `seed`.
pub fn scatter(count: usize, seed: u64) -> Vec<(f32, f32)> {
    let order = shuffled((0..count).collect::<Vec<usize>>(), seed);
    let slots = slots(count);
    order.into_iter().map(|slot| slots[slot]).collect()
}

/// A seed from an item id and its review count (a new arrangement every review).
pub fn drill_seed(id: &str, reps: u32) -> u64 {
    id.bytes().fold(u64::from(reps), |hash, byte| hash.rotate_left(5) ^ u64::from(byte))
}
