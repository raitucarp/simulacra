//! The thinking tools' rows (sb E1): maps, credences and tension pairs round-trip and upsert.

use crate::domain::thinking::argument::{ArgNode, ArgumentMap, NodeRole};
use crate::domain::thinking::credence::{Credence, Outcome};
use crate::domain::thinking::tensions::{Classification, TensionRecord, TensionStatus, Verdict};
use crate::domain::PostId;
use crate::storage::db::Database;
use chrono::{NaiveDate, TimeZone, Utc};

const NOW: i64 = 1_800_000_000;

fn map(root: &str, text: &str) -> ArgumentMap {
    let root = PostId::from(root);
    let node = ArgNode { id: "n1".into(), role: NodeRole::Claim, text: text.into(), source: root.clone() };
    ArgumentMap { root, nodes: vec![node], edges: Vec::new() }
}

#[test]
fn the_schema_is_idempotent_and_maps_upsert_by_root() {
    let db = Database::in_memory().unwrap();
    db.init_schema().unwrap();
    db.save_argument_map(&map("r1", "first"), NOW).unwrap();
    db.save_argument_map(&map("r1", "second"), NOW + 1).unwrap();
    db.save_argument_map(&map("r2", "other"), NOW).unwrap();
    let mut maps = db.load_argument_maps().unwrap();
    maps.sort_by(|a, b| a.root.as_str().cmp(b.root.as_str()));
    assert_eq!(maps, vec![map("r1", "second"), map("r2", "other")]);
    db.save_argument_map(&ArgumentMap::empty(PostId::from("r2")), NOW).unwrap();
    assert_eq!(db.load_argument_maps().unwrap().len(), 1, "an empty map forgets its row");
}

#[test]
fn credences_round_trip_with_dates_and_outcomes() {
    let db = Database::in_memory().unwrap();
    let created = Utc.with_ymd_and_hms(2026, 3, 1, 9, 30, 0).unwrap();
    let open = Credence::new(PostId::from("p1"), 70, created).with_due(NaiveDate::from_ymd_opt(2026, 6, 1)).with_sentence(Some("Rates fall".into()));
    let resolved = Credence::new(PostId::from("p2"), 20, created).resolved(Outcome::False, "It did not happen", created);
    db.save_credence(&open).unwrap();
    db.save_credence(&resolved).unwrap();
    let mut loaded = db.load_credences().unwrap();
    loaded.sort_by(|a, b| a.post_id.as_str().cmp(b.post_id.as_str()));
    assert_eq!(loaded, vec![open.clone(), resolved]);
    db.save_credence(&open.clone().with_percent(85)).unwrap();
    assert_eq!(db.load_credences().unwrap().iter().find(|c| c.post_id == open.post_id).map(|c| c.percent), Some(85));
    db.delete_credence(&open.post_id).unwrap();
    assert_eq!(db.load_credences().unwrap().len(), 1);
}

#[test]
fn tension_records_keep_verdicts_and_statuses() {
    let db = Database::in_memory().unwrap();
    let fresh = TensionRecord::new("hash1", "post:a", "post:b");
    let classified = TensionRecord::new("hash2", "post:a", "passage:d#3")
        .classified(&Classification { pair: "hash2".into(), verdict: Verdict::Contradicts, reason: "One says all, one says none.".into() });
    db.save_tension_record(&fresh, NOW).unwrap();
    db.save_tension_record(&classified, NOW).unwrap();
    db.save_tension_record(&fresh.clone().with_status(TensionStatus::Dismissed), NOW + 1).unwrap();
    let mut loaded = db.load_tension_records().unwrap();
    loaded.sort_by(|a, b| a.pair.cmp(&b.pair));
    assert_eq!(loaded, vec![fresh.with_status(TensionStatus::Dismissed), classified]);
}
