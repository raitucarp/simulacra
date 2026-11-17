use super::section::{visible_count, SECTION_PREVIEW_LIMIT};
use super::*;
use gpui_kit::px;

const BREAKPOINT: f32 = 1100.0;

fn mode(width: f32, pref: SidebarPreference) -> SidebarMode {
    sidebar_mode(px(width), pref, px(BREAKPOINT))
}

fn toggled(width: f32, pref: SidebarPreference) -> SidebarPreference {
    toggled_preference(px(width), pref, px(BREAKPOINT))
}

#[test]
fn auto_folds_into_the_rail_below_the_breakpoint() {
    assert_eq!(mode(1400.0, SidebarPreference::Auto), SidebarMode::Expanded);
    assert_eq!(mode(BREAKPOINT, SidebarPreference::Auto), SidebarMode::Expanded, "the breakpoint itself is wide");
    assert_eq!(mode(900.0, SidebarPreference::Auto), SidebarMode::Rail);
}

#[test]
fn explicit_choices_win_over_the_window_width() {
    assert_eq!(mode(1400.0, SidebarPreference::Rail), SidebarMode::Rail);
    assert_eq!(mode(900.0, SidebarPreference::Expanded), SidebarMode::Expanded);
}

#[test]
fn toggling_always_flips_what_is_on_screen() {
    for width in [900.0, 1400.0] {
        for pref in [SidebarPreference::Auto, SidebarPreference::Expanded, SidebarPreference::Rail] {
            let next = toggled(width, pref);
            assert_ne!(mode(width, next), mode(width, pref), "toggle at {width} from {pref:?} did nothing");
        }
    }
}

#[test]
fn expanding_on_a_wide_window_returns_to_automatic() {
    assert_eq!(toggled(1400.0, SidebarPreference::Rail), SidebarPreference::Auto);
    assert_eq!(toggled(900.0, SidebarPreference::Auto), SidebarPreference::Expanded);
    assert_eq!(toggled(900.0, SidebarPreference::Expanded), SidebarPreference::Rail);
}

#[test]
fn sections_preview_a_few_rows_until_show_all() {
    assert_eq!(visible_count(3, false), 3);
    assert_eq!(visible_count(SECTION_PREVIEW_LIMIT + 4, false), SECTION_PREVIEW_LIMIT);
    assert_eq!(visible_count(SECTION_PREVIEW_LIMIT + 4, true), SECTION_PREVIEW_LIMIT + 4);
}

#[test]
fn show_all_is_toggled_per_section() {
    let state = SidebarState { prefs: SidebarPrefs::default(), window_width: px(BREAKPOINT), showing_all: Vec::new() };
    let shown = state.with_show_all_toggled(SidebarSection::Members);
    assert!(shown.is_showing_all(SidebarSection::Members));
    assert!(!shown.is_showing_all(SidebarSection::Sources));
    assert!(!shown.with_show_all_toggled(SidebarSection::Members).is_showing_all(SidebarSection::Members));
}

#[test]
fn section_titles_and_hints_are_translated() {
    for section in SidebarSection::ALL {
        for key in [section.title_key(), section.empty_key()] {
            assert_ne!(rust_i18n::t!(key, locale = "en"), key);
        }
    }
}
