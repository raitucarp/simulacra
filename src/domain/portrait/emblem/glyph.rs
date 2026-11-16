//! The emblem's central glyph, one visual cue per MBTI axis:
//! E/I → outer rays or an inner ring, N/S → spark or block core, T/F → straight ticks or
//! curved brackets, J/P → solid or dashed silhouette stroke (drawn by the silhouette).

use crate::domain::traits::MbtiType;
use serde::{Deserialize, Serialize};
use std::f32::consts::TAU;
use std::fmt::Write;

/// Letters that select the first pole of each axis.
const OUTWARD_LETTER: u8 = b'E';
const INTUITIVE_LETTER: u8 = b'N';
const THINKING_LETTER: u8 = b'T';
const JUDGING_LETTER: u8 = b'J';
/// Glyph geometry on the 256-unit emblem canvas.
const SPARK_OUTER: f32 = 34.0;
const SPARK_INNER: f32 = 10.0;
const BLOCK_SIDE: f32 = 44.0;
const BLOCK_RADIUS: f32 = 8.0;
const TICK_FROM: f32 = 42.0;
const TICK_TO: f32 = 56.0;
const BRACKET_RADIUS: f32 = 50.0;
const BRACKET_SPAN: f32 = 0.9;
const RAY_RADIUS: f32 = 104.0;
const RAY_DOT: f32 = 5.0;
const RAY_COUNT: usize = 8;
const RING_RADIUS: f32 = 62.0;
const INK_WIDTH: f32 = 4.0;
const RING_WIDTH: f32 = 3.0;
const RING_OPACITY: f32 = 0.5;
/// Vertices of the spark (four tips, four notches).
const SPARK_VERTICES: usize = 8;

/// The four MBTI axes as booleans (true = E, N, T, J).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(default)]
pub struct EmblemGlyph {
    pub outward: bool,
    pub intuitive: bool,
    pub thinking: bool,
    pub judging: bool,
}

impl EmblemGlyph {
    /// The glyph of an MBTI type (`ENTJ` → all four true).
    pub fn from_mbti(mbti: MbtiType) -> Self {
        let code = mbti.code().as_bytes();
        let letter = |i: usize, pole: u8| code.get(i).is_some_and(|c| c.eq_ignore_ascii_case(&pole));
        Self {
            outward: letter(0, OUTWARD_LETTER),
            intuitive: letter(1, INTUITIVE_LETTER),
            thinking: letter(2, THINKING_LETTER),
            judging: letter(3, JUDGING_LETTER),
        }
    }

    /// SVG elements of the glyph around (`c`, `c`); colors are already-formatted fills.
    pub(super) fn fragment(&self, c: f32, primary: &str, secondary: &str, ink: &str) -> String {
        let mut out = String::new();
        let point = |r: f32, a: f32| (c + r * a.sin(), c - r * a.cos());
        if self.outward {
            for i in 0..RAY_COUNT {
                let (x, y) = point(RAY_RADIUS, TAU * i as f32 / RAY_COUNT as f32);
                let _ = write!(out, r#"<circle cx="{x:.1}" cy="{y:.1}" r="{RAY_DOT}" fill="{primary}"/>"#);
            }
        } else {
            let _ = write!(out, r#"<circle cx="{c}" cy="{c}" r="{RING_RADIUS}" stroke="{secondary}" stroke-width="{RING_WIDTH}" stroke-opacity="{RING_OPACITY}"/>"#);
        }
        if self.intuitive {
            let spark: Vec<String> = (0..SPARK_VERTICES)
                .map(|i| point(if i % 2 == 0 { SPARK_OUTER } else { SPARK_INNER }, TAU * i as f32 / SPARK_VERTICES as f32))
                .map(|(x, y)| format!("{x:.1} {y:.1}"))
                .collect();
            let _ = write!(out, r#"<path d="M{}Z" fill="{secondary}"/>"#, spark.join("L"));
        } else {
            let o = c - BLOCK_SIDE / 2.0;
            let _ = write!(out, r#"<rect x="{o}" y="{o}" width="{BLOCK_SIDE}" height="{BLOCK_SIDE}" rx="{BLOCK_RADIUS}" fill="{secondary}"/>"#);
        }
        out.push_str(&self.judgment_marks(c, ink));
        out
    }

    /// Four straight ticks (thinking) or two facing brackets (feeling).
    fn judgment_marks(&self, c: f32, ink: &str) -> String {
        let mut d = String::new();
        if self.thinking {
            for (dx, dy) in [(0.0, -1.0), (1.0, 0.0), (0.0, 1.0), (-1.0, 0.0)] {
                let _ = write!(d, "M{:.1} {:.1}L{:.1} {:.1}", c + dx * TICK_FROM, c + dy * TICK_FROM, c + dx * TICK_TO, c + dy * TICK_TO);
            }
        } else {
            let (sx, sy) = (BRACKET_RADIUS * BRACKET_SPAN.sin(), BRACKET_RADIUS * BRACKET_SPAN.cos());
            let r = BRACKET_RADIUS;
            let _ = write!(d, "M{:.1} {:.1}A{r} {r} 0 0 1 {:.1} {:.1}", c + sx, c - sy, c + sx, c + sy);
            let _ = write!(d, "M{:.1} {:.1}A{r} {r} 0 0 1 {:.1} {:.1}", c - sx, c + sy, c - sx, c - sy);
        }
        format!(r#"<path d="{d}" stroke="{ink}" stroke-width="{INK_WIDTH}"/>"#)
    }
}
