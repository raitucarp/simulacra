//! The two radial charts of the Activity view: the weekly rhythm (a radial heatmap with one
//! ring per weekday, Monday innermost, and one sector per hour from midnight at the top) and
//! the agents' trait radar. Both are square canvases sized to their card (clamped to the token
//! range), with hover tooltips and accessible summaries.

pub mod geometry;
mod paint;

use super::super::panel::{Hover, InsightsPanel, LegendId};
use super::common::{card, card_title, tr};
use crate::domain::insights::activity::{HOURS, WEEKDAYS};
use crate::domain::insights::facts::TRAIT_KEYS;
use crate::domain::insights::InsightsReport;
use crate::ui::features::insights::chart::legend::{legend, LegendEntry};
use crate::ui::features::insights::chart::palette::series;
use crate::ui::features::insights::chart::summary::summary_line;
use crate::ui::features::insights::chart::tooltip::{floating_tooltip, tooltip_card};
use crate::ui::features::mind_graph::viewport::Frame;
use crate::ui::icons::LucideIcon;
use geometry::{radar_axis_at, rhythm_cell_at};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

pub use geometry::RADAR_AGENTS;

/// Square aspect of the radial canvases.
const SQUARE: f32 = 1.0;
/// Estimated tooltip height used for flipping.
const TIP_HEIGHT: f32 = 64.0;

/// Weekday name (Monday = 0).
pub fn weekday_name(day: usize) -> String {
    tr(&format!("insights.weekday.{day}"))
}

impl InsightsPanel {
    /// A square canvas container wired to `on_move`.
    fn radial_box(&self, id: &'static str, surface: AnyElement, on_move: fn(&mut Self, (f32, f32), &mut Context<Self>), cx: &mut Context<Self>) -> Stateful<Div> {
        let theme = &self.theme;
        div()
            .id(id)
            .relative()
            .w_full()
            .min_w(theme.insights_radial_min)
            .max_w(theme.insights_radial_max)
            .aspect_ratio(SQUARE)
            .on_mouse_move(cx.listener(move |panel, event: &MouseMoveEvent, _, cx| on_move(panel, (event.position.x.as_f32(), event.position.y.as_f32()), cx)))
            .on_hover(cx.listener(|panel, hovered: &bool, _, cx| if !hovered { panel.on_canvas_leave(cx) }))
            .child(surface)
    }

    fn rhythm_move(&mut self, at: (f32, f32), cx: &mut Context<Self>) {
        let hover = self.frames.rhythm.get().and_then(|frame| rhythm_cell_at(&frame, at)).map_or(Hover::None, |(d, h)| Hover::Rhythm(d, h));
        if hover != self.hover || self.pointer != at {
            (self.hover, self.pointer) = (hover, at);
            cx.notify();
        }
    }

    fn radar_move(&mut self, at: (f32, f32), cx: &mut Context<Self>) {
        let hover = self.frames.radar.get().and_then(|frame| radar_axis_at(&frame, at)).map_or(Hover::None, Hover::Trait);
        if hover != self.hover || self.pointer != at {
            (self.hover, self.pointer) = (hover, at);
            cx.notify();
        }
    }

    fn tip_over(&self, frame: Option<Frame>, title: String, lines: Vec<String>) -> Option<Div> {
        frame.map(|frame| floating_tooltip(&frame, self.pointer, tooltip_card(title, lines, &self.theme), TIP_HEIGHT, &self.theme))
    }

    pub(super) fn rhythm_card(&self, report: &Rc<InsightsReport>, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let rhythm = report.activity.rhythm;
        let hover = match self.hover { Hover::Rhythm(d, h) => Some((d, h)), _ => None };
        let surface = paint::rhythm_canvas(rhythm, hover, Rc::clone(&self.frames.rhythm), theme.clone());
        let tip = hover.and_then(|(d, h)| {
            let title = t!("insights.rhythm.slot", day = weekday_name(d), from = format!("{h:02}:00"), to = format!("{:02}:00", (h + 1) % HOURS)).to_string();
            self.tip_over(self.frames.rhythm.get(), title, vec![crate::i18n::t_count("insights.posts", rhythm[d][h])])
        });
        let summary = report.activity.peak().map_or_else(|| tr("insights.summary.no_posts"), |(d, h, c)| t!("insights.summary.rhythm", day = weekday_name(d), hour = format!("{h:02}:00"), count = c).to_string());
        let canvas = self.radial_box("insights-rhythm", surface, Self::rhythm_move, cx).children(tip);
        card(theme)
            .child(card_title(LucideIcon::Clock, tr("insights.card.rhythm"), None, theme))
            .child(div().w_full().flex().justify_center().child(canvas))
            .child(summary_line(summary, theme))
    }

    pub(super) fn radar_card(&self, report: &Rc<InsightsReport>, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let agents: Vec<(usize, [u8; TRAIT_KEYS.len()])> = report.activity.traits.iter().take(RADAR_AGENTS).copied().collect();
        let name = |a: usize| report.authors.get(a).map(|f| f.name.clone()).unwrap_or_default();
        let shown: Vec<(usize, [u8; TRAIT_KEYS.len()])> = agents.iter().enumerate().filter(|(i, _)| !self.is_hidden(LegendId::Radar, *i)).map(|(i, (_, v))| (i, *v)).collect();
        let axis = match self.hover { Hover::Trait(a) => Some(a), _ => None };
        let labels: Vec<String> = TRAIT_KEYS.iter().map(|k| tr(&format!("insights.trait.{k}"))).collect();
        let surface = paint::radar_canvas(shown, labels.clone(), axis, Rc::clone(&self.frames.radar), theme.clone());
        let tip = axis.and_then(|a| self.tip_over(self.frames.radar.get(), labels[a].clone(), agents.iter().map(|(i, v)| format!("{}: {}", name(*i), v[a])).collect()));
        let entries: Vec<LegendEntry> = agents.iter().enumerate().map(|(i, (a, _))| LegendEntry { colour: series(i, theme), label: name(*a), value: None, hidden: self.is_hidden(LegendId::Radar, i) }).collect();
        let entity = cx.entity().downgrade();
        let toggle = Rc::new(move |i: usize, _: &mut Window, cx: &mut App| {
            let _ = entity.update(cx, |panel, cx| panel.toggle_legend(LegendId::Radar, i, cx));
        });
        let summary = if agents.is_empty() { tr("insights.summary.no_agents") } else { t!("insights.summary.radar", count = agents.len()).to_string() };
        let canvas = self.radial_box("insights-radar", surface, Self::radar_move, cx).children(tip);
        card(theme)
            .child(card_title(LucideIcon::Brain, tr("insights.card.radar"), None, theme))
            .when(!agents.is_empty(), |c| c.child(div().w_full().flex().justify_center().child(canvas)).child(legend("insights-radar-legend", &entries, Some(toggle), theme)))
            .child(summary_line(summary, theme))
    }
}

/// Hours shown around the rhythm dial.
pub const HOUR_MARKS: [usize; 4] = [0, 6, 12, 18];
/// Rings of the rhythm dial.
pub const RINGS: usize = WEEKDAYS;
