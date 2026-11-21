//! Emoji reactions and claps (R1): the v8 table, upserts, zero counts and deletion with posts.

use crate::domain::reaction::{Reaction, CLAP};
use crate::domain::{Post, PostId, ProfileId};
use crate::storage::db::versions::LATEST_VERSION;
use crate::storage::db::Database;

fn reaction(post: &str, profile: &str, emoji: &str, count: u32, hour: i8) -> Reaction {
    Reaction {
        post_id: PostId::new(post),
        profile_id: ProfileId::new(profile),
        emoji: emoji.to_string(),
        count,
        updated_at: crate::time::utc(2026, 11, 20, hour, 0, 0),
    }
}

fn table_exists(db: &Database) -> bool {
    let sql = "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'post_reactions'";
    db.conn.query_row(sql, [], |row| row.get::<_, i64>(0)).unwrap() == 1
}

#[test]
fn a_v7_vault_gains_the_reactions_table_and_keeps_its_likes() {
    let db = Database::in_memory().unwrap();
    let mut post = Post::create("p1", "net1", "u1", "liked before reactions").unwrap();
    post.toggle_like();
    db.save_post(&post).unwrap();
    db.conn.execute_batch("DROP TABLE post_reactions; PRAGMA user_version = 7;").unwrap();
    assert!(!table_exists(&db));

    db.init_schema().expect("v7 -> v8");
    assert_eq!(db.schema_version().unwrap(), LATEST_VERSION);
    assert!(table_exists(&db));
    let loaded = db.load_posts().unwrap();
    assert_eq!((loaded[0].likes, loaded[0].is_liked), (1, true), "likes stay beside reactions (decision #20)");
    assert!(db.load_reactions().unwrap().is_empty(), "no like becomes a reaction");
}

#[test]
fn reactions_upsert_by_post_profile_and_emoji() {
    let db = Database::in_memory().unwrap();
    db.save_reaction(&reaction("p1", "u1", "\u{1F525}", 1, 9)).unwrap();
    db.save_reaction(&reaction("p1", "u1", CLAP, 3, 10)).unwrap();
    db.save_reaction(&reaction("p1", "agent", "\u{1F525}", 1, 11)).unwrap();
    db.save_reaction(&reaction("p1", "u1", CLAP, 7, 12)).unwrap();
    let loaded = db.load_reactions().unwrap();
    assert_eq!(loaded.len(), 3, "the second clap row replaced the first");
    let claps = loaded.iter().find(|r| r.is_clap()).unwrap();
    assert_eq!(claps, &reaction("p1", "u1", CLAP, 7, 12));
    assert_eq!(loaded[0], reaction("p1", "u1", "\u{1F525}", 1, 9), "oldest first");
}

#[test]
fn a_zero_count_or_a_delete_removes_the_row() {
    let db = Database::in_memory().unwrap();
    db.save_reaction(&reaction("p1", "u1", CLAP, 2, 9)).unwrap();
    db.save_reaction(&reaction("p1", "u1", "\u{2764}\u{FE0F}", 1, 9)).unwrap();
    db.save_reaction(&reaction("p1", "u1", CLAP, 0, 10)).unwrap();
    db.delete_reaction(&PostId::new("p1"), &ProfileId::new("u1"), "\u{2764}\u{FE0F}").unwrap();
    assert!(db.load_reactions().unwrap().is_empty());
}

#[test]
fn deleting_a_post_deletes_its_reactions() {
    let db = Database::in_memory().unwrap();
    for id in ["gone", "purged", "kept"] {
        db.save_post(&Post::create(id, "net1", "u1", id).unwrap()).unwrap();
        db.save_reaction(&reaction(id, "u1", CLAP, 1, 9)).unwrap();
    }
    db.delete_post_with_reposts(&PostId::new("gone")).unwrap();
    db.purge_post_traces(&PostId::new("purged")).unwrap();
    let left: Vec<String> = db.load_reactions().unwrap().into_iter().map(|r| r.post_id.as_str().to_string()).collect();
    assert_eq!(left, vec!["kept"]);
}
