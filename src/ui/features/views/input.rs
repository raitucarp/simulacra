//! What every list view renders from: the posts in view, the scope they come from, and how
//! a post is opened.

use crate::domain::views::ViewScope;
use crate::domain::{Post, PostId};
use crate::state::AppState;
use crate::theme::ThemeTokens;
use gpui_kit::{App, SharedString, Window};
use std::rc::Rc;

/// Opens a post (selects it in the timeline).
pub type OpenPost = Rc<dyn Fn(PostId, &mut Window, &mut App) + 'static>;

/// The inputs of a list view.
pub struct ViewInput<'a> {
    pub state: &'a AppState,
    pub theme: &'a ThemeTokens,
    /// The posts shown (published ones; the scope picks them and their order).
    pub posts: Vec<&'a Post>,
    pub scope: ViewScope,
    pub on_open: OpenPost,
}

impl ViewInput<'_> {
    /// An element id unique to this scope: `<scope>-<part>`.
    pub fn id(&self, part: impl std::fmt::Display) -> SharedString {
        let scope = match self.scope {
            ViewScope::Thread => "thread-view",
            ViewScope::Profile => "profile-view",
        };
        SharedString::from(format!("{scope}-{part}"))
    }

    /// A click handler opening `post`.
    pub fn opener(&self, post: &PostId) -> impl Fn(&gpui_kit::ClickEvent, &mut Window, &mut App) + 'static {
        let (on_open, id) = (self.on_open.clone(), post.clone());
        move |_, window, cx| on_open(id.clone(), window, cx)
    }
}
