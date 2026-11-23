//! Settings › Code (P4): off by default, a clamped timeout, runtimes switched off one by one,
//! and never switched on by an imported export.

use crate::domain::code::Runtime;
use crate::domain::data::export::SETTINGS_ALLOW_LIST;
use crate::domain::preferences::{CodeField, CodePrefs, Preferences, SettingField, SettingValue, RUN_TIMEOUT_SECONDS};
use crate::storage::SettingKey;

#[test]
fn running_code_is_off_by_default_and_nothing_is_allowed() {
    let prefs = Preferences::default();
    assert!(!prefs.code.enabled, "the master switch starts off");
    assert!(Runtime::ALL.iter().all(|runtime| !prefs.code.allows(*runtime)));
    assert_eq!(serde_json::from_str::<CodePrefs>("{}").expect("empty row"), CodePrefs::default(), "a missing row reads as off");
}

#[test]
fn runtimes_switch_off_one_by_one_once_running_is_on() {
    let on = CodePrefs { enabled: true, ..CodePrefs::default() };
    assert!(Runtime::ALL.iter().all(|runtime| on.allows(*runtime)), "every runtime is on with the master switch");
    let no_go = on.clone().with_runtime(Runtime::Go, false).with_runtime(Runtime::Go, false);
    assert!(!no_go.allows(Runtime::Go) && no_go.allows(Runtime::Python));
    assert_eq!(no_go.disabled_runtimes, vec!["go".to_string()]);
    assert!(no_go.clone().with_runtime(Runtime::Go, true).allows(Runtime::Go));
    let off = CodePrefs { enabled: false, ..no_go };
    assert!(!off.allows(Runtime::Python), "the master switch wins");
}

#[test]
fn the_timeout_is_clamped_and_edited_as_a_field() {
    let wild = CodePrefs { timeout_seconds: 99_999, disabled_runtimes: vec!["lua".into(), "lua".into(), "cobol".into()], ..CodePrefs::default() }.validated();
    assert_eq!(wild.timeout_seconds as i64, RUN_TIMEOUT_SECONDS.max);
    assert_eq!(wild.disabled_runtimes, vec!["lua".to_string(), "cobol".to_string()], "unknown keys are kept, once");
    let field = SettingField::Code(CodeField::Timeout);
    let prefs = field.set(Preferences::default(), &SettingValue::Number(30));
    assert_eq!(prefs.code.timeout_seconds, 30);
    let enabled = SettingField::Code(CodeField::Enabled).set(Preferences::default(), &SettingValue::Toggle(true));
    assert!(enabled.code.enabled);
}

#[test]
fn an_export_never_carries_the_code_switches() {
    assert!(!SETTINGS_ALLOW_LIST.contains(&SettingKey::PrefsCode), "an imported file cannot switch code running on");
}
