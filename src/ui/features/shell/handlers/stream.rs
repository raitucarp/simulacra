//! Stream navigation: feed tabs, reply expansion, and revealing freshly published posts.

use super::super::view::AppView;
use crate::domain::PostId;
use crate::state::{feed_position, FeedTab, NavigationTab};
use gpui_kit::*;

impl AppView {
    /// Switches the stream between "For you" and "Dialectic".
    pub(crate) fn select_feed_tab(&mut self, tab: FeedTab, _: &mut Window, cx: &mut Context<Self>) {
        self.feed.feed_tab = tab;
        cx.notify();
    }

    /// Shows or collapses the replies the inspector hides behind "show more".
    pub(crate) fn toggle_inspector_replies(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.feed.inspector_expanded = !self.feed.inspector_expanded;
        cx.notify();
    }

    /// Brings a just-published post into view: opens the Timeline, clears a search that
    /// could hide it, focuses it in the inspector and scrolls the stream to it.
    pub(crate) fn reveal_post(&mut self, post_id: PostId, window: &mut Window, cx: &mut Context<Self>) {
        if self.state.active_tab != NavigationTab::Timeline {
            self.select_nav(NavigationTab::Timeline, window, cx);
        }
        if !self.feed.search_input.read(cx).value().trim().is_empty() {
            self.feed.search_input.update(cx, |input, cx| input.set_value("", window, cx));
        }
        self.feed.selected_thread_post_id = Some(post_id.clone());
        self.feed.inspector_expanded = false;
        // Clearing the search re-windows the stream on its change event; scroll after that.
        cx.defer_in(window, move |this, _, cx| this.scroll_stream_to(&post_id, cx));
        cx.notify();
    }

    /// Widens the stream window to include `post_id` (falling back to "For you" when the
    /// active tab filters it out) and scrolls it into view.
    fn scroll_stream_to(&mut self, post_id: &PostId, cx: &mut Context<Self>) {
        let mut position = feed_position(&self.state, self.feed.feed_tab, post_id);
        if position.is_none() && self.feed.feed_tab != FeedTab::ForYou {
            self.feed.feed_tab = FeedTab::ForYou;
            position = feed_position(&self.state, FeedTab::ForYou, post_id);
        }
        if let Some(index) = position {
            self.feed.feed_visible_count = self.feed.feed_visible_count.max(index + 1);
            self.feed.stream_scroll.scroll_to_item(index);
        }
        cx.notify();
    }
}
