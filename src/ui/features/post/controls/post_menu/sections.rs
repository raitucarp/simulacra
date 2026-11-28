//! The post menu's submenus, in the architecture's section order: Think with AI, Study,
//! Organize, View, Share & copy, Edit & history. Each leaves out what does not apply.

use super::{action_item, PostMenuSource};
use crate::ui::features::post::controls::actions_menu::action_group_submenus;
use crate::ui::features::post::controls::learn::learn_items;
use crate::ui::features::post::controls::share::copy_with_toast;
use crate::ui::icons::LucideIcon;
use crate::ui::menus::{post_ids as ids, MenuEntry, MenuItem, MenuSection, MenuSpec, MenuText, Submenu};
use rust_i18n::t;

/// The heading of the destructive section closing a submenu.
const DANGER_ZONE_KEY: &str = "menus.section.danger";

/// Understand ▸ · Challenge ▸ · Explore ▸, then Find similar and Fork a what-if….
pub(super) fn think(source: &PostMenuSource, branch: Vec<MenuItem>) -> Submenu {
    let explore = MenuSection::new().entry(action_item(ids::FIND_SIMILAR, LucideIcon::Compass, "post.menu_find_similar", &source.on_find_similar)).entries(branch);
    let spec = MenuSpec::new().section(MenuSection::new().entries(action_group_submenus(source.on_post_action.as_ref()))).section(explore);
    Submenu::new(ids::THINK, MenuText::key("menus.post.think"), spec).icon(LucideIcon::Brain)
}

/// Review (cards and words), Questions (open questions, the room, answers), Terms and
/// confidence (glossary, credence).
pub(super) fn study(source: &PostMenuSource, words: Vec<MenuItem>, questions: Vec<MenuItem>, terms: Vec<MenuItem>) -> Submenu {
    let credence = source.set_credence.clone().map(|open| MenuItem::new(ids::SET_CREDENCE, MenuText::key("thinking.credence.set")).icon(LucideIcon::Percent).handler(open));
    let spec = MenuSpec::new()
        .section(MenuSection::labelled("menus.post.section.review").entries(learn_items(&source.learn)).entries(words))
        .section(MenuSection::labelled("menus.post.section.questions").entries(questions))
        .section(MenuSection::labelled("menus.post.section.terms").entries(terms).entry_opt(credence));
    Submenu::new(ids::STUDY, MenuText::key("menus.post.study"), spec).icon(LucideIcon::GraduationCap)
}

/// Keep and sort, Lifecycle, then the destructive timer.
pub(super) fn organize(keep: Vec<MenuEntry>, lifecycle: Vec<MenuEntry>, expiry: Option<Submenu>) -> Submenu {
    let spec = MenuSpec::new()
        .section(MenuSection::labelled("menus.post.section.keep").entries(keep))
        .section(MenuSection::labelled("menus.post.section.lifecycle").entries(lifecycle))
        .section(MenuSection::labelled(DANGER_ZONE_KEY).entry_opt(expiry));
    Submenu::new(ids::ORGANIZE, MenuText::key("data.menu.organize"), spec).icon(LucideIcon::Folder)
}

/// Translation, the lens fan and the transcript.
pub(super) fn view(items: Vec<MenuItem>) -> Submenu {
    Submenu::new(ids::VIEW, MenuText::key("menus.post.view"), MenuSpec::of(items)).icon(LucideIcon::Eye)
}

fn copy_item(id: &'static str, icon: LucideIcon, key: &'static str, payload: &str, toast_key: &'static str) -> MenuItem {
    let (payload, toast) = (payload.to_string(), t!(toast_key).to_string());
    MenuItem::new(id, MenuText::key(key)).icon(icon).on_click(move |window, cx| copy_with_toast(payload.clone(), toast.clone(), window, cx))
}

/// The three copies, then who can read the post.
pub(super) fn share(source: &PostMenuSource, audience: Option<Submenu>) -> Submenu {
    let [text, link, embed] = &source.copies;
    let copies = MenuSection::new()
        .entry(copy_item(ids::COPY_TEXT, LucideIcon::Copy, "post.menu_copy_text", text, "post.copied_text"))
        .entry(copy_item(ids::COPY_LINK, LucideIcon::Link2, "post.menu_copy_link", link, "post.copied_link"))
        .entry(copy_item(ids::COPY_EMBED, LucideIcon::Layers, "post.menu_copy_embed", embed, "post.copied_embed"));
    let spec = MenuSpec::new().section(copies).section(MenuSection::new().entry_opt(audience));
    Submenu::new(ids::SHARE, MenuText::key("menus.post.share"), spec).icon(LucideIcon::Share2)
}

/// Edit, History, corrections and changes of mind, then retracting.
pub(super) fn edit_history(edit: Vec<MenuItem>, retract: Option<MenuItem>) -> Submenu {
    let spec = MenuSpec::new().section(MenuSection::new().entries(edit)).section(MenuSection::labelled(DANGER_ZONE_KEY).entry_opt(retract));
    Submenu::new(ids::EDIT_HISTORY, MenuText::key("menus.post.edit_history"), spec).icon(LucideIcon::Pencil)
}
