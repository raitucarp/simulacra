//! Book jackets.

use super::shapes::{circle, n, paint, polygon, rect, BLACK, WHITE};
use super::{CoverSeed, COVER_HEIGHT, COVER_WIDTH};
use std::f32::consts::TAU;

/// Book jackets: one of three layouts (a centred emblem in rings, a large cropped disc,
/// diagonal bands), all with a spine and top and bottom rules.
pub fn book(seed: &CoverSeed, ink: &str) -> String {
    let spine = rect(0.0, 0.0, 10.0, COVER_HEIGHT, BLACK, 0.28);
    let rules = rect(18.0, 22.0, 90.0, 2.0, WHITE, 0.6) + &rect(18.0, 156.0, 90.0, 2.0, WHITE, 0.6);
    let face = match seed.pick(9, 3) {
        0 => emblem_face(seed, ink),
        1 => disc_face(seed, ink),
        _ => band_face(seed, ink),
    };
    spine + &face + &rules
}

fn emblem_face(seed: &CoverSeed, ink: &str) -> String {
    let (cx, cy) = (COVER_WIDTH / 2.0 + 4.0, seed.between(10, 70.0, 96.0));
    let rings: String = (0..2 + seed.pick(2, 3)).map(|i| circle(cx, cy, 18.0 + i as f32 * 9.0, WHITE, 1.5, 0.35)).collect();
    let emblem = polygon(cx, cy, seed.between(3, 12.0, 20.0), 4 + seed.pick(4, 5), seed.between(5, 0.0, TAU), ink);
    rings + &emblem + &rect(10.0, seed.between(6, 126.0, 140.0), COVER_WIDTH - 10.0, seed.between(7, 6.0, 12.0), ink, 0.85)
}

fn disc_face(seed: &CoverSeed, ink: &str) -> String {
    let (cx, cy, r) = (seed.between(11, 60.0, 110.0), seed.between(12, 60.0, 120.0), seed.between(13, 40.0, 62.0));
    let disc = format!(r#"<circle cx="{}" cy="{}" r="{}" fill="{}"/>"#, n(cx), n(cy), n(r), paint(ink));
    disc + &circle(cx, cy, r * 0.62, WHITE, 2.0, 0.45) + &rect(22.0, 138.0, 60.0, 4.0, WHITE, 0.55)
}

fn band_face(seed: &CoverSeed, ink: &str) -> String {
    let turn = seed.between(14, -35.0, -15.0);
    let bands: String = (0..3)
        .map(|i| {
            let y = 40.0 + i as f32 * seed.between(15 + i, 22.0, 34.0);
            let fill = if i == 1 { WHITE } else { ink };
            format!(r#"<g transform="rotate({} 60 90)">{}</g>"#, n(turn), rect(-30.0, y, 180.0, 12.0 + i as f32 * 4.0, fill, 0.85))
        })
        .collect();
    bands + &rect(22.0, 138.0, 54.0, 4.0, WHITE, 0.55)
}
