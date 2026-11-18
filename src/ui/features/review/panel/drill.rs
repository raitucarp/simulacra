//! Connection recall in a session: the map's points scattered without connections, drawn
//! by clicking pairs, checked against the reference and graded by how exactly they match.

use super::ReviewPanel;
use crate::domain::review::connections::layout::{drill_seed, scatter};
use crate::domain::review::connections::{drill_grade, Drill, DrillEffort};
use crate::domain::review::{Grade, ItemId, ReviewItem};
use gpui_kit::Context;
use std::time::Instant;

/// The drill of the map item on screen.
#[derive(Debug, Clone)]
pub struct DrillRun {
    pub item: ItemId,
    pub drill: Drill,
    /// Where each point sits (fractions of the free area).
    pub positions: Vec<(f32, f32)>,
    pub started: Instant,
    /// The grade the checked drawing earned.
    pub grade: Option<Grade>,
}

impl DrillRun {
    /// A drill of `item` (a connection map), its points scattered for this review.
    pub fn start(item: &ReviewItem) -> Self {
        let count = item.subject.map().map_or(0, |map| map.points.len());
        let positions = scatter(count, drill_seed(item.id.as_str(), item.schedule.reps));
        Self { item: item.id.clone(), drill: Drill::default(), positions, started: Instant::now(), grade: None }
    }
}

impl ReviewPanel {
    fn step_drill(&mut self, step: impl FnOnce(Drill) -> Drill, cx: &mut Context<Self>) {
        if let Some(run) = self.drill.as_mut() {
            run.drill = step(std::mem::take(&mut run.drill));
            cx.notify();
        }
    }

    pub(in crate::ui::features::review) fn click_drill_point(&mut self, point: usize, cx: &mut Context<Self>) {
        self.step_drill(|drill| drill.click(point), cx);
    }

    pub(in crate::ui::features::review) fn set_drill_label(&mut self, label: Option<String>, cx: &mut Context<Self>) {
        self.step_drill(|drill| drill.with_label(label), cx);
    }

    pub(in crate::ui::features::review) fn clear_drill(&mut self, cx: &mut Context<Self>) {
        self.step_drill(Drill::cleared, cx);
    }

    /// Scores the drawing and works out its grade.
    pub(super) fn check_drill(&mut self, cx: &mut Context<Self>) {
        let Some(map) = self.drill.as_ref().and_then(|run| self.data.item(&run.item)).and_then(|item| item.subject.map()).cloned() else { return };
        let Some(run) = self.drill.as_mut() else { return };
        run.drill = std::mem::take(&mut run.drill).checked(&map);
        let effort = DrillEffort { seconds: run.started.elapsed().as_secs(), corrections: run.drill.corrections };
        run.grade = run.drill.score.as_ref().map(|score| drill_grade(score, effort));
        cx.notify();
    }
}
