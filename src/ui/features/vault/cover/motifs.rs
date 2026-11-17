//! One abstract motif per document kind (an emblem for books, text columns for articles,
//! ruled lines for notes, a frame for web pages, a waveform for transcripts, a grid for
//! datasets, rings for media), varied by the seed.

use super::{CoverSeed, COVER_HEIGHT, COVER_WIDTH};
use crate::domain::document::DocumentKind;
use std::f32::consts::TAU;

use super::shapes::{circle, n, polygon, rect, BLACK, WHITE};

fn article(seed: &CoverSeed, ink: &str) -> String {
    let header = rect(12.0, 14.0, 96.0, 26.0, ink, 1.0);
    let lines: String = (0..14)
        .flat_map(|row| {
            let y = 52.0 + row as f32 * 8.5;
            [(12.0, 0u64), (64.0, 1)].map(|(x, col)| rect(x, y, seed.between(row * 2 + col + 6, 24.0, 44.0), 3.0, WHITE, 0.45))
        })
        .collect();
    header + &lines
}

fn note(seed: &CoverSeed, ink: &str) -> String {
    let ruled: String = (0..13).map(|row| rect(14.0, 30.0 + row as f32 * 11.0, 94.0, 1.0, WHITE, 0.3)).collect();
    let margin = rect(26.0, 12.0, 1.5, 160.0, ink, 1.0);
    let corner = r##"<path d="M96 0 L120 0 L120 24 Z" fill="#fff" fill-opacity=".35"/>"##;
    let mark = circle(seed.between(7, 50.0, 90.0), seed.between(8, 50.0, 140.0), seed.between(9, 8.0, 16.0), ink, 3.0, 1.0);
    ruled + &margin + corner + &mark
}

fn web_page(seed: &CoverSeed, ink: &str) -> String {
    let bar = rect(0.0, 0.0, COVER_WIDTH, 18.0, BLACK, 0.3);
    let dots: String = (0..3).map(|i| circle(10.0 + i as f32 * 8.0, 9.0, 2.0, WHITE, 2.0, 0.7)).collect();
    let blocks: String = (0..6)
        .map(|i| {
            let (col, row) = ((i % 2) as f32, (i / 2) as f32);
            let fill = if seed.pick(10 + i, 3) == 0 { ink } else { WHITE };
            rect(12.0 + col * 50.0, 30.0 + row * 46.0, 46.0, seed.between(20 + i, 24.0, 40.0), fill, 0.5)
        })
        .collect();
    bar + &dots + &blocks
}

fn transcript(seed: &CoverSeed, ink: &str) -> String {
    let bars: String = (0..17)
        .map(|i| {
            let h = seed.between(i + 3, 8.0, 70.0);
            rect(12.0 + i as f32 * 5.8, COVER_HEIGHT / 2.0 - h / 2.0, 3.2, h, if i % 4 == 0 { WHITE } else { ink }, 0.9)
        })
        .collect();
    bars + &rect(12.0, 150.0, 60.0, 4.0, WHITE, 0.5)
}

fn dataset(seed: &CoverSeed, ink: &str) -> String {
    (0..54)
        .map(|i| {
            let (col, row) = ((i % 6) as f32, (i / 6) as f32);
            let level = seed.pick(i + 2, 4) as f32;
            rect(12.0 + col * 16.0, 16.0 + row * 16.5, 13.0, 13.0, if level == 0.0 { WHITE } else { ink }, 0.2 + level * 0.25)
        })
        .collect()
}

fn media(seed: &CoverSeed, ink: &str) -> String {
    let (cx, cy) = (COVER_WIDTH / 2.0, COVER_HEIGHT * 0.46);
    let rings: String = (0..3 + seed.pick(3, 3)).map(|i| circle(cx, cy, 14.0 + i as f32 * 8.0, ink, 3.0, 0.9)).collect();
    let play = format!(r##"<path d="M{} {} L{} {} L{} {} Z" fill="#fff" fill-opacity=".9"/>"##, n(cx - 6.0), n(cy - 9.0), n(cx + 10.0), n(cy), n(cx - 6.0), n(cy + 9.0));
    rings + &play
}

fn other(seed: &CoverSeed, ink: &str) -> String {
    (0..3 + seed.pick(5, 3))
        .map(|i| {
            let (x, y) = (seed.between(i * 3 + 7, 20.0, 100.0), seed.between(i * 3 + 8, 30.0, 150.0));
            let size = seed.between(i * 3 + 9, 14.0, 36.0);
            if i % 2 == 0 { polygon(x, y, size, 4, seed.between(i, 0.0, TAU), ink) } else { circle(x, y, size, WHITE, 4.0, 0.6) }
        })
        .collect()
}

/// The kind's motif in `ink` over the ground.
pub fn motif(kind: DocumentKind, seed: &CoverSeed, ink: &str) -> String {
    match kind {
        DocumentKind::Book => super::book::book(seed, ink),
        DocumentKind::Article => article(seed, ink),
        DocumentKind::Note => note(seed, ink),
        DocumentKind::WebPage => web_page(seed, ink),
        DocumentKind::Transcript => transcript(seed, ink),
        DocumentKind::Dataset => dataset(seed, ink),
        DocumentKind::Media => media(seed, ink),
        DocumentKind::Other => other(seed, ink),
    }
}

/// A soft light from the top edge and a shadow at the foot, common to every cover.
pub fn finish(_kind: DocumentKind) -> String {
    rect(0.0, 0.0, COVER_WIDTH, 3.0, WHITE, 0.25) + &rect(0.0, COVER_HEIGHT - 6.0, COVER_WIDTH, 6.0, BLACK, 0.2)
}
