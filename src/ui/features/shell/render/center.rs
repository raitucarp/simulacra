//! Center column: the main feed (timeline), the Knowledge Vault, the Mind Graph or the agent workspace.

use super::super::binder::Binder;
use super::super::view::AppView;
use crate::state::NavigationTab;
use crate::ui::features::day_filter::render_day_filter_chip;
use crate::ui::features::feed::inspector_outline::InspectorView;
use crate::ui::features::feed::{render_main_feed, FeedParams};
use crate::ui::responsive::LayoutClass;
use gpui_kit::*;
use std::rc::Rc;

impl AppView {
    /// The Agent Forge while it is open, the roster while a lab tab is active, the Mind Graph
    /// view on its tab, otherwise the main feed.
    pub(in crate::ui::features::shell) fn center_panel(&self, bind: &Binder, class: LayoutClass) -> AnyElement {
        if let Some(session) = &self.forge {
            session.view.clone().into_any_element()
        } else if self.is_lab_tab() {
            self.agent_workspace(bind).into_any_element()
        } else if self.state.active_tab == NavigationTab::MindGraph {
            self.graph.panel.clone().into_any_element()
        } else if self.state.active_tab == NavigationTab::Knowledge {
            self.knowledge_page(bind)
        } else if self.state.active_tab == NavigationTab::Review {
            self.review.panel.clone().into_any_element()
        } else if self.state.active_tab == NavigationTab::Insights {
            self.insights.panel.clone().into_any_element()
        } else if self.state.active_tab == NavigationTab::Journal {
            self.journal.panel.clone().into_any_element()
        } else if self.state.active_tab == NavigationTab::Arena {
            self.arena.panel.clone().into_any_element()
        } else {
            self.filtered_feed(bind, class).into_any_element()
        }
    }

    /// The main feed, topped by the "Showing <date> ×" chip while a day filter is active.
    fn filtered_feed(&self, bind: &Binder, class: LayoutClass) -> impl IntoElement {
        let day = self.state.feed_day_filter.filter(|_| self.state.active_tab == NavigationTab::Timeline);
        let chip = day.map(|day| render_day_filter_chip(day, bind.boxed_act(Self::clear_day_filter), &self.theme));
        div()
            .size_full()
            .flex()
            .flex_col()
            .children(chip)
            .child(div().flex_1().min_h_0().w_full().child(self.main_feed(bind, class)))
    }

    fn main_feed(&self, bind: &Binder, layout: LayoutClass) -> impl IntoElement {
        let custom_profile = self
            .feed
            .custom_feed_profile_id
            .as_ref()
            .and_then(|id| self.state.user_profiles.iter().find(|p| &p.id == id));
        let (reply_entity, learn_entity) = (bind.entity().clone(), bind.entity().clone());
        render_main_feed(FeedParams {
            state: &self.state,
            custom_profile,
            selected_post_id: self.feed.selected_thread_post_id.clone(),
            composer: self.feed.composer_view.clone(),
            theme: &self.theme,
            feed_tab: self.feed.feed_tab,
            inspector_expanded: self.feed.inspector_expanded, layout,
            stream: self.feed.stream.inputs(bind.entity()),
            on_load_more: Some(bind.boxed_act(Self::load_more_posts)),
            on_select_feed_tab: bind.boxed(Self::select_feed_tab),
            on_toggle_replies: bind.boxed_act(Self::toggle_inspector_replies),
            on_select_post: bind.boxed(Self::select_post),
            on_like_post: bind.boxed(Self::like_post),
            on_reply_post: Box::new(move |post_id, _author_handle: &str, window, cx| {
                reply_entity.update(cx, |this, cx| this.reply_to_post(post_id, window, cx));
            }),
            on_repost_post: bind.boxed(Self::repost_post), on_quote_post: bind.boxed(Self::quote_post),
            on_delete_post: bind.boxed(Self::delete_post),
            on_find_similar_post: bind.boxed(Self::find_similar_post),
            on_learn_post: Box::new(move |post_id, kind, window, cx| learn_entity.update(cx, |this, cx| this.learn_post_menu(post_id, kind, window, cx))),
            learning_posts: self.learning_posts(),
            on_open_tag: Rc::new(bind.with(|this, tag: String, window, cx| this.search_topic(&tag, window, cx))),
            on_open_wikilink: Rc::new(bind.with(Self::open_wikilink)),
            on_open_source: Rc::new(bind.with(Self::open_source)),
            on_open_media: Rc::new(bind.with(Self::open_lightbox)),
            on_cycle_profile: bind.boxed_act(Self::cycle_profile),
            on_run_command: Rc::new(bind.with(Self::run_command)),
            on_post_action: Self::post_action_callback(bind), credence: Some(self.credence_source(bind)), data: Some(self.data_source(bind)), reactions: Some(self.reaction_source(bind)),
            inspector: InspectorView { argument: Some(self.thinking.argument.clone()), ..self.inspector.view() },
            on_inspector: Rc::new(bind.with(Self::run_inspector_command)), discovery: self.feed_discovery(bind),
        })
    }
}
