//! N3: the garden entry of the user's own post: "Plant in the garden" while it is in the
//! stream; once planted, a submenu to grow it to the next stage, cut it back to the one
//! before, or return it to the stream.

use super::events::DataEvent;
use super::post_menu::{item, DataControls};
use crate::domain::post::garden::Maturity;
use crate::ui::features::views::maturity_label;
use crate::ui::icons::LucideIcon;
use crate::ui::menus::{post_ids as ids, MenuEntry, MenuSection, MenuSpec, MenuText, Submenu};
use rust_i18n::t;

fn garden_submenu(controls: &DataControls, stage: Maturity) -> Submenu {
    let (post, on_event) = (&controls.post_id, &controls.on_event);
    let grow = stage.next().map(|next| item(ids::GARDEN_GROW, MenuText::keyed("data.menu.garden_grow", t!("data.menu.garden_grow", stage = maturity_label(next)).to_string()), LucideIcon::Sprout, DataEvent::SetGarden(post.clone(), Some(next)), on_event));
    let cut = stage.previous().map(|previous| item(ids::GARDEN_CUT, MenuText::keyed("data.menu.garden_cut", t!("data.menu.garden_cut", stage = maturity_label(previous)).to_string()), LucideIcon::Undo2, DataEvent::SetGarden(post.clone(), Some(previous)), on_event));
    let back = item(ids::GARDEN_RETURN, MenuText::key("data.menu.garden_return"), LucideIcon::Inbox, DataEvent::SetGarden(post.clone(), None), on_event);
    let spec = MenuSpec::new().section(MenuSection::new().entry_opt(grow).entry_opt(cut)).section(MenuSection::new().entry(back));
    let label = MenuText::keyed("data.menu.garden", t!("data.menu.garden", stage = maturity_label(stage)).to_string());
    Submenu::new(ids::GARDEN, label, spec).icon(LucideIcon::Sprout)
}

/// "Plant in the garden", or the garden submenu of a planted post (the user's own only).
pub(super) fn garden_entry(controls: &DataControls) -> Option<MenuEntry> {
    if !controls.own {
        return None;
    }
    Some(match controls.facts.garden {
        None => item(ids::GARDEN_PLANT, MenuText::key("data.menu.garden_plant"), LucideIcon::Sprout, DataEvent::SetGarden(controls.post_id.clone(), Some(Maturity::Seed)), &controls.on_event).into(),
        Some(stage) => garden_submenu(controls, stage).into(),
    })
}
