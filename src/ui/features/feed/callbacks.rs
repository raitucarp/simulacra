//! Shared post interaction callbacks and the post card renderer wired to them.

use super::params::{
    LikePostCallback, QuotePostCallback, ReplyPostCallback, RepostPostCallback, SelectPostCallback,
};
use crate::domain::Post;
use crate::state::AppState;
use crate::theme::ThemeTokens;
use crate::ui::features::post::{render_post_item, ThreadlineStyle};
use gpui_kit::{AnyElement, IntoElement};
use std::rc::Rc;

/// Reference-counted post interaction callbacks shared by every rendered card.
#[derive(Clone)]
pub(crate) struct PostCallbacks {
    pub on_like: Rc<LikePostCallback>,
    pub on_reply: Rc<ReplyPostCallback>,
    pub on_select: Rc<SelectPostCallback>,
    pub on_repost: Rc<RepostPostCallback>,
    pub on_quote: Rc<QuotePostCallback>,
}

/// Visual and interaction mode of a rendered card.
#[derive(Clone, Copy)]
pub(crate) struct CardMode {
    pub is_focused: bool,
    pub style: ThreadlineStyle,
    /// Whether clicking the card selects it (the inspected root card is not selectable).
    pub selectable: bool,
}

impl PostCallbacks {
    /// Renders `post` as a card whose actions dispatch to these callbacks.
    pub(crate) fn render_post(
        &self,
        post: &Post,
        state: &AppState,
        theme: &ThemeTokens,
        mode: CardMode,
    ) -> AnyElement {
        let select = mode.selectable.then(|| (self.on_select.clone(), post.id.clone()));
        let (like, like_id) = (self.on_like.clone(), post.id.clone());
        let (reply, reply_id) = (self.on_reply.clone(), post.id.clone());
        let author_handle = state.author_of(post).handle.clone();
        let (repost, repost_id) = (self.on_repost.clone(), post.id.clone());
        let (quote, quote_id) = (self.on_quote.clone(), post.id.clone());
        render_post_item(
            post,
            state,
            theme,
            mode.is_focused,
            mode.style,
            move |window, cx| {
                if let Some((cb, id)) = &select {
                    cb(id.clone(), window, cx);
                }
            },
            move |_, window, cx| like(like_id.clone(), window, cx),
            move |_, window, cx| reply(reply_id.clone(), &author_handle, window, cx),
            move |_, window, cx| repost(repost_id.clone(), window, cx),
            move |_, window, cx| quote(quote_id.clone(), window, cx),
        )
        .into_any_element()
    }
}
