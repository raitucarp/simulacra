//! N7–N10: the items of the user's own post in its "…" menu: re-pin a drifted quote (N7) and
//! who may read the post (N8: everyone, only the user, or chosen agents).

use super::events::DataEvent;
use super::post_menu::{item, DataControls};
use super::quote_sheet::QuoteEvent;
use super::text::tr;
use crate::domain::post::audience::Audience;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::menu::PopupMenu;
use gpui_kit::component::Icon;
use gpui_kit::*;

fn audience_submenu(menu: PopupMenu, controls: &DataControls) -> PopupMenu {
    let (post, current) = (&controls.post_id, controls.facts.audience.as_ref());
    let choice = |label: String, icon: LucideIcon, chosen: bool, audience: Option<Audience>| item(label, if chosen { LucideIcon::Check } else { icon }, DataEvent::SetAudience(post.clone(), audience), &controls.on_event);
    let menu = menu
        .item(choice(tr("posts.audience.everyone"), LucideIcon::Globe, current.is_none(), None))
        .item(choice(tr("posts.audience.only_me"), LucideIcon::EyeOff, current == Some(&Audience::OnlyMe), Some(Audience::OnlyMe)));
    let menu = if controls.ideas.agents.is_empty() { menu } else { menu.separator() };
    controls.ideas.agents.iter().fold(menu, |menu, (agent, name)| {
        let chosen = current.is_some_and(|audience| audience.admits(agent));
        let next = current.unwrap_or(&Audience::OnlyMe).toggled(agent);
        menu.item(choice(name.clone(), LucideIcon::Bot, chosen, Some(next)))
    })
}

/// `menu` with the N7–N10 items of the user's own post.
pub(super) fn with_idea_items(menu: PopupMenu, controls: &DataControls, window: &mut Window, cx: &mut Context<PopupMenu>) -> PopupMenu {
    let post = &controls.post_id;
    let mut menu = menu;
    if controls.facts.repinnable {
        menu = menu.item(item(tr("posts.pinned.repin"), LucideIcon::Quote, DataEvent::Quote(QuoteEvent::Repin(post.clone())), &controls.on_event));
    }
    let scoped = controls.clone();
    menu.submenu_with_icon(Some(Icon::new(LucideIcon::Users)), tr("posts.audience.menu"), window, cx, move |submenu, _, _| audience_submenu(submenu, &scoped))
}
