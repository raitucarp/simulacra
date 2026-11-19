//! Keyboard use of the network canvas: arrows walk the nodes in ranking order, Enter opens
//! the picked node, Escape clears the pick, `0` refits the view. Also the mark sizes shared
//! by painting and hit testing.

use super::interact::CanvasSurface;
use super::{Hover, InsightsPanel};
use crate::domain::insights::graph::metrics::Measure;
use gpui_kit::*;

const NEXT_KEYS: [&str; 2] = ["right", "down"];
const PREVIOUS_KEYS: [&str; 2] = ["left", "up"];
const OPEN_KEY: &str = "enter";
const CLEAR_KEY: &str = "escape";
const REFIT_KEY: &str = "0";

/// Radius of a node whose measure is `share` (0–1) of the largest, between `min` and `max`
/// (square-root growth, so area tracks the value).
pub fn node_radius(share: f64, (min, max): (f32, f32)) -> f32 {
    min + (max - min) * (share.clamp(0.0, 1.0).sqrt() as f32)
}

impl InsightsPanel {
    /// Radius of mark `i` on `surface`.
    pub(in crate::ui::features::insights) fn mark_radius(&self, surface: CanvasSurface, i: usize) -> f32 {
        let theme = &self.theme;
        match (surface, &self.report) {
            (CanvasSurface::Network, Some(report)) => {
                let measure = self.state.measure;
                let share = report.network.metrics.value(measure, i) / report.network.metrics.max(measure);
                node_radius(share, (theme.insights_node_radius_min.as_f32(), theme.insights_node_radius_max.as_f32()))
            }
            _ => theme.insights_point_radius.as_f32(),
        }
    }

    /// Nodes in ranking order of the current measure (ties by index).
    fn ranking(&self) -> Vec<usize> {
        let Some(report) = &self.report else { return Vec::new() };
        let metrics = &report.network.metrics;
        let measure = if self.state.measure == Measure::Clustering { Measure::Degree } else { self.state.measure };
        metrics.top(measure, report.network.nodes.len(), |_| true).into_iter().map(|(i, _)| i).collect()
    }

    /// Moves the pick `step` places along the ranking.
    fn step_selection(&mut self, step: isize) {
        let ranking = self.ranking();
        if ranking.is_empty() {
            return;
        }
        let at = self.selected.and_then(|s| ranking.iter().position(|&i| i == s));
        let next = match at {
            Some(at) => (at as isize + step).rem_euclid(ranking.len() as isize) as usize,
            None => 0,
        };
        self.selected = Some(ranking[next]);
        self.hover = Hover::None;
    }

    pub(in crate::ui::features::insights) fn on_network_key(&mut self, event: &KeyDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        let key = event.keystroke.key.as_str();
        if event.keystroke.modifiers.modified() {
            return;
        }
        if NEXT_KEYS.contains(&key) {
            self.step_selection(1);
        } else if PREVIOUS_KEYS.contains(&key) {
            self.step_selection(-1);
        } else if key == OPEN_KEY {
            if let Some(i) = self.selected {
                self.open_node(i, cx);
            }
        } else if key == CLEAR_KEY {
            self.selected = None;
        } else if key == REFIT_KEY {
            self.network.reset();
        } else {
            return;
        }
        cx.stop_propagation();
        cx.notify();
    }
}
