//! N7–N10: the items of the user's own post in its "…" menu: re-pin a drifted quote (N7), ask
//! the room (N9), add it to a series or leave one (N10), and who may read it (N8: everyone,
//! only the user, or chosen agents).

use super::events::DataEvent;
use super::post_menu::{item, DataControls};
use super::question::QuestionEvent;
use super::quote_sheet::QuoteEvent;
use super::series_sheet::SeriesEvent;
use super::text::tr;
use crate::domain::post::audience::Audience;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::menu::PopupMenu;
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;

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

fn series_submenu(menu: PopupMenu, controls: &DataControls) -> PopupMenu {
    let post = &controls.post_id;
    let add = |title: &String| item(title.clone(), LucideIcon::ListOrdered, DataEvent::Series(SeriesEvent::Add(post.clone(), title.clone())), &controls.on_event);
    let menu = controls.ideas.series.iter().fold(menu, |menu, title| menu.item(add(title)));
    let menu = if controls.ideas.series.is_empty() { menu } else { menu.separator() };
    menu.item(item(tr("posts.series.new"), LucideIcon::Plus, DataEvent::Series(SeriesEvent::New(post.clone())), &controls.on_event))
}

/// `menu` with the N7–N10 items of the user's own post.
pub(super) fn with_idea_items(menu: PopupMenu, controls: &DataControls, window: &mut Window, cx: &mut Context<PopupMenu>) -> PopupMenu {
    let post = &controls.post_id;
    let mut menu = menu;
    if controls.facts.repinnable {
        menu = menu.item(item(tr("posts.pinned.repin"), LucideIcon::Quote, DataEvent::Quote(QuoteEvent::Repin(post.clone())), &controls.on_event));
    }
    if !controls.locked && !controls.facts.asked {
        menu = menu.item(item(tr("posts.question.menu"), LucideIcon::HelpCircle, DataEvent::Question(QuestionEvent::Ask(post.clone())), &controls.on_event)); // N9
    }
    menu = match &controls.facts.series {
        Some(title) => menu.item(item(t!("posts.series.leave", title = title.clone()).to_string(), LucideIcon::ListOrdered, DataEvent::Series(SeriesEvent::Leave(post.clone())), &controls.on_event)),
        None => {
            let scoped = controls.clone();
            menu.submenu_with_icon(Some(Icon::new(LucideIcon::ListOrdered)), tr("posts.series.menu"), window, cx, move |submenu, _, _| series_submenu(submenu, &scoped))
        }
    };
    let scoped = controls.clone();
    menu.submenu_with_icon(Some(Icon::new(LucideIcon::Users)), tr("posts.audience.menu"), window, cx, move |submenu, _, _| audience_submenu(submenu, &scoped))
}
