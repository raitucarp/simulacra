//! Pure mapping from an imported file's metadata to a Knowledge Vault catalogue record.
//!
//! Imported metadata is whatever the file says, so fields the catalogue would refuse (an
//! unreadable date, a malformed language tag or identifier) are left out rather than
//! blocking the import; the title is cut to the catalogue's limit or named after the format.

use super::vault_adapter::VaultEntry;
use crate::domain::document::intake::{validate, FieldProblem, RecordField, MAX_TITLE_CHARS};
use crate::domain::document::record::{is_valid_doi, is_valid_isbn, is_valid_url, Creator, Identifier, IdentifierScheme};
use crate::domain::document::{CatalogueRecord, DocumentKind};

/// Prefixes that mark an ISBN or DOI inside a free-form identifier.
const ISBN_PREFIXES: [&str; 3] = ["urn:isbn:", "isbn:", "isbn "];
const DOI_PREFIXES: [&str; 3] = ["https://doi.org/", "doi:", "urn:doi:"];

/// The resource kind of an imported file, from its MIME type.
pub fn document_kind(mime: &str) -> DocumentKind {
    match mime {
        "application/epub+zip" | "application/x-mobipocket-ebook" | "application/vnd.amazon.ebook" => DocumentKind::Book,
        "application/pdf" | "application/vnd.openxmlformats-officedocument.wordprocessingml.document" | "application/vnd.oasis.opendocument.text" | "application/rtf" => DocumentKind::Article,
        "text/html" | "application/xhtml+xml" => DocumentKind::WebPage,
        "text/csv" | "text/tab-separated-values" | "application/json" => DocumentKind::Dataset,
        "text/plain" | "text/markdown" => DocumentKind::Note,
        other if ["image/", "audio/", "video/"].iter().any(|prefix| other.starts_with(prefix)) => DocumentKind::Media,
        _ => DocumentKind::Other,
    }
}

fn strip_any<'a>(value: &'a str, prefixes: &[&str]) -> Option<&'a str> {
    let lower = value.to_ascii_lowercase();
    prefixes.iter().find(|prefix| lower.starts_with(*prefix)).map(|prefix| value[prefix.len()..].trim())
}

/// One raw identifier as a catalogue identifier, when it is a valid ISBN, DOI or URL.
pub fn parse_identifier(raw: &str) -> Option<Identifier> {
    let raw = raw.trim();
    let isbn = strip_any(raw, &ISBN_PREFIXES).unwrap_or(raw);
    let doi = strip_any(raw, &DOI_PREFIXES).unwrap_or(raw);
    [
        is_valid_isbn(isbn).then(|| Identifier::new(IdentifierScheme::Isbn, isbn)),
        is_valid_doi(doi).then(|| Identifier::new(IdentifierScheme::Doi, doi)),
        is_valid_url(raw).then(|| Identifier::new(IdentifierScheme::Url, raw)),
    ]
    .into_iter()
    .flatten()
    .next()
}

/// The first identifier of `raws` the catalogue accepts, preferring ISBN, then DOI, then URL.
pub fn best_identifier(raws: &[String]) -> Identifier {
    let parsed: Vec<Identifier> = raws.iter().filter_map(|raw| parse_identifier(raw)).collect();
    [IdentifierScheme::Isbn, IdentifierScheme::Doi, IdentifierScheme::Url]
        .into_iter()
        .find_map(|scheme| parsed.iter().find(|id| id.scheme == Some(scheme)).cloned())
        .unwrap_or_default()
}

/// The record with every field that fails validation cleared.
fn without_refused_fields(mut record: CatalogueRecord, problems: &[FieldProblem]) -> CatalogueRecord {
    for problem in problems {
        match problem.field() {
            RecordField::Date => record.date.clear(),
            RecordField::Language => record.language.clear(),
            RecordField::Identifier => record.identifier = Identifier::default(),
            RecordField::Pages => record.extent.pages = None,
            RecordField::Title => {}
        }
    }
    record
}

/// The catalogue title: the entry's title cut to the limit, or the format label when empty.
pub fn catalogue_title(entry: &VaultEntry) -> String {
    let title = entry.title.trim();
    let title = if title.is_empty() { entry.format_label.trim() } else { title };
    title.chars().take(MAX_TITLE_CHARS).collect()
}

/// The catalogue record of `entry` (descriptive fields only; provenance is set by the caller).
pub fn catalogue_record(entry: &VaultEntry) -> CatalogueRecord {
    let meta = &entry.metadata;
    let text = |value: &Option<String>| value.as_deref().unwrap_or_default().trim().to_string();
    let record = CatalogueRecord {
        kind: document_kind(&entry.mime),
        creators: meta.creators.iter().map(|name| name.trim()).filter(|name| !name.is_empty()).map(Creator::author).collect(),
        date: text(&meta.date),
        publisher: text(&meta.publisher),
        language: text(&meta.language),
        identifier: best_identifier(&meta.identifiers),
        description: text(&meta.description),
        ..CatalogueRecord::default()
    };
    let problems = validate(&catalogue_title(entry), &record);
    without_refused_fields(record, &problems)
}

#[cfg(test)]
mod tests;
