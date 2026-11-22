//! The index of earlier post versions (Q6): kept in sync, backfilled, and never holding a
//! sealed version.

use crate::domain::data::history::{Revision, RevisionSubject};
use crate::domain::post::lock::LOCKED_PLACEHOLDER;
use crate::domain::{Post, PostId};
use crate::storage::db::{query_match_expression, Database};

const INSERT_V9_REVISION: &str = "INSERT INTO revisions (id, subject_kind, subject_id, saved_at, content, record_json) VALUES (?1, ?2, 'p1', '2026-01-01T00:00:00Z', ?3, ?4)";

fn version(id: &str, content: &str, record_json: &str) -> Revision {
    let base = Revision::of(RevisionSubject::Post(PostId::new("p1")), content, jiff::Timestamp::now());
    Revision { id: crate::domain::data::history::RevisionId::new(id), record_json: record_json.to_string(), ..base }
}

fn found(db: &Database, word: &str) -> Vec<(String, String)> {
    let expression = query_match_expression(&[word.to_string()], &[]).expect("terms");
    db.score_revisions(&expression, 10).unwrap().into_iter().map(|(id, text, _)| (id.as_str().to_string(), text)).collect()
}

fn rows(db: &Database) -> i64 {
    db.conn.query_row("SELECT COUNT(*) FROM revisions_fts", [], |row| row.get(0)).unwrap()
}

#[test]
fn the_placeholder_the_index_skips_is_the_lock_placeholder() {
    assert_eq!(LOCKED_PLACEHOLDER, "\u{1F512}");
}

#[test]
fn kept_versions_are_found_and_sealed_ones_never_indexed() {
    let db = Database::in_memory().unwrap();
    db.save_post(&Post::create("p1", "net", "u1", "today's words").unwrap()).unwrap();
    db.save_revision(&version("r1", "an older lighthouse thought", "")).unwrap();
    db.save_revision(&version("r1", "an older lighthouse thought", "")).unwrap();
    assert_eq!(rows(&db), 1, "re-saving a version keeps one row");
    db.save_revision(&version("sealed", LOCKED_PLACEHOLDER, r#"{"lock":{"format":"x"}}"#)).unwrap();
    db.save_revision(&version("leak", "lighthouse secret", r#"{"lock":{"format":"x"}}"#)).unwrap();
    db.save_revision(&Revision::of(RevisionSubject::Document("d1".into()), "lighthouse in a document", jiff::Timestamp::now())).unwrap();
    assert_eq!(found(&db, "lighth"), vec![("p1".to_string(), "an older lighthouse thought".to_string())]);
    assert!(found(&db, "secret").is_empty(), "a sealed record is never indexed");
    db.conn.execute("DELETE FROM revisions WHERE id = 'r1'", []).unwrap();
    assert!(found(&db, "lighthouse").is_empty());
    assert_eq!(rows(&db), 0);
}

#[test]
fn versions_kept_before_the_index_are_backfilled() {
    let db = Database::in_memory().unwrap();
    db.conn
        .execute_batch("DROP TRIGGER revisions_fts_after_insert; DROP TRIGGER revisions_fts_after_update; DROP TRIGGER revisions_fts_after_delete; DROP TABLE revisions_fts;")
        .unwrap();
    for (id, kind, content, record) in [("r1", "post", "an archived reflection", ""), ("r2", "post", LOCKED_PLACEHOLDER, r#"{"lock":{}}"#), ("r3", "document", "reflection", "")] {
        db.conn.execute(INSERT_V9_REVISION, rusqlite::params![id, kind, content, record]).unwrap();
    }
    db.conn.execute_batch("PRAGMA user_version = 9;").unwrap();
    db.init_schema().expect("v9 -> latest");
    assert_eq!(found(&db, "reflection"), vec![("p1".to_string(), "an archived reflection".to_string())]);
    assert_eq!(rows(&db), 1);
    db.init_schema().expect("a second run is a no-op");
    assert_eq!(rows(&db), 1);
}
