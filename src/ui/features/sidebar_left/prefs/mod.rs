//! [`SidebarPrefs`]: the sidebar's persisted choices (width preference, collapsed sections,
//! pins and recently opened items) as immutable data with pure updates, plus the edge
//! functions that load and save it.

mod persist;
mod refs;

#[cfg(test)]
mod tests;

pub use persist::{load_prefs, save_prefs};
pub use refs::{NavRef, SidebarSection};

use super::mode::SidebarPreference;
use serde::{Deserialize, Serialize};

/// Most pins kept.
pub const PIN_CAPACITY: usize = 24;
/// Most recently opened items remembered.
pub const RECENT_CAPACITY: usize = 12;

/// Everything the sidebar remembers between sessions.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SidebarPrefs {
    pub preference: SidebarPreference,
    pub collapsed_sections: Vec<SidebarSection>,
    /// Pinned items, newest first.
    pub pins: Vec<NavRef>,
    /// Recently opened items, most recent first.
    pub recents: Vec<NavRef>,
}

impl SidebarPrefs {
    pub fn with_preference(&self, preference: SidebarPreference) -> Self {
        Self { preference, ..self.clone() }
    }

    pub fn is_collapsed(&self, section: SidebarSection) -> bool {
        self.collapsed_sections.contains(&section)
    }

    /// The prefs with `section` folded or unfolded.
    pub fn with_section_toggled(&self, section: SidebarSection) -> Self {
        let collapsed_sections = if self.is_collapsed(section) {
            self.collapsed_sections.iter().copied().filter(|other| *other != section).collect()
        } else {
            self.collapsed_sections.iter().copied().chain([section]).collect()
        };
        Self { collapsed_sections, ..self.clone() }
    }

    pub fn is_pinned(&self, item: &NavRef) -> bool {
        self.pins.contains(item)
    }

    /// The prefs with `item` unpinned, or pinned at the top.
    pub fn with_pin_toggled(&self, item: &NavRef) -> Self {
        let pins = if self.is_pinned(item) {
            self.pins.iter().filter(|pin| *pin != item).cloned().collect()
        } else {
            front_capped(item, &self.pins, PIN_CAPACITY)
        };
        Self { pins, ..self.clone() }
    }

    /// The prefs with `item` as the most recently opened.
    pub fn with_recent(&self, item: &NavRef) -> Self {
        Self { recents: front_capped(item, &self.recents, RECENT_CAPACITY), ..self.clone() }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    /// Parses stored JSON; anything unreadable starts from the defaults.
    pub fn from_json(raw: &str) -> Self {
        serde_json::from_str(raw).unwrap_or_default()
    }
}

/// `item` followed by `list` without it, at most `capacity` long.
fn front_capped(item: &NavRef, list: &[NavRef], capacity: usize) -> Vec<NavRef> {
    std::iter::once(item.clone()).chain(list.iter().filter(|other| *other != item).cloned()).take(capacity).collect()
}
