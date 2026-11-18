use crate::domain::post::SourceLocator;
use crate::domain::review::card::{expand_draft, CardDraft, NewCardContext};
use crate::domain::review::schedule::Policy;
use crate::domain::review::{apply_answer, Answer, CardId, CardSource, DraftKind, Grade, ReviewMode};
use crate::domain::NetworkId;
use crate::storage::db::Database;
use chrono::{NaiveDate, Utc};

fn cards() -> Vec<crate::domain::review::Card> {
    let locator = SourceLocator { chapter: 2, paragraph: Some(4), sentence: Some(1), ..SourceLocator::default() };
    let draft = CardDraft {
        kind: DraftKind::Cloze { text: "{{c1::Spinoza}} wrote the {{c2::Ethics}}.".into() },
        source: CardSource::document("doc_1", Some(locator)),
        tags: vec!["philosophy".into()],
    };
    let context = NewCardContext { network_id: NetworkId::new("net"), created_at: Utc::now(), today: NaiveDate::from_ymd_opt(2026, 3, 1).unwrap() };
    let mut n = 0;
    expand_draft(&draft, &context, || {
        n += 1;
        CardId::new(format!("card_{n}"))
    })
}

#[test]
fn cards_and_answers_round_trip() {
    let db = Database::in_memory().unwrap();
    let cards = cards();
    db.save_review_cards(&cards).unwrap();
    let answer = Answer { card_id: cards[0].id.clone(), grade: Grade::Good, mode: ReviewMode::Recall, confidence: None, correct: Some(true) };
    let today = cards[0].schedule.due;
    let (reviewed, entry) = apply_answer(&cards[0], &answer, Utc::now(), today, &Policy::with_retention_percent(90));
    db.save_review_cards(std::slice::from_ref(&reviewed)).unwrap();
    db.append_review_entry(&entry).unwrap();
    let deck = db.load_review_deck().unwrap();
    assert_eq!(deck.cards.len(), 2, "an update replaces the row");
    let loaded = deck.card(&reviewed.id).unwrap();
    assert_eq!((&loaded.kind, &loaded.source, &loaded.tags, &loaded.schedule), (&reviewed.kind, &reviewed.source, &reviewed.tags, &reviewed.schedule));
    assert_eq!(deck.log.len(), 1);
    assert_eq!(deck.log[0].grade, Grade::Good);
    assert_eq!(deck.log[0].correct, Some(true));
}

#[test]
fn deleting_a_card_keeps_its_log_and_broken_rows_are_skipped() {
    let db = Database::in_memory().unwrap();
    let cards = cards();
    db.save_review_cards(&cards).unwrap();
    db.delete_review_card(&cards[1].id).unwrap();
    db.conn.execute("INSERT INTO review_cards (id, network_id, kind_json, source_json, created_at, schedule_json, due) VALUES ('x', 'net', 'not json', '{}', '', '{}', '')", []).unwrap();
    db.conn.execute("INSERT INTO review_log (card_id, network_id, reviewed_at, review_day, grade, mode, entry_json) VALUES ('x', 'net', '', '', 'good', 'flashcard', 'broken')", []).unwrap();
    let deck = db.load_review_deck().unwrap();
    assert_eq!((deck.cards.len(), deck.log.len()), (1, 0));
}

#[test]
fn the_tables_are_idempotent() {
    let db = Database::in_memory().unwrap();
    db.save_review_cards(&cards()).unwrap();
    db.init_schema().expect("re-running the schema keeps the rows");
    assert_eq!(db.load_review_deck().unwrap().cards.len(), 2);
}
