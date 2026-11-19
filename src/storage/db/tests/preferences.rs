use crate::domain::preferences::{Preferences, StartupScreen};
use crate::storage::db::{preference_rows, Database, SettingKey, PREFERENCE_KEYS};

#[test]
fn a_fresh_vault_migrates_the_legacy_scheduler_switch_once() {
    let db = Database::in_memory().expect("in-memory db");
    db.set_setting(SettingKey::SchedulerEnabled, "true").unwrap();
    let prefs = db.load_preferences();
    assert!(prefs.agents.autonomous_posting, "the legacy value survives");
    assert_eq!(db.get_setting(SettingKey::SchedulerEnabled).unwrap(), None, "legacy row removed");
    assert!(PREFERENCE_KEYS.iter().all(|key| db.get_setting(*key).unwrap().is_some()), "migration is persisted");
    assert_eq!(db.load_preferences(), prefs);
}

#[test]
fn an_empty_vault_loads_the_defaults() {
    let db = Database::in_memory().expect("in-memory db");
    assert_eq!(db.load_preferences(), Preferences::default());
}

#[test]
fn preferences_round_trip_through_the_settings_table() {
    let db = Database::in_memory().expect("in-memory db");
    let mut prefs = Preferences::default();
    prefs.general.startup_screen = StartupScreen::Knowledge;
    prefs.learning.new_cards_per_day = 35;
    prefs.agents.quiet_hours = true;
    db.save_preferences(&prefs).expect("save");
    assert_eq!(db.load_preferences(), prefs);
    assert_eq!(preference_rows(&prefs).unwrap().len(), PREFERENCE_KEYS.len());
}

#[test]
fn an_unreadable_row_only_resets_its_category() {
    let db = Database::in_memory().expect("in-memory db");
    let mut prefs = Preferences::default();
    prefs.general.startup_screen = StartupScreen::MindGraph;
    prefs.learning.daily_review_limit = 600;
    db.save_preferences(&prefs).expect("save");
    db.set_setting(SettingKey::PrefsLearning, "{ not json").unwrap();
    db.set_setting(SettingKey::PrefsAgents, r#"{"max_concurrent": 99}"#).unwrap();
    let loaded = db.load_preferences();
    assert_eq!(loaded.general, prefs.general);
    assert_eq!(loaded.learning, Preferences::default().learning);
    assert_eq!(loaded.agents.max_concurrent, 4, "stored values are clamped on load");
}

#[test]
fn backup_writes_a_readable_copy() {
    let dir = std::env::temp_dir().join(format!("simulacra-backup-test-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("copy.db");
    let db = Database::in_memory().expect("in-memory db");
    db.set_setting(SettingKey::OnboardingDone, "true").unwrap();
    db.backup_to(&path).expect("backup");
    let copy = Database::open(&path).expect("open copy");
    assert_eq!(copy.get_setting(SettingKey::OnboardingDone).unwrap().as_deref(), Some("true"));
    drop(copy);
    let _ = std::fs::remove_dir_all(&dir);
}
