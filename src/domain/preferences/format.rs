//! Number formatting by the user's [`NumberFormat`] (pure).

use super::NumberFormat;
use crate::i18n::Language;

/// Digits per thousands group.
const GROUP_SIZE: usize = 3;
const COMMA: char = ',';
const PERIOD: char = '.';
/// A narrow no-break space, the typographic thousands separator.
const THIN_SPACE: char = '\u{202f}';
const MINUS: char = '-';

impl NumberFormat {
    /// The thousands separator, resolving [`NumberFormat::Language`] for `language`.
    pub fn thousands_separator(self, language: Language) -> char {
        match self {
            Self::CommaPeriod => COMMA,
            Self::PeriodComma => PERIOD,
            Self::SpaceComma => THIN_SPACE,
            Self::Language => match language {
                Language::German | Language::Indonesian | Language::Spanish | Language::Portuguese => PERIOD,
                Language::French | Language::Russian => THIN_SPACE,
                _ => COMMA,
            },
        }
    }
}

/// `value` with its digits grouped in thousands (`1,234,567`).
pub fn group_thousands(value: i64, separator: char) -> String {
    let digits = value.unsigned_abs().to_string();
    let lead = digits.len() % GROUP_SIZE;
    let grouped = digits.chars().enumerate().fold(String::new(), |mut out, (index, digit)| {
        if index > 0 && (index + GROUP_SIZE - lead).is_multiple_of(GROUP_SIZE) {
            out.push(separator);
        }
        out.push(digit);
        out
    });
    if value < 0 { format!("{MINUS}{grouped}") } else { grouped }
}
