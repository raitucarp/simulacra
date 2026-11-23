//! N12: keeping strokes small. Repeated points go, then points that lie within a unit of the
//! line between their neighbours (Ramer–Douglas–Peucker), so a slow straight stroke keeps its
//! two ends and a curve keeps its bends.

/// How far, in canvas units, a point may stray from the line and still be dropped.
const TOLERANCE: f32 = 1.0;

fn distance_to_line(point: (i16, i16), start: (i16, i16), end: (i16, i16)) -> f32 {
    let (px, py, ax, ay, bx, by) = (point.0 as f32, point.1 as f32, start.0 as f32, start.1 as f32, end.0 as f32, end.1 as f32);
    let (dx, dy) = (bx - ax, by - ay);
    let length = (dx * dx + dy * dy).sqrt();
    if length == 0.0 {
        return ((px - ax).powi(2) + (py - ay).powi(2)).sqrt();
    }
    ((px - ax) * dy - (py - ay) * dx).abs() / length
}

/// Marks the points of `points[first..=last]` to keep.
fn keep_between(points: &[(i16, i16)], first: usize, last: usize, keep: &mut [bool]) {
    if last <= first + 1 {
        return;
    }
    let (mut far, mut at) = (0.0f32, first);
    for index in first + 1..last {
        let distance = distance_to_line(points[index], points[first], points[last]);
        if distance > far {
            (far, at) = (distance, index);
        }
    }
    if far > TOLERANCE {
        keep[at] = true;
        keep_between(points, first, at, keep);
        keep_between(points, at, last, keep);
    }
}

/// `points` without repeats and without the points a straight line already passes through.
pub fn simplify(points: &[(i16, i16)]) -> Vec<(i16, i16)> {
    let mut unique: Vec<(i16, i16)> = Vec::with_capacity(points.len());
    for point in points {
        if unique.last() != Some(point) {
            unique.push(*point);
        }
    }
    if unique.len() <= 2 {
        return unique;
    }
    let mut keep = vec![false; unique.len()];
    let last = unique.len() - 1;
    (keep[0], keep[last]) = (true, true);
    keep_between(&unique, 0, last, &mut keep);
    unique.into_iter().zip(keep).filter_map(|(point, kept)| kept.then_some(point)).collect()
}
