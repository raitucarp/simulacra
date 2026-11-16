//! Cached Mind Graph scene (model + layout) of the active network.
//!
//! The force layout is computed off the UI thread and only rebuilt when the cache key
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
    /// Builds and lays out the graph of the active network (synchronously).
    pub fn of(state: &AppState) -> Self {
        LayoutJob::of(state).run()
    }
}

/// The force layout of a freshly built model, runnable on a background thread.
#[derive(Debug)]
pub struct LayoutJob {
    model: GraphModel,
}

impl LayoutJob {
    /// Builds the model of the active network (cheap); the layout is left to [`Self::run`].
    fn of(state: &AppState) -> Self {
        let model = GraphModel::build(&GraphSource {
            posts: &state.posts,
            documents: &state.documents,
            profiles: &state.user_profiles,
            network: &state.active_network_id,
        });
        Self { model }
    }

    /// Runs the (expensive) force layout.
    pub fn run(self) -> GraphScene {
        let layout = GraphLayout::compute(&self.model);
        let degrees = self.model.degrees();
        GraphScene { model: self.model, layout, degrees }
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
///
/// [`Self::scene`] lays out synchronously (tests, tools). The UI uses [`Self::refresh`] to get a
/// [`LayoutJob`] for a background thread and [`Self::install`] to swap the result in; until
/// then [`Self::current`] keeps serving the previous scene.
#[derive(Default)]
pub struct GraphCache {
    key: Option<SceneKey>,
    scene: Rc<GraphScene>,
    /// Generation of the newest requested layout, and of the installed one.
    requested: u64,
    installed: u64,
}

impl GraphCache {
    /// The scene for the current store contents, rebuilt only when the key changed.
    pub fn scene(&mut self, state: &AppState) -> Rc<GraphScene> {
        if let Some((generation, job)) = self.refresh(state) {
            self.install(generation, job.run());
        }
        Rc::clone(&self.scene)
    }

    /// A layout job (with its generation) when the store changed since the last request.
    pub fn refresh(&mut self, state: &AppState) -> Option<(u64, LayoutJob)> {
        let key = SceneKey::of(state);
        if self.key.as_ref() == Some(&key) {
            return None;
        }
        self.key = Some(key);
        self.requested += 1;
        Some((self.requested, LayoutJob::of(state)))
    }

    /// Installs a finished layout unless a newer one was installed already; returns whether
    /// the scene changed.
    pub fn install(&mut self, generation: u64, scene: GraphScene) -> bool {
        if generation <= self.installed {
            return false;
        }
        self.installed = generation;
        self.scene = Rc::new(scene);
        true
    }

    /// Whether a requested layout has not been installed yet.
    pub fn is_pending(&self) -> bool {
        self.installed < self.requested
    }

    /// The most recently installed scene (empty before the first layout lands).
    pub fn current(&self) -> Rc<GraphScene> {
        Rc::clone(&self.scene)
    }
}
