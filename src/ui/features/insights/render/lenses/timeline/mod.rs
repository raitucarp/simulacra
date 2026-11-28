//! The Concept timeline view: a lane chart of the busiest concepts over time (drag across it
//! to zoom into a range, click a lane to pick its concept), the concepts' phases as a ranked
//! list (keyboard reachable), and the picked concept's mentions in order with the concepts
//! mentioned alongside it per period.

mod detail;
mod interact;
mod paint;

use super::super::common::{card, card_title, pair, tr};
use super::super::Room;
use super::lens_updating;
use crate::domain::insights::lenses::timeline::{ConceptTimeline, Phase};
use crate::domain::insights::lenses::LensReport;
use crate::ui::features::day_filter::{format_day, month_label};
use crate::ui::features::insights::chart::lanes::PeriodRange;
use crate::ui::features::insights::chart::ranked::{ranked_list, RankedRow};
use crate::ui::features::insights::chart::summary::summary_line;
use crate::ui::features::insights::chart::tooltip::{floating_tooltip, tooltip_card};
use crate::ui::features::insights::panel::InsightsPanel;
use crate::ui::icons::LucideIcon;
use jiff::civil::Date;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use paint::{LanePaint, PaintedRow};
use rust_i18n::t;
use std::rc::Rc;

/// Concepts drawn at most (fewer on narrow pages).
const MAX_ROWS: usize = 16;
const MAX_ROWS_NARROW: usize = 10;
/// Period length (days) from which axis labels name the month and year.
const MONTHLY_DAYS: usize = 28;
/// Axis widths one date label takes.
const AXIS_LABEL_SPAN: f32 = 2.0;
/// Estimated tooltip height used for flipping.
const TIP_HEIGHT: f32 = 64.0;

/// A short label for the period starting on `day` (the chart's axis and rows).
fn period_label(day: Date, days: usize) -> String {
    if days >= MONTHLY_DAYS { format!("{} {}", month_label(day.month()), day.year()) } else { format!("{} {}", month_label(day.month()), day.day()) }
}

/// The period starting on `day` inside a sentence beside other dates: a month stays a month,
/// a shorter period is its first day written the way the settings ask.
fn period_in_text(day: Date, days: usize) -> String {
    if days >= MONTHLY_DAYS { period_label(day, days) } else { format_day(day) }
}

/// Name of phase `phase`.
pub fn phase_label(phase: Phase) -> String {
    tr(&format!("insight.phase.{}", phase.key()))
}

impl InsightsPanel {
    /// The shown periods: the zoom, or the whole span.
    pub(super) fn timeline_range(&self, timeline: &ConceptTimeline) -> PeriodRange {
        self.lenses.timeline.zoom.map_or_else(|| timeline.full_range(), |zoom| timeline.clamp(zoom))
    }

    /// The lanes drawn in `range`, most mentioned there first.
    pub(super) fn shown_lanes(&self, timeline: &ConceptTimeline, range: PeriodRange, room: Room) -> Vec<usize> {
        timeline.active_lanes(range, if room == Room::Narrow { MAX_ROWS_NARROW } else { MAX_ROWS })
    }

    pub(in super::super) fn render_concept_timeline(&self, report: &Rc<LensReport>, room: Room, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let timeline = &report.timeline;
        if timeline.is_empty() {
            return self.view_empty("insight.timeline.empty_title", "insight.timeline.empty_hint");
        }
        let range = self.timeline_range(timeline);
        let lanes = self.shown_lanes(timeline, range, room);
        let chart = self.lane_chart_card(timeline, range, &lanes, cx);
        let phases = self.phase_card(timeline, range, &lanes, cx);
        div().w_full().flex().flex_col().gap(theme.section_gap).child(chart).child(pair(room, phases, self.concept_detail_card(timeline, range, cx), theme))
    }

