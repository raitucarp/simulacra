//! The Insights page's thinking views in the shell (sb E1): the Calibration snapshot (the
//! summary of every credence and the open predictions by due date) and the Tensions
//! snapshot, pushed to the page while it is open; the Tensions view scans when stale.

use super::super::view::AppView;
use crate::domain::thinking::credence::{open_by_due, summarize, Credence};
use crate::state::semantic::hits::shortened;
use crate::ui::features::insights::InsightsView;
use crate::ui::features::thinking::insights::{CalibrationData, PredictionRow, ThinkingInsights};
use chrono::Local;
use gpui_kit::*;

/// Characters of a post shown in the predictions list.
const EXCERPT_CHARS: usize = 90;

impl AppView {
    fn calibration_data(&self) -> CalibrationData {
        let today = Local::now().date_naive();
        let credences: Vec<Credence> = self.thinking.credences.values().cloned().collect();
        let open = open_by_due(&credences).into_iter().map(|credence| PredictionRow {
            post_id: credence.post_id.clone(),
            excerpt: self.state.get_post(&credence.post_id).map(|post| shortened(&post.content, EXCERPT_CHARS)).unwrap_or_default(),
            percent: credence.percent,
            due: credence.resolves_by,
            is_due: credence.is_due(today),
        });
        CalibrationData { summary: Some(summarize(&credences)), open: open.collect(), due_only: self.thinking.due_only }
    }

    /// Render hook (Insights page open): fresh data for the two views.
    pub(super) fn sync_thinking_insights(&mut self, cx: &mut Context<Self>) {
        if self.insights.panel.read(cx).state().view == InsightsView::Tensions {
            self.ensure_tension_scan(cx);
        }
        let data = ThinkingInsights { calibration: self.calibration_data(), tensions: self.tensions_data() };
        self.insights.panel.update(cx, |panel, cx| panel.set_thinking(data, cx));
    }
}
