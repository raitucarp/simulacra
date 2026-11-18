//! Connection recall in a session: the points scattered with no connections; the learner
//! connects them (click one point, then another), optionally naming each relation, then
//! checks. The check overlays right, wrong and missing connections in their own colours.

use super::super::panel::{DrillRun, ReviewPanel};
use super::super::text::tr;
use super::common::{block_title, caption, control_row, surface};
use super::map::{point_name, MapCanvas, PointAction, Stroke};
use super::result::render_drill_result;
use crate::domain::review::connections::{ConnectionMap, MapScore};
use crate::domain::review::ReviewItem;
use crate::ui::components::{chip_row, choice_chip};
use crate::ui::icons::LucideIcon;
use crate::ui::keymap::render_key_cap;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Disableable, Icon, Sizable};
use gpui_kit::*;
use std::rc::Rc;

const CANVAS_ID: &str = "review-drill-map";
const LABEL_ID_PREFIX: &str = "review-drill-label-";
const CLEAR_ID: &str = "review-drill-clear";
const CHECK_ID: &str = "review-drill-check";

impl ReviewPanel {
    pub(super) fn render_drill(&self, item: &ReviewItem, run: &DrillRun, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let Some(map) = item.subject.map() else { return surface(theme) };
        let checked = run.drill.score.as_ref();
        let instructions = if checked.is_some() { tr("review.drill.checked_hint") } else { tr("review.drill.instructions") };
        let entity = cx.entity().downgrade();
        let on_point: Option<PointAction> = checked.is_none().then(|| -> PointAction {
            Rc::new(move |point, _, cx| {
                let _ = entity.update(cx, |panel, cx| panel.click_drill_point(point, cx));
            })
        });
        let chips = self.map_chips(map, None);
        let titles: Vec<String> = chips.iter().map(point_name).collect();
        let canvas = MapCanvas {
            id: CANVAS_ID.into(),
            chips,
            positions: run.positions.clone(),
            strokes: checked.map_or_else(|| self.drawn_strokes(map, run), |score| self.result_strokes(map, score)),
            pending: run.drill.pending,
            on_point,
            theme: theme.clone(),
        };
        let footer = match (checked, run.grade) {
            (Some(score), Some(grade)) => render_drill_result(score, grade, &titles, theme, cx).into_any_element(),
            _ => self.drill_controls(run, cx).into_any_element(),
        };
        surface(theme)
            .child(block_title(LucideIcon::Share2, theme.accent_brand, map.title.clone(), theme))
            .child(caption(instructions, theme))
            .children((checked.is_none() && !map.labels().is_empty()).then(|| self.relation_chips(map, run, cx)))
            .child(canvas)
            .child(footer)
    }

    /// The relations new connections can carry ("no relation" first).
    fn relation_chips(&self, map: &ConnectionMap, run: &DrillRun, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let choices = std::iter::once(None).chain(map.labels().into_iter().map(Some));
        let chips = choices.enumerate().map(|(index, label)| {
            let text = label.clone().unwrap_or_else(|| tr("review.drill.no_relation"));
            let selected = run.drill.label == label;
            let entity = cx.entity().downgrade();
            choice_chip(ElementId::NamedInteger(LABEL_ID_PREFIX.into(), index as u64), &text, selected, theme, move |_, cx| {
                let _ = entity.update(cx, |panel, cx| panel.set_drill_label(label.clone(), cx));
            })
        });
        div().flex().flex_col().gap(theme.spacing_xs).child(caption(tr("review.drill.relation"), theme)).child(chip_row(theme).children(chips))
    }

    fn drill_controls(&self, run: &DrillRun, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let drawn = run.drill.drawn.len();
        let clear = Button::new(CLEAR_ID).ghost().small().icon(Icon::new(LucideIcon::Eraser)).label(tr("review.drill.clear")).disabled(drawn == 0)
            .on_click(cx.listener(|this, _, _, cx| this.clear_drill(cx)));
        let check = Button::new(CHECK_ID).primary().small().icon(Icon::new(LucideIcon::Check)).label(tr("review.drill.check"))
            .on_click(cx.listener(|this, _, _, cx| this.reveal(cx)));
        control_row(theme)
            .child(caption(crate::i18n::t_count("review.drill.drawn", drawn), theme))
            .child(div().flex_1())
            .child(clear)
            .child(check)
            .child(render_key_cap(tr("keymap.key.space"), theme))
    }

    /// The learner's connections, in the brand colour (arrows when the map has one-way ones).
    fn drawn_strokes(&self, map: &ConnectionMap, run: &DrillRun) -> Vec<Stroke> {
        let arrows = map.edges.iter().any(|edge| edge.directed);
        let color = self.theme.accent_brand;
        run.drill.drawn.iter().map(|edge| Stroke { from: edge.from, to: edge.to, color, curved: false, arrow: arrows, label: edge.label.clone() }).collect()
    }

    /// Right (success), wrong (danger) and missing (warning, curved) connections.
    fn result_strokes(&self, map: &ConnectionMap, score: &MapScore) -> Vec<Stroke> {
        let theme = &self.theme;
        let arrows = map.edges.iter().any(|edge| edge.directed);
        let drawn = |edge: &crate::domain::review::connections::DrawnEdge, color| Stroke { from: edge.from, to: edge.to, color, curved: false, arrow: arrows, label: edge.label.clone() };
        let correct = score.correct.iter().map(|edge| drawn(edge, theme.accent_success));
        let wrong = score.wrong.iter().map(|(edge, _)| drawn(edge, theme.accent_danger));
        let missing = score.missing.iter().map(|edge| Stroke { from: edge.from, to: edge.to, color: theme.warning, curved: true, arrow: edge.directed, label: edge.label.clone() });
        correct.chain(wrong).chain(missing).collect()
    }
}
