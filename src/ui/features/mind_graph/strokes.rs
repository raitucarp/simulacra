//! Curved edge strokes: dimmed background edges and highlighted edges of the focused node.

use super::geometry::{control_point, faded};
use super::paint::{to_point, ScenePainter};
use gpui_kit::{PathBuilder, Rgba, Window};

impl ScenePainter<'_> {
    pub(super) fn paint_edges(&self, window: &mut Window) {
        let width = self.theme.graph_edge_width;
        let (mut dim, mut lit) = (PathBuilder::stroke(width), PathBuilder::stroke(width));
        let (mut dim_count, mut lit_count) = (0usize, 0usize);
        let anchor = self.anchor();
        for edge in self.scene.model.edges() {
            let (a, b) = (self.screen(edge.source), self.screen(edge.target));
            let ctrl = control_point(a, b, self.theme.graph_curve_bend);
            let touches = anchor.is_some_and(|f| edge.source == f || edge.target == f);
            let (builder, count) = if touches { (&mut lit, &mut lit_count) } else { (&mut dim, &mut dim_count) };
            builder.move_to(to_point(a));
            builder.curve_to(to_point(b), to_point(ctrl));
            *count += 1;
        }
        let base = match anchor {
            Some(_) => faded(self.theme.graph_edge, self.theme.graph_dim_opacity),
            None => self.theme.graph_edge,
        };
        stroke(window, dim, dim_count, base);
        stroke(window, lit, lit_count, self.theme.accent_brand);
    }
}

/// Strokes a built path when it has at least one segment.
fn stroke(window: &mut Window, builder: PathBuilder, segments: usize, color: Rgba) {
    if segments == 0 {
        return;
    }
    if let Ok(path) = builder.build() {
        window.paint_path(path, color);
    }
}
