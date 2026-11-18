//! Callbacks a rendered post card dispatches to its host.

use super::super::body::BodyLinks;
use crate::domain::PostId;
use gpui_kit::{App, Window};
use std::rc::Rc;

/// A shared, re-invocable card action (menus rebuild their items on every open).
pub type PostAction = Rc<dyn Fn(&mut Window, &mut App) + 'static>;
/// Opens the given thought in the conversation inspector.
pub type OpenPostAction = Rc<dyn Fn(PostId, &mut Window, &mut App) + 'static>;

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
    /// Opens the card maker on this thought (sb C1).
    pub on_make_flashcard: PostAction,
    /// Opens a referenced thought (or this one) in the inspector.
    pub on_open_post: OpenPostAction,
    /// Deletes the thought; `None` when the user did not write it.
    pub on_delete: Option<PostAction>,
    /// Hashtag and wikilink clicks inside the body.
    pub links: BodyLinks,
}
