//! A menu opened by state (a command or a shortcut as well as its trigger), not only by a
//! dropdown's own click: the owner keeps an [`OpenMenu`], opens it with a spec, renders its
//! view where it belongs, and hears when the kit dismisses it (an item, Escape, a click
//! outside). The kit menu brings keyboard navigation, scrolling and ticks.

use super::render::render_menu;
use super::spec::MenuSpec;
use gpui_kit::component::menu::PopupMenu;
use gpui_kit::{Context, DismissEvent, Entity, Focusable, Subscription, Window};

/// The open menu of one place, if any.
#[derive(Default)]
pub struct OpenMenu {
    menu: Option<Entity<PopupMenu>>,
    _dismiss: Option<Subscription>,
}

impl OpenMenu {
    pub fn is_open(&self) -> bool {
        self.menu.is_some()
    }

    /// The menu to draw while it is open.
    pub fn view(&self) -> Option<Entity<PopupMenu>> {
        self.menu.clone()
    }

    pub fn close(&mut self) {
        (self.menu, self._dismiss) = (None, None);
    }

    /// Opens `spec`, focused for the keyboard; `on_dismiss` runs on the owner when the kit
    /// closes it.
    pub fn open<V: 'static>(&mut self, spec: MenuSpec, window: &mut Window, cx: &mut Context<V>, on_dismiss: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static) {
        let menu = PopupMenu::build(window, cx, move |menu, window, cx| render_menu(menu, &spec, window, cx));
        menu.focus_handle(cx).focus(window, cx);
        self._dismiss = Some(cx.subscribe_in(&menu, window, move |owner, _, _: &DismissEvent, window, cx| on_dismiss(owner, window, cx)));
        self.menu = Some(menu);
    }
}
