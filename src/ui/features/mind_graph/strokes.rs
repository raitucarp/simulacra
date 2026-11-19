//! Curved edge strokes: dimmed background edges and highlighted edges of the focused node.
//! Edges whose curve box misses the canvas are skipped, and strokes go out in bounded batches (one path
//! for thousands of edges would overflow the tessellator's 16-bit indices and vanish).

use super::geometry::{control_point, faded};
use super::paint::ScenePainter;
use crate::ui::features::insights::chart::cull::segment_visible;
use crate::ui::features::insights::chart::paint::StrokeBatch;
use gpui_kit::Window;

impl ScenePainter<'_> {
    pub(super) fn paint_edges(&self, window: &mut Window) {
        let width = self.theme.graph_edge_width;
        let anchor = self.anchor();
        let base = match anchor {
            Some(_) => faded(self.theme.graph_edge, self.theme.graph_dim_opacity),
            None => self.theme.graph_edge,
        };
        let (mut dim, mut lit) = (StrokeBatch::new(width, base), StrokeBatch::new(width, self.theme.accent_brand));
        let frame = (self.frame.x, self.frame.y, self.frame.width, self.frame.height);
        for edge in self.scene.model.edges() {
            let (a, b) = (self.screen(edge.source), self.screen(edge.target));
            let ctrl = control_point(a, b, self.theme.graph_curve_bend);
            let low = (a.0.min(b.0).min(ctrl.0), a.1.min(b.1).min(ctrl.1));
            let high = (a.0.max(b.0).max(ctrl.0), a.1.max(b.1).max(ctrl.1));
            if !segment_visible(low, high, frame) {
                continue;
            }
            let touches = anchor.is_some_and(|f| edge.source == f || edge.target == f);
            let batch = if touches { &mut lit } else { &mut dim };
            batch.curve(a, b, ctrl, window);
        }
        dim.finish(window);
        lit.finish(window);
    }
}
