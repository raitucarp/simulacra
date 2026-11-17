//! Stream navigation: feed tabs, reply expansion, and revealing freshly published posts.

use super::super::view::AppView;
use crate::domain::post::{SourceKind, SourceRef};
use crate::domain::{PostId, ProfileId};
use crate::state::{feed_position, FeedTab, NavigationTab, WikilinkTarget};
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
            self.sync_stream(cx);
            self.feed.stream.scroll_to_row(index);
        }
        cx.notify();
    }

    /// Opens what a clicked `[[wikilink]]` names: the post in the stream and inspector, or
    /// the knowledge vault for a document.
    pub(crate) fn open_wikilink(&mut self, target: String, window: &mut Window, cx: &mut Context<Self>) {
        match self.state.resolve_wikilink_target(&target) {
            Some(WikilinkTarget::Post(id)) => self.reveal_post(id, window, cx),
            Some(WikilinkTarget::Document(_)) => self.select_nav(NavigationTab::Knowledge, window, cx),
            None => {}
        }
    }

    /// Opens what an agent post's source chip names: the vault (for a document), the post in
    /// the stream, the profile in the roster, or the web page in the browser.
    pub(crate) fn open_source(&mut self, source: SourceRef, window: &mut Window, cx: &mut Context<Self>) {
        match source.kind {
            SourceKind::VaultDoc => self.select_nav(NavigationTab::Knowledge, window, cx),
            SourceKind::Post => self.reveal_post(PostId::from(source.id), window, cx),
            SourceKind::Profile => {
                let id = ProfileId::from(source.id);
                let tab = match self.state.user_profiles.iter().find(|p| p.id == id) {
                    Some(profile) if !profile.is_agent() => NavigationTab::PersonaLab,
                    _ => NavigationTab::AgentLab,
                };
                self.select_nav(tab, window, cx);
                self.select_lab_profile(id, window, cx);
            }
            SourceKind::Web => cx.open_url(&source.id),
        }
    }

    /// Brings the virtualized stream's rows up to date (cheap when nothing changed).
    pub(crate) fn sync_stream(&mut self, cx: &App) {
        let query = self.feed.search_input.read(cx).value().to_string();
        let (tab, limit) = (self.feed.feed_tab, self.feed.feed_visible_count);
        self.feed.stream.sync(&self.state, tab, &query, limit);
    }
}
