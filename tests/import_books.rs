//! End-to-end imports of real books (ignored by default; the books are never committed).
//!
//! Set `SIMULACRA_TEST_BOOKS` to a folder holding `.epub`, `.azw3` or `.pdf` files (directly or
//! one folder down), then run `cargo test --test import_books -- --ignored --nocapture`.
//! Every book is parsed, segmented by paragraph and by sentence, and imported as a pseudonym
//! into an in-memory vault; the timings are printed.

use simulacra::import::apply::{vault_entry, ImportRequest, OriginalFile};
use simulacra::import::plan::AuthorChoice;
use simulacra::import::{load_file, segment_document, Granularity, LoadedFile};
use simulacra::state::AppState;
use simulacra::storage::{Database, Vault};
use std::path::{Path, PathBuf};
use std::time::Instant;

const BOOKS_ENV: &str = "SIMULACRA_TEST_BOOKS";
const BOOK_EXTENSIONS: [&str; 3] = ["epub", "azw3", "pdf"];
const MAX_BYTES: u64 = 256 * 1024 * 1024;
const PSEUDONYM: &str = "Ishmael Reader";

fn books(dir: &Path) -> Vec<PathBuf> {
    let entries = std::fs::read_dir(dir).map(|it| it.flatten().map(|e| e.path()).collect::<Vec<_>>()).unwrap_or_default();
    let mut found: Vec<PathBuf> = entries
        .iter()
        .flat_map(|path| if path.is_dir() { books(path) } else { vec![path.clone()] })
        .filter(|path| path.extension().is_some_and(|ext| BOOK_EXTENSIONS.iter().any(|b| ext.eq_ignore_ascii_case(b))))
        .collect();
    found.sort();
    found
}

fn import(file: &LoadedFile, granularity: Granularity) -> (usize, u128) {
    let mut state = AppState::from_vault(Vault::spawn(Database::in_memory().expect("db")));
    let started = Instant::now();
    let units = segment_document(&file.doc, granularity);
    let title = file.doc.metadata.title.clone().unwrap_or_default();
    let request = ImportRequest {
        entry: vault_entry(&file.doc, &title, file.doc.metadata.clone()),
        original: Some(OriginalFile { name: file.name.clone(), extension: file.extension(), sha256: file.doc.facts.sha256.clone(), bytes: file.bytes.clone() }),
        units,
        chapter_titles: file.doc.sections.iter().map(|s| s.title.clone()).collect(),
        author: AuthorChoice::Pseudonym(PSEUDONYM.into()),
        create_posts: true,
    };
    let outcome = state.apply_import(request).expect("import applies");
    state.db.as_ref().expect("vault").flush().expect("writes land");
    let stored = state.db.as_ref().expect("vault").load_posts().expect("posts").len();
    assert!(stored >= outcome.posts, "every post reached the database");
    (outcome.posts, started.elapsed().as_millis())
}

#[test]
#[ignore = "needs SIMULACRA_TEST_BOOKS"]
fn real_books_import_by_paragraph_and_sentence() {
    let Some(dir) = std::env::var_os(BOOKS_ENV) else { return };
    let books = books(Path::new(&dir));
    assert!(!books.is_empty(), "no books under {dir:?}");
    for path in books {
        let started = Instant::now();
        let file = load_file(&path, MAX_BYTES).expect("book reads");
        let parsed_ms = started.elapsed().as_millis();
        let doc = &file.doc;
        assert!(doc.notes.is_empty(), "{path:?}: {:?}", doc.notes);
        assert!(doc.metadata.title.is_some() && !doc.metadata.creators.is_empty(), "{path:?} metadata {:?}", doc.metadata);
        assert!(doc.paragraph_count() > 10, "{path:?} has {} paragraphs", doc.paragraph_count());
        let (paragraph_posts, paragraph_ms) = import(&file, Granularity::Paragraph);
        let (sentence_posts, sentence_ms) = import(&file, Granularity::Sentence);
        assert!(sentence_posts >= paragraph_posts / 2);
        println!(
            "{}: {} ({:?}) {} sections, {} paragraphs; parse {parsed_ms} ms; by paragraph {paragraph_posts} posts in {paragraph_ms} ms; by sentence {sentence_posts} posts in {sentence_ms} ms",
            file.name,
            doc.metadata.title.as_deref().unwrap_or_default(),
            doc.format,
            doc.sections.len(),
            doc.paragraph_count(),
        );
    }
}
