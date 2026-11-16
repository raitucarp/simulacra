use crate::domain::portrait::{AvatarMode, PortraitSpec};
use crate::domain::ProfileId;
use crate::state::AppState;
use std::fs;

#[test]
fn photos_import_into_the_vault_folder_and_avatars_persist() {
    let dir = std::env::temp_dir().join(format!("simulacra_state_avatar_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&dir).unwrap();
    let db_path = dir.join("simulacra.db");
    let source = dir.join("picked.webp");
    fs::write(&source, b"webp").unwrap();
    {
        let mut state = AppState::with_db_path(&db_path);
        assert_eq!(state.vault_dir().as_deref().map(|p| p.canonicalize().unwrap()), Some(dir.canonicalize().unwrap()));
        let mut profile = state.user_profiles[0].clone();
        let mode = state.import_avatar_photo(&profile.id, &source).expect("import");
        assert_eq!(mode, AvatarMode::photo(format!("avatars/{}.webp", profile.id.as_str())));
        assert!(dir.join("avatars").join(format!("{}.webp", profile.id.as_str())).is_file());
        profile.config.avatar = Some(AvatarMode::Pcg(PortraitSpec::random(11)));
        state.update_profile(profile).unwrap();
    }
    let reloaded = AppState::with_db_path(&db_path);
    assert_eq!(reloaded.user_profiles[0].config.avatar, Some(AvatarMode::Pcg(PortraitSpec::random(11))));
    let memory = AppState::new_in_memory();
    assert!(memory.vault_dir().is_none());
    assert!(memory.import_avatar_photo(&ProfileId::new("p"), &source).is_err());
    drop((reloaded, memory));
    let _ = fs::remove_dir_all(&dir);
}
