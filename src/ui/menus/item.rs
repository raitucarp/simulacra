//! A menu item: one clickable row.

use super::text::MenuText;
use crate::ui::icons::LucideIcon;
use gpui_kit::{Action, App, SharedString, Window};
use std::rc::Rc;

/// What clicking an item does.
pub type MenuHandler = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

/// One clickable row.
#[derive(Clone)]
pub struct MenuItem {
    /// Stable identifier of the action (a family of items, one per collection, shares one).
    pub id: SharedString,
    pub label: MenuText,
    /// A second, muted line under the label ("3 docs · 2 agents").
    pub detail: Option<MenuText>,
    pub icon: Option<LucideIcon>,
    /// The action whose shortcut the row shows; it runs when there is no handler.
    pub action: Option<Rc<dyn Action>>,
    /// `Some` for a choice: ticked (on the right) when `true`.
    pub checked: Option<bool>,
    /// Shown greyed out with this reason instead of being hidden.
    pub disabled: Option<MenuText>,
    /// Red, and always in the menu's last section.
    pub destructive: bool,
    pub on_click: Option<MenuHandler>,
}

impl MenuItem {
    pub fn new(id: impl Into<SharedString>, label: MenuText) -> Self {
        Self { id: id.into(), label, detail: None, icon: None, action: None, checked: None, disabled: None, destructive: false, on_click: None }
    }

    pub fn icon(mut self, icon: LucideIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn detail(mut self, detail: MenuText) -> Self {
        self.detail = Some(detail);
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    pub fn handler(mut self, handler: MenuHandler) -> Self {
        self.on_click = Some(handler);
        self
    }

    pub fn action(mut self, action: Box<dyn Action>) -> Self {
        self.action = Some(Rc::from(action));
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    pub fn disabled_because(mut self, reason: MenuText) -> Self {
        self.disabled = Some(reason);
        self
    }

    /// A row that only says something (greyed out, no reason line).
    pub fn inert(mut self) -> Self {
        self.disabled = Some(MenuText::plain(""));
        self
    }

    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }
}
