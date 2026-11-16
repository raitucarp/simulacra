use crate::storage::db::Database;

#[test]
fn test_db_schema_migration_from_legacy_vault() {
    // Create an in-memory database with the old schema (pre-M2)
    let conn = rusqlite::Connection::open_in_memory().unwrap();
    conn.execute_batch(
        r#"
        CREATE TABLE posts (
            id TEXT PRIMARY KEY,
            network_id TEXT NOT NULL,
            author_id TEXT NOT NULL,
            author_json TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at TEXT NOT NULL,
            likes INTEGER NOT NULL DEFAULT 0,
            is_liked INTEGER NOT NULL DEFAULT 0,
            replies_count INTEGER NOT NULL DEFAULT 0,
            mentions_json TEXT NOT NULL DEFAULT '[]',
            linked_posts_json TEXT NOT NULL DEFAULT '[]'
        );
        "#,
    ).unwrap();

    // Insert legacy post
    conn.execute(
        "INSERT INTO posts (id, network_id, author_id, author_json, content, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params!["old_1", "net1", "u1", "{}", "Legacy vault thought", "2026-01-01T00:00:00Z"]
    ).unwrap();

    // Wrap into Database and run init_schema
    let db = Database { conn };
    db.init_schema().expect("Migration must succeed without error");

    // Running init_schema a second time must also succeed idempotently
    db.init_schema().expect("Idempotent migration must succeed");

    let loaded = db.load_posts().expect("Load migrated posts");
    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[0].id.as_str(), "old_1");
    assert_eq!(loaded[0].reposts_count, 0);
    assert_eq!(loaded[0].reply_to_id, None);
    assert_eq!(loaded[0].link_url, None);
}

#[test]
fn test_db_high_performance_pragmas() {
    let temp_dir = std::env::temp_dir().join(format!("simulacra_pragma_test_{}.db", uuid::Uuid::new_v4()));
    let db = Database::open(&temp_dir).expect("Database::open");

    let journal_mode: String = db
        .conn
        .query_row("PRAGMA journal_mode;", [], |row| row.get(0))
        .expect("query journal_mode");
    assert_eq!(journal_mode.to_lowercase(), "wal");

    let synchronous: i64 = db
        .conn
        .query_row("PRAGMA synchronous;", [], |row| row.get(0))
        .expect("query synchronous");
    assert_eq!(synchronous, 1); // 1 = NORMAL

    let temp_store: i64 = db
        .conn
        .query_row("PRAGMA temp_store;", [], |row| row.get(0))
        .expect("query temp_store");
    assert_eq!(temp_store, 2); // 2 = MEMORY

    let cache_size: i64 = db
        .conn
        .query_row("PRAGMA cache_size;", [], |row| row.get(0))
        .expect("query cache_size");
    assert_eq!(cache_size, -64000);

    let mmap_size: i64 = db
        .conn
        .query_row("PRAGMA mmap_size;", [], |row| row.get(0))
        .expect("query mmap_size");
    assert!(mmap_size >= 268435456);

    drop(db);
    let _ = std::fs::remove_file(&temp_dir);
    let _ = std::fs::remove_file(format!("{}-wal", temp_dir.display()));
    let _ = std::fs::remove_file(format!("{}-shm", temp_dir.display()));
}