    fn lane_chart_card(&self, timeline: &ConceptTimeline, range: PeriodRange, lanes: &[usize], cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let periods = &timeline.periods;
        let ui = &self.lenses.timeline;
        let rows = lanes.iter().map(|&lane| {
            let l = &timeline.lanes[lane];
            PaintedRow { label: l.key.clone(), counts: l.counts.clone(), peak: l.peak, quiet: l.phase == Phase::Dormant, selected: ui.selected == Some(lane) }
        }).collect();
        let axis_width = theme.insights_axis_width.as_f32() * AXIS_LABEL_SPAN;
        let layout = ui.frame.get().map(|frame| self.lane_layout(frame, range, lanes.len()));
        let axis = layout.map_or_else(|| vec![range.0], |layout| layout.axis_periods(axis_width)).into_iter().map(|p| (p, period_label(periods.start_of(p), periods.days))).collect();
        let hover_row = ui.hover.and_then(|(lane, period)| lanes.iter().position(|&l| l == lane).map(|row| (row, period)));
        let canvas = LanePaint { rows, lanes: lanes.to_vec(), range, axis, hover: hover_row, brush: ui.brush, generation: self.lenses.generation, slot: Rc::clone(&ui.frame), cache: Rc::clone(&ui.cells), theme: theme.clone() }.into_canvas();
        let height = theme.insight_lane_height * lanes.len() as f32 + theme.insight_axis_height;
        let tip = ui.hover.and_then(|(lane, period)| self.lane_tip(timeline, lane, period));
        let surface = self.timeline_surface(canvas, height, cx).children(tip);
        let reset = (ui.zoom.is_some()).then(|| {
            let entity = cx.entity().downgrade();
            Button::new("insight-timeline-reset").ghost().xsmall().icon(Icon::new(LucideIcon::RotateCcw)).label(tr("insight.timeline.show_all")).on_click(move |_, _, cx| {
                let _ = entity.update(cx, |panel, cx| panel.reset_timeline_zoom(cx));
            }).into_any_element()
        });
        let span = t!("insight.timeline.summary", concepts = timeline.active_lanes(range, timeline.lanes.len()).len(), from = format_day(periods.start_of(range.0)), to = format_day(periods.end_of(range.1))).to_string();
        card(theme).child(card_title(LucideIcon::Activity, tr("insight.timeline.title"), reset.or_else(|| lens_updating(self.lenses.pending, theme)), theme)).child(surface).child(summary_line(span, theme))
    }

    fn lane_tip(&self, timeline: &ConceptTimeline, lane: usize, period: usize) -> Option<Div> {
        let frame = self.lenses.timeline.frame.get()?;
        let l = timeline.lanes.get(lane)?;
        let periods = &timeline.periods;
        let span = if periods.days > 1 { format!("{} – {}", format_day(periods.start_of(period)), format_day(periods.end_of(period))) } else { format_day(periods.start_of(period)) };
        let lines = vec![span, crate::i18n::t_count("insight.mentions", l.counts.get(period).copied().unwrap_or_default() as usize), phase_label(l.phase)];
        Some(floating_tooltip(&frame, self.pointer, tooltip_card(l.key.clone(), lines, &self.theme), TIP_HEIGHT, &self.theme))
    }

    /// The concepts' phases, busiest in range first; a row picks its concept.
    fn phase_card(&self, timeline: &ConceptTimeline, range: PeriodRange, lanes: &[usize], cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let rows: Vec<RankedRow> = lanes.iter().map(|&lane| {
            let l = &timeline.lanes[lane];
            let value = timeline.total_in(lane, range);
            RankedRow { label: format!("{} · {}", l.key, phase_label(l.phase)), value: f64::from(value), value_text: value.to_string(), colour: phase_colour(l.phase, theme), active: self.lenses.timeline.selected == Some(lane) }
        }).collect();
        let picks = lanes.to_vec();
        let entity = cx.entity().downgrade();
        let open = Rc::new(move |i: usize, _: &mut Window, cx: &mut App| {
            let lane = picks[i];
            let _ = entity.update(cx, |panel, cx| panel.toggle_lane(lane, cx));
        });
        card(theme).child(card_title(LucideIcon::ListOrdered, tr("insight.timeline.phases"), None, theme)).child(ranked_list("insight-phases", &rows, Some(open), theme)).child(summary_line(tr("insight.timeline.phases_hint"), theme))
    }
}

/// Colour of a phase: new and rising in the brand accents, steady neutral, fading and quiet muted.
pub fn phase_colour(phase: Phase, theme: &crate::theme::ThemeTokens) -> Rgba {
    match phase {
        Phase::New => theme.accent_success,
        Phase::Rising => theme.accent_brand,
        Phase::Steady => theme.info,
        Phase::Declining => theme.accent_dialectic,
        Phase::Dormant => theme.text_muted,
    }
}
