//! Version chains of revisions (V1): numbering on save and the v6 backfill.

use crate::domain::data::history::{edited_post, Revision, RevisionId, RevisionSubject, SemVer};
use crate::domain::{Post, PostId};
use crate::storage::db::versions::LATEST_VERSION;
use crate::storage::db::Database;

/// `revisions` as v5 vaults stored it (no parent, no version).
const V5_REVISIONS: &str = r#"
    CREATE TABLE revisions (
        id TEXT PRIMARY KEY, subject_kind TEXT NOT NULL, subject_id TEXT NOT NULL,
        saved_at TEXT NOT NULL, title TEXT NOT NULL DEFAULT '', content TEXT NOT NULL,
        record_json TEXT NOT NULL DEFAULT ''
    );
    PRAGMA user_version = 5;
"#;
const INSERT_V5_REVISION: &str = "INSERT INTO revisions (id, subject_kind, subject_id, saved_at, content) VALUES (?1, ?2, ?3, ?4, ?5)";

fn at(hour: i8) -> jiff::Timestamp {
    crate::time::utc(2026, 3, 10, hour, 0, 0)
}

fn v5_vault() -> Database {
    let conn = rusqlite::Connection::open_in_memory().unwrap();
    conn.execute_batch(V5_REVISIONS).unwrap();
    // Inserted out of order: the chain follows `saved_at`, not insertion.
    for (id, kind, subject, hour) in [("r3", "post", "p1", 12), ("r1", "post", "p1", 9), ("r2", "post", "p1", 10), ("d1", "document", "doc", 8), ("q1", "post", "p2", 7)] {
        conn.execute(INSERT_V5_REVISION, rusqlite::params![id, kind, subject, crate::time::rfc3339(at(hour)), format!("text of {id}")]).unwrap();
    }
    Database { conn }
}

/// `(id, parent, version)` of every revision of `subject`, oldest first.
fn chain(db: &Database, subject: RevisionSubject) -> Vec<(String, Option<String>, String)> {
    let mut revisions = db.revisions_of(&subject).unwrap();
    revisions.reverse();
    revisions.into_iter().map(|r| (r.id.as_str().to_string(), r.parent_id.map(|p| p.as_str().to_string()), r.version.unwrap().to_string())).collect()
}

fn link(id: &str, parent: Option<&str>, version: &str) -> (String, Option<String>, String) {
    (id.to_string(), parent.map(str::to_string), version.to_string())
}

#[test]
fn semver_parses_prints_and_steps() {
    assert_eq!(SemVer::ORIGINAL.to_string(), "1.0.0");
    assert_eq!(SemVer::ORIGINAL.next_edit(), SemVer::parse("1.1.0").unwrap());
    assert_eq!(SemVer::after_edits(3).to_string(), "1.3.0");
    assert_eq!(SemVer::parse(" 2.10.4 "), Some(SemVer { major: 2, minor: 10, patch: 4 }));
    for bad in ["", "1.0", "1.0.0.0", "1.x.0", "-1.0.0"] {
        assert_eq!(SemVer::parse(bad), None, "{bad:?}");
    }
}

#[test]
fn v5_vault_backfills_linear_chains_per_subject() {
    let db = v5_vault();
    db.init_schema().expect("v5 -> latest");
    assert_eq!(db.schema_version().unwrap(), LATEST_VERSION);
    let post = RevisionSubject::Post(PostId::new("p1"));
    assert_eq!(chain(&db, post.clone()), vec![link("r1", None, "1.0.0"), link("r2", Some("r1"), "1.1.0"), link("r3", Some("r2"), "1.2.0")]);
    assert_eq!(chain(&db, RevisionSubject::Post(PostId::new("p2"))), vec![link("q1", None, "1.0.0")]);
    assert_eq!(chain(&db, RevisionSubject::Document("doc".into())), vec![link("d1", None, "1.0.0")]);

    db.init_schema().expect("a second run is a no-op");
    assert_eq!(chain(&db, post).len(), 3);
}

#[test]
fn new_revisions_continue_the_chain() {
    let db = v5_vault();
    db.init_schema().unwrap();
    let subject = RevisionSubject::Post(PostId::new("p1"));
    let next = Revision { id: RevisionId::new("r4"), ..Revision::of(subject.clone(), "fourth", at(13)) };
    db.save_revision(&next).unwrap();
    assert_eq!(chain(&db, subject).last().unwrap(), &link("r4", Some("r3"), "1.3.0"));
}

#[test]
fn editing_a_post_numbers_its_versions_from_the_original() {
    let db = Database::in_memory().unwrap();
    db.save_post(&Post::create("p1", "net", "u1", "first words").unwrap()).unwrap();
    let subject = RevisionSubject::Post(PostId::new("p1"));
    for (n, text) in ["second words", "third words"].into_iter().enumerate() {
        let current = db.load_posts().unwrap().remove(0);
        let revision = Revision { id: RevisionId::new(format!("e{n}")), ..Revision::of(subject.clone(), &current.content, at(9 + n as i8)) };
        db.edit_post_content(&edited_post(&current, text), &revision).unwrap();
    }
    assert_eq!(chain(&db, subject.clone()), vec![link("e0", None, "1.0.0"), link("e1", Some("e0"), "1.1.0")]);
    let original = db.revisions_of(&subject).unwrap().pop().unwrap();
    assert_eq!(original.content, "first words", "1.0.0 holds the original text; the current text is 1.2.0");
}

#[test]
fn a_given_version_is_stored_as_is() {
    let db = Database::in_memory().unwrap();
    let subject = RevisionSubject::Document("doc".into());
    let imported = Revision { version: Some(SemVer::after_edits(7)), parent_id: None, ..Revision::of(subject.clone(), "kept", at(9)) };
    db.save_revision(&imported).unwrap();
    assert_eq!(db.revisions_of(&subject).unwrap(), vec![imported]);
}
