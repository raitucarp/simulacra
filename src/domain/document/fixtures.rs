//! Shared test fixtures: a small vocabulary and three catalogued documents.

use super::record::{CatalogueRecord, Classification, Creator, CreatorRole, Discipline, DocumentKind, Period, Reading};
use super::{CollectionId, GroundingDocument, Library, Subject, SubjectId};
use crate::domain::NetworkId;
use chrono::{TimeZone, Utc};

pub const NET: &str = "net_test";
pub const PHILOSOPHY: &str = "subj_philosophy";
pub const METAPHYSICS: &str = "subj_metaphysics";
pub const LANGUAGE: &str = "subj_language";
pub const ETHICS_TERM: &str = "subj_ethics";
pub const SHELF: &str = "coll_rationalists";

/// Philosophy › {Metaphysics, Philosophy of language, Ethics}.
pub fn library() -> Library {
    Library {
        subjects: vec![
            Subject::new(PHILOSOPHY, "Philosophy"),
            Subject::new(METAPHYSICS, "Metaphysics").under(PHILOSOPHY),
            Subject::new(LANGUAGE, "Philosophy of language").under(PHILOSOPHY),
            Subject::new(ETHICS_TERM, "Ethics").under(PHILOSOPHY),
        ],
        collections: Vec::new(),
    }
}

fn record(kind: DocumentKind, author: &str, date: &str, subjects: &[&str]) -> CatalogueRecord {
    CatalogueRecord {
        kind,
        creators: vec![Creator::new(author, CreatorRole::Author)],
        date: date.to_string(),
        language: "en".into(),
        subjects: subjects.iter().map(|s| SubjectId::new(*s)).collect(),
        classification: Classification { discipline: Some(Discipline::Philosophy), period: Some(Period::EarlyModern), ..Default::default() },
        ..Default::default()
    }
}

fn doc(id: &str, title: &str, day: u32, record: CatalogueRecord) -> GroundingDocument {
    let mut doc = GroundingDocument::new(id, NetworkId::new(NET), title, format!("{title} body text about substance and words."));
    doc.created_at = Utc.with_ymd_and_hms(2026, 1, day, 12, 0, 0).unwrap();
    doc.with_record(record)
}

/// Ethics (book, 1677, metaphysics + ethics, shelved), Tractatus (book, 1921, language),
/// "A Note on Modes" (note, no date, metaphysics, being read).
pub fn documents() -> Vec<GroundingDocument> {
    let mut ethics = record(DocumentKind::Book, "Baruch Spinoza", "1677", &[METAPHYSICS, ETHICS_TERM]);
    ethics.collections = vec![CollectionId::new(SHELF)];
    let mut tractatus = record(DocumentKind::Book, "Ludwig Wittgenstein", "1921", &[LANGUAGE]);
    tractatus.classification.period = Some(Period::Modern);
    let mut note = record(DocumentKind::Note, "Arya Wiratama", "", &[METAPHYSICS]);
    note.reading = Reading::at(40);
    vec![
        doc("doc_ethics", "Ethics", 3, ethics),
        doc("doc_tractatus", "Tractatus Logico-Philosophicus", 1, tractatus),
        doc("doc_note", "A Note on Modes", 2, note),
    ]
}
