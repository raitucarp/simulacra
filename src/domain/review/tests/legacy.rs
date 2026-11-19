use super::day;
use crate::domain::review::legacy::{legacy_subject, migrated_items, LegacyRow};
use crate::domain::review::{Anchor, AnchorSide, ItemSchedule, Subject};

const POST: &str = r#"{"kind":"post","id":"p1"}"#;
const DOCUMENT: &str = r#"{"kind":"document","id":"doc","locator":{"chapter":2,"paragraph":4}}"#;
const SENTENCE: &str = r#"{"kind":"sentence","text":"typed by hand"}"#;
const BASIC: &str = r#"{"type":"basic","front":"What is conatus?","back":"The striving to persist."}"#;
const CLOZE: &str = r#"{"type":"cloze","text":"{{c1::Spinoza}} wrote the {{c2::Ethics::a book}}.","ordinal":2}"#;
const CONCEPT_BACK: &str = r#"{"type":"concept","term":"Akrasia","definition":"Weakness of will","direction":"definition_to_term"}"#;
const CONCEPT: &str = r#"{"type":"concept","term":"Akrasia","definition":"Weakness of will","direction":"term_to_definition"}"#;

fn row(id: &str, minute: i8, kind: &str, source: &str) -> LegacyRow {
    LegacyRow {
        id: id.into(),
        network_id: "net".into(),
        kind_json: kind.into(),
        source_json: source.into(),
        tags_json: r#"["ethics"]"#.into(),
        created_at: crate::time::utc(2026, 3, 1, 9, minute, 0),
        schedule_json: serde_json::to_string(&ItemSchedule { reps: 3, ..ItemSchedule::new_item(day(0)) }).unwrap(),
        suspended: false,
    }
}

#[test]
fn a_card_on_a_post_keeps_its_question_with_the_post_as_answer() {
    let subject = legacy_subject(BASIC, POST).unwrap();
    assert_eq!(subject, Subject::Anchored { anchor: Anchor::Post { id: "p1".into() }, side: AnchorSide::Back, written: "What is conatus?".into() });
}

#[test]
fn a_card_on_a_document_becomes_a_passage_with_its_place() {
    let Some(Subject::Anchored { anchor: Anchor::Passage { document_id, locator, text }, written, .. }) = legacy_subject(CLOZE, DOCUMENT) else { panic!("kept") };
    assert_eq!((document_id.as_str(), text.as_str()), ("doc", "Spinoza wrote the Ethics."));
    assert_eq!(written, "Spinoza wrote the … (a book).");
    assert_eq!(locator.and_then(|l| l.paragraph), Some(4));
    assert!(matches!(legacy_subject(CONCEPT, DOCUMENT), Some(Subject::Anchored { written, .. }) if written == "Akrasia"));
}

#[test]
fn free_form_cards_reverse_concepts_and_broken_rows_are_dropped() {
    assert!(legacy_subject(BASIC, SENTENCE).is_none());
    assert!(legacy_subject(CONCEPT_BACK, POST).is_none());
    assert!(legacy_subject("not json", POST).is_none());
}

#[test]
fn migration_keeps_ids_schedules_and_one_item_per_post() {
    let rows = vec![row("late", 9, CONCEPT, POST), row("first", 1, BASIC, POST), row("doc", 2, BASIC, DOCUMENT), row("free", 3, BASIC, SENTENCE)];
    let items = migrated_items(&rows);
    let ids: Vec<&str> = items.iter().map(|i| i.id.as_str()).collect();
    assert_eq!(ids, vec!["first", "doc"]);
    assert!(items.iter().all(|i| i.schedule.reps == 3 && i.tags == ["ethics"] && !i.retired));
}
