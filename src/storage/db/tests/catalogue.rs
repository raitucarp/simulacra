use crate::domain::document::facets::Facet;
use crate::domain::document::fixtures::{documents, library, LANGUAGE, NET, PHILOSOPHY, SHELF};
use crate::domain::document::query::{CatalogueQuery, SortKey};
use crate::domain::document::record::{DocumentKind, OriginKind};
use crate::domain::document::{Collection, DocumentId, Subject, SubjectId};
use crate::domain::{NetworkId, Post, Profile};
use crate::storage::db::versions::{CATALOGUE_VERSION, LATEST_VERSION};
use crate::storage::db::{build_catalogue_search, fts_match_expression, Database};

fn catalogued() -> Database {
    let db = Database::in_memory().unwrap();
    library().subjects.iter().for_each(|s| db.save_subject(s).unwrap());
    let shelf = Collection { id: SHELF.into(), network_id: NetworkId::new(NET), name: "Rationalists".into(), description: String::new() };
    db.save_collection(&shelf).unwrap();
    documents().iter().for_each(|d| db.save_document(d).unwrap());
    db
}

fn query(text: &str) -> CatalogueQuery {
    CatalogueQuery { text: text.into(), ..CatalogueQuery::new(NetworkId::new(NET)) }
}

fn search(db: &Database, query: &CatalogueQuery) -> Vec<String> {
    db.search_catalogue(query, &library().tree()).unwrap()
}

#[test]
fn documents_round_trip_with_record_and_memberships() {
    let db = catalogued();
    let mut loaded = db.load_all_documents().unwrap();
    let mut expected = documents();
    loaded.sort_by(|a, b| a.id.cmp(&b.id));
    expected.sort_by(|a, b| a.id.cmp(&b.id));
    assert_eq!(loaded, expected);
    assert_eq!(db.load_library().unwrap().collections.len(), 1);
}

#[test]
fn full_text_search_ranks_and_matches_prefixes_subjects_and_creators() {
    let db = catalogued();
    assert_eq!(search(&db, &query("wittgen")), ["doc_tractatus"]);
    assert_eq!(search(&db, &query("language")), ["doc_tractatus"], "subject labels are indexed");
    assert_eq!(search(&db, &query("\"modes\" substance*")), ["doc_note"], "user text cannot inject FTS syntax");
    assert_eq!(search(&db, &query("SUBSTANCE")).len(), 3);
}

#[test]
fn facet_filters_widen_subjects_to_narrower_terms() {
    let db = catalogued();
    let mut q = query("");
    q.selection = q.selection.toggled(Facet::Subject, PHILOSOPHY);
    assert_eq!(search(&db, &q).len(), 3);
    q.selection = q.selection.only(Facet::Subject, LANGUAGE).toggled(Facet::Kind, "book");
    assert_eq!(search(&db, &q), ["doc_tractatus"]);
    let mut shelved = query("");
    shelved.selection = shelved.selection.toggled(Facet::Collection, SHELF).toggled(Facet::Language, "EN");
    assert_eq!(search(&db, &shelved), ["doc_ethics"]);
}

#[test]
fn sql_and_in_memory_orders_agree() {
    let db = catalogued();
    let docs = documents();
    for key in [SortKey::Title, SortKey::DateAdded, SortKey::DatePublished] {
        for q in [query("").sorted_by(key), query("").sorted_by(key).sorted_by(key)] {
            let memory: Vec<String> = crate::domain::document::query::run(&docs, &q, &library().tree(), Default::default())
                .iter()
                .map(|d| d.id.clone())
                .collect();
            assert_eq!(search(&db, &q), memory, "{key:?} descending={}", q.descending);
        }
    }
}

