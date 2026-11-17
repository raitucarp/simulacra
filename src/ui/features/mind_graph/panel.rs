//! The Mind Graph view entity: owns the viewport, hover and selection over a shared scene.

use super::paint::{Focus, PaintOptions};
use super::viewport::{Fit, Frame, Viewport};
use crate::domain::graph::{GraphNode, NodeRef};
use crate::state::graph::GraphScene;
use crate::theme::ThemeTokens;
use gpui_kit::{Context, EventEmitter};
use std::cell::Cell;
use std::rc::Rc;

/// Requests the shell handles on behalf of the graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MindGraphEvent {
    /// A node was clicked: select it (posts become the inspected thread).
    Select(NodeRef),
    /// A node was double-clicked or opened from the details bar.
    Open(NodeRef),
}

/// An in-progress drag: pointer and pan at press time, and whether it moved yet.
#[derive(Debug, Clone, Copy)]
pub(super) struct Drag {
    pub origin: (f32, f32),
    pub pan: (f32, f32),
    pub moved: bool,
}

/// Pan/zoom Mind Graph canvas with hover labels and click selection.
pub struct MindGraphPanel {
    pub(super) scene: Rc<GraphScene>,
    pub(super) theme: ThemeTokens,
    pub(super) view: Viewport,
    pub(super) hovered: Option<usize>,
    pub(super) selected: Option<NodeRef>,
    pub(super) drag: Option<Drag>,
    /// Canvas bounds of the last paint, for hit testing.
    pub(super) frame: Rc<Cell<Option<Frame>>>,
}

impl EventEmitter<MindGraphEvent> for MindGraphPanel {}

impl MindGraphPanel {
    pub fn new(scene: Rc<GraphScene>, theme: ThemeTokens) -> Self {
        Self {
            scene,
            theme,
            view: Viewport::default(),
            hovered: None,
            selected: None,
            drag: None,
            frame: Rc::new(Cell::new(None)),
        }
    }

    /// Swaps in a rebuilt scene and/or theme, repainting only when either changed.
    pub fn set_scene(&mut self, scene: Rc<GraphScene>, theme: &ThemeTokens, cx: &mut Context<Self>) {
        let scene_changed = !Rc::ptr_eq(&self.scene, &scene);
        let theme_changed = &self.theme != theme;
        if scene_changed {
            self.scene = scene;
            self.hovered = None;
        }
        if theme_changed {
            self.theme = theme.clone();
        }
        if scene_changed || theme_changed {
            cx.notify();
        }
    }

    /// Index of the selected node in the current scene.
    pub(super) fn selected_index(&self) -> Option<usize> {
        self.selected.as_ref().and_then(|node| self.scene.model.index_of(node))
    }

    pub(super) fn selected_node(&self) -> Option<&GraphNode> {
        self.selected_index().map(|idx| &self.scene.model.nodes()[idx])
    }

    pub(super) fn focus(&self) -> Focus {
        Focus { hovered: self.hovered, selected: self.selected_index() }
    }

    /// Inner padding of the fitted layout.
    pub(super) fn padding(&self) -> f32 {
        self.theme.spacing_xl.as_f32()
    }

    /// How the scene's nodes are fitted to the canvas.
    pub(super) fn fit(&self) -> Fit {
        Fit::new(self.padding(), self.scene.layout.extent())
    }

    pub(super) fn radius_range(&self) -> (f32, f32) {
        (self.theme.graph_node_radius_min.as_f32(), self.theme.graph_node_radius_max.as_f32())
    }

    pub(super) fn options(&self) -> PaintOptions {
        PaintOptions { padding: self.padding(), radius_range: self.radius_range(), labels: true }
    }

    /// Fits the whole graph to the canvas (on opening; it stays fitted while resizing until
    /// the user pans or zooms).
    pub fn fit_to_view(&mut self, cx: &mut Context<Self>) {
        self.reset_view(cx);
    }

    /// Restores the fitted, unzoomed view.
    pub(super) fn reset_view(&mut self, cx: &mut Context<Self>) {
        self.view = Viewport::default();
        cx.notify();
    }

    /// Emits [`MindGraphEvent::Open`] for the selected node.
    pub(super) fn open_selected(&mut self, cx: &mut Context<Self>) {
        if let Some(node) = self.selected_node().map(|n| n.node.clone()) {
            cx.emit(MindGraphEvent::Open(node));
        }
    }
}
