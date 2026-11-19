use crate::domain::post::SourceLocator;
use crate::domain::review::connections::{ConnectionMap, MapEdge, MapPoint};
use crate::domain::review::item::{new_item, NewItemContext};
use crate::domain::review::schedule::Policy;
use crate::domain::review::{apply_answer, Anchor, AnchorSide, Answer, Grade, ItemDraft, ItemId, ReviewItem, ReviewMode, Subject};
use crate::domain::{NetworkId, PostId};
use crate::storage::db::Database;
use chrono::{NaiveDate, Utc};

fn context() -> NewItemContext {
    NewItemContext { network_id: NetworkId::new("net"), created_at: Utc::now(), today: NaiveDate::from_ymd_opt(2026, 3, 1).unwrap() }
}

fn items() -> Vec<ReviewItem> {
    let locator = SourceLocator { chapter: 2, paragraph: Some(4), sentence: Some(1), ..SourceLocator::default() };
    let passage = ItemDraft { anchor: Anchor::passage("doc_1", Some(locator), "Spinoza wrote the Ethics."), side: AnchorSide::Back, written: "Who wrote the Ethics?".into(), tags: vec!["philosophy".into()] };
    let post = ItemDraft { anchor: Anchor::post(&PostId::new("p1")), side: AnchorSide::Front, written: "Desire".into(), tags: vec![] };
    let map = ConnectionMap { title: "t".into(), points: vec![MapPoint::Post { id: "p1".into() }, MapPoint::Concept { label: "c".into() }], edges: vec![MapEdge::new(0, 1)] };
    let mut map_item = new_item(&post, &context(), ItemId::new("map")).unwrap();
    map_item.subject = Subject::Connections { map };
    vec![new_item(&passage, &context(), ItemId::new("a")).unwrap(), new_item(&post, &context(), ItemId::new("b")).unwrap(), map_item]
}

#[test]
fn items_maps_and_answers_round_trip() {
    let db = Database::in_memory().unwrap();
    let items = items();
    db.save_review_items(&items).unwrap();
    let answer = Answer { item_id: items[0].id.clone(), grade: Grade::Good, mode: ReviewMode::Recall, confidence: None, correct: Some(true) };
    let (reviewed, entry) = apply_answer(&items[0], &answer, Utc::now(), items[0].schedule.due, &Policy::with_retention_percent(90));
    db.save_review_items(std::slice::from_ref(&reviewed)).unwrap();
    db.append_review_entry(&entry).unwrap();
    let deck = db.load_review_deck().unwrap();
    assert_eq!(deck.items.len(), 3, "an update replaces the row");
    assert_eq!(deck.item(&reviewed.id), Some(&reviewed));
    assert_eq!(deck.item(&ItemId::new("map")), Some(&items[2]));
    assert_eq!((deck.log.len(), deck.log[0].grade, deck.log[0].correct), (1, Grade::Good, Some(true)));
    let anchored: i64 = db.conn.query_row("SELECT COUNT(*) FROM review_items WHERE anchor_key = 'post:p1'", [], |r| r.get(0)).unwrap();
    assert_eq!(anchored, 1, "the anchor key column finds the item on a post");
}

#[test]
fn deleting_an_item_keeps_its_log_and_broken_rows_are_skipped() {
    let db = Database::in_memory().unwrap();
    db.save_review_items(&items()).unwrap();
    db.delete_review_item(&ItemId::new("b")).unwrap();
    db.conn.execute("INSERT INTO review_items (id, network_id, kind, subject_json, created_at, schedule_json, due) VALUES ('x', 'net', 'question', 'not json', '', '{}', '')", []).unwrap();
    db.conn.execute("INSERT INTO review_log (card_id, network_id, reviewed_at, review_day, grade, mode, entry_json) VALUES ('x', 'net', '', '', 'good', 'flashcard', 'broken')", []).unwrap();
    let deck = db.load_review_deck().unwrap();
    assert_eq!((deck.items.len(), deck.log.len()), (2, 0));
}

#[test]
fn the_tables_are_idempotent() {
    let db = Database::in_memory().unwrap();
    db.save_review_items(&items()).unwrap();
    db.init_schema().expect("re-running the schema keeps the rows");
    assert_eq!(db.load_review_deck().unwrap().items.len(), 3);
}

/// The C1 table and two of its cards: one on a post, one typed by hand, plus an answer.
const LEGACY: &str = r#"
    CREATE TABLE review_cards (id TEXT PRIMARY KEY, network_id TEXT NOT NULL, kind_json TEXT NOT NULL, source_json TEXT NOT NULL,
        tags_json TEXT NOT NULL DEFAULT '[]', created_at TEXT NOT NULL, schedule_json TEXT NOT NULL, due TEXT NOT NULL, suspended INTEGER NOT NULL DEFAULT 0);
    INSERT INTO review_cards VALUES ('card_post', 'net', '{"type":"basic","front":"What is desire?","back":"x"}', '{"kind":"post","id":"p9"}',
        '["ethics"]', '2026-03-01T09:00:00+00:00', '{"phase":"review","memory":null,"due":"2026-03-05","last_review":"2026-03-01","interval":4,"reps":2,"lapses":0}', '2026-03-05', 1);
    INSERT INTO review_cards VALUES ('card_free', 'net', '{"type":"basic","front":"q","back":"a"}', '{"kind":"sentence","text":"q"}',
        '[]', '2026-03-01T09:00:00+00:00', '{"phase":"new","memory":null,"due":"2026-03-01","last_review":null,"interval":0,"reps":0,"lapses":0}', '2026-03-01', 0);
    INSERT INTO review_log (card_id, network_id, reviewed_at, review_day, grade, mode, entry_json) VALUES ('card_post', 'net', '', '2026-03-01', 'good', 'flashcard',
        '{"card_id":"card_post","network_id":"net","reviewed_at":"2026-03-01T09:00:00Z","day":"2026-03-01","grade":"good","mode":"flashcard","phase_before":"new","interval":4,"stability":3.1,"difficulty":5.0}');
    PRAGMA user_version = 4;
"#;

#[test]
fn v5_carries_cards_on_posts_over_drops_free_ones_and_keeps_the_log() {
    let db = Database::in_memory().unwrap();
    db.conn.execute_batch(LEGACY).unwrap();
    db.init_schema().unwrap();
    assert!(db.schema_version().unwrap() >= crate::storage::db::review_migration::REVIEW_ITEMS_VERSION);
    let deck = db.load_review_deck().unwrap();
    assert_eq!(deck.items.len(), 1);
    let item = &deck.items[0];
    assert_eq!((item.id.as_str(), item.schedule.reps, item.suspended), ("card_post", 2, true));
    assert_eq!(item.subject, Subject::Anchored { anchor: Anchor::Post { id: "p9".into() }, side: AnchorSide::Back, written: "What is desire?".into() });
    assert_eq!(deck.log[0].item_id.as_str(), "card_post", "old log entries read as items");
    let left: i64 = db.conn.query_row("SELECT COUNT(*) FROM sqlite_master WHERE name = 'review_cards'", [], |r| r.get(0)).unwrap();
    assert_eq!(left, 0, "the old table is dropped");
    db.init_schema().unwrap();
    assert_eq!(db.load_review_deck().unwrap().items.len(), 1, "running again changes nothing");
}
