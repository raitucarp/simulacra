//! Shared post interaction callbacks and the post card renderer wired to them.

use super::params::{
    DeletePostCallback, LikePostCallback, QuotePostCallback, ReplyPostCallback, RepostPostCallback,
    SelectPostCallback,
};
use crate::domain::{Post, PostId, Profile};
use crate::state::AppState;
use crate::theme::ThemeTokens;
use crate::ui::features::post::{render_post_item, BodyLinks, PostAction, PostCardHandlers, ThreadlineStyle};
use gpui_kit::{AnyElement, App, IntoElement, Window};
use std::rc::Rc;

/// Reference-counted post interaction callbacks shared by every rendered card.
#[derive(Clone)]
pub(crate) struct PostCallbacks {
    pub on_like: Rc<LikePostCallback>,
    pub on_reply: Rc<ReplyPostCallback>,
    pub on_select: Rc<SelectPostCallback>,
    pub on_repost: Rc<RepostPostCallback>,
    pub on_quote: Rc<QuotePostCallback>,
    pub on_delete: Rc<DeletePostCallback>,
    /// Hashtag and wikilink clicks inside post bodies.
    pub links: BodyLinks,
}

/// Visual and interaction mode of a rendered card.
#[derive(Clone, Copy)]
pub(crate) struct CardMode {
    pub is_focused: bool,
    pub style: ThreadlineStyle,
    /// Whether clicking the card selects it (the inspected root card is not selectable).
    pub selectable: bool,
}

/// Binds a `(PostId, Window, App)` callback to one post.
fn bind<F: Fn(PostId, &mut Window, &mut App) + ?Sized + 'static>(cb: &Rc<Box<F>>, id: &PostId) -> PostAction {
    let (cb, id) = (cb.clone(), id.clone());
    Rc::new(move |window, cx| cb(id.clone(), window, cx))
}

impl PostCallbacks {
    /// Card handlers for `post`; delete is offered only on thoughts written by a user persona.
    fn handlers_for(&self, post: &Post, author: &Profile, selectable: bool) -> PostCardHandlers {
        let (reply, reply_id, handle) = (self.on_reply.clone(), post.id.clone(), author.handle.clone());
        let select = self.on_select.clone();
        PostCardHandlers {
            on_select: if selectable { bind(&self.on_select, &post.id) } else { Rc::new(|_, _| {}) },
            on_like: bind(&self.on_like, &post.id),
            on_reply: Rc::new(move |window, cx| reply(reply_id.clone(), &handle, window, cx)),
            on_repost: bind(&self.on_repost, &post.id),
            on_quote: bind(&self.on_quote, &post.id),
            on_open_post: Rc::new(move |id, window, cx| select(id, window, cx)),
            on_delete: author.is_human_persona().then(|| bind(&self.on_delete, &post.id)),
            links: self.links.clone(),
        }
    }

    /// Renders `post` as a card whose actions dispatch to these callbacks.
    pub(crate) fn render_post(
        &self,
        post: &Post,
        state: &AppState,
        theme: &ThemeTokens,
        mode: CardMode,
    ) -> AnyElement {
        let handlers = self.handlers_for(post, state.author_of(post), mode.selectable);
        render_post_item(post, state, theme, mode.is_focused, mode.style, handlers).into_any_element()
    }
}
