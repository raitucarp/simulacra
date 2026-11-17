//! The settings window fits inside the app window, and the chrome texts resolve.

use super::layout::dialog_size;
use super::SettingsTab;
use crate::theme::ThemeTokens;
use gpui_kit::{px, size};
use rust_i18n::t;

#[test]
fn the_dialog_always_fits_inside_the_window() {
    let theme = ThemeTokens::default();
    let inset = theme.settings_modal_inset + theme.settings_modal_inset;
    for (width, height) in [(1920.0, 1080.0), (1280.0, 720.0), (800.0, 600.0), (420.0, 360.0), (10.0, 10.0)] {
        let (w, h) = dialog_size(size(px(width), px(height)), &theme);
        assert!(w <= theme.settings_modal_max_width && h <= theme.settings_modal_max_height);
        assert!(w + inset <= px(width).max(inset) && h + inset <= px(height).max(inset), "{width}x{height} overflows");
        assert!(w >= px(0.0) && h >= px(0.0));
    }
    let (w, h) = dialog_size(size(px(4000.0), px(3000.0)), &theme);
    assert_eq!((w, h), (theme.settings_modal_max_width, theme.settings_modal_max_height), "large windows cap the size");
}

#[test]
fn long_paths_keep_their_start_and_end() {
    use super::controls::path::elide_middle;
    assert_eq!(elide_middle("C:/short", 20, 4), "C:/short");
    let long = "C:/Users/someone/AppData/Roaming/simulacra/simulacra.db";
    let shown = elide_middle(long, 24, 6);
    assert_eq!(shown.chars().count(), 24);
    assert!(shown.starts_with("C:/Use") && shown.ends_with("simulacra.db") && shown.contains('…'));
}

#[test]
fn chrome_texts_resolve_next_to_nested_blocks() {
    assert_eq!(t!("settings.save_apply", locale = "en"), "Save & Apply Settings");
    for tab in SettingsTab::ALL {
        let key = tab.text_key("title");
        assert_ne!(t!(key.as_str(), locale = "id"), key);
    }
}
