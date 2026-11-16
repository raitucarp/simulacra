//! Timeline state of the shell: thread selection, feed window and the feed's inputs.

use super::consts::{COMPOSER_ROWS, DOC_CONTENT_ROWS};
use super::stream_list::StreamList;
use super::view::AppView;
use crate::domain::{PostId, ProfileId};
use crate::state::{AppState, FeedTab};
use crate::ui::features::composer::PostComposerView;
use crate::ui::features::feed::DEFAULT_FEED_WINDOW_SIZE;
use gpui_kit::component::input::{InputEvent, InputState, TextareaState};
use gpui_kit::*;
use rust_i18n::t;

/// Feed selection, stream tab and scroll, plus the inline composer, search and vault inputs.
pub struct FeedState {
    pub selected_thread_post_id: Option<PostId>,
    pub custom_feed_profile_id: Option<ProfileId>,
    pub feed_visible_count: usize,
    /// Active stream tab ("For you" / "Dialectic").
    pub feed_tab: FeedTab,
    /// Whether the inspector shows every reply instead of the collapsed preview.
    pub inspector_expanded: bool,
    /// Virtualized main stream (cached rows + list state, used to bring posts into view).
    pub stream: StreamList,
    pub composer_input: Entity<TextareaState>,
    pub composer_view: Entity<PostComposerView>,
    pub search_input: Entity<InputState>,
    pub doc_title_input: Entity<InputState>,
    pub doc_content_input: Entity<TextareaState>,
}

impl FeedState {
    pub(super) fn new(state: &AppState, window: &mut Window, cx: &mut Context<AppView>) -> Self {
        let (min_rows, max_rows) = COMPOSER_ROWS;
        let composer_placeholder = t!("composer.placeholder").to_string();
        let composer_input = cx.new(|cx| {
            TextareaState::new(window, cx)
                .placeholder(composer_placeholder)
                .auto_grow(min_rows, max_rows)
        });
        let search_placeholder = t!("sidebar.search_placeholder").to_string();
        let search_input = cx.new(|cx| InputState::new(window, cx).placeholder(search_placeholder));
        let title_placeholder = t!("vault.doc_title_placeholder").to_string();
        let doc_title_input = cx.new(|cx| InputState::new(window, cx).placeholder(title_placeholder));
        let composer_view = Self::composer_view(state, composer_input.clone(), window, cx);
        Self {
            selected_thread_post_id: None,
            custom_feed_profile_id: None,
            feed_visible_count: DEFAULT_FEED_WINDOW_SIZE,
            feed_tab: FeedTab::default(),
            inspector_expanded: false,
            stream: StreamList::default(),
            composer_input,
            composer_view,
            search_input,
            doc_title_input,
            doc_content_input: Self::doc_content_input(window, cx),
        }
    }

    fn doc_content_input(window: &mut Window, cx: &mut Context<AppView>) -> Entity<TextareaState> {
        let (min_rows, max_rows) = DOC_CONTENT_ROWS;
        let placeholder = t!("vault.doc_content_placeholder").to_string();
        cx.new(|cx| {
            TextareaState::new(window, cx)
                .placeholder(placeholder)
                .auto_grow(min_rows, max_rows)
        })
    }

    /// The isolated inline composer view, submitting through [`AppView::submit_post_content`].
    fn composer_view(
        state: &AppState,
        composer_input: Entity<TextareaState>,
        window: &mut Window,
        cx: &mut Context<AppView>,
    ) -> Entity<PostComposerView> {
        let entity_self = cx.entity().clone();
        let profile = state.current_profile().cloned();
        let theme = state.theme_tokens.clone();
        cx.new(|cx| {
            let on_submit = move |content: String, window: &mut Window, cx: &mut App| {
                entity_self.update(cx, |this, cx| this.submit_post_content(&content, window, cx))
            };
            PostComposerView::new(profile, composer_input, theme, on_submit, window, cx)
        })
    }

    /// Resets the feed window whenever the search query changes.
    pub(super) fn subscribe_search(&self, cx: &mut Context<AppView>) -> Subscription {
        cx.subscribe(&self.search_input, |this, _, event: &InputEvent, cx| {
            if matches!(event, InputEvent::Change) {
                this.feed.feed_visible_count = DEFAULT_FEED_WINDOW_SIZE;
                cx.notify();
            }
        })
    }
}
