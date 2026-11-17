//! The catalogue record of a document: Dublin Core-style descriptive metadata, subjects,
//! classification, collections, provenance and reading state.

mod classification;
mod creator;
mod identifier;
mod kind;
mod provenance;
mod reading;

pub use classification::{Classification, Discipline, DocumentForm, Period};
pub use creator::{format_creators, parse_creators, Creator, CreatorRole, PersonName};
pub use identifier::{is_valid_doi, is_valid_isbn, is_valid_url, Identifier, IdentifierProblem, IdentifierScheme};
pub use kind::DocumentKind;
pub use provenance::{fnv1a, text_checksum, OriginKind, Provenance};
pub use reading::{Reading, ReadingStatus, PROGRESS_COMPLETE};

use super::ids::{CollectionId, SubjectId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Physical or textual size of a work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Extent {
    /// Printed pages, when known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<u32>,
    /// Words of the stored text (derived from the content).
    pub words: usize,
}

/// Descriptive record of one document. The title and full text stay on the document itself.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CatalogueRecord {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub subtitle: String,
    pub kind: DocumentKind,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub creators: Vec<Creator>,
    /// Publication date as written: a year ("1677"), year-month or full ISO date.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub date: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub publisher: String,
    /// Language code ("en", "la", "de").
    #[serde(skip_serializing_if = "String::is_empty")]
    pub language: String,
    /// MIME type of the stored text; empty means the kind's default.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub format: String,
    #[serde(skip_serializing_if = "Identifier::is_empty")]
    pub identifier: Identifier,
    pub provenance: Provenance,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub rights: String,
    /// Abstract or summary.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
    pub extent: Extent,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<SubjectId>,
    pub classification: Classification,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub collections: Vec<CollectionId>,
    pub reading: Reading,
    /// Last edit of the record (none until it is edited).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

/// Marker of a negative (before the common era) year.
const NEGATIVE_YEAR: char = '-';

impl CatalogueRecord {
    /// The year of the publication date, when it starts with one ("1677", "-350", "1921-07").
    pub fn published_year(&self) -> Option<i32> {
        let date = self.date.trim();
        let (sign, digits) = match date.strip_prefix(NEGATIVE_YEAR) {
            Some(rest) => (-1, rest),
            None => (1, date),
        };
        let year: String = digits.chars().take_while(char::is_ascii_digit).collect();
        year.parse::<i32>().ok().map(|year| sign * year)
    }

    /// The effective MIME type: the recorded format or the kind's default.
    pub fn effective_format(&self) -> &str {
        if self.format.trim().is_empty() {
            self.kind.default_format()
        } else {
            &self.format
        }
    }

    /// Creators in the author role (every creator when no one is an author).
    pub fn authors(&self) -> Vec<&Creator> {
        let authors: Vec<&Creator> = self.creators.iter().filter(|c| c.role == CreatorRole::Author).collect();
        if authors.is_empty() {
            self.creators.iter().collect()
        } else {
            authors
        }
    }

    /// Whether the record carries `subject`.
    pub fn has_subject(&self, subject: &SubjectId) -> bool {
        self.subjects.contains(subject)
    }
}

/// Number of whitespace-separated words in `text`.
pub fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}
