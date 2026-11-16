//! The settings chrome is translated: tab labels, titles and descriptions resolve from the
//! locale files (the phase 5 keys are flat dotted keys appended after the nested blocks).

use super::SettingsTab;
use rust_i18n::t;

const LOCALES: [&str; 2] = ["en", "id"];

#[test]
fn tab_texts_resolve_in_every_locale() {
    for locale in LOCALES {
        for tab in SettingsTab::ALL {
            for key in [tab.label_key(), tab.title_key(), tab.description_key()] {
                let text = t!(key, locale = locale).to_string();
                assert_ne!(text, key, "{locale}: unresolved {key}");
                assert!(!text.contains(key), "{locale}: unresolved {key}");
            }
        }
    }
}

#[test]
fn flat_keys_resolve_next_to_nested_blocks() {
    assert_eq!(t!("settings.cancel", locale = "id"), "Batal");
    assert_eq!(t!("settings.title", locale = "en"), "Simulacra — System Preferences");
    assert_eq!(t!("feed.window_summary", locale = "en", shown = 3, total = 9), "Showing 3 of 9 thoughts");
}
