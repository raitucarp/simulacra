//! N7–N10: the items of the user's own post in its "…" menu: re-pin a drifted quote (N7).

use super::events::DataEvent;
use super::post_menu::{item, DataControls};
use super::quote_sheet::QuoteEvent;
use super::text::tr;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::menu::PopupMenu;
use gpui_kit::*;

/// `menu` with the N7–N10 items of the user's own post.
pub(super) fn with_idea_items(menu: PopupMenu, controls: &DataControls, _: &mut Window, _: &mut Context<PopupMenu>) -> PopupMenu {
    let post = &controls.post_id;
    let mut menu = menu;
    if controls.facts.repinnable {
        menu = menu.item(item(tr("posts.pinned.repin"), LucideIcon::Quote, DataEvent::Quote(QuoteEvent::Repin(post.clone())), &controls.on_event));
    }
    menu
}
