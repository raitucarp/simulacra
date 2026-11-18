//! The atomic-note check (sb E1), UI side: a quiet hint row under a composer draft (never
//! blocking, dismissible, off in Settings › Reading & Writing) and the "Check atomicity"
//! sheet with the score, the findings that explain it and a proposal to split the post into
//! one post per claim.

mod hint;
mod sheet;

pub use hint::render_atomic_hint;
pub use sheet::{render_atomic_sheet, AtomicSheet, AtomicSheetEvent};

use crate::domain::thinking::atomic::ATOMIC_SCORE;
use crate::theme::ThemeTokens;
use gpui_kit::Rgba;

/// A score at or above this is "mixed" rather than "not atomic".
const MIXED_SCORE: u8 = 50;

/// The colour of an atomicity score.
pub fn score_colour(score: u8, theme: &ThemeTokens) -> Rgba {
    match score {
        s if s >= ATOMIC_SCORE => theme.accent_success,
        s if s >= MIXED_SCORE => theme.warning,
        _ => theme.accent_danger,
    }
}
