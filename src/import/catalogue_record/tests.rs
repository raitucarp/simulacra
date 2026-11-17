use super::*;
use crate::import::document::DocumentMetadata;

fn entry(metadata: DocumentMetadata, mime: &str) -> VaultEntry {
    VaultEntry {
        title: "Moby Dick".into(),
        metadata,
        format_label: "EPUB".into(),
        mime: mime.into(),
        size_bytes: 1,
        sha256: "00".into(),
        content: "Call me Ishmael.".into(),
        original_file: None,
        source_name: "moby-dick.epub".into(),
    }
}

fn book_metadata() -> DocumentMetadata {
    DocumentMetadata {
        title: Some("Moby Dick".into()),
        creators: vec!["Herman Melville".into(), "  ".into()],
        language: Some("en-US".into()),
        date: Some("1851".into()),
        identifiers: vec!["https://standardebooks.org/ebooks/herman-melville/moby-dick".into(), "urn:isbn:9780306406157".into()],
        publisher: Some("Standard Ebooks".into()),
        description: Some("A whaling voyage.".into()),
    }
}

#[test]
fn book_metadata_becomes_a_full_catalogue_record() {
    let record = catalogue_record(&entry(book_metadata(), "application/epub+zip"));
    assert_eq!(record.kind, DocumentKind::Book);
    assert_eq!(record.creators, vec![Creator::author("Herman Melville")]);
    assert_eq!((record.date.as_str(), record.language.as_str()), ("1851", "en-US"));
    assert_eq!(record.publisher, "Standard Ebooks");
    assert_eq!(record.description, "A whaling voyage.");
    assert_eq!(record.identifier, Identifier::new(IdentifierScheme::Isbn, "9780306406157"));
}

#[test]
fn fields_the_catalogue_refuses_are_dropped_not_fatal() {
    let metadata = DocumentMetadata { date: Some("sometime".into()), language: Some("English (US)".into()), identifiers: vec!["uuid:1234".into()], ..book_metadata() };
    let record = catalogue_record(&entry(metadata, "application/pdf"));
    assert_eq!(record.kind, DocumentKind::Article);
    assert!(record.date.is_empty() && record.language.is_empty() && record.identifier.is_empty());
    assert!(validate(&catalogue_title(&entry(book_metadata(), "")), &record).is_empty());
}

#[test]
fn identifiers_prefer_isbn_then_doi_then_url() {
    let raws = |values: &[&str]| values.iter().map(|value| value.to_string()).collect::<Vec<_>>();
    assert_eq!(best_identifier(&raws(&["https://example.org/a", "doi:10.1000/xyz123"])).scheme, Some(IdentifierScheme::Doi));
    assert_eq!(best_identifier(&raws(&["https://example.org/a"])).scheme, Some(IdentifierScheme::Url));
    assert!(best_identifier(&raws(&["urn:uuid:abc"])).is_empty());
}

#[test]
fn kinds_follow_the_mime_type() {
    assert_eq!(document_kind("text/html"), DocumentKind::WebPage);
    assert_eq!(document_kind("text/csv"), DocumentKind::Dataset);
    assert_eq!(document_kind("image/png"), DocumentKind::Media);
    assert_eq!(document_kind("text/markdown"), DocumentKind::Note);
    assert_eq!(document_kind("application/octet-stream"), DocumentKind::Other);
}

#[test]
fn an_empty_title_falls_back_to_the_format_and_long_titles_are_cut() {
    let untitled = VaultEntry { title: "  ".into(), ..entry(DocumentMetadata::default(), "") };
    assert_eq!(catalogue_title(&untitled), "EPUB");
    let long = VaultEntry { title: "x".repeat(MAX_TITLE_CHARS + 9), ..entry(DocumentMetadata::default(), "") };
    assert_eq!(catalogue_title(&long).chars().count(), MAX_TITLE_CHARS);
}

#[test]
fn the_new_document_records_its_import_origin() {
    let new = crate::import::vault_adapter::new_document(&entry(book_metadata(), "application/epub+zip"), "net".into(), "Ada");
    assert_eq!(new.record.provenance.origin, crate::domain::document::record::OriginKind::Import);
    assert_eq!((new.record.provenance.source.as_str(), new.record.provenance.added_by.as_str()), ("moby-dick.epub", "Ada"));
    assert!(new.problems().is_empty());
}
