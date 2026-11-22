//! Version heads (V3): the v9 backfill of edit times, bumps and heads, and the one write that
//! shows a new version of a post.

use crate::domain::data::history::{Bump, Revision, RevisionId, RevisionSubject, SemVer, VersionHead};
use crate::domain::{Post, PostId};
use crate::storage::db::versions::LATEST_VERSION;
use crate::storage::db::Database;

const INSERT_V8_REVISION: &str = "INSERT INTO revisions (id, subject_kind, subject_id, saved_at, content, parent_id, version) VALUES (?1, 'post', 'p1', ?2, ?3, ?4, ?5)";

fn at(hour: i8) -> jiff::Timestamp {
    crate::time::utc(2026, 4, 2, hour, 0, 0)
}

/// A vault at v8 with post `p1` (written at 08:00) edited three times before heads.
fn v8_vault() -> Database {
    let db = Database::in_memory().unwrap();
    db.save_post(&Post { created_at: at(8), ..Post::create("p1", "net", "u1", "fourth").unwrap() }).unwrap();
    for (id, hour, parent, version) in [("r1", 9, None, "1.0.0"), ("r2", 10, Some("r1"), "1.1.0"), ("r3", 11, Some("r2"), "1.1.1")] {
        db.conn.execute(INSERT_V8_REVISION, rusqlite::params![id, crate::time::rfc3339(at(hour)), format!("text of {id}"), parent, version]).unwrap();
    }
    db.conn.execute_batch("PRAGMA user_version = 8;").unwrap();
    db
}

#[test]
fn v8_vault_gets_edit_times_bumps_and_a_head_for_each_edited_post() {
    let db = v8_vault();
    db.init_schema().expect("v8 -> latest");
    assert_eq!(db.schema_version().unwrap(), LATEST_VERSION);
    let mut revisions = db.revisions_of(&RevisionSubject::Post(PostId::new("p1"))).unwrap();
    revisions.reverse();
    let times: Vec<_> = revisions.iter().map(|r| r.edited_at).collect();
    assert_eq!(times, vec![Some(at(8)), Some(at(9)), Some(at(10))], "each version was written when the one before was replaced");
    assert_eq!(revisions.iter().skip(1).map(|r| r.bump).collect::<Vec<_>>(), vec![Bump::Minor, Bump::Patch]);
    let post = db.load_posts().unwrap().remove(0);
    let head = post.extras.head.expect("a head");
    assert_eq!((head.version, head.parent, head.edited_at), (SemVer::new(1, 2, 0), Some(RevisionId::new("r3")), Some(at(11))));
    db.init_schema().expect("a second run is a no-op");
    assert_eq!(db.load_posts().unwrap().remove(0).extras.head.unwrap().id, head.id);
}

#[test]
fn a_new_version_and_the_one_it_replaced_are_stored_together() {
    let db = Database::in_memory().unwrap();
    let post = Post { created_at: at(8), ..Post::create("p1", "net", "u1", "first").unwrap() };
    db.save_post(&post).unwrap();
    let head = VersionHead::original(post.created_at);
    let kept = Revision { note: String::new(), ..head.clone().into_revision(RevisionSubject::Post(post.id.clone()), "first", at(12)) };
    let next = VersionHead { id: RevisionId::new("h2"), version: SemVer::new(1, 1, 0), parent: Some(head.id.clone()), bump: Some(Bump::Minor), note: "added a line".into(), edited_at: Some(at(12)) };
    let mut shown = crate::domain::data::history::edited_post(&post, "first, then more");
    shown.extras.head = Some(next.clone());
    db.commit_post_version(&shown, &kept, None).unwrap();
    let stored = db.revisions_of(&RevisionSubject::Post(post.id.clone())).unwrap();
    assert_eq!(stored, vec![kept.clone()], "the original keeps its id, number and time");
    let loaded = db.load_posts().unwrap().remove(0);
    assert_eq!((loaded.content.as_str(), loaded.extras.head), ("first, then more", Some(next)));
    db.commit_post_version(&shown, &Revision { id: RevisionId::new("other"), ..kept.clone() }, Some(&kept.id)).unwrap();
    let ids: Vec<String> = db.revisions_of(&RevisionSubject::Post(post.id)).unwrap().into_iter().map(|r| r.id.as_str().to_string()).collect();
    assert_eq!(ids, vec!["other".to_string()], "the revision made current again is dropped");
}
