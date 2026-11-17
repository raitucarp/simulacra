//! The sidebar's two widths and the pure rule that picks one from the window width and the
//! user's choice.

use gpui_kit::Pixels;
use serde::{Deserialize, Serialize};

/// What the user chose with the collapse button or Ctrl+B (persisted).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SidebarPreference {
    /// Expanded on wide windows, the icon rail below the breakpoint.
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

/// The mode for a window `window_width` wide; `breakpoint` is the theme's
/// `left_sidebar_rail_breakpoint` (narrower windows fold `Auto` into the rail).
pub fn sidebar_mode(window_width: Pixels, user_pref: SidebarPreference, breakpoint: Pixels) -> SidebarMode {
    match user_pref {
        SidebarPreference::Rail => SidebarMode::Rail,
        SidebarPreference::Expanded => SidebarMode::Expanded,
        SidebarPreference::Auto if window_width < breakpoint => SidebarMode::Rail,
        SidebarPreference::Auto => SidebarMode::Expanded,
    }
}

/// The preference after the toggle: collapse what is expanded; expand what is collapsed,
/// pinning it open only when the window is narrow (otherwise back to automatic).
pub fn toggled_preference(window_width: Pixels, user_pref: SidebarPreference, breakpoint: Pixels) -> SidebarPreference {
    match sidebar_mode(window_width, user_pref, breakpoint) {
        SidebarMode::Expanded => SidebarPreference::Rail,
        SidebarMode::Rail if window_width < breakpoint => SidebarPreference::Expanded,
        SidebarMode::Rail => SidebarPreference::Auto,
    }
}
