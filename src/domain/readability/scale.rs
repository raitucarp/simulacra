//! Bands and the shared ease scale.
//!
//! Flesch's own table (1948) reads the reading-ease score as: 90–100 very easy, 80–90 easy,
//! 70–80 fairly easy, 60–70 standard, 50–60 fairly difficult, 30–50 difficult, 0–30 very
//! difficult. The bands here merge it into five (≥ 80, 60–80, 50–60, 30–50, < 30); Amstad,
//! Kandel–Moles, Fernández Huerta, the Portuguese adaptation and Oborneva are calibrated to
//! the same 0–100 scale and use the same table.
//!
//! LIX is read with Björnsson's usual interpretation (below 30 very easy, 30–40 easy, 40–50
//! medium, 50–60 difficult, above 60 very difficult). Characters per sentence has no
//! published table; the bands follow plain-language style guides for Japanese, which advise
//! sentences of about 40–50 characters. Both are placed on the 0–100 ease scale by
//! piecewise-linear interpolation through their band edges, so texts scored by different
//! formulas can share one chart. That placement is a display convention, not a formula.

use super::formulas::Formula;

/// How hard a text reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Band {
    VeryEasy,
    Plain,
    FairlyDifficult,
    Difficult,
    VeryDifficult,
}

/// Lower ease edges of the first four bands (the last takes the rest).
const EASE_EDGES: [f64; 4] = [80.0, 60.0, 50.0, 30.0];
const EASE_MAX: f64 = 100.0;
/// `(LIX, ease)` through the band edges, from the easiest end.
const LIX_ANCHORS: [(f64, f64); 6] = [(20.0, 100.0), (30.0, 80.0), (40.0, 60.0), (50.0, 50.0), (60.0, 30.0), (75.0, 0.0)];
/// `(characters per sentence, ease)` through the band edges.
const CJK_ANCHORS: [(f64, f64); 6] = [(10.0, 100.0), (25.0, 80.0), (40.0, 60.0), (50.0, 50.0), (70.0, 30.0), (100.0, 0.0)];

impl Band {
    pub const ALL: [Self; 5] = [Self::VeryEasy, Self::Plain, Self::FairlyDifficult, Self::Difficult, Self::VeryDifficult];

    /// Stable key (locale keys `insight.band.<key>`).
    pub const fn key(self) -> &'static str {
        match self {
            Self::VeryEasy => "very_easy",
            Self::Plain => "plain",
            Self::FairlyDifficult => "fairly_difficult",
            Self::Difficult => "difficult",
            Self::VeryDifficult => "very_difficult",
        }
    }

    pub fn index(self) -> usize {
        Self::ALL.iter().position(|band| *band == self).unwrap_or_default()
    }

    /// The band of a score on the ease scale.
    pub fn of_ease(ease: f64) -> Self {
        EASE_EDGES.iter().position(|edge| ease >= *edge).map_or(Self::VeryDifficult, |at| Self::ALL[at])
    }

    /// Whether the band reads as hard (difficult or very difficult).
    pub fn is_hard(self) -> bool {
        matches!(self, Self::Difficult | Self::VeryDifficult)
    }
}

/// `value` of `formula` on the shared 0–100 ease scale.
pub fn ease(formula: Formula, value: f64) -> f64 {
    match formula {
        Formula::Lix => interpolate(&LIX_ANCHORS, value),
        Formula::CjkSentenceLength => interpolate(&CJK_ANCHORS, value),
        _ => value.clamp(0.0, EASE_MAX),
    }
}

/// The band of `value` of `formula`.
pub fn band(formula: Formula, value: f64) -> Band {
    Band::of_ease(ease(formula, value))
}

/// Piecewise-linear interpolation through `anchors` (ascending inputs), clamped at both ends.
pub fn interpolate(anchors: &[(f64, f64)], value: f64) -> f64 {
    let (Some(first), Some(last)) = (anchors.first(), anchors.last()) else { return 0.0 };
    if value <= first.0 {
        return first.1;
    }
    anchors
        .windows(2)
        .find(|pair| value <= pair[1].0)
        .map_or(last.1, |pair| {
            let ((x0, y0), (x1, y1)) = (pair[0], pair[1]);
            y0 + (y1 - y0) * (value - x0) / (x1 - x0)
        })
}
