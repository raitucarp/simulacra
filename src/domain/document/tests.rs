use super::intake::{first_sentence, legacy_record, validate_in_year, FieldProblem};
use super::record::*;
use super::{chunk_count, GroundingDocument, NewDocument};

#[test]
fn legacy_documents_become_notes_with_a_derived_record() {
    let content = "The world is everything that is the case. What can be said at all can be said clearly.";
    let doc = GroundingDocument::new("doc_old", "net", "Tractatus", content);
    assert_eq!(doc.record.kind, DocumentKind::Note);
    assert_eq!(doc.record.provenance.origin, OriginKind::Legacy);
    assert_eq!(doc.record.description, "The world is everything that is the case.");
    assert_eq!(doc.record.extent.words, word_count(content));
    assert_eq!(doc.record.provenance.checksum, text_checksum(content));
    assert_eq!(doc.record, legacy_record(content), "the migration and the constructor agree");
}

#[test]
fn a_legacy_record_serialized_before_catalogues_existed_still_loads() {
    let json = r#"{"id":"d","network_id":"n","title":"T","content":"c","chunk_count":1,"created_at":"2026-01-01T00:00:00Z"}"#;
    let doc: GroundingDocument = serde_json::from_str(json).expect("old JSON parses");
    assert_eq!(doc.record, CatalogueRecord::default());
}

#[test]
fn new_documents_are_trimmed_and_get_their_derived_fields() {
    let record = CatalogueRecord { kind: DocumentKind::WebPage, ..Default::default() };
    let added = crate::time::utc(2026, 3, 1, 9, 0, 0);
    let doc = NewDocument::new("net", "  Notes  ", " one two three ")
        .with_record(record)
        .from_origin(OriginKind::Url, "https://example.org/a")
        .added_by("Arya")
        .into_document("doc_x", added);
    assert_eq!((doc.title.as_str(), doc.content.as_str()), ("Notes", "one two three"));
    assert_eq!(doc.record.extent.words, 3);
    assert_eq!(doc.record.format, "text/html");
    assert_eq!(doc.record.provenance.source, "https://example.org/a");
    assert_eq!(doc.record.provenance.added_by, "Arya");
    assert_eq!((doc.created_at, doc.chunk_count), (added, chunk_count(3)));
}

#[test]
fn validation_reports_each_bad_field() {
    let record = CatalogueRecord {
        date: "someday".into(),
        language: "english!".into(),
        identifier: Identifier::new(IdentifierScheme::Isbn, "978-0-00-000000-1"),
        extent: Extent { pages: Some(0), words: 0 },
        ..Default::default()
    };
    let problems = validate_in_year(" ", &record, 2026);
    assert_eq!(
        problems,
        vec![
            FieldProblem::TitleMissing,
            FieldProblem::DateUnreadable,
            FieldProblem::LanguageUnreadable,
            FieldProblem::Identifier(IdentifierProblem::BadIsbn),
            FieldProblem::PagesZero,
        ]
    );
    let fine = CatalogueRecord { date: "-350".into(), language: "pt-BR".into(), ..Default::default() };
    assert!(validate_in_year("Politics", &fine, 2026).is_empty());
    let future = CatalogueRecord { date: "2100".into(), ..Default::default() };
    assert_eq!(validate_in_year("Later", &future, 2026), vec![FieldProblem::DateUnreadable]);
}

#[test]
fn identifiers_are_checked_by_scheme() {
    assert!(is_valid_isbn("978-0-306-40615-7") && is_valid_isbn("0-306-40615-2") && is_valid_isbn("0-8044-2957-X"));
    assert!(!is_valid_isbn("978-0-306-40615-8") && !is_valid_isbn("12345"));
    assert!(is_valid_doi("10.1000/xyz123") && is_valid_doi("https://doi.org/10.1093/mind/LIX.236.433"));
    assert!(!is_valid_doi("11.1000/x") && !is_valid_doi("10./x"));
    assert!(is_valid_url("https://standardebooks.org/ebooks") && !is_valid_url("ftp://x.org") && !is_valid_url("https://nohost"));
    assert_eq!(Identifier::default().problem(), None);
}

#[test]
fn names_split_for_citation() {
    let natural = PersonName::parse("Ludwig Josef Wittgenstein");
    assert_eq!((natural.family.as_str(), natural.initials().as_str()), ("Wittgenstein", "L. J."));
    let inverted = PersonName::parse("Spinoza, Baruch");
    assert_eq!((inverted.natural().as_str(), inverted.inverted().as_str()), ("Baruch Spinoza", "Spinoza, Baruch"));
    assert_eq!(PersonName::parse("Plato").initials(), "");
}

#[test]
fn creator_lines_round_trip_with_roles() {
    let creators = parse_creators("Baruch Spinoza; Elwes, R. H. M. (translator)\n  ");
    assert_eq!(creators, vec![Creator::author("Baruch Spinoza"), Creator::new("Elwes, R. H. M.", CreatorRole::Translator)]);
    assert_eq!(parse_creators(&format_creators(&creators)), creators);
    assert_eq!(parse_creators("Someone (dancer)"), vec![Creator::author("Someone (dancer)")]);
}

#[test]
fn reading_progress_drives_the_status_and_never_goes_back() {
    assert_eq!(Reading::at(0).status, ReadingStatus::Unread);
    assert_eq!(Reading::at(250), Reading { status: ReadingStatus::Read, progress: PROGRESS_COMPLETE });
    let reading = Reading::at(40);
    assert_eq!(reading.status, ReadingStatus::Reading);
    assert_eq!(reading.advanced_to(20), reading);
    assert_eq!(reading.advanced_to(60).progress, 60);
    assert_eq!(reading.with_status(ReadingStatus::Unread).progress, 0);
}

#[test]
fn years_and_periods_are_read_from_the_date() {
    let at = |date: &str| CatalogueRecord { date: date.into(), ..Default::default() }.published_year();
    assert_eq!((at("1677"), at("1921-07-01"), at("-350"), at("c. 1500")), (Some(1677), Some(1921), Some(-350), None));
    assert_eq!(Period::of_year(1677), Period::EarlyModern);
    assert_eq!(Period::of_year(-350), Period::Antiquity);
    assert_eq!(first_sentence("## Heading\nNo stop here"), "Heading\nNo stop here");
}
