use super::*;
use crate::domain::data::export::{BundleFile, FileBody};
use crate::domain::data::history::{Revision, RevisionSubject};
use crate::domain::data::saved::{Bookmark, CollectionEntry, ItemRef};
use crate::domain::data::snapshot::SnapshotError;
use crate::domain::document::{CollectionId, DocumentId};
use crate::domain::{Post, PostId};
use crate::storage::Database;
use jiff::Timestamp;
use std::io::Read;
use std::path::PathBuf;
use zeroize::Zeroizing;

/// A fresh scratch folder under the system temp dir.
fn scratch() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("simulacra-data-test-{}", uuid::Uuid::new_v4().simple()));
    std::fs::create_dir_all(&dir).expect("scratch dir");
    dir
}

fn vault_with_post(path: &std::path::Path, text: &str) -> Database {
    let db = Database::open(path).expect("open");
    db.save_post(&Post::create("post_1", "net", "user_main", text).unwrap()).expect("post");
    db
}

fn passphrase(text: &str) -> Zeroizing<String> {
    Zeroizing::new(text.to_string())
}

#[test]
fn bundle_round_trips_through_the_zip() {
    let dir = scratch();
    let copied = dir.join("image.png");
    std::fs::write(&copied, b"png bytes").unwrap();
    let files = vec![
        BundleFile { path: "manifest.json".into(), body: FileBody::Text("{}".into()) },
        BundleFile { path: "threads/media/image.png".into(), body: FileBody::Copy(copied) },
        BundleFile { path: "threads/media/gone.png".into(), body: FileBody::Copy(dir.join("gone.png")) },
    ];
    let out = dir.join("export.zip");
    let report = write_bundle(&out, &files).expect("written");
    assert_eq!((report.files, report.missing), (2, 1));
    let mut archive = zip::ZipArchive::new(std::fs::File::open(&out).unwrap()).unwrap();
    let mut image = String::new();
    archive.by_name("threads/media/image.png").unwrap().read_to_string(&mut image).unwrap();
    assert_eq!(image, "png bytes");
    assert!(archive.by_name("threads/media/gone.png").is_err());
    assert!(!dir.join("export.zip.partial").exists());
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn snapshot_seals_a_database_copy_and_stages_it_verified() {
    let dir = scratch();
    let db = vault_with_post(&dir.join("simulacra.db"), "kept in the snapshot");
    let copy = dir.join("copy.db");
    db.backup_to(&copy).unwrap();
    let file = dir.join("vault.simsnap");
    let size = create_snapshot(&copy, &file, &passphrase("a long passphrase"), 1_795_000_000).expect("sealed");
    assert!(size > 0 && !copy.exists(), "the plain copy is removed");
    let sealed = std::fs::read(&file).unwrap();
    assert!(!sealed.windows(b"kept in the snapshot".len()).any(|w| w == b"kept in the snapshot"));
    let staged = staged_restore_path(&dir.join("simulacra.db"));
    let restore = open_snapshot(&file, &passphrase("a long passphrase"), &staged).expect("staged");
    assert_eq!((restore.summary.posts, restore.created_at), (1, 1_795_000_000));
    assert!(staged.exists());
    drop(db);
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn a_wrong_passphrase_stages_nothing() {
    let dir = scratch();
    let db = vault_with_post(&dir.join("simulacra.db"), "secret");
    db.backup_to(&dir.join("copy.db")).unwrap();
    let file = dir.join("vault.simsnap");
    create_snapshot(&dir.join("copy.db"), &file, &passphrase("right passphrase"), 0).unwrap();
    let staged = dir.join("staged.db");
    assert_eq!(open_snapshot(&file, &passphrase("wrong passphrase"), &staged), Err(StageFailure::Snapshot(SnapshotError::WrongPassphrase)));
    assert!(!staged.exists());
    drop(db);
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn a_sealed_file_that_is_not_a_vault_is_refused_and_unstaged() {
    let dir = scratch();
    let not_a_vault = dir.join("notes.txt");
    std::fs::write(&not_a_vault, b"just some text, not SQLite").unwrap();
    let file = dir.join("odd.simsnap");
    create_snapshot(&not_a_vault, &file, &passphrase("a long passphrase"), 0).unwrap();
    let staged = dir.join("staged.db");
    assert!(matches!(open_snapshot(&file, &passphrase("a long passphrase"), &staged), Err(StageFailure::NotAVault(_))));
    assert!(!staged.exists());
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn restore_swaps_the_live_database_and_keeps_a_pre_restore_copy() {
    let dir = scratch();
    let live_path = dir.join("simulacra.db");
    let snapshot_db = vault_with_post(&dir.join("old.db"), "from the snapshot");
    drop(snapshot_db);
    let live = vault_with_post(&live_path, "written after the snapshot");
    let keep = pre_restore_path(&live_path);
    live.backup_to(&keep).unwrap();
    live.restore_from_file(&dir.join("old.db")).expect("restored");
    assert_eq!(live.load_posts().unwrap()[0].content, "from the snapshot");
    let kept = Database::open(&keep).unwrap();
    assert_eq!(kept.load_posts().unwrap()[0].content, "written after the snapshot");
    assert_ne!(pre_restore_path(&live_path), keep, "a second restore keeps both copies");
    drop((live, kept));
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn revisions_and_post_edits_persist_together() {
    let db = Database::in_memory().unwrap();
    db.save_post(&Post::create("post_1", "net", "user_main", "first words").unwrap()).unwrap();
    let revision = Revision::of(RevisionSubject::Post(PostId::new("post_1")), "first words", Timestamp::now());
    let edited = crate::domain::data::history::edited_post(&db.load_posts().unwrap()[0], "second words");
    db.edit_post_content(&edited, &revision).unwrap();
    assert_eq!(db.load_posts().unwrap()[0].content, "second words");
    let stored = db.revisions_of(&RevisionSubject::Post(PostId::new("post_1"))).unwrap();
    let numbered = Revision { version: Some(crate::domain::data::history::SemVer::ORIGINAL), ..revision };
    assert_eq!(stored, vec![numbered]);
    assert_eq!(db.revision_counts("post").unwrap(), vec![("post_1".to_string(), 1)]);
}

#[test]
fn bookmarks_and_collection_items_round_trip() {
    let db = Database::in_memory().unwrap();
    let now = "2026-11-18T09:00:00Z".parse::<jiff::Timestamp>().unwrap();
    db.save_bookmark(&Bookmark { post: PostId::new("p1"), created_at: now }).unwrap();
    let shelf = CollectionId::new("coll_1");
    let entry = |item: ItemRef, position: i64| CollectionEntry { collection: shelf.clone(), item, position, note: "why it matters".into(), added_at: now };
    db.save_collection_entries(&[entry(ItemRef::Post(PostId::new("p1")), 1), entry(ItemRef::Document(DocumentId::new("d1")), 0)]).unwrap();
    db.save_collection_entries(&[CollectionEntry { note: "edited".into(), ..entry(ItemRef::Post(PostId::new("p1")), 2) }]).unwrap();
    let loaded = db.load_saved_shelf().unwrap();
    assert_eq!(loaded.bookmarks.len(), 1);
    assert_eq!(loaded.entries.iter().map(|e| (e.item.id(), e.position, e.note.as_str())).collect::<Vec<_>>(), vec![("d1", 0, "why it matters"), ("p1", 2, "edited")]);
    db.delete_collection_entry(&shelf, &ItemRef::Post(PostId::new("p1"))).unwrap();
    db.delete_bookmark(&PostId::new("p1")).unwrap();
    let loaded = db.load_saved_shelf().unwrap();
    assert!(loaded.bookmarks.is_empty());
    assert_eq!(loaded.entries.len(), 1);
}
