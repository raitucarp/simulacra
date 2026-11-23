//! N16–N20: a post's "…" menu items: the lens fan (any post), and defining a term in it or
//! taking the definition back (the user's own). None on a locked post.

use super::events::DataEvent;
use super::glossary_sheet::GlossaryEvent;
use super::lens_sheet::LensEvent;
use super::post_menu::{item, DataControls};
use super::text::tr;
use super::thought_events::ThoughtEvent;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::menu::PopupMenu;
use gpui_kit::*;
use rust_i18n::t;

fn thought(event: ThoughtEvent) -> DataEvent {
    DataEvent::Thought(event)
}

/// N17: define a term in the user's post, or stop defining it.
fn own_items(menu: PopupMenu, controls: &DataControls) -> PopupMenu {
    let (post, on_event) = (&controls.post_id, &controls.on_event);
    let menu = menu.item(item(tr("posts.glossary.menu"), LucideIcon::BookOpen, thought(ThoughtEvent::Glossary(GlossaryEvent::Open(post.clone()))), on_event));
    match &controls.facts.defines {
        Some(term) => menu.item(item(t!("posts.glossary.undefine", term = term.clone()).to_string(), LucideIcon::X, thought(ThoughtEvent::Glossary(GlossaryEvent::Undefine(post.clone()))), on_event)),
        None => menu,
    }
}

/// `menu` with the N16–N20 items of the post.
pub(super) fn with_thought_items(menu: PopupMenu, controls: &DataControls, _window: &mut Window, _cx: &mut Context<PopupMenu>) -> PopupMenu {
    let (post, on_event) = (&controls.post_id, &controls.on_event);
    if controls.locked {
        return menu;
    }
    let menu = menu.item(item(tr("posts.lens.menu"), LucideIcon::Layers, thought(ThoughtEvent::Lens(LensEvent::Open(post.clone()))), on_event)); // N16
    if controls.own { own_items(menu, controls) } else { menu }
}
