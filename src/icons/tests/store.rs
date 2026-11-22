//! The icon store (S12): sets written under a temp folder with an index and their licence.

use super::{fixture_package, scratch_dir, tarball};
use crate::domain::icons::icon_set;
use crate::icons::{symbols_dir_for, IconStore, InstallError, LICENCE_FILE, SYMBOLS_DIR_NAME};
use std::path::Path;

fn now() -> jiff::Timestamp {
    crate::time::utc(2026, 11, 22, 8, 0, 0)
}

#[test]
fn the_store_lives_next_to_the_vault() {
    let db = Path::new("vault").join("simulacra.db");
    assert_eq!(symbols_dir_for(&db.to_string_lossy()), Path::new("vault").join(SYMBOLS_DIR_NAME));
    assert!(symbols_dir_for("").starts_with(std::env::temp_dir()));
}

#[test]
fn installing_a_set_writes_its_icons_licence_and_index() {
    let store = IconStore::new(scratch_dir("install"));
    let set = icon_set("fi").unwrap();
    let stored = store.install(&fixture_package(), set, "9.9.9", now()).expect("installed");
    assert_eq!((stored.id.as_str(), stored.icons, stored.version.as_str()), ("fi", 2, "9.9.9"));
    assert_eq!(store.icon_names("fi"), ["zap", "zoom-out"]);
    let zap = String::from_utf8(store.icon("fi", "zap").expect("icon")).unwrap();
    assert!(zap.starts_with("<svg xmlns=\"http://www.w3.org/2000/svg\"") && !zap.contains("script") && !zap.contains("onload"), "{zap}");
    let licence = store.licence("fi").expect("licence");
    assert!(licence.starts_with("Feather - https://feathericons.com/") && licence.contains("A Package Author"), "{licence}");
    assert!(store.root().join("fi").join(LICENCE_FILE).is_file());
    assert_eq!(store.index().get("fi"), Some(&stored));
    assert_eq!(IconStore::new(store.root()).index().sets.len(), 1, "the index is read back");
}

#[test]
fn removing_a_set_and_unsafe_names_leave_the_rest_alone() {
    let store = IconStore::new(scratch_dir("remove"));
    for id in ["fi", "lu"] {
        store.install(&fixture_package(), icon_set(id).unwrap(), "1.0.0", now()).unwrap();
    }
    assert_eq!(store.remove("fi").ok(), Some(true));
    assert_eq!(store.remove("fi").ok(), Some(false), "already gone");
    assert!(store.icon("fi", "zap").is_none() && store.icon("lu", "zap").is_some());
    assert_eq!(store.index().sets.iter().map(|s| s.id.as_str()).collect::<Vec<_>>(), ["lu"]);
    assert!(store.icon("..", "lu").is_none() && store.icon("lu", "../index").is_none());
}

#[test]
fn a_package_without_the_set_installs_nothing() {
    let store = IconStore::new(scratch_dir("missing"));
    let empty = tarball(&[("package/LICENSE", b"x")]);
    assert!(matches!(store.install(&empty, icon_set("fi").unwrap(), "1.0.0", now()), Err(InstallError::Package(_))));
    assert!(store.index().sets.is_empty() && !store.root().join("fi").exists());
}
