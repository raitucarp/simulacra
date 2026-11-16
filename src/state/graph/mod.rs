//! Cached Mind Graph scene (model + layout) of the active network.
//!
//! The force layout is computed outside render and only rebuilt when the cache key
//! (network, collection sizes and newest post/document) changes.

#[cfg(test)]
mod tests;

use super::AppState;
use crate::domain::graph::layout::GraphLayout;
use crate::domain::graph::{GraphModel, GraphSource};
use crate::domain::{NetworkId, PostId};
use std::rc::Rc;

/// A graph with its layout and per-node degrees, shared by the full and mini views.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GraphScene {
    pub model: GraphModel,
    pub layout: GraphLayout,
    pub degrees: Vec<usize>,
}

impl GraphScene {
    /// Builds and lays out the graph of the active network.
    pub fn of(state: &AppState) -> Self {
        let model = GraphModel::build(&GraphSource {
            posts: &state.posts,
            documents: &state.documents,
            profiles: &state.user_profiles,
            network: &state.active_network_id,
        });
        let layout = GraphLayout::compute(&model);
        let degrees = model.degrees();
        Self { model, layout, degrees }
    }
}

/// Inputs whose change invalidates the cached scene.
#[derive(Debug, Clone, PartialEq, Eq)]
struct SceneKey {
    network: NetworkId,
    posts: usize,
    documents: usize,
    profiles: usize,
    newest_post: Option<PostId>,
    newest_document: Option<String>,
}

impl SceneKey {
    fn of(state: &AppState) -> Self {
        Self {
            network: state.active_network_id.clone(),
            posts: state.posts.len(),
            documents: state.documents.len(),
            profiles: state.user_profiles.len(),
            newest_post: state.current_network_posts().next().map(|p| p.id.clone()),
            newest_document: state.current_documents().next().map(|d| d.id.clone()),
        }
    }
}

/// Memoizes the [`GraphScene`] of the active network.
#[derive(Default)]
pub struct GraphCache {
    key: Option<SceneKey>,
    scene: Rc<GraphScene>,
}

impl GraphCache {
    /// The scene for the current store contents, rebuilt only when the key changed.
    pub fn scene(&mut self, state: &AppState) -> Rc<GraphScene> {
        let key = SceneKey::of(state);
        if self.key.as_ref() != Some(&key) {
            self.scene = Rc::new(GraphScene::of(state));
            self.key = Some(key);
        }
        Rc::clone(&self.scene)
    }

    /// The most recently built scene (empty before the first [`Self::scene`] call).
    pub fn current(&self) -> Rc<GraphScene> {
        Rc::clone(&self.scene)
    }
}
