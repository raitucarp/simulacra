//! Insights requests (sb D2): opening the page or one of its views, the author filter's
//! list, and the page's events (save its state, open a post, a graph node or an author).

use super::super::view::AppView;
use crate::domain::graph::NodeRef;
use crate::state::NavigationTab;
use crate::storage::SettingKey;
use crate::ui::features::command_palette::CommandId;
use crate::ui::features::insights::{InsightsEvent, InsightsPanel, InsightsView};
use gpui_kit::*;
use std::collections::HashMap;

impl AppView {
    /// The network's authors, busiest first, for the author filter.
    pub(in crate::ui::features::shell) fn sync_insights_authors(&mut self, cx: &mut Context<Self>) {
        let fingerprint = self.insights_data_fingerprint();
        if self.insights.authors_for == Some(fingerprint) {
            return;
        }
        self.insights.authors_for = Some(fingerprint);
        let mut counts: HashMap<&str, usize> = HashMap::new();
        self.state.current_network_posts().for_each(|p| *counts.entry(p.author_id.as_str()).or_default() += 1);
        let mut authors: Vec<(String, String, usize)> = self.state.user_profiles.iter().filter_map(|p| counts.get(p.id.as_str()).map(|c| (p.id.as_str().to_string(), p.name.clone(), *c))).collect();
        authors.sort_by(|a, b| b.2.cmp(&a.2).then_with(|| a.1.cmp(&b.1)));
        let list = authors.into_iter().map(|(id, name, _)| (id, name)).collect();
        self.insights.panel.update(cx, |panel, cx| panel.set_authors(list, cx));
    }

    /// Opens the page (and the view of a view command).
    pub(in crate::ui::features::shell) fn open_insights(&mut self, command: CommandId, window: &mut Window, cx: &mut Context<Self>) {
        self.select_nav(NavigationTab::Insights, window, cx);
        if let Some(view) = InsightsView::of_command(command) {
            self.insights.panel.update(cx, |panel, cx| panel.show(view, cx));
        }
    }

    pub(in crate::ui::features::shell) fn handle_insights_event(&mut self, _: &Entity<InsightsPanel>, event: &InsightsEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event.clone() {
            InsightsEvent::Changed(page) => {
                if let Some(db) = &self.state.db {
                    if let Err(err) = db.set_setting(SettingKey::InsightsState, &page.to_json()) {
                        eprintln!("simulacra: insights state not saved: {err}");
                    }
                }
                cx.notify();
            }
            InsightsEvent::OpenPost(id) => self.reveal_post(id, window, cx),
            InsightsEvent::OpenNode(node) => self.open_graph_node(node, window, cx),
            InsightsEvent::OpenAuthor(id) => self.open_graph_node(NodeRef::Profile(id), window, cx),
            InsightsEvent::Thinking(request) => self.on_thinking_request(request, window, cx),
        }
    }
}
