//! Database location resolution and legacy-file migration.

use crate::storage::db::location::*;
use crate::storage::db::DEFAULT_DB_FILE;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// An injected environment backed by a fixed map.
fn env_of(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
    let map: HashMap<String, String> =
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
    move |key| map.get(key).cloned()
}

/// A fresh, empty scratch directory under the OS temp dir.
fn scratch_dir() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("simulacra-test-{}", uuid::Uuid::new_v4().simple()));
    std::fs::create_dir_all(&dir).expect("create scratch dir");
    dir
}

#[test]
fn resolves_per_platform_data_dirs() {
    let env = env_of(&[("APPDATA", "C:/Users/me/AppData/Roaming"), ("HOME", "/home/me")]);
    assert_eq!(
        resolve_db_path(&env, Platform::Windows, DEFAULT_DB_FILE),
        Path::new("C:/Users/me/AppData/Roaming").join(APP_DIR_NAME).join(DEFAULT_DB_FILE)
    );
    assert_eq!(
        resolve_db_path(&env, Platform::MacOs, DEFAULT_DB_FILE),
        Path::new("/home/me/Library/Application Support").join(APP_DIR_NAME).join(DEFAULT_DB_FILE)
    );
    assert_eq!(
        resolve_db_path(&env, Platform::Unix, DEFAULT_DB_FILE),
        Path::new("/home/me/.local/share").join(APP_DIR_NAME).join(DEFAULT_DB_FILE)
    );
    let xdg = env_of(&[("XDG_DATA_HOME", "/xdg"), ("HOME", "/home/me")]);
    assert_eq!(
        resolve_db_path(&xdg, Platform::Unix, DEFAULT_DB_FILE),
        Path::new("/xdg").join(APP_DIR_NAME).join(DEFAULT_DB_FILE)
    );
}

#[test]
fn env_override_wins_and_missing_env_falls_back_to_cwd() {
    let env = env_of(&[(DB_PATH_ENV, "/tmp/custom.db"), ("APPDATA", "C:/x")]);
    assert_eq!(resolve_db_path(&env, Platform::Windows, DEFAULT_DB_FILE), PathBuf::from("/tmp/custom.db"));

    let blank = env_of(&[(DB_PATH_ENV, "  "), ("APPDATA", "  ")]);
    assert_eq!(resolve_db_path(&blank, Platform::Windows, DEFAULT_DB_FILE), PathBuf::from(DEFAULT_DB_FILE));
}

#[test]
fn migration_copies_only_when_target_is_missing_and_legacy_exists() {
    let dir = scratch_dir();
    let legacy = dir.join("legacy.db");
    let target = dir.join("data").join("simulacra.db");
    assert_eq!(plan_migration(&target, &legacy), Migration::NotNeeded, "no legacy file");

    std::fs::write(&legacy, b"legacy").expect("write legacy");
    let plan = plan_migration(&target, &legacy);
    assert_eq!(plan, Migration::CopyLegacy { from: legacy.clone(), to: target.clone() });

    std::fs::create_dir_all(target.parent().expect("parent")).expect("mkdir");
    std::fs::write(&target, b"current").expect("write target");
    assert_eq!(plan_migration(&target, &legacy), Migration::NotNeeded, "target already exists");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn prepare_copies_legacy_db_and_wal_once_without_moving_it() {
    let dir = scratch_dir();
    let legacy = dir.join(DEFAULT_DB_FILE);
    std::fs::write(&legacy, b"main").expect("write legacy");
    std::fs::write(dir.join(format!("{DEFAULT_DB_FILE}-wal")), b"wal").expect("write wal");
    let appdata = dir.join("appdata");
    let env = env_of(&[("APPDATA", appdata.to_str().expect("utf8 path"))]);

    let target = prepare_db_path(&env, Platform::Windows, &legacy);
    assert_eq!(target, appdata.join(APP_DIR_NAME).join(DEFAULT_DB_FILE));
    assert_eq!(std::fs::read(&target).expect("copied"), b"main");
    assert_eq!(std::fs::read(format!("{}-wal", target.display())).expect("wal copied"), b"wal");
    assert!(legacy.is_file(), "legacy file is copied, not moved");

    std::fs::write(&target, b"newer").expect("update target");
    prepare_db_path(&env, Platform::Windows, &legacy);
    assert_eq!(std::fs::read(&target).expect("kept"), b"newer", "second run does not overwrite");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn prepare_with_override_skips_migration() {
    let dir = scratch_dir();
    let legacy = dir.join(DEFAULT_DB_FILE);
    std::fs::write(&legacy, b"main").expect("write legacy");
    let explicit = dir.join("explicit.db");
    let env = env_of(&[(DB_PATH_ENV, explicit.to_str().expect("utf8 path"))]);
    assert_eq!(prepare_db_path(&env, Platform::Windows, &legacy), explicit);
    assert!(!explicit.exists(), "override path is never seeded from the legacy file");
    let _ = std::fs::remove_dir_all(&dir);
}
