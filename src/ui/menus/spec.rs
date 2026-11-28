//! A menu: sections (each optionally labelled) of entries, plus how the popup is sized.

use super::entry::MenuEntry;
use super::text::MenuText;
use gpui_kit::{FocusHandle, Pixels};

/// One group of entries, set apart from the next by a separator.
#[derive(Clone, Default)]
pub struct MenuSection {
    pub label: Option<MenuText>,
    pub entries: Vec<MenuEntry>,
}

impl MenuSection {
    pub fn new() -> Self {
        Self::default()
    }

    /// A section headed by the translation of `key`.
    pub fn labelled(key: &'static str) -> Self {
        Self { label: Some(MenuText::key(key)), entries: Vec::new() }
    }

    /// Adds `entry`.
    pub fn entry(mut self, entry: impl Into<MenuEntry>) -> Self {
        let entry = entry.into();
        if entry.is_shown() {
            self.entries.push(entry);
        }
        self
    }

    /// Adds `entry` when there is one (the visibility predicate of the builders).
    pub fn entry_opt(self, entry: Option<impl Into<MenuEntry>>) -> Self {
        match entry {
            Some(entry) => self.entry(entry),
            None => self,
        }
    }

    /// Adds every entry of `entries`.
    pub fn entries(self, entries: impl IntoIterator<Item = impl Into<MenuEntry>>) -> Self {
        entries.into_iter().fold(self, Self::entry)
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// A whole menu (the top level, or one submenu).
#[derive(Clone, Default)]
pub struct MenuSpec {
    pub sections: Vec<MenuSection>,
    pub min_width: Option<Pixels>,
    /// Scrolls past the kit's default height (half the window) instead of growing.
    pub scrollable: bool,
    /// Where the items' actions are dispatched and their key hints resolved.
    pub action_context: Option<FocusHandle>,
}

impl MenuSpec {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds `section` unless it is empty.
    pub fn section(mut self, section: MenuSection) -> Self {
        if !section.is_empty() {
            self.sections.push(section);
        }
        self
    }

    /// A menu of one unlabelled section.
    pub fn of(entries: impl IntoIterator<Item = impl Into<MenuEntry>>) -> Self {
        Self::new().section(MenuSection::new().entries(entries))
    }

    pub fn min_width(mut self, width: Pixels) -> Self {
        self.min_width = Some(width);
        self
    }

    pub fn scrollable(mut self) -> Self {
        self.scrollable = true;
        self
    }

    pub fn action_context(mut self, focus: FocusHandle) -> Self {
        self.action_context = Some(focus);
        self
    }

    pub fn is_empty(&self) -> bool {
        self.sections.iter().all(MenuSection::is_empty)
    }

    /// Every entry, top to bottom.
    pub fn entries(&self) -> impl Iterator<Item = &MenuEntry> {
        self.sections.iter().flat_map(|section| section.entries.iter())
    }
}
