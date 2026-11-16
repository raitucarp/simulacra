//! Node sampling, golden-angle spiral placement and edge resolution for the dot graph.

use std::collections::HashMap;

use super::{
    CANVAS_HEIGHT, CANVAS_PADDING, CANVAS_WIDTH, GOLDEN_ANGLE_RADIANS, MAX_GRAPH_SCAN_WINDOW,
    MAX_OBSIDIAN_GRAPH_NODES,
};
use crate::domain::{Post, PostId};

/// Innermost spiral radius.
const SPIRAL_BASE_RADIUS: f32 = 18.0;
/// Radius growth per placed node.
const SPIRAL_RADIUS_STEP: f32 = 11.5;
/// Maximum radius growth on top of the base radius.
const SPIRAL_MAX_GROWTH: f32 = 52.0;
/// Vertical squash factor turning the spiral into an ellipse.
const SPIRAL_Y_SCALE: f32 = 0.72;

/// A canvas coordinate `(x, y)`.
pub(super) type Point = (f32, f32);

/// Samples up to `MAX_OBSIDIAN_GRAPH_NODES` posts within a bounded scan window,
/// prioritizing connected thoughts.
pub(super) fn sample_posts(posts: &[Post]) -> Vec<&Post> {
    let scan_slice = &posts[..posts.len().min(MAX_GRAPH_SCAN_WINDOW)];
    if scan_slice.len() <= MAX_OBSIDIAN_GRAPH_NODES {
        return scan_slice.iter().collect();
    }
    let mut sampled: Vec<&Post> = scan_slice
        .iter()
        .filter(|p| !p.linked_posts.is_empty() || p.reply_to_id.is_some())
        .take(MAX_OBSIDIAN_GRAPH_NODES)
        .collect();
    for p in scan_slice {
        if sampled.len() >= MAX_OBSIDIAN_GRAPH_NODES {
            break;
        }
        if !sampled.iter().any(|s| s.id == p.id) {
            sampled.push(p);
        }
    }
    sampled
}

/// Distributes `count` nodes (at least one) along a golden-angle spiral clamped to the canvas.
pub(super) fn spiral_coords(count: usize) -> Vec<Point> {
    let center_x = CANVAS_WIDTH / 2.0;
    let center_y = CANVAS_HEIGHT / 2.0;
    (0..count.max(1))
        .map(|idx| {
            let golden_angle = (idx as f32) * GOLDEN_ANGLE_RADIANS;
            let r = SPIRAL_BASE_RADIUS + ((idx as f32) * SPIRAL_RADIUS_STEP).min(SPIRAL_MAX_GROWTH);
            let x = center_x + r * golden_angle.cos();
            let y = center_y + r * SPIRAL_Y_SCALE * golden_angle.sin();
            (
                x.clamp(CANVAS_PADDING, CANVAS_WIDTH - CANVAS_PADDING),
                y.clamp(CANVAS_PADDING, CANVAS_HEIGHT - CANVAS_PADDING),
            )
        })
        .collect()
}

/// Resolves relational edges between sampled linked thoughts as `(from, to)` point pairs.
pub(super) fn edge_segments(sampled: &[&Post], coords: &[Point]) -> Vec<(Point, Point)> {
    let id_to_coord: HashMap<&PostId, Point> =
        sampled.iter().zip(coords.iter()).map(|(p, c)| (&p.id, *c)).collect();
    let mut segments = Vec::new();
    for (post, &from) in sampled.iter().zip(coords.iter()) {
        for linked_id in &post.linked_posts {
            if let Some(&to) = id_to_coord.get(linked_id) {
                if from != to {
                    segments.push((from, to));
                }
            }
        }
    }
    segments
}
