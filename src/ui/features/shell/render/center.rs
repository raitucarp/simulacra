//! Center column: the main feed (timeline, vault), the Mind Graph or the agent workspace.

use super::super::binder::Binder;
use super::super::view::AppView;
use crate::state::NavigationTab;
use crate::ui::features::day_filter::render_day_filter_chip;
use crate::ui::features::feed::{render_main_feed, FeedParams};
use gpui_kit::*;
use std::rc::Rc;

impl AppView {
    /// The Agent Forge while it is open, the roster while a lab tab is active, the Mind Graph
    /// view on its tab, otherwise the main feed.
    pub(in crate::ui::features::shell) fn center_panel(&self, bind: &Binder) -> AnyElement {
        if let Some(session) = &self.forge {
            session.view.clone().into_any_element()
        } else if self.is_lab_tab() {
            self.agent_workspace(bind).into_any_element()
        } else if self.state.active_tab == NavigationTab::MindGraph {
            self.graph.panel.clone().into_any_element()
        } else {
            self.filtered_feed(bind).into_any_element()
        }
    }

    /// The main feed, topped by the "Showing <date> ×" chip while a day filter is active.
    fn filtered_feed(&self, bind: &Binder) -> impl IntoElement {
        let day = self.state.feed_day_filter.filter(|_| self.state.active_tab == NavigationTab::Timeline);
        let chip = day.map(|day| render_day_filter_chip(day, bind.boxed_act(Self::clear_day_filter), &self.theme));
        div()
            .size_full()
            .flex()
            .flex_col()
            .children(chip)
            .child(div().flex_1().min_h_0().w_full().child(self.main_feed(bind)))
    }

    fn main_feed(&self, bind: &Binder) -> impl IntoElement {
        let custom_profile = self
            .feed
            .custom_feed_profile_id
            .as_ref()
            .and_then(|id| self.state.user_profiles.iter().find(|p| &p.id == id));
        let (reply_entity, doc_entity) = (bind.entity().clone(), bind.entity().clone());
        render_main_feed(FeedParams {
            state: &self.state,
            custom_profile,
            selected_post_id: self.feed.selected_thread_post_id.clone(),
            composer: self.feed.composer_view.clone(),
            doc_title_input: &self.feed.doc_title_input,
            doc_content_input: &self.feed.doc_content_input,
            theme: &self.theme,
            feed_tab: self.feed.feed_tab,
            inspector_expanded: self.feed.inspector_expanded,
            stream: self.feed.stream.inputs(bind.entity()),
            on_load_more: Some(bind.boxed_act(Self::load_more_posts)),
            on_select_feed_tab: bind.boxed(Self::select_feed_tab),
            on_toggle_replies: bind.boxed_act(Self::toggle_inspector_replies),
            on_select_post: bind.boxed(Self::select_post),
            on_like_post: bind.boxed(Self::like_post),
            on_reply_post: Box::new(move |post_id, _author_handle: &str, window, cx| {
                reply_entity.update(cx, |this, cx| this.reply_to_post(post_id, window, cx));
            }),
            on_repost_post: bind.boxed(Self::repost_post),
            on_quote_post: bind.boxed(Self::quote_post),
            on_delete_post: bind.boxed(Self::delete_post),
            on_open_tag: Rc::new(bind.with(|this, tag: String, window, cx| this.search_topic(&tag, window, cx))),
            on_open_wikilink: Rc::new(bind.with(Self::open_wikilink)),
            on_add_document: Box::new(move |title, content, window, cx| {
                doc_entity.update(cx, |this, cx| this.add_document(title, content, window, cx));
            }),
            on_delete_document: bind.boxed(|this, doc_id, _, cx| this.delete_document(doc_id, cx)),
            on_import_document: bind.boxed_act(Self::import_document),
            on_cycle_profile: bind.boxed_act(Self::cycle_profile),
        })
    }
}
