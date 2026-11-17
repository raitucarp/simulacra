use crate::domain::embedding::{EmbeddingOwner, StoredChunk, VectorIndex};
use crate::domain::PostId;
use crate::storage::db::embeddings_schema::EMBEDDING_MODEL_VERSION;
use crate::storage::db::Database;
use crate::storage::SettingKey;

fn chunk(index: usize, hash: &str, vector: &[f32]) -> StoredChunk {
    StoredChunk { index, model: "gemini-embedding-001".into(), content_hash: hash.into(), vector: vector.to_vec() }
}

fn rows() -> Vec<(EmbeddingOwner, StoredChunk)> {
    let doc = EmbeddingOwner::document("d1");
    vec![
        (doc.clone(), chunk(0, "a", &[0.25, -0.5, 1.0])),
        (doc, chunk(1, "b", &[1.0, 0.0, 0.0])),
        (EmbeddingOwner::post(&PostId::new("p1")), chunk(0, "c", &[0.0, 1.0, 0.0])),
    ]
}

#[test]
fn vectors_round_trip_as_f32_blobs_and_upsert_by_chunk() {
    let db = Database::in_memory().unwrap();
    db.save_embeddings(&rows()).unwrap();
    let loaded = VectorIndex::from_rows(db.load_embeddings().unwrap());
    assert_eq!(loaded, VectorIndex::from_rows(rows()));
    let replaced = [(EmbeddingOwner::document("d1"), chunk(1, "b2", &[0.0, 0.0, 1.0]))];
    db.save_embeddings(&replaced).unwrap();
    assert_eq!(db.count_embeddings().unwrap(), 3, "same owner and chunk is an update");
    let loaded = VectorIndex::from_rows(db.load_embeddings().unwrap());
    assert_eq!(loaded.chunk(&EmbeddingOwner::document("d1"), 1).unwrap().content_hash, "b2");
    let dims: i64 = db.conn.query_row("SELECT dims FROM embeddings WHERE owner_id = 'd1' AND chunk_index = 0", [], |r| r.get(0)).unwrap();
    assert_eq!(dims, 3);
}

#[test]
fn trimming_drops_trailing_chunks_and_whole_owners() {
    let db = Database::in_memory().unwrap();
    db.save_embeddings(&rows()).unwrap();
    db.trim_embeddings(&EmbeddingOwner::document("d1"), 1).unwrap();
    assert_eq!(db.count_embeddings().unwrap(), 2);
    db.trim_embeddings(&EmbeddingOwner::post(&PostId::new("p1")), 0).unwrap();
    assert_eq!(db.count_embeddings().unwrap(), 1);
}

#[test]
fn corrupt_rows_are_skipped_and_the_table_is_idempotent() {
    let db = Database::in_memory().unwrap();
    db.save_embeddings(&rows()).unwrap();
    db.conn.execute("INSERT INTO embeddings VALUES ('alien', 'x', 0, 'm', 1, 'h', x'00')", []).unwrap();
    db.conn.execute("INSERT INTO embeddings VALUES ('post', 'bad', 0, 'm', 1, 'h', x'000000')", []).unwrap();
    assert_eq!(db.load_embeddings().unwrap().len(), 3, "unknown kinds and broken blobs are ignored");
    db.init_schema().expect("re-running the schema keeps the rows");
    assert_eq!(db.count_embeddings().unwrap(), 5);
}

#[test]
fn v4_moves_a_gemini_gemma_placeholder_to_the_working_model_once() {
    let db = Database::in_memory().unwrap();
    assert!(db.schema_version().unwrap() >= EMBEDDING_MODEL_VERSION);
    let old = r#"{"provider":"gemini","model":"embeddinggemma","chunk_size":512}"#;
    db.set_setting(SettingKey::PrefsEmbeddings, old).unwrap();
    db.conn.execute_batch("PRAGMA user_version = 1").unwrap();
    db.init_schema().unwrap();
    let migrated = db.get_setting(SettingKey::PrefsEmbeddings).unwrap().unwrap();
    assert!(migrated.contains("gemini-embedding-001") && migrated.contains("512"), "{migrated}");
    assert_eq!(db.schema_version().unwrap(), EMBEDDING_MODEL_VERSION);

    let local = r#"{"provider":"local_server","model":"embeddinggemma"}"#;
    db.set_setting(SettingKey::PrefsEmbeddings, local).unwrap();
    db.conn.execute_batch("PRAGMA user_version = 1").unwrap();
    db.init_schema().unwrap();
    assert_eq!(db.get_setting(SettingKey::PrefsEmbeddings).unwrap().unwrap(), local, "local Gemma stays");
}
