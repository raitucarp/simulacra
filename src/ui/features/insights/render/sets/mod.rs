//! Sets: posts grouped by concept, author, kind or vault subject. Two or three chosen sets
//! draw a Venn diagram, four or more an UpSet matrix; the Jaccard table compares every pair.

mod upset;
pub mod venn;

use super::common::{card, card_title, option_chip, option_group, tr};
use super::Room;
use crate::domain::insights::sets::{exclusive_regions, upset_rows, NamedSet, Region, SetDimension};
use crate::domain::insights::InsightsReport;
use crate::ui::features::insights::chart::columns::ColumnClick;
use crate::ui::features::insights::chart::legend::{legend, LegendEntry};
use crate::ui::features::insights::chart::palette::series;
use crate::ui::features::insights::chart::summary::summary_line;
use crate::ui::features::insights::chart::tooltip::{floating_tooltip, tooltip_card};
use crate::ui::features::insights::panel::{Hover, InsightsPanel};
use crate::ui::icons::LucideIcon;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

/// Sets chosen by default and at most.
const DEFAULT_CHOSEN: usize = 3;
const MAX_CHOSEN: usize = 8;
/// UpSet columns shown at most.
const UPSET_COLUMNS: usize = 16;
/// Post excerpts listed in a region's tooltip.
const TIP_EXCERPTS: usize = 3;
const TIP_HEIGHT: f32 = 110.0;

/// The label of a set: kind sets carry their kind key and are translated.
fn set_label(dimension: SetDimension, set: &NamedSet) -> String {
    if dimension == SetDimension::Kind { tr(&format!("insights.kind.{}", set.key)) } else { set.label.clone() }
}

impl InsightsPanel {
    /// The family of the chosen dimension, and the chosen sets in it (labels localised).
    fn chosen_sets(&self, report: &InsightsReport) -> (Vec<NamedSet>, Vec<NamedSet>) {
        let dimension = self.state.dimension;
        let family: Vec<NamedSet> = report.families.iter().find(|(d, _)| *d == dimension).map(|(_, f)| f.clone()).unwrap_or_default();
        let family: Vec<NamedSet> = family.into_iter().map(|s| NamedSet { label: set_label(dimension, &s), ..s }).collect();
        let picked: Vec<NamedSet> = family.iter().filter(|s| self.state.chosen_sets.contains(&s.key)).cloned().collect();
        let chosen = if picked.is_empty() { family.iter().take(DEFAULT_CHOSEN).cloned().collect() } else { picked };
        (family, chosen)
    }

    fn set_controls(&self, family: &[NamedSet], chosen: &[NamedSet], cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let dims = SetDimension::ALL.map(|d| option_chip(ElementId::Name(format!("insights-dim-{}", d.key()).into()), tr(&format!("insights.dimension.{}", d.key())), self.state.dimension == d, theme, self.pick(cx, move |s| { s.dimension = d; s.chosen_sets.clear(); })));
        let current: Vec<String> = chosen.iter().map(|s| s.key.clone()).collect();
        let sets = family.iter().map(|set| {
            let (key, base) = (set.key.clone(), current.clone());
            let pick = self.pick(cx, move |s| {
                let mut next = if s.chosen_sets.is_empty() { base.clone() } else { s.chosen_sets.clone() };
                match next.iter().position(|k| *k == key) { Some(at) => { next.remove(at); } None if next.len() < MAX_CHOSEN => next.push(key.clone()), None => {} }
                s.chosen_sets = next;
            });
            option_chip(ElementId::Name(format!("insights-set-{}", set.key).into()), format!("{} · {}", set.label, set.len()), current.contains(&set.key), theme, pick)
        });
        div().flex().flex_col().gap(theme.spacing_sm).child(option_group(tr("insights.control.group_by"), dims.into(), theme)).child(option_group(tr("insights.control.sets"), sets.collect(), theme))
    }

    /// Opens the first post of a region.
    fn region_opener(&self, regions: Vec<Region>, cx: &mut Context<Self>) -> ColumnClick {
        let entity = cx.entity().downgrade();
        Rc::new(move |i, _, cx| {
            if let Some(&post) = regions.get(i).and_then(|r| r.members.first()) {
                let _ = entity.update(cx, |panel, cx| panel.open_post(post as usize, cx));
            }
        })
    }

