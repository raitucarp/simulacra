//! Mind Graph wiring of the shell: the cached scene, the graph view entity, and the
//! handling of its select / open requests.

use super::view::AppView;
use crate::domain::graph::NodeRef;
use crate::state::graph::{GraphCache, GraphScene};
use crate::state::{AppState, NavigationTab};
use crate::ui::features::mind_graph::{MindGraphEvent, MindGraphPanel};
use gpui_kit::*;
use std::rc::Rc;

/// Graph scene cache (shared by the full view and the sidebar mini graph) and the view entity.
pub struct GraphState {
    pub cache: GraphCache,
    pub panel: Entity<MindGraphPanel>,
}

impl GraphState {
    /// Builds the initial scene and graph view, returning the view's event subscription.
    pub(super) fn new(state: &AppState, window: &mut Window, cx: &mut Context<AppView>) -> (Self, Subscription) {
        let mut cache = GraphCache::default();
        let scene = cache.scene(state);
        let theme = state.theme_tokens.clone();
        let panel = cx.new(|_| MindGraphPanel::new(scene, theme));
        let subscription = cx.subscribe_in(&panel, window, AppView::handle_mind_graph_event);
        (Self { cache, panel }, subscription)
    }
}

impl AppView {
    /// Current scene, rebuilt only when posts, documents, profiles or the network changed.
    pub(super) fn graph_scene(&mut self) -> Rc<GraphScene> {
        self.graph.cache.scene(&self.state)
    }

    /// Pushes the current scene and theme into the graph view entity.
    pub(super) fn sync_mind_graph(&mut self, cx: &mut Context<Self>) {
        let scene = self.graph_scene();
        let theme = &self.theme;
        self.graph.panel.update(cx, |panel, cx| panel.set_scene(scene, theme, cx));
    }

    /// Opens the full Mind Graph view.
    pub(crate) fn open_mind_graph(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.select_nav(NavigationTab::MindGraph, window, cx);
    }

    fn handle_mind_graph_event(
        &mut self,
        _: &Entity<MindGraphPanel>,
        event: &MindGraphEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            MindGraphEvent::Select(NodeRef::Post(id)) => self.select_post(id.clone(), window, cx),
            MindGraphEvent::Select(_) => {}
            MindGraphEvent::Open(node) => self.open_graph_node(node.clone(), window, cx),
        }
    }

    /// Navigates to where a graph node lives: thread, vault, tag search or lab profile.
    fn open_graph_node(&mut self, node: NodeRef, window: &mut Window, cx: &mut Context<Self>) {
        match node {
            NodeRef::Post(id) => {
                self.select_post(id, window, cx);
                self.select_nav(NavigationTab::Timeline, window, cx);
            }
            NodeRef::Document(_) => self.select_nav(NavigationTab::Knowledge, window, cx),
            NodeRef::Hashtag(tag) => {
                self.select_nav(NavigationTab::Timeline, window, cx);
                self.search_topic(&tag, window, cx);
            }
            NodeRef::Profile(id) => match self.state.user_profiles.iter().find(|p| p.id == id) {
                Some(profile) if profile.is_agent() => self.select_sidebar_agent(id, window, cx),
                Some(_) => {
                    self.select_nav(NavigationTab::PersonaLab, window, cx);
                    self.select_lab_profile(id, window, cx);
                }
                None => {}
            },
        }
    }
}
