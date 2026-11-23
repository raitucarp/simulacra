//! N3: the garden items of the user's own post in its "…" menu: "Plant in the garden" while
//! it is in the stream; once planted, a submenu to grow it to the next stage, cut it back to
//! the one before, or return it to the stream.

use super::events::DataEvent;
use super::post_menu::{item, DataControls};
use super::text::tr;
use crate::domain::post::garden::Maturity;
use crate::ui::features::views::maturity_label;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::menu::PopupMenu;
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;

fn garden_submenu(menu: PopupMenu, controls: &DataControls, stage: Maturity) -> PopupMenu {
    let post = &controls.post_id;
    let grow = stage.next().map(|next| item(t!("data.menu.garden_grow", stage = maturity_label(next)).to_string(), LucideIcon::Sprout, DataEvent::SetGarden(post.clone(), Some(next)), &controls.on_event));
    let cut = stage.previous().map(|previous| item(t!("data.menu.garden_cut", stage = maturity_label(previous)).to_string(), LucideIcon::Undo2, DataEvent::SetGarden(post.clone(), Some(previous)), &controls.on_event));
    let menu = [grow, cut].into_iter().flatten().fold(menu, PopupMenu::item);
    menu.separator().item(item(tr("data.menu.garden_return"), LucideIcon::Inbox, DataEvent::SetGarden(post.clone(), None), &controls.on_event))
}

/// `menu` with "Plant in the garden", or the garden submenu of a planted post.
pub(super) fn with_garden_items(menu: PopupMenu, controls: &DataControls, window: &mut Window, cx: &mut Context<PopupMenu>) -> PopupMenu {
    match controls.garden {
        None => menu.item(item(tr("data.menu.garden_plant"), LucideIcon::Sprout, DataEvent::SetGarden(controls.post_id.clone(), Some(Maturity::Seed)), &controls.on_event)),
        Some(stage) => {
            let (controls, label) = (controls.clone(), t!("data.menu.garden", stage = maturity_label(stage)).to_string());
            menu.submenu_with_icon(Some(Icon::new(LucideIcon::Sprout)), label, window, cx, move |submenu, _, _| garden_submenu(submenu, &controls, stage))
        }
    }
}
