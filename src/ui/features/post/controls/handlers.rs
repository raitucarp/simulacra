//! Callbacks a rendered post card dispatches to its host.

use super::super::body::BodyLinks;
use crate::domain::PostId;
use gpui_kit::{App, Window};
use std::rc::Rc;

/// A shared, re-invocable card action (menus rebuild their items on every open).
pub type PostAction = Rc<dyn Fn(&mut Window, &mut App) + 'static>;
/// Opens the given thought in the conversation inspector.
pub type OpenPostAction = Rc<dyn Fn(PostId, &mut Window, &mut App) + 'static>;
/// Runs a thinking action on the card's thought (sb C2).
pub type PickPostAction = Rc<dyn Fn(crate::domain::post_actions::ActionKind, &mut Window, &mut App) + 'static>;

/// Every interaction of one post card.
#[derive(Clone)]
pub struct PostCardHandlers {
    /// Card mouse-down: focus the thought in the inspector.
    pub on_select: PostAction,
    pub on_like: PostAction,
    pub on_reply: PostAction,
    pub on_repost: PostAction,
    pub on_quote: PostAction,
    /// Opens the items closest in meaning to this thought (sb B3).
    pub on_find_similar: PostAction,
    /// The "Learn" submenu and the learning indicator (sb C1b).
    pub learn: super::learn::LearnControls,
    /// Opens a referenced thought (or this one) in the inspector.
    pub on_open_post: OpenPostAction,
    /// Deletes the thought; `None` when the user did not write it.
    pub on_delete: Option<PostAction>,
    /// Hashtag and wikilink clicks inside the body.
    pub links: BodyLinks,
    /// "…" menu › Actions (sb C2); `None` hides the submenu.
    pub on_post_action: Option<PickPostAction>,
    /// sb E1: the credence chip and popover; `None` on posts the user did not write.
    pub credence: Option<crate::ui::features::thinking::credence::CredenceControls>,
    /// sb F2: Edit, History, Bookmark, collections and the "edited" mark; `None` hides them.
    pub data: Option<crate::ui::features::data::DataControls>,
    /// Epic R: the reaction picker; `None` hides it.
    pub reactions: Option<crate::ui::features::reactions::ReactionControls>,
    /// X4: a locked post's unlock state and actions; `None` on posts that are not locked.
    pub lock: Option<super::super::locked::LockControls>,
}
