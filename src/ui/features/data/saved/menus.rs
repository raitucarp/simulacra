//! M1: the right-click menus of the Saved page: a shelf chip (open it, learn it) and a shelf
//! row (open it, edit its note, remove it), each running the page's own events.

use super::super::events::{DataAction, DataEvent, SavedTarget};
use super::{SavedItemView, SavedView};
use crate::ui::icons::LucideIcon;
use crate::ui::menus::{render_menu, MenuItem, MenuSection, MenuSpec, MenuText};
use gpui_kit::component::menu::PopupMenu;
use gpui_kit::{Context, Window};

fn event_item(id: &'static str, key: &'static str, icon: LucideIcon, events: Vec<DataEvent>, on_event: &DataAction) -> MenuItem {
    let on_event = on_event.clone();
    MenuItem::new(id, MenuText::key(key)).icon(icon).on_click(move |window, cx| events.iter().for_each(|event| on_event(event.clone(), window, cx)))
}

/// A shelf chip: open the shelf, or learn it (opening it first).
pub(super) fn shelf_menu(target: &SavedTarget, on_event: &DataAction) -> MenuSpec {
    let open = event_item("saved.shelf.open", "data.saved.open", LucideIcon::ExternalLink, vec![DataEvent::SelectShelf(target.clone())], on_event);
    let learn = event_item("saved.shelf.learn", "data.saved.learn", LucideIcon::GraduationCap, vec![DataEvent::SelectShelf(target.clone()), DataEvent::LearnShelf], on_event);
    MenuSpec::of([open, learn])
}

/// A shelf row: open it, edit its note (collections only), remove it from the shelf.
pub(super) fn item_menu(view: &SavedView, item: &SavedItemView, on_event: &DataAction) -> MenuSpec {
    let open = event_item("saved.item.open", "data.saved.open", LucideIcon::ExternalLink, vec![DataEvent::OpenItem(item.item.clone())], on_event);
    let note = view.reorderable.then(|| event_item("saved.item.note", "menus.saved.edit_note", LucideIcon::Pencil, vec![DataEvent::EditNote(item.item.clone())], on_event));
    let remove = event_item("saved.item.remove", "data.saved.remove", LucideIcon::X, vec![DataEvent::RemoveItem(item.item.clone())], on_event);
    MenuSpec::new().section(MenuSection::new().entry(open).entry_opt(note)).section(MenuSection::new().entry(remove))
}

/// A context-menu builder showing `spec`.
pub(super) fn builder(spec: MenuSpec) -> impl Fn(PopupMenu, &mut Window, &mut Context<PopupMenu>) -> PopupMenu + 'static {
    move |menu, window, cx| render_menu(menu, &spec, window, cx)
}
