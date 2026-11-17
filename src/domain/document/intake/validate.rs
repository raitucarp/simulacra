//! Validation of a document's title and catalogue record before it is stored.

use crate::domain::document::record::{CatalogueRecord, IdentifierProblem};

/// A record field the editor can point a problem at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecordField {
    Title,
    Date,
    Language,
    Identifier,
    Pages,
}

/// What is wrong with a field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldProblem {
    /// The title is empty.
    TitleMissing,
    /// The title is longer than [`MAX_TITLE_CHARS`].
    TitleTooLong,
    /// The date does not start with a plausible year.
    DateUnreadable,
    /// The language is not a short code such as `en` or `pt-BR`.
    LanguageUnreadable,
    /// The identifier is malformed for its scheme.
    Identifier(IdentifierProblem),
    /// A page count of zero.
    PagesZero,
}

impl FieldProblem {
    /// The field the problem belongs to.
    pub const fn field(self) -> RecordField {
        match self {
            Self::TitleMissing | Self::TitleTooLong => RecordField::Title,
            Self::DateUnreadable => RecordField::Date,
            Self::LanguageUnreadable => RecordField::Language,
            Self::Identifier(_) => RecordField::Identifier,
            Self::PagesZero => RecordField::Pages,
        }
    }
}

/// Longest accepted title, in characters.
pub const MAX_TITLE_CHARS: usize = 300;
/// Accepted publication years (inclusive): early writing to a few years ahead (forthcoming).
const EARLIEST_YEAR: i32 = -3500;
const YEARS_AHEAD: i32 = 5;
/// A language code: 2–3 letter primary tag, optional subtags, at most this long.
const MAX_LANGUAGE_CHARS: usize = 12;
const MIN_PRIMARY_TAG: usize = 2;
const MAX_PRIMARY_TAG: usize = 3;
const LANGUAGE_SUBTAG_SEPARATOR: char = '-';

fn is_language_code(code: &str) -> bool {
    let mut tags = code.split(LANGUAGE_SUBTAG_SEPARATOR);
    let primary = tags.next().unwrap_or_default();
    code.len() <= MAX_LANGUAGE_CHARS
        && (MIN_PRIMARY_TAG..=MAX_PRIMARY_TAG).contains(&primary.len())
        && primary.chars().all(|c| c.is_ascii_alphabetic())
        && tags.all(|tag| !tag.is_empty() && tag.chars().all(|c| c.is_ascii_alphanumeric()))
}

fn date_problem(record: &CatalogueRecord, current_year: i32) -> Option<FieldProblem> {
    if record.date.trim().is_empty() {
        return None;
    }
    let plausible = record.published_year().is_some_and(|year| (EARLIEST_YEAR..=current_year + YEARS_AHEAD).contains(&year));
    (!plausible).then_some(FieldProblem::DateUnreadable)
}

/// Every problem of `title` and `record`, in field order (empty when valid).
pub fn validate(title: &str, record: &CatalogueRecord) -> Vec<FieldProblem> {
    validate_in_year(title, record, chrono::Datelike::year(&chrono::Utc::now()))
}

/// [`validate`] as of `current_year` (pure, for tests).
pub fn validate_in_year(title: &str, record: &CatalogueRecord, current_year: i32) -> Vec<FieldProblem> {
    let title = title.trim();
    let language = record.language.trim();
    [
        title.is_empty().then_some(FieldProblem::TitleMissing),
        (title.chars().count() > MAX_TITLE_CHARS).then_some(FieldProblem::TitleTooLong),
        date_problem(record, current_year),
        (!language.is_empty() && !is_language_code(language)).then_some(FieldProblem::LanguageUnreadable),
        record.identifier.problem().map(FieldProblem::Identifier),
        (record.extent.pages == Some(0)).then_some(FieldProblem::PagesZero),
    ]
    .into_iter()
    .flatten()
    .collect()
}
