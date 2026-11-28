//! The post menu (M1): one spec for the card's "…" dropdown and its right-click menu.
//!
//! Reply · Quote · Reply in the margin… | Think with AI ▸ · Study ▸ · Organize ▸ · View ▸ |
//! Share & copy ▸ · Edit & history ▸ | Delete. Every entry keeps the condition it had in the
//! old menu; `docs/decisions/M1-menu-architecture.md` maps each old path to its new one.

mod sections;
#[cfg(test)]
mod tests;

use super::handlers::{PickPostAction, PostAction, PostCardHandlers};
use super::learn::LearnControls;
use crate::domain::Post;
use crate::ui::features::data::{data_menu, DataControls, DataMenu};
use crate::ui::icons::LucideIcon;
use crate::ui::menus::{post_ids as ids, MenuHandler, MenuItem, MenuSection, MenuSpec, MenuText};

/// Everything the post menu needs of one card, read once when the menu opens.
#[derive(Clone)]
pub(crate) struct PostMenuSource {
    /// The post's text, permalink and pinned embed, for the copy items.
    pub copies: [String; 3],
    pub on_reply: PostAction,
    pub on_quote: PostAction,
    pub on_find_similar: PostAction,
    /// Think with AI (sb C2); `None` on a locked post.
    pub on_post_action: Option<PickPostAction>,
    pub learn: LearnControls,
    /// "Set a credence…" on the user's post without one (sb E1).
    pub set_credence: Option<MenuHandler>,
    pub data: Option<DataControls>,
    /// Deletes the post (after its confirmation); `None` when the user did not write it.
    pub on_delete: Option<PostAction>,
}

impl PostMenuSource {
    /// The menu source of `post`'s card.
    pub(crate) fn of(post: &Post, handlers: &PostCardHandlers) -> Self {
        Self {
            copies: [post.content.clone(), super::share::post_permalink(post), super::menu::embed_code(post)],
            on_reply: handlers.on_reply.clone(),
            on_quote: handlers.on_quote.clone(),
            on_find_similar: handlers.on_find_similar.clone(),
            on_post_action: handlers.on_post_action.clone(),
            learn: handlers.learn.clone(),
            set_credence: crate::ui::features::thinking::credence::credence_opener(handlers.credence.as_ref()),
            data: handlers.data.clone(),
            on_delete: handlers.on_delete.clone(),
        }
    }
}

pub(super) fn action_item(id: &'static str, icon: LucideIcon, key: &'static str, action: &PostAction) -> MenuItem {
    let action = action.clone();
    MenuItem::new(id, MenuText::key(key)).icon(icon).on_click(move |window, cx| action(window, cx))
}

/// The whole post menu of `source`.
pub(crate) fn post_menu_spec(source: &PostMenuSource) -> MenuSpec {
    let DataMenu { respond, think, review, questions, terms, keep, lifecycle, expiry, view, audience, edit, retract } = source.data.as_ref().map(data_menu).unwrap_or_default();
    let respond = MenuSection::new()
        .entry(action_item(ids::REPLY, LucideIcon::MessageSquare, "post.menu_reply", &source.on_reply))
        .entry(action_item(ids::QUOTE, LucideIcon::Quote, "post.menu_quote", &source.on_quote))
        .entries(respond);
    let study = sections::study(source, review, questions, terms);
    let thinking = MenuSection::new()
        .entry(sections::think(source, think))
        .entry(study)
        .entry(sections::organize(keep, lifecycle, expiry))
        .entry(sections::view(view));
    let keeping = MenuSection::new().entry(sections::share(source, audience)).entry(sections::edit_history(edit, retract));
    let danger = MenuSection::new().entry_opt(source.on_delete.clone().map(super::menu::delete_item));
    MenuSpec::new().section(respond).section(thinking).section(keeping).section(danger)
}