#[test]
fn most_linked_counts_wikilinks_and_titles_from_posts() {
    let db = catalogued();
    let author = Profile::new_primary("u1", "Ada", "@ada");
    for (id, text) in [("p1", "on [[doc_note]]"), ("p2", "again [[A Note on Modes]]"), ("p3", "[[doc_tractatus]]")] {
        let mut post = Post::create(id, NET, &author, text).unwrap();
        post.linked_posts = Post::parse_linked_posts(text);
        db.save_post(&post).unwrap();
    }
    assert_eq!(search(&db, &query("").sorted_by(SortKey::MostLinked)), ["doc_note", "doc_tractatus", "doc_ethics"]);
}

#[test]
fn the_builder_is_parameterised_and_quotes_terms() {
    let mut q = query("Spinoza's \"ethics\"");
    q.selection = q.selection.toggled(Facet::Kind, "book").toggled(Facet::Kind, "note");
    assert_eq!(fts_match_expression(&q).as_deref(), Some("\"spinoza\"* \"ethics\"*"));
    let built = build_catalogue_search(&q, &library().tree());
    assert!(built.sql.contains("documents_fts MATCH ?") && built.sql.contains("c.kind IN (?, ?)"));
    assert!(built.sql.ends_with("ORDER BY score ASC, c.sort_title ASC, d.id ASC"));
    assert_eq!(built.params, ["\"spinoza\"* \"ethics\"*", NET, "book", "note"]);
}

#[test]
fn deleting_removes_record_index_and_memberships() {
    let db = catalogued();
    db.delete_document("doc_tractatus").unwrap();
    assert!(search(&db, &query("wittgenstein")).is_empty());
    assert!(db.document_text(&DocumentId::new("doc_tractatus")).unwrap().is_none());
    let text = db.document_text(&DocumentId::new("doc_ethics")).unwrap().expect("text by id");
    assert!(text.text.starts_with("Ethics body") && text.checksum.starts_with("fnv1a64:"));
}

#[test]
fn deleting_a_subject_moves_its_narrower_terms_up() {
    let db = catalogued();
    db.save_subject(&Subject::new("subj_tense", "Tense logic").under(LANGUAGE)).unwrap();
    db.delete_subject(&SubjectId::new(LANGUAGE)).unwrap();
    let library = db.load_library().unwrap();
    assert_eq!(library.subject(&SubjectId::new("subj_tense")).and_then(|s| s.broader.clone()), Some(SubjectId::new(PHILOSOPHY)));
    let tractatus = db.load_all_documents().unwrap().into_iter().find(|d| d.id == "doc_tractatus").unwrap();
    assert!(tractatus.record.subjects.is_empty());
}

#[test]
fn version_one_vaults_get_legacy_records_and_an_index() {
    let conn = rusqlite::Connection::open_in_memory().unwrap();
    conn.execute_batch(
        "CREATE TABLE documents (id TEXT PRIMARY KEY, network_id TEXT NOT NULL, title TEXT NOT NULL, content TEXT NOT NULL,
             chunk_count INTEGER NOT NULL DEFAULT 1, created_at TEXT NOT NULL);
         INSERT INTO documents VALUES ('doc_old', 'net', 'Old notes', 'Monism unifies mind and body. More text.', 1, '2025-05-01T10:00:00Z');
         PRAGMA user_version = 1;",
    )
    .unwrap();
    let db = Database { conn };
    db.init_schema().expect("v1 -> v2");
    assert_eq!((db.schema_version().unwrap(), CATALOGUE_VERSION), (LATEST_VERSION, 2));
    let doc = db.load_all_documents().unwrap().pop().unwrap();
    assert_eq!((doc.title.as_str(), doc.created_at.to_rfc3339().as_str()), ("Old notes", "2025-05-01T10:00:00+00:00"));
    assert_eq!((doc.record.kind, doc.record.provenance.origin), (DocumentKind::Note, OriginKind::Legacy));
    assert_eq!(doc.record.description, "Monism unifies mind and body.");
    let found = db.search_catalogue(&CatalogueQuery { text: "monism".into(), ..CatalogueQuery::new(NetworkId::new("net")) }, &Default::default());
    assert_eq!(found.unwrap(), ["doc_old"]);
    db.init_schema().expect("idempotent");
    assert_eq!(db.load_all_documents().unwrap().len(), 1);
}
