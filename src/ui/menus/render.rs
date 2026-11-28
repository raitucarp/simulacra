//! The one renderer from a [`MenuSpec`] to the kit's `PopupMenu`, shared by the "…"
//! dropdowns and the right-click context menus. Ticks always sit on the right, so an item
//! keeps its own icon while ticked (one convention app-wide).

use super::entry::{MenuEntry, Submenu};
use super::item::MenuItem;
use super::rows::{destructive_row, detail_row, disabled_row};
use super::spec::MenuSpec;
use gpui_kit::component::menu::{PopupMenu, PopupMenuItem};
use gpui_kit::component::{ActiveTheme, Icon, Side};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{Context, Styled, Window};

/// `menu` filled from `spec`: its sections apart by separators, each under its label.
pub fn render_menu(menu: PopupMenu, spec: &MenuSpec, window: &mut Window, cx: &mut Context<PopupMenu>) -> PopupMenu {
    let mut menu = menu.check_side(Side::Right).scrollable(spec.scrollable);
    if let Some(width) = spec.min_width {
        menu = menu.min_w(width);
    }
    if let Some(focus) = &spec.action_context {
        menu = menu.action_context(focus.clone());
    }
    for (index, section) in spec.sections.iter().filter(|section| !section.is_empty()).enumerate() {
        menu = if index == 0 { menu } else { menu.separator() };
        menu = match &section.label {
            Some(label) => menu.label(label.text.clone()),
            None => menu,
        };
        menu = section.entries.iter().fold(menu, |menu, entry| render_entry(menu, entry, window, cx));
    }
    menu
}

/// A builder for `ContextMenuExt::context_menu` (or a dropdown) that builds `spec` when the
/// menu opens.
pub fn context_menu_of(spec: impl Fn() -> MenuSpec + 'static) -> impl Fn(PopupMenu, &mut Window, &mut Context<PopupMenu>) -> PopupMenu + 'static {
    move |menu, window, cx| render_menu(menu, &spec(), window, cx)
}

fn render_entry(menu: PopupMenu, entry: &MenuEntry, window: &mut Window, cx: &mut Context<PopupMenu>) -> PopupMenu {
    match entry {
        MenuEntry::Item(item) => {
            let row = item_row(item, cx);
            menu.item(row)
        }
        MenuEntry::Submenu(submenu) => render_submenu(menu, submenu, window, cx),
    }
}

fn render_submenu(menu: PopupMenu, submenu: &Submenu, window: &mut Window, cx: &mut Context<PopupMenu>) -> PopupMenu {
    let danger = cx.theme().danger;
    let icon = submenu.icon.map(|icon| Icon::new(icon).when(submenu.destructive, |icon| icon.text_color(danger)));
    let spec = submenu.spec.clone();
    menu.submenu_with_icon(icon, submenu.label.text.clone(), window, cx, move |menu, window, cx| render_menu(menu, &spec, window, cx))
}

fn item_row(item: &MenuItem, cx: &mut Context<PopupMenu>) -> PopupMenuItem {
    let danger = cx.theme().danger;
    let row = match (&item.disabled, item.destructive, &item.detail) {
        (Some(reason), _, _) => disabled_row(&item.label, reason).disabled(true),
        (None, true, _) => destructive_row(&item.label),
        (None, false, Some(detail)) => detail_row(&item.label, detail),
        (None, false, None) => PopupMenuItem::new(item.label.text.clone()),
    };
    let row = match item.icon {
        Some(icon) => row.icon(Icon::new(icon).when(item.destructive, |icon| icon.text_color(danger))),
        None => row,
    };
    let row = match &item.action {
        Some(action) => row.action(action.boxed_clone()),
        None => row,
    };
    let row = row.checked(item.checked.unwrap_or(false));
    match item.on_click.clone() {
        Some(handler) => row.on_click(move |_, window, cx| handler(window, cx)),
        None => row,
    }
}
