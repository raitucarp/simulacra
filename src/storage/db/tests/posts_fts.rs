//! The post full-text index (Q1): kept in sync on save, edit and delete; backfilled.

use crate::domain::data::history::{edited_post, Revision, RevisionSubject};
use crate::domain::{Post, PostId};
use crate::storage::db::{post_match_expression, Database};

fn post(id: &str, text: &str) -> Post {
    Post::create(id, "net1", "u1", text).unwrap()
}

fn ids(found: Vec<PostId>) -> Vec<String> {
    found.into_iter().map(|id| id.as_str().to_string()).collect()
}

fn search(db: &Database, text: &str) -> Vec<String> {
    ids(db.search_posts(text, 50).unwrap())
}

fn index_rows(db: &Database) -> i64 {
    db.conn.query_row("SELECT COUNT(*) FROM posts_fts", [], |row| row.get(0)).unwrap()
}

#[test]
fn match_expression_quotes_every_term_as_a_prefix() {
    assert_eq!(post_match_expression("Spinoza, ethics!").as_deref(), Some("\"spinoza\"* \"ethics\"*"));
    assert_eq!(post_match_expression("\"x\" OR NEAR(a b)* -").as_deref(), Some("\"or\"* \"near\"*"));
    assert_eq!(post_match_expression("a - ? !"), None);
}

#[test]
fn saved_posts_are_found_by_prefix_without_accents_and_best_first() {
    let db = Database::in_memory().unwrap();
    db.save_post(&post("long", "A long wandering note that mentions spinoza once among many other words here")).unwrap();
    db.save_post(&post("short", "Spinoza and spinoza again")).unwrap();
    db.save_post(&post("other", "Nothing to see")).unwrap();
    db.save_post(&post("cafe", "Thinking at the café")).unwrap();
    assert_eq!(search(&db, "spino"), vec!["short", "long"], "bm25 ranks the denser match first");
    assert_eq!(search(&db, "cafe"), vec!["cafe"]);
    assert_eq!(search(&db, "spinoza nothing"), Vec::<String>::new(), "every term is required");
    assert_eq!(ids(db.search_posts("spinoza", 1).unwrap()), vec!["short"]);
    assert!(search(&db, "!!").is_empty());
}

#[test]
fn upserts_and_edits_replace_the_indexed_text() {
    let db = Database::in_memory().unwrap();
    let original = post("p1", "first words");
    db.save_post(&original).unwrap();
    db.save_post(&Post { likes: 3, ..original.clone() }).unwrap();
    assert_eq!(index_rows(&db), 1, "re-saving keeps one row");

    db.save_post(&Post { content: "resaved words".into(), ..original.clone() }).unwrap();
    assert!(search(&db, "first").is_empty());
    assert_eq!(search(&db, "resaved"), vec!["p1"]);

    let current = db.load_posts().unwrap().remove(0);
    let revision = Revision::of(RevisionSubject::Post(PostId::new("p1")), &current.content, jiff::Timestamp::now());
    db.edit_post_content(&edited_post(&current, "edited words"), &revision).unwrap();
    assert!(search(&db, "resaved").is_empty());
    assert_eq!(search(&db, "edited"), vec!["p1"]);
    assert_eq!(index_rows(&db), 1);
}

#[test]
fn deleting_removes_rows_and_reposts_are_never_indexed() {
    let db = Database::in_memory().unwrap();
    db.save_post(&post("p1", "doomed thought")).unwrap();
    db.save_post(&post("keep", "a thought that stays")).unwrap();
    let repost = Post::create_with_attachments("r1", "net1", "u2", "", None, None, None, None, Some(PostId::new("p1"))).unwrap();
    db.save_post(&repost).unwrap();
    assert_eq!(index_rows(&db), 2);

    db.delete_post_with_reposts(&PostId::new("p1")).unwrap();
    assert!(search(&db, "doomed").is_empty());
    assert_eq!(search(&db, "thought"), vec!["keep"]);
    assert_eq!(index_rows(&db), 1);
}

#[test]
fn scheduled_posts_are_found_only_once_published() {
    use crate::domain::post::{PostLifecycle, PostStatus};
    let db = Database::in_memory().unwrap();
    let at = jiff::Timestamp::now();
    let waiting = Post { lifecycle: PostLifecycle::scheduled(at), ..post("later", "a lighthouse at dawn") };
    db.save_post(&waiting).unwrap();
    db.save_post(&post("now", "a lighthouse at dusk")).unwrap();
    let archived = Post { lifecycle: PostLifecycle { status: PostStatus::Archived, ..PostLifecycle::default() }, ..post("old", "a lighthouse long ago") };
    db.save_post(&archived).unwrap();
    let mut found = search(&db, "lighthouse");
    found.sort();
    assert_eq!(found, vec!["now", "old"], "archived posts stay searchable");
    db.save_post(&Post { lifecycle: PostLifecycle::default(), ..waiting }).unwrap();
    assert_eq!(search(&db, "dawn"), vec!["later"]);
}

#[test]
fn posts_stored_before_the_index_are_backfilled() {
    let conn = rusqlite::Connection::open_in_memory().unwrap();
    conn.execute_batch(
        "CREATE TABLE posts (id TEXT PRIMARY KEY, network_id TEXT NOT NULL, author_id TEXT NOT NULL, author_json TEXT NOT NULL, content TEXT NOT NULL, created_at TEXT NOT NULL);
         INSERT INTO posts VALUES ('old_1', 'net1', 'u1', '{}', 'An archived reflection', '2026-01-01T00:00:00Z');
         INSERT INTO posts VALUES ('old_2', 'net1', 'u1', '{}', '', '2026-01-02T00:00:00Z');",
    )
    .unwrap();
    let db = Database { conn };
    db.init_schema().expect("upgrade");
    assert_eq!(search(&db, "reflection"), vec!["old_1"]);
    assert_eq!(index_rows(&db), 1);
    db.init_schema().expect("a second run is a no-op");
    assert_eq!(index_rows(&db), 1);
}

#[test]
fn a_query_reads_its_words_whole_but_the_last_and_its_phrases_in_order() {
    use crate::storage::db::query_match_expression;
    let words = |list: &[&str]| list.iter().map(|w| w.to_string()).collect::<Vec<_>>();
    assert_eq!(query_match_expression(&words(&["Spinoza,", "ethi"]), &[]).as_deref(), Some("\"spinoza\" \"ethi\"*"));
    assert_eq!(query_match_expression(&words(&["rye"]), &words(&["starter  Recipe!"])).as_deref(), Some("\"rye\"* \"starter recipe\""));
    assert_eq!(query_match_expression(&words(&["a", "-"]), &words(&["\"\""])), None);
}
