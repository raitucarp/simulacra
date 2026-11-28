//! The store of installed packs: install, list, read back and remove, built-ins protected.

use super::{scratch_dir, SAMPLE_SIMPACK};
use crate::domain::packs::{sha256_hex, PackError};
use crate::packs::{packs_dir_for, read_simpack, PackStore, PACKS_DIR_NAME};

#[test]
fn a_verified_pack_installs_lists_and_removes() {
    let root = scratch_dir("store");
    let store = PackStore::new(&root);
    assert!(store.installed().is_empty());
    let pack = read_simpack(SAMPLE_SIMPACK).expect("sample");
    let installed = store.install(&pack).expect("installed");
    assert_eq!(installed.dir, root.join("sample"));
    assert_eq!(store.installed(), vec![installed.clone()]);
    let entry = installed.manifest.file("spot/ideas").expect("covered");
    let bytes = std::fs::read(installed.file_path(&entry.file).expect("safe path")).expect("written");
    assert_eq!(sha256_hex(&bytes), entry.sha256);
    assert!(!root.join("sample.partial").exists(), "the scratch folder is moved into place");
    assert_eq!(store.install(&pack).map(|p| p.dir), Ok(root.join("sample")), "reinstalling replaces the copy");
    assert_eq!(store.remove("sample"), Ok(true));
    assert_eq!(store.remove("sample"), Ok(false));
    assert!(store.installed().is_empty());
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn built_in_ids_unsafe_ids_and_stray_folders_are_never_touched() {
    let root = scratch_dir("store-guard");
    let store = PackStore::new(&root);
    assert_eq!(store.remove("flat"), Err(PackError::BuiltIn("flat".into())));
    assert_eq!(store.remove("classic"), Err(PackError::BuiltIn("classic".into())));
    assert_eq!(store.remove("../escape"), Ok(false));
    std::fs::create_dir_all(root.join("stray")).unwrap();
    std::fs::write(root.join("stray").join("pack.json"), r#"{"id":"other","name":"X","version":"1"}"#).unwrap();
    assert!(store.installed().is_empty(), "a manifest must name its own folder");
    assert!(store.get("..").is_none() && store.get("flat").is_none());
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn the_store_lives_next_to_the_vault() {
    let db = std::path::Path::new("data").join("vault.db");
    assert_eq!(packs_dir_for(&db.to_string_lossy()), std::path::Path::new("data").join(PACKS_DIR_NAME));
    assert!(packs_dir_for("").starts_with(std::env::temp_dir()));
}
