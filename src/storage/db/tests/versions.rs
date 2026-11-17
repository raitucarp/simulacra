use crate::domain::{Profile, ProfileId};
use crate::storage::db::versions::AUTHOR_ID_VERSION;
use crate::storage::db::Database;

/// Schema of an unversioned (v0) vault: posts embed the full author profile as JSON.
const V0_SCHEMA: &str = r#"
    CREATE TABLE profiles (
        id TEXT PRIMARY KEY, name TEXT NOT NULL, handle TEXT NOT NULL,
        avatar_initial TEXT NOT NULL, role_json TEXT NOT NULL
    );
    CREATE TABLE posts (
        id TEXT PRIMARY KEY, network_id TEXT NOT NULL, author_id TEXT NOT NULL,
        author_json TEXT NOT NULL, content TEXT NOT NULL, created_at TEXT NOT NULL,
        likes INTEGER NOT NULL DEFAULT 0, is_liked INTEGER NOT NULL DEFAULT 0,
        replies_count INTEGER NOT NULL DEFAULT 0,
        mentions_json TEXT NOT NULL DEFAULT '[]', linked_posts_json TEXT NOT NULL DEFAULT '[]'
    );
"#;
const INSERT_V0_POST: &str = "INSERT INTO posts (id, network_id, author_id, author_json, content, created_at) \
                              VALUES (?1, 'net1', ?2, ?3, ?4, '2026-01-01T00:00:00Z')";
const INSERT_V0_PROFILE: &str = "INSERT INTO profiles VALUES ('u1', 'Live', '@live', 'L', '\"UserPrimary\"')";

fn v0_vault() -> Database {
    let conn = rusqlite::Connection::open_in_memory().unwrap();
    conn.execute_batch(V0_SCHEMA).unwrap();
    conn.execute(INSERT_V0_PROFILE, []).unwrap();
    let live = serde_json::to_string(&Profile::new_primary("u1", "Live", "@live")).unwrap();
    let ghost = serde_json::to_string(&Profile::new_primary("ghost", "Ghost", "@ghost")).unwrap();
    // A stale author_id column must be corrected from the embedded snapshot.
    conn.execute(INSERT_V0_POST, rusqlite::params!["p1", "stale", live, "live author"]).unwrap();
    conn.execute(INSERT_V0_POST, rusqlite::params!["p2", "ghost", ghost, "deleted author"]).unwrap();
    conn.execute(INSERT_V0_POST, rusqlite::params!["p3", "u1", "not json", "broken snapshot"]).unwrap();
    Database { conn }
}

fn author_json(db: &Database, post_id: &str) -> String {
    db.conn
        .query_row("SELECT author_json FROM posts WHERE id = ?1", [post_id], |row| row.get(0))
        .unwrap()
}

#[test]
fn v0_vault_migrates_to_author_ids_and_archives_orphans() {
    let db = v0_vault();
    assert_eq!(db.schema_version().unwrap(), 0);
    db.init_schema().expect("v0 -> v1 migration");
    assert!(db.schema_version().unwrap() >= AUTHOR_ID_VERSION, "later versions (sb B3: 4) follow v1");

    let posts = db.load_posts().unwrap();
    let author = |id: &str| posts.iter().find(|p| p.id.as_str() == id).unwrap().author_id.clone();
    assert_eq!(author("p1"), ProfileId::new("u1"));
    assert_eq!(author("p2"), ProfileId::new("ghost"));
    assert_eq!(author("p3"), ProfileId::new("u1"), "undecodable snapshots keep the stored id");

    assert_eq!(author_json(&db, "p1"), "{}", "live authors drop their snapshot");
    let archived = db.load_archived_authors().unwrap();
    assert_eq!(archived.len(), 1);
    assert_eq!(archived[&ProfileId::new("ghost")].name, "Ghost");
}

#[test]
fn versioned_migration_is_idempotent() {
    let db = v0_vault();
    db.init_schema().unwrap();
    let ghost_snapshot = author_json(&db, "p2");
    db.init_schema().expect("second run is a no-op");
    assert!(db.schema_version().unwrap() >= AUTHOR_ID_VERSION, "later versions (sb B3: 4) follow v1");
    assert_eq!(author_json(&db, "p2"), ghost_snapshot);
    assert_eq!(db.load_posts().unwrap().len(), 3);
}

#[test]
fn fresh_vault_starts_at_current_version_and_archives_deleted_authors() {
    let db = Database::in_memory().unwrap();
    assert!(db.schema_version().unwrap() >= AUTHOR_ID_VERSION, "later versions (sb B3: 4) follow v1");
    let gone = Profile::new_primary("gone", "Gone", "@gone");
    db.save_profile(&gone).unwrap();
    let post = crate::domain::Post::create("p1", "net1", &gone, "hello").unwrap();
    db.save_post(&post).unwrap();
    assert!(db.load_archived_authors().unwrap().is_empty());

    db.archive_post_author(&gone).unwrap();
    db.delete_profile(&gone.id).unwrap();
    assert_eq!(db.load_archived_authors().unwrap()[&gone.id], gone);
}
