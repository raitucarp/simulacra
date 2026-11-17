use super::*;
use crate::domain::{ProfileKind, ANONYMOUS_PROFILE_ID};
use crate::import::detect::detect_format;
use crate::import::fixtures;
use crate::import::parse::parse_document;
use crate::import::plan::AuthorChoice;
use crate::import::segment::{segment_document, Granularity};
use crate::storage::Database;
use std::sync::Arc;

fn request(bytes: Vec<u8>, name: &str, granularity: Granularity, author: AuthorChoice, create_posts: bool) -> ImportRequest {
    let doc = parse_document(detect_format(name, None, &bytes), &bytes, name);
    let title = doc.metadata.title.clone().unwrap_or_default();
    let original = OriginalFile { name: name.into(), extension: Some("epub".into()), sha256: doc.facts.sha256.clone(), bytes: Arc::new(bytes) };
    ImportRequest {
        entry: vault_entry(&doc, &title, doc.metadata.clone()),
        original: Some(original),
        units: segment_document(&doc, granularity),
        chapter_titles: doc.sections.iter().map(|s| s.title.clone()).collect(),
        author,
        create_posts,
    }
}

#[test]
fn imports_a_book_by_sentence_as_a_pseudonym() {
    let mut state = AppState::new_in_memory();
    let before = state.posts.len();
    let pen = AuthorChoice::Pseudonym("Publius".into());
    let outcome = state.apply_import(request(fixtures::epub(), "tiny.epub", Granularity::Sentence, pen.clone(), true)).expect("import");
    assert_eq!(outcome.posts, 4, "three sentences in chapter one, one in chapter two");
    assert_eq!(state.posts.len(), before + 4);
    let newest = &state.posts[0];
    assert_eq!(newest.content, "Finally, the end.");
    assert_eq!(state.posts[3].content, "It was a dark night.");
    let author = state.user_profiles.iter().find(|p| p.id == newest.author_id).expect("pseudonym exists");
    assert_eq!((author.name.as_str(), author.kind), ("Publius", ProfileKind::Pseudonym));
    assert_eq!(newest.extras.sources[0].id, outcome.document_id);
    assert!(state.documents.iter().any(|d| d.id == outcome.document_id && d.title == "A Tiny Book"));
    // The same pseudonym is reused on the next import.
    let profiles = state.user_profiles.len();
    state.apply_import(request(fixtures::epub(), "tiny.epub", Granularity::Paragraph, pen, true)).expect("import");
    assert_eq!(state.user_profiles.len(), profiles);
}

#[test]
fn just_adding_to_the_vault_creates_no_posts() {
    let mut state = AppState::new_in_memory();
    let before = state.posts.len();
    let outcome = state.apply_import(request(fixtures::epub(), "tiny.epub", Granularity::Paragraph, AuthorChoice::Yourself, false)).expect("import");
    assert_eq!((outcome.posts, state.posts.len()), (0, before));
    assert!(outcome.original.as_ref().is_some_and(|path| path.is_file()), "keep original is on by default");
}

#[test]
fn anonymous_imports_use_one_stable_account_and_persist() {
    let mut state = AppState::from_vault(crate::storage::Vault::spawn(Database::in_memory().expect("db")));
    for _ in 0..2 {
        state.apply_import(request(fixtures::docx(), "notes.docx", Granularity::Paragraph, AuthorChoice::Anonymous, true)).expect("import");
    }
    let anonymous: Vec<_> = state.user_profiles.iter().filter(|p| p.id.as_str() == ANONYMOUS_PROFILE_ID).collect();
    assert_eq!(anonymous.len(), 1);
    assert_eq!(anonymous[0].kind, ProfileKind::Anonymous);
    let db = state.db.as_ref().expect("vault");
    let stored = db.load_profiles().expect("profiles");
    assert!(stored.iter().any(|p| p.id.as_str() == ANONYMOUS_PROFILE_ID && p.kind == ProfileKind::Anonymous));
    let posts = db.load_posts().expect("posts");
    assert!(posts.iter().filter(|p| p.author_id.as_str() == ANONYMOUS_PROFILE_ID).count() >= 8);
}

#[test]
fn originals_over_the_cap_are_refused_and_paths_are_by_checksum() {
    let dir = std::env::temp_dir().join(format!("simulacra-import-test-{}", uuid::Uuid::new_v4().simple()));
    let path = original_path(&dir, "abcdef0123456789ffff", Some("pdf"));
    assert!(path.ends_with("imports/abcdef0123456789.pdf") || path.ends_with("imports\\abcdef0123456789.pdf"));
    assert!(keep_original(&path, &[0u8; 10], 5).is_err());
    assert_eq!(keep_original(&path, b"tiny", 5).expect("kept"), path);
    assert_eq!(std::fs::read(&path).expect("written"), b"tiny");
    assert!(original_path(&dir, "ab", Some("../x")).ends_with("ab"));
    let _ = std::fs::remove_dir_all(dir);
}
