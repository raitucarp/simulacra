use crate::storage::db::{Database, SettingKey};
use crate::theme::palette::PaletteColorKey;
use crate::theme::registry::{DEFAULT_PALETTE_ID, OBSIDIAN_ZINC_ID};
use crate::theme::settings::{ThemeSettings, MIGRATED_PALETTE_ID};
use crate::theme::{Density, RadiusScale, TypeScale};
use std::collections::HashSet;

/// Keys that existed before `SettingKey`; their stored strings must never change.
const LEGACY_KEY_STRINGS: [(SettingKey, &str); 10] = [
    (SettingKey::ActiveNetwork, "active_network_id"),
    (SettingKey::ActiveProfile, "active_profile_id"),
    (SettingKey::SchedulerEnabled, "scheduler_enabled"),
    (SettingKey::LlmProvider, "llm_provider"),
    (SettingKey::LlmApiKey, "llm_api_key"),
    (SettingKey::LlmModel, "llm_model"),
    (SettingKey::LlmEndpoint, "llm_endpoint"),
    (SettingKey::LlmTemperature, "llm_temperature"),
    (SettingKey::LlmMaxTokens, "llm_max_tokens"),
    (SettingKey::LegacyThemeTokens, "theme_tokens"),
];

#[test]
fn test_settings_kv_store() {
    let db = Database::in_memory().expect("in-memory db");
    let key = SettingKey::SchedulerEnabled;
    assert_eq!(db.get_setting(key).unwrap(), None);

    db.set_setting(key, "value_1").unwrap();
    assert_eq!(db.get_setting(key).unwrap(), Some("value_1".to_string()));

    // Conflict update
    db.set_setting(key, "value_2").unwrap();
    assert_eq!(db.get_setting(key).unwrap(), Some("value_2".to_string()));

    db.delete_setting(key).unwrap();
    assert_eq!(db.get_setting(key).unwrap(), None);
}

#[test]
fn setting_keys_roundtrip_and_are_unique() {
    let mut seen = HashSet::new();
    for key in SettingKey::ALL {
        assert_eq!(SettingKey::from_key(key.as_str()), Some(*key));
        assert!(seen.insert(key.as_str()), "duplicate key {}", key.as_str());
    }
    assert_eq!(SettingKey::from_key("no_such_key"), None);
    for (key, stored) in LEGACY_KEY_STRINGS {
        assert_eq!(key.as_str(), stored, "persisted key string changed");
    }
}

#[test]
fn theme_settings_roundtrip() {
    let db = Database::in_memory().expect("in-memory db");
    assert_eq!(db.load_theme_settings(), ThemeSettings::default());

    let mut settings = ThemeSettings {
        density: Density::Cozy,
        type_scale: TypeScale::Small,
        radius_scale: RadiusScale::Round,
        font_family: "Georgia".into(),
        ..ThemeSettings::default()
    };
    settings.select(OBSIDIAN_ZINC_ID);
    settings.duplicate_active("Mine".into());
    settings.set_color(PaletteColorKey::Accent, gpui_kit::rgb(0xff00ff));
    db.save_theme_settings(&settings).expect("save");

    let loaded = db.load_theme_settings();
    assert_eq!(loaded, settings);
    assert_eq!(loaded.tokens(), settings.tokens());
    assert_eq!(loaded.tokens().accent_brand, gpui_kit::rgb(0xff00ff));
}

#[test]
fn unreadable_theme_rows_fall_back_to_defaults() {
    let db = Database::in_memory().expect("in-memory db");
    db.set_setting(SettingKey::ThemeOverrides, "{ not json").unwrap();
    db.set_setting(SettingKey::ThemeCustomPalettes, "[42]").unwrap();
    db.set_setting(SettingKey::ThemeDensity, "enormous").unwrap();
    db.set_setting(SettingKey::ThemeFontFamily, "   ").unwrap();
    assert_eq!(db.load_theme_settings(), ThemeSettings::default());
}

#[test]
fn legacy_custom_tokens_migrate_to_a_custom_palette_once() {
    let db = Database::in_memory().expect("in-memory db");
    let mut legacy = crate::theme::obsidian_zinc();
    legacy.accent_brand = gpui_kit::rgb(0xff00aa);
    let json = serde_json::to_string(&legacy).expect("serialize legacy tokens");
    db.set_setting(SettingKey::LegacyThemeTokens, &json).unwrap();

    let migrated = db.load_theme_settings();
    assert_eq!(migrated.preset_id, MIGRATED_PALETTE_ID);
    assert_eq!(migrated.tokens().accent_brand, gpui_kit::rgb(0xff00aa));
    assert_eq!(migrated.tokens().bg_app, legacy.bg_app);
    assert_eq!(db.get_setting(SettingKey::LegacyThemeTokens).unwrap(), None, "legacy row removed");
    assert_eq!(db.load_theme_settings(), migrated, "migration result is persisted");
}

#[test]
fn legacy_preset_tokens_and_corrupt_rows_migrate_cleanly() {
    let db = Database::in_memory().expect("in-memory db");
    let json = serde_json::to_string(&crate::theme::minimalist_dark()).expect("serialize");
    db.set_setting(SettingKey::LegacyThemeTokens, &json).unwrap();
    let migrated = db.load_theme_settings();
    assert_eq!(migrated.preset_id, DEFAULT_PALETTE_ID);
    assert!(migrated.custom.is_empty());

    db.set_setting(SettingKey::LegacyThemeTokens, "{ invalid json }").unwrap();
    assert_eq!(db.load_theme_settings(), ThemeSettings::default());
    assert_eq!(db.get_setting(SettingKey::LegacyThemeTokens).unwrap(), None);
}

#[test]
fn dismissed_seeds_accumulate_and_tolerate_a_corrupt_row() {
    use crate::domain::ProfileId;
    let db = Database::in_memory().expect("in-memory db");
    assert!(db.dismissed_seeds().unwrap().is_empty());
    let (a, b) = (ProfileId::new("agent_a"), ProfileId::new("agent_b"));
    db.dismiss_seed(&a).unwrap();
    db.dismiss_seed(&b).unwrap();
    db.dismiss_seed(&a).unwrap();
    assert_eq!(db.dismissed_seeds().unwrap().into_iter().collect::<Vec<_>>(), vec![a.clone(), b]);

    db.set_setting(SettingKey::DismissedSeeds, "{not json").unwrap();
    assert!(db.dismissed_seeds().unwrap().is_empty(), "a corrupt list reads as empty");
    db.dismiss_seed(&a).unwrap();
    assert_eq!(db.dismissed_seeds().unwrap().len(), 1);
}
