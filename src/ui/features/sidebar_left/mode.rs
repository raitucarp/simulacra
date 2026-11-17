//! The sidebar's two widths and the pure rule that picks one from the window's layout class
//! (`crate::ui::responsive::LayoutClass`) and the user's choice.

use crate::theme::ThemeTokens;
use crate::ui::responsive::LayoutClass;
use gpui_kit::Pixels;
use serde::{Deserialize, Serialize};

/// What the user chose with the collapse button or Ctrl+B (persisted).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SidebarPreference {
    /// Expanded in the wide layout, the icon rail in the medium and compact ones.
    #[default]
    Auto,
    /// Expanded even on narrow windows (the user expanded it while narrow).
    Expanded,
    /// Always the icon rail.
    Rail,
}

/// The width the sidebar renders at.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidebarMode {
    /// Icons, labels, counts and the collapsible sections.
    Expanded,
    /// Icons only, with tooltips.
    Rail,
}

/// Whether `Auto` folds into the rail in `layout` (everything narrower than wide).
const fn folds(layout: LayoutClass) -> bool {
    !matches!(layout, LayoutClass::Wide)
}

/// The mode in a window of layout class `layout`.
pub fn sidebar_mode_for(layout: LayoutClass, user_pref: SidebarPreference) -> SidebarMode {
    match user_pref {
        SidebarPreference::Rail => SidebarMode::Rail,
        SidebarPreference::Expanded => SidebarMode::Expanded,
        SidebarPreference::Auto if folds(layout) => SidebarMode::Rail,
        SidebarPreference::Auto => SidebarMode::Expanded,
    }
}

/// The mode for a window `window_width` wide, using the theme's layout breakpoints.
pub fn sidebar_mode(window_width: Pixels, user_pref: SidebarPreference, theme: &ThemeTokens) -> SidebarMode {
    sidebar_mode_for(LayoutClass::for_width(window_width, theme), user_pref)
}

/// The preference after the toggle: collapse what is expanded; expand what is collapsed,
/// pinning it open only when the layout would fold it (otherwise back to automatic).
pub fn toggled_preference(layout: LayoutClass, user_pref: SidebarPreference) -> SidebarPreference {
    match sidebar_mode_for(layout, user_pref) {
        SidebarMode::Expanded => SidebarPreference::Rail,
        SidebarMode::Rail if folds(layout) => SidebarPreference::Expanded,
        SidebarMode::Rail => SidebarPreference::Auto,
    }
}
