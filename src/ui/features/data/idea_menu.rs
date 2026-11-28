//! N7, N8, N10: entries of the user's own post: re-pin a drifted quote (N7), add it to a
//! series or leave one (N10), and who may read it (N8: everyone, only the user, or chosen
//! agents, the current choice ticked).

use super::events::DataEvent;
use super::post_menu::{item, DataControls};
use super::quote_sheet::QuoteEvent;
use super::series_sheet::SeriesEvent;
use crate::domain::post::audience::Audience;
use crate::ui::icons::LucideIcon;
use crate::ui::menus::{post_ids as ids, MenuEntry, MenuItem, MenuSection, MenuSpec, MenuText, Submenu};
use rust_i18n::t;

/// "Who can read this": everyone, only me, then each agent of the network.
pub(super) fn audience_submenu(controls: &DataControls) -> Option<Submenu> {
    if !controls.own {
        return None;
    }
    let (post, current) = (&controls.post_id, controls.facts.audience.as_ref());
    let choice = |id: &'static str, label: MenuText, icon: LucideIcon, chosen: bool, audience: Option<Audience>| item(id, label, icon, DataEvent::SetAudience(post.clone(), audience), &controls.on_event).checked(chosen);
    let everyone = choice(ids::AUDIENCE_EVERYONE, MenuText::key("posts.audience.everyone"), LucideIcon::Globe, current.is_none(), None);
    let only_me = choice(ids::AUDIENCE_ONLY_ME, MenuText::key("posts.audience.only_me"), LucideIcon::EyeOff, current == Some(&Audience::OnlyMe), Some(Audience::OnlyMe));
    let agents = controls.ideas.agents.iter().map(|(agent, name)| {
        let chosen = current.is_some_and(|audience| audience.admits(agent));
        let next = current.unwrap_or(&Audience::OnlyMe).toggled(agent);
        choice(ids::AUDIENCE_AGENT, MenuText::plain(name.clone()), LucideIcon::Bot, chosen, Some(next))
    });
    let spec = MenuSpec::new().section(MenuSection::new().entry(everyone).entry(only_me)).section(MenuSection::new().entries(agents)).scrollable();
    Some(Submenu::new(ids::AUDIENCE, MenuText::key("posts.audience.menu"), spec).icon(LucideIcon::Users))
}

fn series_submenu(controls: &DataControls) -> Submenu {
    let post = &controls.post_id;
    let add = controls.ideas.series.iter().map(|title| item(ids::SERIES_ADD, MenuText::plain(title.clone()), LucideIcon::ListOrdered, DataEvent::Series(SeriesEvent::Add(post.clone(), title.clone())), &controls.on_event));
    let new = item(ids::SERIES_NEW, MenuText::key("posts.series.new"), LucideIcon::Plus, DataEvent::Series(SeriesEvent::New(post.clone())), &controls.on_event);
    let spec = MenuSpec::new().section(MenuSection::new().entries(add)).section(MenuSection::new().entry(new)).scrollable();
    Submenu::new(ids::SERIES, MenuText::key("posts.series.menu"), spec).icon(LucideIcon::ListOrdered)
}

/// "Add to a series" (a submenu), or "Leave “…”" once the post is part of one.
pub(super) fn series_entry(controls: &DataControls) -> Option<MenuEntry> {
    if !controls.own {
        return None;
    }
    Some(match &controls.facts.series {
        Some(title) => {
            let label = MenuText::keyed("posts.series.leave", t!("posts.series.leave", title = title.clone()).to_string());
            item(ids::SERIES_LEAVE, label, LucideIcon::ListOrdered, DataEvent::Series(SeriesEvent::Leave(controls.post_id.clone())), &controls.on_event).into()
        }
        None => series_submenu(controls).into(),
    })
}

/// "Re-pin the quote to its source", once the quoted source changed but still holds the words.
pub(super) fn repin_item(controls: &DataControls) -> Option<MenuItem> {
    (controls.own && controls.facts.repinnable).then(|| item(ids::REPIN_QUOTE, MenuText::key("posts.pinned.repin"), LucideIcon::Quote, DataEvent::Quote(QuoteEvent::Repin(controls.post_id.clone())), &controls.on_event))
}
