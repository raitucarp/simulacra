//! Readability scores (sb F3): pure, per-language formulas over counted text.
//!
//! - [`counts`]: sentences (the import sentence splitter), words, syllables, letters, long
//!   words and CJK characters of a text.
//! - [`syllables`]: syllable estimates per language (vowel groups with each language's rules).
//! - [`formulas`]: the published formulas, each documented with its source.
//! - [`scale`]: the bands of each formula and a shared 0–100 "ease" scale for comparing
//!   texts scored by different formulas.
//! - [`script`]: which formula a text gets (its dominant script, then the reading language).

pub mod counts;
pub mod formulas;
pub mod scale;
pub mod script;
pub mod syllables;
#[cfg(test)]
mod tests;

pub use counts::TextCounts;
pub use formulas::Formula;
pub use scale::Band;
pub use script::{formula_for, ReadingLanguage};

/// Fewest words (or CJK characters) worth a score: shorter texts swing wildly.
pub const MIN_WORDS: usize = 3;
pub const MIN_CJK_CHARS: usize = 10;

/// A text's readability.
#[derive(Debug, Clone, PartialEq)]
pub struct Readability {
    pub formula: Formula,
    /// The formula's own value (an ease score, an index or characters per sentence).
    pub value: f64,
    /// The value on the shared 0–100 scale (higher reads more easily).
    pub ease: f64,
    pub band: Band,
    /// Flesch–Kincaid grade level (English only).
    pub grade: Option<f64>,
    pub counts: TextCounts,
}

/// The readability of `text` for a reader of `language`, or `None` when it is too short.
pub fn readability(text: &str, language: ReadingLanguage) -> Option<Readability> {
    let formula = formula_for(text, language);
    let counts = TextCounts::of(text, formula.syllable_rules());
    let enough = if formula == Formula::CjkSentenceLength { counts.cjk_chars >= MIN_CJK_CHARS } else { counts.words >= MIN_WORDS };
    if !enough {
        return None;
    }
    let value = formula.value(&counts);
    let grade = (formula == Formula::FleschEnglish).then(|| formulas::flesch_kincaid_grade(&counts));
    Some(Readability { formula, value, ease: scale::ease(formula, value), band: scale::band(formula, value), grade, counts })
}
