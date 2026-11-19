//! The experiment on show: its question and progress, its controls (stop, answer the rest,
//! run again, publish a summary, delete), the grid of answers (one row per combination of
//! values, one column per agent, each cell a stance and a one-line verdict) and the
//! comparison.

use super::super::events::ArenaEvent;
use super::super::kit::{block_title, caption, control_row, ghost_button, primary_button, prose, small_button, surface};
use super::super::panel::ArenaPanel;
use super::super::text::{stance_label, tr};
use super::lab_compare::stance_colors;
use crate::domain::arena::lab::{cells, conditions, Experiment};
use crate::ui::components::meta_badge;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::base::Disableable;
use gpui_kit::*;
use rust_i18n::t;

const GRID_ID: &str = "arena-lab-grid";
const STOP_ID: &str = "arena-lab-stop";
const RESUME_ID: &str = "arena-lab-resume";
const PUBLISH_ID: &str = "arena-lab-publish";
const RERUN_ID: &str = "arena-lab-rerun";
const DELETE_ID: &str = "arena-lab-delete";
const CONDITION_SEPARATOR: &str = " · ";

impl ArenaPanel {
    fn experiment_controls(&self, experiment: &Experiment, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let running = self.data.run.runs(&experiment.id);
        let emit = |event: ArenaEvent| {
            let entity = cx.entity().downgrade();
            move |_: &ClickEvent, _: &mut Window, cx: &mut App| {
                let _ = entity.update(cx, |_, cx| cx.emit(event.clone()));
            }
        };
        let busy = self.data.run.is_running();
        let rerun = ArenaEvent::RunExperiment { id: Some(experiment.id.clone()), scenario: experiment.scenario.clone(), agents: experiment.agents.clone(), preset: experiment.preset.clone() };
        let mut row = control_row(theme);
        row = match (running, experiment.is_complete()) {
            (true, _) => row.child(small_button(STOP_ID, LucideIcon::CircleStop, tr("arena.stop")).on_click(emit(ArenaEvent::Cancel))),
            (false, false) => row.child(primary_button(RESUME_ID, LucideIcon::Play, tr("arena.lab.resume")).disabled(busy).on_click(emit(ArenaEvent::ResumeExperiment(experiment.id.clone())))),
            (false, true) => row.child(primary_button(PUBLISH_ID, LucideIcon::Send, tr("arena.lab.publish")).on_click(emit(ArenaEvent::PublishExperiment(experiment.id.clone())))),
        };
        row.child(small_button(RERUN_ID, LucideIcon::RotateCcw, tr("arena.lab.rerun")).disabled(busy).on_click(emit(rerun)))
            .child(ghost_button(DELETE_ID, LucideIcon::Trash, tr("arena.delete")).disabled(running).on_click(emit(ArenaEvent::DeleteExperiment(experiment.id.clone()))))
    }

    fn grid_cell(&self, experiment: &Experiment, agent: &crate::domain::ProfileId, cell: usize) -> Div {
        let theme = &self.theme;
        let body = match experiment.answer(agent, cell) {
            Some(answer) => div()
                .flex()
                .flex_col()
                .gap(theme.spacing_3xs)
                .child(div().flex().child(meta_badge(&stance_label(answer.stance), stance_colors(answer.stance, theme), theme)))
                .child(caption(answer.verdict.clone(), theme)),
            None => caption(tr("arena.lab.waiting"), theme).italic(),
        };
        div().w(theme.arena_grid_cell_width).flex_none().p(theme.spacing_xs).child(body)
    }

    fn render_grid(&self, experiment: &Experiment) -> impl IntoElement {
        let theme = &self.theme;
        let label_cell = |text: String| div().w(theme.arena_grid_label_width).flex_none().p(theme.spacing_xs).child(text);
        let head = div()
            .flex()
            .border_b(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .text_size(theme.font_size_xs)
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(theme.text_secondary)
            .child(label_cell(tr("arena.lab.case")))
            .children(experiment.agents.iter().map(|agent| div().w(theme.arena_grid_cell_width).flex_none().p(theme.spacing_xs).truncate().child(self.data.agent_name(agent))));
        let rows = cells(&experiment.scenario).into_iter().enumerate().map(|(index, cell)| {
            let case: Vec<String> = conditions(&experiment.scenario, &cell).into_iter().map(|(name, value)| format!("{name}: {value}")).collect();
            let label = label_cell(case.join(CONDITION_SEPARATOR)).text_size(theme.font_size_xs).text_color(theme.text_primary);
            div().flex().border_b(theme.border_width_hairline).border_color(theme.border_subtle).child(label).children(experiment.agents.iter().map(|agent| self.grid_cell(experiment, agent, index)))
        });
        div().id(GRID_ID).w_full().overflow_x_scrollbar().child(div().flex().flex_col().child(head).children(rows))
    }

    pub(super) fn render_experiment(&self, experiment: &Experiment, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let total = cells(&experiment.scenario).len() * experiment.agents.len();
        let status = t!("arena.saved.answers", done = experiment.answers.len(), total = total).to_string();
        let head = surface(theme)
            .child(block_title(LucideIcon::FlaskConical, theme.accent_brand, experiment.scenario.title.clone(), theme))
            .child(prose(experiment.scenario.question.clone(), theme).font_weight(FontWeight::SEMIBOLD))
            .child(caption(status, theme))
            .child(self.experiment_controls(experiment, cx));
        let grid = surface(theme).child(block_title(LucideIcon::LayoutGrid, theme.text_secondary, tr("arena.lab.answers"), theme)).child(self.render_grid(experiment));
        div().w_full().flex().flex_col().gap(theme.section_gap).child(head).child(grid).child(self.render_compare(experiment))
    }
}
