use crate::storage::db::Database;

#[test]
fn test_settings_kv_store() {
    let db = Database::in_memory().expect("in-memory db");
    assert_eq!(db.get_setting("custom_key").unwrap(), None);

    db.set_setting("custom_key", "value_1").unwrap();
    assert_eq!(db.get_setting("custom_key").unwrap(), Some("value_1".to_string()));

    // Conflict update
    db.set_setting("custom_key", "value_2").unwrap();
    assert_eq!(db.get_setting("custom_key").unwrap(), Some("value_2".to_string()));
}

#[test]
fn test_db_theme_tokens_persistence() {
    let db = Database::in_memory().expect("in-memory db");
    assert_eq!(db.load_theme_tokens().unwrap(), None);

    let mut custom_tokens = crate::theme::obsidian_zinc();
    custom_tokens.titlebar_height = gpui_kit::px(42.0);
    custom_tokens.accent_brand = gpui_kit::rgb(0xff00ff);

    db.save_theme_tokens(&custom_tokens).expect("save theme tokens");

    let loaded = db.load_theme_tokens().unwrap().expect("loaded tokens");
    assert_eq!(loaded.titlebar_height, gpui_kit::px(42.0));
    assert_eq!(loaded.accent_brand, gpui_kit::rgb(0xff00ff));
    assert_eq!(loaded, custom_tokens);
}

#[test]
fn test_db_theme_tokens_corrupted_json() {
    let db = Database::in_memory().expect("in-memory db");
    db.set_setting("theme_tokens", "{ invalid json }").unwrap();
    let result = db.load_theme_tokens();
    assert!(result.is_err());
}
