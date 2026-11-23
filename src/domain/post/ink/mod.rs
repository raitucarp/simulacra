//! N12: ink posts. A post can carry one hand drawing ([`InkDrawing`], `extras.ink`): vector
//! strokes on a small canvas, each a run of points with a width and an ink, kept compact
//! (whole canvas units, near-duplicate and collinear points dropped). The drawing renders as
//! SVG ([`to_svg`]) for the file saved next to the post and for exports, and is painted from
//! the same strokes on its card. It counts as an attachment, so an ink post is never locked.

mod simplify;
mod svg;

pub use simplify::simplify;
pub use svg::{to_svg, SVG_TEXT_INK};

use serde::{Deserialize, Serialize};

/// The canvas a drawing is made on, in its own units (16:9).
pub const INK_WIDTH: u16 = 480;
pub const INK_HEIGHT: u16 = 270;
/// The most strokes, and points in all, one drawing keeps.
pub const MAX_STROKES: usize = 400;
pub const MAX_POINTS: usize = 20_000;
/// Stroke widths offered, in canvas units.
pub const PEN_WIDTHS: [u8; 3] = [2, 4, 8];

/// The ink of a stroke: theme colours, so a drawing follows the palette it is shown in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Ink {
    /// The text colour.
    #[default]
    Text,
    Accent,
    Danger,
    Success,
}

impl Ink {
    pub const ALL: [Self; 4] = [Self::Text, Self::Accent, Self::Danger, Self::Success];
}

/// One stroke: points as `[x0, y0, x1, y1, …]` in canvas units.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Stroke {
    pub points: Vec<i16>,
    pub width: u8,
    pub ink: Ink,
}

impl Stroke {
    /// The points as pairs.
    pub fn pairs(&self) -> impl Iterator<Item = (i16, i16)> + '_ {
        self.points.chunks_exact(2).map(|pair| (pair[0], pair[1]))
    }
}

/// A hand drawing on a post (`extras.ink`, boxed).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct InkDrawing {
    pub width: u16,
    pub height: u16,
    pub strokes: Vec<Stroke>,
    /// The SVG file saved in the media folder, once written.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svg_path: Option<String>,
}

impl InkDrawing {
    /// An empty drawing on the standard canvas.
    pub fn blank() -> Self {
        Self { width: INK_WIDTH, height: INK_HEIGHT, strokes: Vec::new(), svg_path: None }
    }

    /// Every point of every stroke.
    pub fn point_count(&self) -> usize {
        self.strokes.iter().map(|stroke| stroke.points.len() / 2).sum()
    }

    /// Whether there is anything drawn.
    pub fn is_blank(&self) -> bool {
        self.strokes.iter().all(|stroke| stroke.points.len() < 2)
    }

    /// The drawing as kept: points clamped to the canvas, strokes simplified, empty strokes
    /// dropped and the limits applied (`None` when nothing is left).
    pub fn finished(mut self) -> Option<Self> {
        let (width, height) = (self.width.max(1) as i16, self.height.max(1) as i16);
        let mut budget = MAX_POINTS;
        self.strokes = std::mem::take(&mut self.strokes)
            .into_iter()
            .take(MAX_STROKES)
            .filter_map(|mut stroke| {
                let clamped: Vec<(i16, i16)> = stroke.pairs().map(|(x, y)| (x.clamp(0, width), y.clamp(0, height))).collect();
                let kept: Vec<(i16, i16)> = simplify(&clamped).into_iter().take(budget).collect();
                budget -= kept.len();
                stroke.points = kept.into_iter().flat_map(|(x, y)| [x, y]).collect();
                stroke.width = stroke.width.clamp(PEN_WIDTHS[0], PEN_WIDTHS[PEN_WIDTHS.len() - 1]);
                (!stroke.points.is_empty()).then_some(stroke)
            })
            .collect();
        (!self.is_blank()).then_some(self)
    }
}

#[cfg(test)]
mod tests;
