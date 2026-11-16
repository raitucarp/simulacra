use super::*;
use std::fs;

fn temp_dir(label: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("simulacra_{label}_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn relative_paths_are_sanitized_and_lowercased() {
    let id = ProfileId::new("agent/../evil id");
    assert_eq!(avatar_relative_path(&id, "PNG"), "avatars/agent____evil_id.png");
    assert_eq!(avatar_relative_path(&ProfileId::new("p_1-a"), "jpg"), "avatars/p_1-a.jpg");
}

#[test]
fn stored_paths_resolve_against_the_vault() {
    let vault = Path::new("/data/simulacra");
    assert_eq!(resolve_photo_path(Some(vault), "avatars/a.png"), vault.join("avatars/a.png"));
    let absolute = std::env::temp_dir().join("x.png");
    let stored = absolute.to_string_lossy().to_string();
    assert_eq!(resolve_photo_path(Some(vault), &stored), absolute);
    assert_eq!(resolve_photo_path(None, "avatars/a.png"), PathBuf::from("avatars/a.png"));
    assert_eq!(vault_dir_for(""), None);
    assert_eq!(vault_dir_for("/data/simulacra/simulacra.db"), Some(PathBuf::from("/data/simulacra")));
}

#[test]
fn import_copies_into_the_vault_and_replaces_other_formats() {
    let vault = temp_dir("vault");
    let outside = temp_dir("outside");
    let id = ProfileId::new("agent_7");
    let jpg = outside.join("Me.JPG");
    fs::write(&jpg, b"jpeg-bytes").unwrap();
    let stored = import_photo(&vault, &id, &jpg).expect("import jpg");
    assert_eq!(stored, "avatars/agent_7.jpg");
    assert_eq!(fs::read(vault.join(&stored)).unwrap(), b"jpeg-bytes");

    let png = outside.join("new.png");
    fs::write(&png, b"png-bytes").unwrap();
    let stored = import_photo(&vault, &id, &png).expect("import png");
    assert_eq!(stored, "avatars/agent_7.png");
    assert!(!vault.join("avatars/agent_7.jpg").exists(), "the old format is removed");
    fs::remove_file(&png).unwrap();
    assert_eq!(fs::read(resolve_photo_path(Some(&vault), &stored)).unwrap(), b"png-bytes", "the vault owns a copy");

    let again = import_photo(&vault, &id, &vault.join(&stored)).expect("re-import own file");
    assert_eq!(again, stored);
    let _ = fs::remove_dir_all(&vault);
    let _ = fs::remove_dir_all(&outside);
}

#[test]
fn import_rejects_unsupported_or_missing_files() {
    let vault = temp_dir("vault_bad");
    let id = ProfileId::new("p");
    let err = import_photo(&vault, &id, Path::new("notes.txt")).unwrap_err();
    assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    assert!(import_photo(&vault, &id, &vault.join("missing.png")).is_err());
    let _ = fs::remove_dir_all(&vault);
}

#[test]
fn the_registered_vault_dir_round_trips() {
    let before = vault_dir();
    register_vault_dir(Some(PathBuf::from("/v")));
    assert_eq!(vault_dir(), Some(PathBuf::from("/v")));
    register_vault_dir(before);
}
