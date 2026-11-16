//! Mind Graph wiring of the shell: the cached scene, the graph view entity, and the
//! handling of its select / open requests.

use super::view::AppView;
use crate::domain::graph::NodeRef;
use crate::state::graph::{GraphCache, GraphScene};
use crate::state::{AppState, NavigationTab};
use crate::ui::features::mind_graph::{MindGraphEvent, MindGraphPanel};
use gpui_kit::*;

/// Graph scene cache (shared by the full view and the sidebar mini graph) and the view entity.
pub struct GraphState {
    pub cache: GraphCache,
    pub panel: Entity<MindGraphPanel>,
}

impl GraphState {
    /// Creates the graph view (empty until the first background layout lands), returning
    /// the view's event subscription.
    pub(super) fn new(state: &AppState, window: &mut Window, cx: &mut Context<AppView>) -> (Self, Subscription) {
        let cache = GraphCache::default();
        let (scene, theme) = (cache.current(), state.theme_tokens.clone());
        let panel = cx.new(|_| MindGraphPanel::new(scene, theme));
        let subscription = cx.subscribe_in(&panel, window, AppView::handle_mind_graph_event);
        (Self { cache, panel }, subscription)
    }
}

impl AppView {
    /// Starts a background layout when posts, documents, profiles or the network changed;
    /// the finished scene is swapped in (the previous one stays on screen meanwhile).
    fn request_graph_layout(&mut self, cx: &mut Context<Self>) {
        let Some((generation, job)) = self.graph.cache.refresh(&self.state) else { return };
        let layout = cx.background_executor().spawn(async move { job.run() });
        cx.spawn(async move |this, cx| {
            let scene: GraphScene = layout.await;
            let _ = this.update(cx, |this, cx| {
                if this.graph.cache.install(generation, scene) {
                    cx.notify();
                }
            });
        })
        .detach();
    }

    /// Pushes the current scene and theme into the graph view entity.
    pub(super) fn sync_mind_graph(&mut self, cx: &mut Context<Self>) {
        self.request_graph_layout(cx);
        let scene = self.graph.cache.current();
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
