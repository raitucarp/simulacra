//! The statistics screen: the due forecast, retention over time, items by state, the review
//! heatmap and confidence calibration. Two columns in the wide layout class, one otherwise;
//! every chart measures its container.

mod bars;
mod breakdown;
mod charts;
mod heatmap;

use super::super::panel::ReviewPanel;
use crate::ui::responsive::LayoutClass;
use gpui_kit::*;

impl ReviewPanel {
    pub(in crate::ui::features::review) fn render_stats(&self, class: LayoutClass) -> impl IntoElement {
        let theme = &self.theme;
        let panels: Vec<Div> = vec![self.forecast_panel(), self.retention_panel(), self.states_panel(), self.calibration_panel()];
        let grid = match class {
            LayoutClass::Wide => {
                let mut panels = panels.into_iter();
                let mut rows = Vec::new();
                while let Some(first) = panels.next() {
                    let second = panels.next().unwrap_or_else(|| div());
                    rows.push(div().w_full().flex().gap(theme.section_gap).child(first.flex_1().min_w_0()).child(second.flex_1().min_w_0()));
                }
                div().flex().flex_col().gap(theme.section_gap).children(rows)
            }
            LayoutClass::Medium | LayoutClass::Compact => div().flex().flex_col().gap(theme.section_gap).children(panels),
        };
        div().flex().flex_col().gap(theme.section_gap).child(grid).child(self.heatmap_panel())
    }
}
