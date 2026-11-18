//! Viewport culling and level-of-detail labels for large canvases: only what intersects the
//! frame is drawn, and labels go to the most important visible nodes that fit without
//! overlapping (greedy placement, so the budget is small and bounded).

/// A screen rectangle `(x, y, width, height)`.
pub type Rect = (f32, f32, f32, f32);

/// Whether a point (with a `margin` around it) touches `frame`.
pub fn point_visible((x, y): (f32, f32), margin: f32, frame: Rect) -> bool {
    x + margin >= frame.0 && x - margin <= frame.0 + frame.2 && y + margin >= frame.1 && y - margin <= frame.1 + frame.3
}

/// Whether the box around segment `a`–`b` touches `frame` (a cheap, conservative test).
pub fn segment_visible(a: (f32, f32), b: (f32, f32), frame: Rect) -> bool {
    let (left, right) = (a.0.min(b.0), a.0.max(b.0));
    let (top, bottom) = (a.1.min(b.1), a.1.max(b.1));
    right >= frame.0 && left <= frame.0 + frame.2 && bottom >= frame.1 && top <= frame.1 + frame.3
}

/// Squared length below which an edge is not worth a stroke at this zoom.
pub fn too_short(a: (f32, f32), b: (f32, f32), min_length: f32) -> bool {
    let (dx, dy) = (a.0 - b.0, a.1 - b.1);
    dx * dx + dy * dy < min_length * min_length
}

fn overlaps(a: Rect, b: Rect) -> bool {
    a.0 < b.0 + b.2 && b.0 < a.0 + a.2 && a.1 < b.1 + b.3 && b.1 < a.1 + a.3
}

/// The labels to draw: `candidates` are `(id, box)` in priority order; each is kept when it
/// lies inside `frame` and overlaps no kept label, until `limit` are kept.
pub fn place_labels(candidates: &[(usize, Rect)], frame: Rect, limit: usize) -> Vec<usize> {
    let mut kept: Vec<(usize, Rect)> = Vec::new();
    for &(id, rect) in candidates {
        if kept.len() >= limit {
            break;
        }
        let inside = rect.0 >= frame.0 && rect.1 >= frame.1 && rect.0 + rect.2 <= frame.0 + frame.2 && rect.1 + rect.3 <= frame.1 + frame.3;
        if inside && kept.iter().all(|(_, other)| !overlaps(rect, *other)) {
            kept.push((id, rect));
        }
    }
    kept.into_iter().map(|(id, _)| id).collect()
}

/// How many labels a frame affords: one per `share` of a label-sized cell of its area.
pub fn label_budget(frame: Rect, label: (f32, f32), share: f32) -> usize {
    let cell = (label.0 * label.1 / share.max(f32::EPSILON)).max(1.0);
    (frame.2 * frame.3 / cell) as usize
}
