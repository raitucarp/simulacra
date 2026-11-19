//! Painting the network: culled, level-of-detail edges (straight and skipped when shorter than
//! a pixel on large graphs), culled nodes sized by the chosen measure and coloured by
//! community, kind or measure, and labels for the most important visible nodes that fit.

use super::colours::{legend_slot, node_colour};
use crate::domain::insights::InsightsReport;
use crate::theme::ThemeTokens;
use crate::ui::features::insights::chart::cull::{label_budget, place_labels, point_visible, segment_visible, too_short, Rect};
use crate::ui::features::insights::chart::paint::{dot, label, LabelStyle, StrokeBatch};
use crate::ui::features::insights::chart::palette::faded;
use crate::ui::features::insights::chart::projection::Projected;
use crate::ui::features::insights::panel::keys::node_radius;
use crate::ui::features::insights::state::PageState;
use gpui_kit::*;
use std::collections::HashSet;

/// Above this many edges the cheap level of detail kicks in.
const DETAILED_EDGES: usize = 3_000;
/// Shortest edge (pixels) still stroked in the cheap level of detail.
const MIN_EDGE_PIXELS: f32 = 1.5;
/// Label candidates considered (by importance).
const LABEL_CANDIDATES: usize = 240;
/// Share of the canvas that labels may cover.
const LABEL_COVER: f32 = 0.18;
/// Average glyph width as a share of the font size (for label boxes).
const GLYPH_SHARE: f32 = 0.56;

/// One frame of network drawing.
pub struct NetworkPainter<'a> {
    pub report: &'a InsightsReport,
    pub projected: &'a Projected,
    pub state: &'a PageState,
    pub frame: Rect,
    /// The picked or hovered node.
    pub anchor: Option<usize>,
    pub hidden: &'a HashSet<usize>,
    pub theme: &'a ThemeTokens,
}

impl NetworkPainter<'_> {
    fn radius(&self, i: usize) -> f32 {
        let metrics = &self.report.network.metrics;
        let share = metrics.value(self.state.measure, i) / metrics.max(self.state.measure);
        node_radius(share, (self.theme.insights_node_radius_min.as_f32(), self.theme.insights_node_radius_max.as_f32()))
    }

    fn shown(&self, i: usize) -> bool {
        !self.hidden.contains(&legend_slot(self.report, self.state.network_colour, i))
    }

    fn neighbours(&self) -> HashSet<usize> {
        let graph = &self.report.network.graph;
        self.anchor.map_or_else(HashSet::new, |a| graph.neighbours(a).iter().map(|&w| w as usize).chain([a]).collect())
    }

    pub fn paint(&self, window: &mut Window, cx: &mut App) {
        let lit = self.neighbours();
        self.paint_edges(window);
        for (i, &at) in self.projected.points.iter().enumerate() {
            let r = self.radius(i);
            if !self.shown(i) || !point_visible(at, r, self.frame) {
                continue;
            }
            let colour = node_colour(self.report, self.state, i, self.theme);
            let colour = if lit.is_empty() || lit.contains(&i) { colour } else { faded(colour, self.theme.graph_dim_opacity) };
            let ring = (self.anchor == Some(i)).then_some((self.theme.border_width_threadline, self.theme.text_primary));
            dot(window, at, r, colour, ring);
        }
        self.paint_labels(&lit, window, cx);
    }

    fn paint_edges(&self, window: &mut Window) {
        let graph = &self.report.network.graph;
        let cheap = graph.edge_count() > DETAILED_EDGES;
        let base = if self.anchor.is_some() { faded(self.theme.graph_edge, self.theme.graph_dim_opacity) } else { self.theme.graph_edge };
        let (mut dim, mut lit) = (StrokeBatch::new(self.theme.insights_edge_width, base), StrokeBatch::new(self.theme.insights_edge_width, self.theme.accent_brand));
        for (a, b) in graph.edges() {
            let (pa, pb) = (self.projected.points[a], self.projected.points[b]);
            if !self.shown(a) || !self.shown(b) || !segment_visible(pa, pb, self.frame) || (cheap && too_short(pa, pb, MIN_EDGE_PIXELS)) {
                continue;
            }
            let touches = self.anchor.is_some_and(|x| x == a || x == b);
            if touches { lit.line(pa, pb, window) } else { dim.line(pa, pb, window) }
        }
        dim.finish(window);
        lit.finish(window);
    }

    fn label_style(&self) -> LabelStyle {
        let t = self.theme;
        LabelStyle { family: t.font_family_ui.clone().into(), size: t.font_size_2xs, line_height: t.font_size_2xs * t.line_height_body, colour: t.text_secondary, max_width: t.insights_label_max_width }
    }

    /// Labels: the anchor and its neighbours first, then the biggest visible nodes.
    fn paint_labels(&self, lit: &HashSet<usize>, window: &mut Window, cx: &mut App) {
        let style = self.label_style();
        let (glyph, height) = (style.size.as_f32() * GLYPH_SHARE, style.line_height.as_f32());
        let max_width = style.max_width.as_f32();
        let labels = &self.report.network.labels;
        let mut order: Vec<usize> = (0..self.projected.points.len()).filter(|&i| self.shown(i) && point_visible(self.projected.points[i], 0.0, self.frame)).collect();
        order.sort_by(|&a, &b| lit.contains(&b).cmp(&lit.contains(&a)).then(self.radius(b).total_cmp(&self.radius(a))).then(a.cmp(&b)));
        order.truncate(LABEL_CANDIDATES);
        let boxes: Vec<(usize, Rect)> = order
            .iter()
            .map(|&i| {
                let (x, y) = self.projected.points[i];
                let width = (labels[i].chars().count() as f32 * glyph).min(max_width);
                (i, (x - width / 2.0, y + self.radius(i) + 1.0, width, height))
            })
            .collect();
        let budget = label_budget(self.frame, (max_width, height), LABEL_COVER);
        for i in place_labels(&boxes, self.frame, budget) {
            let rect = boxes.iter().find(|(j, _)| *j == i).map(|(_, r)| *r).unwrap_or_default();
            label(window, cx, &labels[i], (rect.0, rect.1), &style);
        }
    }
}
