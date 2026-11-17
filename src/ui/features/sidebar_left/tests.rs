use super::section::{visible_count, SECTION_PREVIEW_LIMIT};
use super::*;
use gpui_kit::px;

use crate::theme::ThemeTokens;
use crate::ui::responsive::LayoutClass;

const LAYOUTS: [LayoutClass; 3] = [LayoutClass::Compact, LayoutClass::Medium, LayoutClass::Wide];
const PREFERENCES: [SidebarPreference; 3] = [SidebarPreference::Auto, SidebarPreference::Expanded, SidebarPreference::Rail];

#[test]
fn auto_folds_into_the_rail_outside_the_wide_layout() {
    assert_eq!(sidebar_mode_for(LayoutClass::Wide, SidebarPreference::Auto), SidebarMode::Expanded);
    assert_eq!(sidebar_mode_for(LayoutClass::Medium, SidebarPreference::Auto), SidebarMode::Rail);
    assert_eq!(sidebar_mode_for(LayoutClass::Compact, SidebarPreference::Auto), SidebarMode::Rail);
}

#[test]
fn window_widths_use_the_shared_layout_breakpoints() {
    let theme = ThemeTokens::default();
    let wide = theme.layout_wide_min_width;
    assert_eq!(sidebar_mode(wide, SidebarPreference::Auto, &theme), SidebarMode::Expanded, "the breakpoint itself is wide");
    assert_eq!(sidebar_mode(wide - px(1.0), SidebarPreference::Auto, &theme), SidebarMode::Rail);
}

#[test]
fn explicit_choices_win_over_the_layout() {
    assert_eq!(sidebar_mode_for(LayoutClass::Wide, SidebarPreference::Rail), SidebarMode::Rail);
    assert_eq!(sidebar_mode_for(LayoutClass::Compact, SidebarPreference::Expanded), SidebarMode::Expanded);
}

#[test]
fn toggling_always_flips_what_is_on_screen() {
    for layout in LAYOUTS {
        for pref in PREFERENCES {
            let next = toggled_preference(layout, pref);
            assert_ne!(sidebar_mode_for(layout, next), sidebar_mode_for(layout, pref), "toggle in {layout:?} from {pref:?} did nothing");
        }
    }
}

#[test]
fn expanding_in_the_wide_layout_returns_to_automatic() {
    assert_eq!(toggled_preference(LayoutClass::Wide, SidebarPreference::Rail), SidebarPreference::Auto);
    assert_eq!(toggled_preference(LayoutClass::Medium, SidebarPreference::Auto), SidebarPreference::Expanded);
    assert_eq!(toggled_preference(LayoutClass::Compact, SidebarPreference::Expanded), SidebarPreference::Rail);
}

#[test]
fn sections_preview_a_few_rows_until_show_all() {
    assert_eq!(visible_count(3, false), 3);
    assert_eq!(visible_count(SECTION_PREVIEW_LIMIT + 4, false), SECTION_PREVIEW_LIMIT);
    assert_eq!(visible_count(SECTION_PREVIEW_LIMIT + 4, true), SECTION_PREVIEW_LIMIT + 4);
}

#[test]
fn show_all_is_toggled_per_section() {
    let state = SidebarState { prefs: SidebarPrefs::default(), window_width: px(0.0), showing_all: Vec::new() };
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
