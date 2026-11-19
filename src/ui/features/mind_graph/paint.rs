//! Canvas painting of a graph scene: curved edges, kind-colored circular nodes and,
//! for focused nodes, text labels. Used by both the full view and the sidebar mini graph.

use super::geometry::{faded, max_degree, node_color, node_radius};
use super::viewport::{Fit, Frame, Viewport};
use crate::state::graph::GraphScene;
use crate::theme::ThemeTokens;
use gpui_kit::{fill, point, px, size, App, Bounds, Window};

/// Diameter as a multiple of the radius.
const DIAMETER_FACTOR: f32 = 2.0;

/// Per-view drawing parameters.
pub(crate) struct PaintOptions {
    /// Inner padding of the fitted layout, in pixels.
    pub padding: f32,
    /// Node radius range `(min, max)` in pixels.
    pub radius_range: (f32, f32),
    /// Whether focused nodes get text labels.
    pub labels: bool,
}

/// Hovered and selected node indices.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct Focus {
    pub hovered: Option<usize>,
    pub selected: Option<usize>,
}

/// One frame's worth of scene drawing.
pub(crate) struct ScenePainter<'a> {
    pub scene: &'a GraphScene,
    pub view: Viewport,
    pub frame: Frame,
    pub options: &'a PaintOptions,
    pub focus: Focus,
    pub theme: &'a ThemeTokens,
    /// The scene's node box fitted to the frame.
    fit: Fit,
    /// Per-node full-strength flag (the anchor and its neighbors, or everything).
    lit: Vec<bool>,
}

impl<'a> ScenePainter<'a> {
    pub(crate) fn new(
        scene: &'a GraphScene,
        (view, frame): (Viewport, Frame),
        options: &'a PaintOptions,
        focus: Focus,
        theme: &'a ThemeTokens,
    ) -> Self {
        let fit = Fit::new(options.padding, scene.layout.extent());
        let mut painter = Self { scene, view, frame, options, focus, theme, fit, lit: Vec::new() };
        painter.lit = painter.lit_mask();
        painter
    }

    fn lit_mask(&self) -> Vec<bool> {
        let count = self.scene.model.node_count();
        let Some(anchor) = self.anchor().filter(|&a| a < count) else {
            return vec![true; count];
        };
        let mut lit = vec![false; count];
        lit[anchor] = true;
        for edge in self.scene.model.edges() {
            if edge.source == anchor || edge.target == anchor {
                lit[edge.source] = true;
                lit[edge.target] = true;
            }
        }
        lit
    }

    /// Node whose neighborhood is highlighted: the selection, else the hovered node.
    pub(super) fn anchor(&self) -> Option<usize> {
        self.focus.selected.or(self.focus.hovered)
    }

    pub(super) fn screen(&self, idx: usize) -> (f32, f32) {
        let p = self.scene.layout.positions()[idx];
        self.view.to_screen(&self.frame, self.fit, p)
    }

    pub(super) fn radius(&self, idx: usize, top: usize) -> f32 {
        node_radius(self.scene.degrees[idx], top, self.options.radius_range)
    }

    /// Whether `idx` is drawn at full strength (no anchor, the anchor, or its neighbor).
    pub(super) fn is_lit(&self, idx: usize) -> bool {
        self.lit.get(idx).copied().unwrap_or(true)
    }

    /// Paints edges, then nodes, then labels.
    pub(crate) fn paint(&self, window: &mut Window, cx: &mut App) {
        if self.scene.model.node_count() == 0 {
            return;
        }
        self.paint_edges(window);
        let top = max_degree(self.scene);
        for idx in 0..self.scene.model.node_count() {
            self.paint_node(idx, top, window);
        }
        if self.options.labels {
            self.paint_labels(top, window, cx);
        }
    }

    fn paint_node(&self, idx: usize, top: usize, window: &mut Window) {
        let (x, y) = self.screen(idx);
        let r = self.radius(idx, top);
        let frame = (self.frame.x, self.frame.y, self.frame.width, self.frame.height);
        if !crate::ui::features::insights::chart::cull::point_visible((x, y), r, frame) {
            return;
        }
        let mut color = node_color(self.scene.model.nodes()[idx].node.kind(), self.theme);
        if !self.is_lit(idx) {
            color = faded(color, self.theme.graph_dim_opacity);
        }
        let diameter = px(r * DIAMETER_FACTOR);
        let bounds = Bounds::new(point(px(x - r), px(y - r)), size(diameter, diameter));
        let mut quad = fill(bounds, color).corner_radii(px(r));
        if self.focus.selected == Some(idx) {
            quad = quad
                .border_widths(self.theme.border_width_threadline)
                .border_color(self.theme.text_primary);
        }
        window.paint_quad(quad);
    }
}
