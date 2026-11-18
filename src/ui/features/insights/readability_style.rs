//! sb F3: how readability is named and coloured, shared by the Readability view and the
//! composer's meter: band colours (theme accents from easy to hard), band and formula names,
//! and each formula's value as text.

use crate::domain::readability::{Band, Formula, Readability};
use crate::theme::ThemeTokens;
use crate::ui::features::insights::chart::ticks::compact;
use crate::ui::features::insights::chart::palette::mix;
use gpui_kit::Rgba;

/// Difficult sits halfway between the gold of fairly difficult and the red of very difficult.
const HALFWAY: f32 = 0.5;
use rust_i18n::t;

/// Colour of `band`: green for very easy through blue, gold and orange to red for very hard.
pub fn band_colour(band: Band, theme: &ThemeTokens) -> Rgba {
    match band {
        Band::VeryEasy => theme.accent_success,
        Band::Plain => theme.info,
        Band::FairlyDifficult => theme.accent_dialectic,
        Band::Difficult => mix(theme.accent_dialectic, theme.accent_danger, HALFWAY),
        Band::VeryDifficult => theme.accent_danger,
    }
}

pub fn band_label(band: Band) -> String {
    let key = format!("insight.band.{}", band.key());
    t!(key.as_str()).to_string()
}

pub fn formula_label(formula: Formula) -> String {
    let key = format!("insight.formula.{}", formula.key());
    t!(key.as_str()).to_string()
}

/// Where `formula` was published.
pub fn formula_source(formula: Formula) -> String {
    let key = format!("insight.formula.{}.source", formula.key());
    t!(key.as_str()).to_string()
}

/// "Flesch reading ease · grade 8", "LIX readability index 42", "Characters per sentence 31":
/// the formula, its own value when it differs from the shared ease, and the grade level.
pub fn score_detail(score: &Readability) -> String {
    let (value, ease) = (score.value.round(), score.ease.round());
    let base = if value == ease { formula_label(score.formula) } else { t!("insight.meter.detail", formula = formula_label(score.formula), value = compact(value)).to_string() };
    match score.grade {
        Some(grade) => format!("{base} · {}", t!("insight.meter.grade", grade = compact(grade.max(0.0).round()))),
        None => base,
    }
}

/// The ease on the shared scale, rounded.
pub fn ease_text(ease: f64) -> String {
    compact(ease.round())
}
