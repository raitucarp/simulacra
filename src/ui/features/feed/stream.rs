//! Main dialectic stream: a virtualized `list()` over the cached stream rows.
//!
//! Only the rows in (and just around) the viewport are rendered. Each row reads the store
//! through a [`FeedReader`] at layout time, so the list closure owns no borrowed state.

use super::callbacks::{CardMode, PostCallbacks};
use super::load_more::render_load_more;
use super::params::LoadMoreCallback;
use super::threadline::compute_stream_threadline_style;
use crate::domain::Post;
use crate::state::AppState;
use crate::theme::ThemeTokens;
use crate::ui::components::{art_state, ArtState};
use crate::ui::illustration::{EmptyKey, Illustration};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, list, AnyElement, App, IntoElement, ListState, ParentElement, Styled, Window};
use rust_i18n::t;
use std::rc::Rc;

/// Read access to the store and theme while the list lays out its visible rows.
pub type FeedReader = Rc<dyn for<'a> Fn(&'a App) -> (&'a AppState, &'a ThemeTokens)>;

/// Windowed stream rows: store positions in display order and how many are shown.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StreamRows {
    /// Store positions of every matching post, in display order.
    pub positions: Rc<[usize]>,
    /// Number of posts shown before the load-more row.
    pub shown: usize,
    /// N4: the day's budget is spent, so the feed ends with the closing card.
    pub ended: bool,
}

impl StreamRows {
    /// Windows `positions` to at most `limit` posts.
    pub fn new(positions: Rc<[usize]>, limit: usize) -> Self {
        let shown = limit.min(positions.len());
        Self { positions, shown, ended: false }
    }

    /// Whether a load-more row follows the shown posts.
    pub fn has_more(&self) -> bool {
        self.shown < self.positions.len()
    }

    /// Number of list rows (shown posts plus the load-more row or the closing card).
    pub fn row_count(&self) -> usize {
        self.shown + usize::from(self.has_more() || self.ended)
    }

    /// Whether the closing card ends the rows (N4): the budget is spent and nothing more loads.
    pub fn ends_with_card(&self) -> bool {
        self.ended && !self.has_more()
    }

    /// The post at display `row`, if it is a shown post.
    pub fn post<'a>(&self, state: &'a AppState, row: usize) -> Option<&'a Post> {
        (row < self.shown).then(|| self.positions.get(row)).flatten().and_then(|&idx| state.posts.get(idx))
    }
}

/// The virtualized stream's state and row source, supplied by the shell.
#[derive(Clone)]
pub struct StreamInputs {
    pub list: ListState,
    pub rows: StreamRows,
    pub reader: FeedReader,
}

/// Renders stream `row`: a post card (with its threadline joins) or the load-more trigger.
fn render_row(
    row: usize,
    rows: &StreamRows,
    reader: &FeedReader,
    callbacks: &PostCallbacks,
    on_load_more: &Option<Rc<LoadMoreCallback>>,
    cx: &App,
) -> AnyElement {
    let (state, theme) = reader(cx);
    let Some(post) = rows.post(state, row) else {
        return match rows.ends_with_card() {
            true => super::end_card::render_end_card(state.preferences().writing.feed_budget, theme), // N4
            false => render_load_more(rows.shown, rows.positions.len(), theme, on_load_more.clone()),
        };
    };
    let neighbours: Vec<&Post> = [row.checked_sub(1), Some(row), Some(row + 1)]
        .into_iter()
        .flatten()
        .filter_map(|r| rows.post(state, r))
        .collect();
    let at = usize::from(row > 0);
    let style = compute_stream_threadline_style(&neighbours, at);
    let mode = CardMode { is_focused: false, style, selectable: true };
    let card = callbacks.render_post(post, state, theme, mode);
    div().w_full().child(card).into_any_element()
}

/// Virtualized stream body with an empty-state fallback.
pub(crate) fn render_stream_list(
    stream: StreamInputs,
    callbacks: PostCallbacks,
    on_load_more: Option<Rc<LoadMoreCallback>>,
    theme: &ThemeTokens,
) -> impl IntoElement {
    let is_empty = stream.rows.row_count() == 0;
    let scrollbar_handle = stream.list.clone();
    let (rows, reader) = (stream.rows, stream.reader);
    let body = list(stream.list, move |row, _: &mut Window, cx: &mut App| {
        render_row(row, &rows, &reader, &callbacks, &on_load_more, cx)
    })
    .size_full();
    div()
        .relative()
        .flex_1()
        .w_full()
        .min_h_0()
        .overflow_hidden()
        .when(is_empty, |this| {
            let art = Illustration::empty(EmptyKey::NoPosts).width(theme.art_empty_width);
            let state = ArtState::titled(art, t!("feed.no_posts").to_string(), t!("art.feed_empty_hint").to_string());
            this.p(theme.panel_padding).flex().items_center().justify_center().child(art_state(state, theme))
        })
        .when(!is_empty, |this| this.child(body).vertical_scrollbar(&scrollbar_handle))
}
