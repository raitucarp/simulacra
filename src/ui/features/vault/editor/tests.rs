use super::draft::{split_list, EditorDraft};
use crate::domain::document::fixtures::{documents, library};
use crate::domain::document::intake::FieldProblem;
use crate::domain::document::record::{CatalogueRecord, CreatorRole, DocumentKind, IdentifierProblem, IdentifierScheme};

#[test]
fn a_document_round_trips_through_the_draft() {
    let doc = &documents()[0];
    let draft = EditorDraft::of_document(doc, &library());
    assert_eq!(draft.subjects, "Metaphysics, Ethics");
    assert_eq!(draft.record(&doc.record), doc.record);
    assert!(draft.problems(&doc.record).is_empty());
}

#[test]
fn draft_fields_are_trimmed_and_parsed() {
    let draft = EditorDraft {
        title: "Ethics".into(),
        kind: DocumentKind::Book,
        creators: "Baruch Spinoza; R. H. M. Elwes (translator)".into(),
        date: " 1677 ".into(),
        pages: "260".into(),
        scheme: Some(IdentifierScheme::Url),
        identifier: " https://example.org/ethics ".into(),
        ..EditorDraft::default()
    };
    let record = draft.record(&CatalogueRecord::default());
    assert_eq!(record.creators[1].role, CreatorRole::Translator);
    assert_eq!((record.date.as_str(), record.extent.pages), ("1677", Some(260)));
    assert_eq!(record.identifier.value, "https://example.org/ethics");
}

#[test]
fn bad_values_are_reported_and_a_blank_identifier_is_dropped() {
    let draft = EditorDraft {
        pages: "many".into(),
        scheme: Some(IdentifierScheme::Doi),
        identifier: "not-a-doi".into(),
        ..EditorDraft::default()
    };
    let problems = draft.problems(&CatalogueRecord::default());
    assert_eq!(problems, [FieldProblem::TitleMissing, FieldProblem::Identifier(IdentifierProblem::BadDoi), FieldProblem::PagesZero]);
    let blank = EditorDraft { title: "T".into(), scheme: Some(IdentifierScheme::Isbn), ..EditorDraft::default() };
    assert!(blank.record(&CatalogueRecord::default()).identifier.is_empty());
}

#[test]
fn lists_split_on_commas_and_semicolons() {
    assert_eq!(split_list(" Metaphysics, Ethics;; Logic ,"), ["Metaphysics", "Ethics", "Logic"]);
    assert!(split_list(" , ").is_empty());
}
