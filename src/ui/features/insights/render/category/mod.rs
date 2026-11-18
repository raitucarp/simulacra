//! Categories: the network as a category. Objects (thoughts, documents, concepts, people)
//! and typed relations as morphisms in a diagram; how relations compose along two-step paths
//! (and how often a composite is also a direct relation); and squares, two routes between the
//! same pair, commuting in type when both carry the same relations.

pub mod diagram;

use super::common::{card, card_title, pair, tr};
use super::Room;
use crate::domain::insights::category::{MorphismKind, Square};
use crate::domain::insights::InsightsReport;
use crate::ui::features::forge::widgets::KeyboardStop;
use crate::ui::features::insights::chart::palette::series;
use crate::ui::features::insights::chart::ranked::{ranked_list, RankedRow};
use crate::ui::features::insights::chart::summary::summary_line;
use crate::ui::features::insights::chart::ticks::{compact, percent};
use crate::ui::features::insights::events::InsightsEvent;
use crate::ui::features::insights::panel::{Hover, InsightsPanel};
use crate::ui::icons::LucideIcon;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

/// Composites and squares listed.
const COMPOSITES_SHOWN: usize = 10;
const SQUARES_SHOWN: usize = 8;

fn relation(kind: MorphismKind) -> String {
    tr(&format!("insights.morphism.{}", kind.key()))
}

impl InsightsPanel {
    fn diagram_move(&mut self, at: (f32, f32), cx: &mut Context<Self>) {
        let reach = self.theme.insights_node_radius_max.as_f32() * 2.0;
        let hover = self.frames.diagram.get().and_then(|f| diagram::object_at(&f, at, reach)).map_or(Hover::None, Hover::Object);
        if hover != self.hover {
            (self.hover, self.pointer) = (hover, at);
            cx.notify();
        }
    }

    fn diagram_card(&self, report: &InsightsReport, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let category = &report.category;
        let focus = match self.hover { Hover::Object(i) => Some(i), _ => None };
        let names = category.arrows.iter().map(|a| relation(a.kind)).collect();
        let surface = diagram::diagram_canvas(category.arrows.clone(), names, category.objects, focus, Rc::clone(&self.frames.diagram), theme.clone());
        let canvas = div()
            .id("insights-diagram")
            .relative()
            .w_full()
            .h(theme.insights_diagram_height)
            .on_mouse_move(cx.listener(|p, e: &MouseMoveEvent, _, cx| p.diagram_move((e.position.x.as_f32(), e.position.y.as_f32()), cx)))
            .on_hover(cx.listener(|p, hovered: &bool, _, cx| if !hovered { p.on_canvas_leave(cx) }))
            .child(surface);
        let morphisms: usize = category.arrows.iter().map(|a| a.count).sum();
        let summary = t!("insights.summary.category", objects = category.objects.iter().sum::<usize>(), morphisms = morphisms, types = category.arrows.len()).to_string();
        card(theme)
            .child(card_title(LucideIcon::GitCompareArrows, tr("insights.card.diagram"), None, theme))
            .child(canvas)
            .child(summary_line(summary, theme))
            .child(summary_line(tr("insights.explain.category"), theme))
    }

    fn composites_card(&self, report: &InsightsReport) -> Div {
        let theme = &self.theme;
        let composition = &report.category.composition;
        let rows: Vec<RankedRow> = composition
            .composites
            .iter()
            .take(COMPOSITES_SHOWN)
            .enumerate()
            .map(|(i, c)| RankedRow {
                label: t!("insights.composite", first = relation(c.first), second = relation(c.second)).to_string(),
                value: c.count as f64,
                value_text: format!("{} · {}", compact(c.count as f64), percent(c.realised as f64 / c.count.max(1) as f64)),
                colour: series(i, theme),
                active: false,
            })
            .collect();
        let summary = composition.composites.first().map_or_else(|| tr("insights.summary.no_composites"), |c| t!("insights.summary.composites", first = relation(c.first), second = relation(c.second), count = c.count).to_string());
        card(theme)
            .child(card_title(LucideIcon::ArrowRight, tr("insights.card.composition"), None, theme))
            .child(ranked_list("insights-composites", &rows, None, theme))
            .child(summary_line(summary, theme))
    }

    fn square_text(&self, report: &InsightsReport, square: &Square) -> String {
        let name = |i: u32| report.category.labels.get(i as usize).cloned().unwrap_or_default();
        let kind = if square.commutes_in_type() { tr("insights.square.commutes") } else { tr("insights.square.mixed") };
        format!("{kind}: {} → {} / {} → {}", name(square.from), name(square.via.0), name(square.via.1), name(square.to))
    }

    fn square_row(&self, index: usize, report: &InsightsReport, square: &Square, cx: &mut Context<Self>) -> Stateful<Div> {
        let theme = &self.theme;
        let colour = if square.commutes_in_type() { theme.accent_success } else { theme.accent_dialectic };
        let node = report.category.nodes.get(square.from as usize).cloned();
        let entity = cx.entity().downgrade();
        let open: Rc<dyn Fn(&mut Window, &mut App)> = Rc::new(move |_, cx| {
            if let Some(node) = node.clone() {
                let _ = entity.update(cx, |_, cx| cx.emit(InsightsEvent::OpenNode(node)));
            }
        });
        let click = Rc::clone(&open);
        div()
            .id(ElementId::NamedInteger("insights-square".into(), index as u64))
            .flex()
            .items_center()
            .gap(theme.inline_gap)
            .px(theme.spacing_xs)
            .py(theme.spacing_3xs)
            .rounded(theme.radius_sm)
            .cursor_pointer()
            .hover(|d| d.bg(theme.hover_bg))
            .on_click(move |_, window, cx| click(window, cx))
            .keyboard_stop(theme, open)
            .child(div().flex_none().size(theme.insights_swatch_size).rounded(theme.radius_full).bg(colour))
            .child(div().flex_1().min_w_0().truncate().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(self.square_text(report, square)))
    }

    fn squares_card(&self, report: &InsightsReport, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let composition = &report.category.composition;
        let rows: Vec<Stateful<Div>> = composition.examples.iter().take(SQUARES_SHOWN).enumerate().map(|(i, s)| self.square_row(i, report, s, cx)).collect();
        let summary = t!("insights.summary.squares", commuting = composition.commuting_squares, mixed = composition.mixed_squares).to_string();
        card(theme)
            .child(card_title(LucideIcon::LayoutGrid, tr("insights.card.squares"), None, theme))
            .child(div().flex().flex_col().gap(theme.spacing_3xs).children(rows))
            .child(summary_line(summary, theme))
    }

    pub(super) fn render_categories(&self, report: &Rc<InsightsReport>, room: Room, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        if report.category.arrows.is_empty() {
            return self.view_empty("insights.empty.category", "insights.empty.category_hint");
        }
        div()
            .w_full()
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .child(self.diagram_card(report, cx))
            .child(pair(room, self.composites_card(report), self.squares_card(report, cx), theme))
    }
}