    fn venn_move(&mut self, at: (f32, f32), count: usize, cx: &mut Context<Self>) {
        let hover = self.frames.venn.get().and_then(|f| venn::region_at(&f, count, at)).map_or(Hover::None, Hover::Region);
        if hover != self.hover || self.pointer != at {
            (self.hover, self.pointer) = (hover, at);
            cx.notify();
        }
    }

    fn venn_view(&self, report: &InsightsReport, chosen: &[NamedSet], cx: &mut Context<Self>) -> Stateful<Div> {
        let theme = &self.theme;
        let regions = exclusive_regions(chosen);
        let hover = match self.hover { Hover::Region(mask) => Some(mask), _ => None };
        let tip = hover.and_then(|mask| regions.iter().find(|r| r.mask == mask)).zip(self.frames.venn.get()).map(|(region, frame)| {
            let title = region.sets().iter().map(|&i| chosen[i].label.clone()).collect::<Vec<_>>().join(" ∩ ");
            let mut lines = vec![crate::i18n::t_count("insights.posts", region.members.len())];
            lines.extend(region.members.iter().take(TIP_EXCERPTS).filter_map(|&p| report.posts.get(p as usize)).map(|p| p.excerpt.clone()));
            floating_tooltip(&frame, self.pointer, tooltip_card(title, lines, theme), TIP_HEIGHT, theme)
        });
        let count = chosen.len();
        let open = self.region_opener(regions.clone(), cx);
        let masks: Vec<u32> = regions.iter().map(|r| r.mask).collect();
        div()
            .id("insights-venn")
            .relative()
            .w_full()
            .h(theme.insights_venn_height)
            .cursor_pointer()
            .on_mouse_move(cx.listener(move |p, e: &MouseMoveEvent, _, cx| p.venn_move((e.position.x.as_f32(), e.position.y.as_f32()), count, cx)))
            .on_hover(cx.listener(|p, hovered: &bool, _, cx| if !hovered { p.on_canvas_leave(cx) }))
            .on_click(move |_, window, cx| {
                if let Some(i) = hover.and_then(|m| masks.iter().position(|x| *x == m)) {
                    open(i, window, cx);
                }
            })
            .child(venn::venn_canvas(count, regions, hover, Rc::clone(&self.frames.venn), theme.clone()))
            .children(tip)
    }

    pub(super) fn render_sets(&self, report: &Rc<InsightsReport>, _room: Room, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let (family, chosen) = self.chosen_sets(report);
        if family.is_empty() {
            return self.view_empty("insights.empty.sets", "insights.empty.sets_hint");
        }
        let diagram: AnyElement = if chosen.len() <= DEFAULT_CHOSEN && chosen.len() >= 2 {
            let names: Vec<LegendEntry> = chosen.iter().enumerate().map(|(i, s)| LegendEntry { colour: series(i, theme), label: s.label.clone(), value: Some(s.len().to_string()), hidden: false }).collect();
            div().w_full().flex().flex_col().gap(theme.stack_gap).child(self.venn_view(report, &chosen, cx)).child(legend("insights-venn-legend", &names, None, theme)).into_any_element()
        } else {
            let rows = upset_rows(&chosen, UPSET_COLUMNS);
            upset::upset_matrix(&chosen, &rows, self.region_opener(rows.clone(), cx), theme).into_any_element()
        };
        let regions = exclusive_regions(&chosen);
        let shared = regions.iter().filter(|r| r.degree() > 1).map(|r| r.members.len()).sum::<usize>();
        let summary = t!("insights.summary.sets", count = chosen.len(), shared = shared).to_string();
        let main = card(theme).child(card_title(LucideIcon::Layers, tr("insights.card.sets"), None, theme)).child(self.set_controls(&family, &chosen, cx)).child(diagram).child(summary_line(summary, theme));
        let similarity = card(theme).child(card_title(LucideIcon::Scale, tr("insights.card.jaccard"), None, theme)).child(upset::jaccard_table(&chosen, theme)).child(summary_line(tr("insights.summary.jaccard"), theme));
        div().w_full().flex().flex_col().gap(theme.section_gap).child(main).child(similarity)
    }
}
