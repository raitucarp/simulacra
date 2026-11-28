//! The entries of a menu: an item that does something, or a submenu that opens another menu.

use super::item::MenuItem;
use super::spec::MenuSpec;
use super::text::MenuText;
use crate::ui::icons::LucideIcon;
use gpui_kit::SharedString;
use std::rc::Rc;

/// A row opening a nested menu.
#[derive(Clone)]
pub struct Submenu {
    pub id: SharedString,
    pub label: MenuText,
    pub icon: Option<LucideIcon>,
    pub destructive: bool,
    pub spec: Rc<MenuSpec>,
}

impl Submenu {
    pub fn new(id: impl Into<SharedString>, label: MenuText, spec: MenuSpec) -> Self {
        Self { id: id.into(), label, icon: None, destructive: false, spec: Rc::new(spec) }
    }

    pub fn icon(mut self, icon: LucideIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }
}

/// An item or a submenu.
#[derive(Clone)]
pub enum MenuEntry {
    Item(MenuItem),
    Submenu(Submenu),
}

impl MenuEntry {
    pub fn id(&self) -> &SharedString {
        match self {
            Self::Item(item) => &item.id,
            Self::Submenu(submenu) => &submenu.id,
        }
    }

    pub fn label(&self) -> &MenuText {
        match self {
            Self::Item(item) => &item.label,
            Self::Submenu(submenu) => &submenu.label,
        }
    }

    pub fn is_destructive(&self) -> bool {
        match self {
            Self::Item(item) => item.destructive,
            Self::Submenu(submenu) => submenu.destructive,
        }
    }

    /// Whether the entry shows anything (a submenu with no entries is left out).
    pub fn is_shown(&self) -> bool {
        match self {
            Self::Item(_) => true,
            Self::Submenu(submenu) => !submenu.spec.is_empty(),
        }
    }
}

impl From<MenuItem> for MenuEntry {
    fn from(item: MenuItem) -> Self {
        Self::Item(item)
    }
}

impl From<Submenu> for MenuEntry {
    fn from(submenu: Submenu) -> Self {
        Self::Submenu(submenu)
    }
}
