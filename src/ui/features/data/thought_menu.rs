//! N16–N20: a post's "…" menu items for the lens fan (any post but a locked one).

use super::events::DataEvent;
use super::lens_sheet::LensEvent;
use super::post_menu::{item, DataControls};
use super::text::tr;
use super::thought_events::ThoughtEvent;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::menu::PopupMenu;
use gpui_kit::*;

/// `menu` with the N16–N20 items of the post.
pub(super) fn with_thought_items(menu: PopupMenu, controls: &DataControls, _window: &mut Window, _cx: &mut Context<PopupMenu>) -> PopupMenu {
    let (post, on_event) = (&controls.post_id, &controls.on_event);
    if controls.locked {
        return menu;
    }
    menu.item(item(tr("posts.lens.menu"), LucideIcon::Layers, DataEvent::Thought(ThoughtEvent::Lens(LensEvent::Open(post.clone()))), on_event)) // N16
}
