//! Post kinds (F1) and lifecycle columns (F2): round trips and the upgrade of old vaults.

use crate::domain::post::{PostKind, PostLifecycle, PostStatus};
use crate::domain::Post;
use crate::storage::db::Database;

/// `posts` as releases before kinds and lifecycles stored it.
const OLD_POSTS: &str = r#"
    CREATE TABLE posts (
        id TEXT PRIMARY KEY, network_id TEXT NOT NULL, author_id TEXT NOT NULL,
        author_json TEXT NOT NULL, content TEXT NOT NULL, created_at TEXT NOT NULL,
        likes INTEGER NOT NULL DEFAULT 0, is_liked INTEGER NOT NULL DEFAULT 0,
        replies_count INTEGER NOT NULL DEFAULT 0,
        mentions_json TEXT NOT NULL DEFAULT '[]', linked_posts_json TEXT NOT NULL DEFAULT '[]',
        extras_json TEXT NOT NULL DEFAULT '{}'
    );
    INSERT INTO posts (id, network_id, author_id, author_json, content, created_at)
        VALUES ('old_1', 'net1', 'u1', '{}', 'An old thought', '2026-01-01T00:00:00Z');
"#;

fn at(raw: &str) -> jiff::Timestamp {
    raw.parse().unwrap()
}

#[test]
fn every_kind_round_trips() {
    let db = Database::in_memory().unwrap();
    for (n, kind) in PostKind::ALL.iter().enumerate() {
        let post = Post { kind: *kind, ..Post::create(format!("p{n}"), "net1", "u1", "a thought").unwrap() };
        db.save_post(&post).unwrap();
    }
    let loaded = db.load_posts().unwrap();
    for (n, kind) in PostKind::ALL.iter().enumerate() {
        assert_eq!(loaded.iter().find(|p| p.id.as_str() == format!("p{n}")).unwrap().kind, *kind);
    }
}

#[test]
fn lifecycle_round_trips_and_updates_on_upsert() {
    let db = Database::in_memory().unwrap();
    let scheduled = PostLifecycle {
        status: PostStatus::Scheduled,
        publish_at: Some(at("2026-12-01T09:00:00Z")),
        archive_at: Some(at("2027-01-01T00:00:00Z")),
        delete_at: Some(at("2027-06-01T00:00:00Z")),
    };
    let post = Post { lifecycle: scheduled, ..Post::create("p1", "net1", "u1", "later").unwrap() };
    db.save_post(&post).unwrap();
    assert_eq!(db.load_posts().unwrap()[0].lifecycle, scheduled);

    let archived = PostLifecycle { status: PostStatus::Archived, ..PostLifecycle::default() };
    db.save_post(&Post { lifecycle: archived, ..post }).unwrap();
    assert_eq!(db.load_posts().unwrap()[0].lifecycle, archived);
}

#[test]
fn old_vault_rows_upgrade_to_standard_published_posts() {
    let conn = rusqlite::Connection::open_in_memory().unwrap();
    conn.execute_batch(OLD_POSTS).unwrap();
    let db = Database { conn };
    db.init_schema().expect("upgrade");
    db.init_schema().expect("a second run is a no-op");

    let old = &db.load_posts().unwrap()[0];
    assert_eq!(old.kind, PostKind::Standard);
    assert_eq!(old.lifecycle, PostLifecycle::default());
    assert_eq!(old.lifecycle.status, PostStatus::Published);
    let (kind, status): (String, String) = db.conn.query_row("SELECT kind, status FROM posts", [], |r| Ok((r.get(0)?, r.get(1)?))).unwrap();
    assert_eq!((kind.as_str(), status.as_str()), ("standard", "published"), "backfilled in the table itself");
}

#[test]
fn unknown_keys_and_malformed_moments_read_as_defaults() {
    let db = Database::in_memory().unwrap();
    db.save_post(&Post::create("p1", "net1", "u1", "a thought").unwrap()).unwrap();
    db.conn.execute("UPDATE posts SET kind = 'poll', status = 'limbo', publish_at = 'soon'", []).unwrap();
    let post = &db.load_posts().unwrap()[0];
    assert_eq!(post.kind, PostKind::Standard);
    assert_eq!(post.lifecycle, PostLifecycle::default());
}

#[test]
fn json_without_the_new_fields_still_loads_and_defaults_are_not_written() {
    let post = Post::create("p1", "net1", "u1", "a thought").unwrap();
    let json = serde_json::to_value(&post).unwrap();
    assert!(json.get("kind").is_none() && json.get("lifecycle").is_none());
    let back: Post = serde_json::from_value(json).unwrap();
    assert_eq!(back, post);

    let question = Post { kind: PostKind::Question, ..post };
    let json = serde_json::to_value(&question).unwrap();
    assert_eq!(json["kind"], "question");
    assert_eq!(serde_json::from_value::<Post>(json).unwrap(), question);
}
