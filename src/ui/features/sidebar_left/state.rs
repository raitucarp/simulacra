//! [`SidebarState`]: the persisted prefs plus this session's window width and the sections
//! the user unfolded with "Show all".

use super::mode::{sidebar_mode, SidebarMode};
use super::prefs::{load_prefs, save_prefs, NavRef, SidebarPrefs, SidebarSection};
use crate::storage::Vault;
use gpui_kit::Pixels;

/// The sidebar's state as the shell holds it.
#[derive(Debug, Clone, PartialEq)]
pub struct SidebarState {
    pub prefs: SidebarPrefs,
    /// Current window width (follows resizes).
    pub window_width: Pixels,
    /// Sections showing every item instead of the first few (this session only).
    pub showing_all: Vec<SidebarSection>,
}

impl SidebarState {
    /// The stored prefs for a window `window_width` wide.
    pub fn load(vault: Option<&Vault>, window_width: Pixels) -> Self {
        Self { prefs: load_prefs(vault), window_width, showing_all: Vec::new() }
    }

    /// The width to render at, given the theme's rail breakpoint.
    pub fn mode(&self, breakpoint: Pixels) -> SidebarMode {
        sidebar_mode(self.window_width, self.prefs.preference, breakpoint)
    }

    pub fn is_showing_all(&self, section: SidebarSection) -> bool {
        self.showing_all.contains(&section)
    }

    /// The state with "Show all" of `section` flipped.
    pub fn with_show_all_toggled(&self, section: SidebarSection) -> Self {
        let showing_all = if self.is_showing_all(section) {
            self.showing_all.iter().copied().filter(|other| *other != section).collect()
        } else {
            self.showing_all.iter().copied().chain([section]).collect()
        };
        Self { showing_all, ..self.clone() }
    }

    /// Replaces the prefs and saves them when they changed.
    pub fn update_prefs(&mut self, prefs: SidebarPrefs, vault: Option<&Vault>) {
        if prefs != self.prefs {
            save_prefs(vault, &prefs);
            self.prefs = prefs;
        }
    }

    /// Remembers `item` as the most recently opened.
    pub fn note_recent(&mut self, item: NavRef, vault: Option<&Vault>) {
        let prefs = self.prefs.with_recent(&item);
        self.update_prefs(prefs, vault);
    }
}
