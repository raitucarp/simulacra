//! The gap radar tool: for every concept (or one goal's), the six dimensions as a radar (the
//! average and the selected concept), and the weakest concepts ranked with one-click actions.

pub mod geometry;
mod list;
mod paint;

use super::super::panel::ReviewPanel;
use super::super::render::common::{block_title, caption, control_row, surface};
use super::super::text::tr;
use super::render::chip;
use super::LearnTool;
use crate::domain::learning::{GapDimension, GapReport, GoalId};
use crate::ui::features::insights::chart::legend::{legend, LegendEntry};
use crate::ui::features::insights::chart::summary::summary_line;
use crate::ui::icons::LucideIcon;
use crate::ui::responsive::LayoutClass;
use gpui_kit::*;
use paint::{radar_canvas, RadarSeries};
use rust_i18n::t;

const SQUARE: f32 = 1.0;
const SCOPE_ID_PREFIX: &str = "learning-gaps-scope-";
const RADAR_ID: &str = "learning-gaps-radar";

impl ReviewPanel {
    /// Shows the radar of `goal` (every concept without one).
    pub(in crate::ui::features::review) fn open_gaps_for(&mut self, goal: Option<GoalId>, cx: &mut Context<Self>) {
        self.learning.gaps_goal = goal;
        self.learning.gaps_selected = None;
        self.open_tool(LearnTool::Gaps, cx);
    }

    pub(super) fn select_concept(&mut self, concept: String, cx: &mut Context<Self>) {
        self.learning.gaps_selected = Some(concept);
        cx.notify();
    }

    fn scope_chips(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let all = chip(ElementId::NamedInteger(SCOPE_ID_PREFIX.into(), 0), tr("learning.gaps.all_concepts"), self.learning.gaps_goal.is_none(), theme).on_click(cx.listener(|this, _, _, cx| this.open_gaps_for(None, cx)));
        let goals = self.data.learning.goals.iter().enumerate().map(|(index, view)| {
            let id = view.goal.id.clone();
            let active = self.learning.gaps_goal.as_ref() == Some(&id);
            chip(ElementId::NamedInteger(SCOPE_ID_PREFIX.into(), index as u64 + 1), view.goal.title.clone(), active, theme).on_click(cx.listener(move |this, _, _, cx| this.open_gaps_for(Some(id.clone()), cx)))
        });
        control_row(theme).child(all).children(goals)
    }

    fn radar_card(&self, report: &GapReport) -> Div {
        let theme = &self.theme;
        let selected = self.learning.gaps_selected.as_ref().and_then(|concept| report.entry(concept)).or_else(|| report.entries.first());
        let mut series = vec![RadarSeries { colour: theme.accent_brand, scores: report.average }];
        let mut entries = vec![LegendEntry { colour: theme.accent_brand, label: tr("learning.gaps.average"), value: None, hidden: false }];
        if let Some(entry) = selected {
            series.push(RadarSeries { colour: theme.accent_danger, scores: entry.scores });
            entries.push(LegendEntry { colour: theme.accent_danger, label: entry.concept.clone(), value: None, hidden: false });
        }
        let labels: Vec<String> = GapDimension::ALL.iter().map(|d| tr(&format!("learning.gaps.dimension.{}", d.key()))).collect();
        let canvas = div().id(RADAR_ID).relative().w_full().min_w(theme.learning_radar_min).max_w(theme.learning_radar_max).aspect_ratio(SQUARE).child(radar_canvas(series, labels, theme.clone()));
        let summary = selected.map_or_else(|| tr("learning.gaps.empty"), |entry| {
            t!("learning.gaps.summary", concept = entry.concept.clone(), dimension = tr(&format!("learning.gaps.dimension.{}", entry.weakest().key()))).to_string()
        });
        surface(theme)
            .child(block_title(LucideIcon::Gauge, theme.accent_dialectic, tr("learning.gaps.radar"), theme))
            .child(div().w_full().flex().justify_center().child(canvas))
            .child(legend("learning-gaps-legend", &entries, None, theme))
            .child(summary_line(summary, theme))
    }

    pub(in crate::ui::features::review) fn render_gaps(&self, class: LayoutClass, cx: &mut Context<Self>) -> AnyElement {
        let theme = &self.theme;
        let report = self.data.learning.radar(self.learning.gaps_goal.as_ref()).clone();
        let head = surface(theme)
            .child(block_title(LucideIcon::Gauge, theme.accent_brand, tr("learning.gaps.title"), theme))
            .child(caption(tr("learning.gaps.intro"), theme))
            .child(self.scope_chips(cx));
        if report.entries.is_empty() {
            return div().w_full().flex().flex_col().gap(theme.section_gap).child(head).child(surface(theme).child(caption(tr("learning.gaps.empty"), theme))).into_any_element();
        }
        let side = if class.is_compact() { Pixels::ZERO } else { theme.learning_side_width };
        let body = div().w_full().flex().flex_wrap().gap(theme.section_gap).child(div().flex_1().min_w(side).child(self.radar_card(&report))).child(div().flex_1().min_w(side).child(self.render_gap_list(&report, cx)));
        div().w_full().flex().flex_col().gap(theme.section_gap).child(head).child(body).into_any_element()
    }
}
