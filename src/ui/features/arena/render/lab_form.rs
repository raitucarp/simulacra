//! The lab form: classic presets, the scenario (title, premise, variables, question), the
//! agents to ask, the run's budget preview, and Run / Save. The tab places the experiment on
//! show beside it.

use super::super::kit::{area_field, block_title, caption, control_row, ghost_button, line_field, primary_button, section_label, surface};
use super::super::panel::ArenaPanel;
use super::super::text::{scenario_problem_text, tr};
use super::two_columns;
use crate::domain::arena::lab::{preview, PresetKey, RunPreview};
use crate::ui::components::{chip_row, choice_chip};
use crate::ui::icons::LucideIcon;
use crate::ui::responsive::LayoutClass;
use gpui_kit::base::Disableable;
use gpui_kit::*;
use rust_i18n::t;

const PRESET_PREFIX: &str = "arena-preset-";
const AGENT_PREFIX: &str = "arena-lab-agent-";
const RUN_ID: &str = "arena-lab-run";
const SAVE_ID: &str = "arena-lab-save";
const TITLE_FIELD: &str = "title";

impl ArenaPanel {
    pub(super) fn render_lab_tab(&self, class: LayoutClass, cx: &mut Context<Self>) -> Div {
        let open = self.open_experiment.as_ref().and_then(|id| self.data.experiment(id)).cloned();
        let main_first = open.is_some();
        let (stack, saved) = (div().flex().flex_col().gap(self.theme.section_gap), self.render_saved_experiments(cx));
        let (side, main) = match open {
            Some(experiment) => (stack.child(self.render_lab_form(cx)).child(saved), self.render_experiment(&experiment, cx)),
            None => (self.render_lab_form(cx), stack.child(self.idle_state("arena.lab.idle_title", "arena.lab.idle_hint")).child(saved)),
        };
        two_columns(class, side, main, main_first, &self.theme)
    }

    fn render_presets(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let chips = PresetKey::ALL.iter().map(|preset| {
            let (preset, entity) = (*preset, cx.entity().downgrade());
            let label = tr(&preset.locale_key(TITLE_FIELD));
            choice_chip(ElementId::Name(format!("{PRESET_PREFIX}{}", preset.key()).into()), &label, self.lab.preset == Some(preset), theme, move |window, cx| {
                let _ = entity.update(cx, |panel, cx| panel.load_preset(preset, window, cx));
            })
        });
        div().flex().flex_col().gap(theme.spacing_xs).child(section_label(tr("arena.lab.presets"), theme)).child(chip_row(theme).children(chips))
    }

    fn render_lab_agents(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let chips = self.data.agents.iter().enumerate().map(|(index, agent)| {
            let (id, entity) = (agent.id.clone(), cx.entity().downgrade());
            choice_chip(ElementId::Name(format!("{AGENT_PREFIX}{index}").into()), &agent.name, self.lab.agents.contains(&agent.id), theme, move |_, cx| {
                let _ = entity.update(cx, |panel, cx| {
                    panel.lab = panel.lab.toggled(&id);
                    cx.notify();
                });
            })
        });
        let none = self.data.agents.is_empty().then(|| caption(tr("arena.lab.need_agents"), theme));
        div().flex().flex_col().gap(theme.spacing_xs).child(section_label(tr("arena.lab.agents"), theme)).child(chip_row(theme).children(chips)).children(none)
    }

    /// The preview of running the form's scenario with the chosen agents.
    fn form_preview(&self, cx: &App) -> Option<RunPreview> {
        let limits = self.data.limits?;
        let scenario = self.current_scenario(cx);
        let agents = self.lab.agents.len();
        let requests = crate::domain::arena::lab::cells(&scenario).len() * agents;
        Some(preview(&scenario, agents, requests, &self.data.usage, limits, &self.data.today))
    }

    pub(super) fn preview_line(&self, run: &RunPreview) -> String {
        let left = run.requests_left.map(|left| t!("arena.lab.preview_left", left = left).to_string()).unwrap_or_else(|| tr("arena.lab.preview_unlimited"));
        let line = t!("arena.lab.preview", cells = run.cells, agents = run.agents, requests = run.requests, tokens = run.tokens).to_string();
        match (run.over_cap(), run.fits()) {
            (true, _) => format!("{line} · {}", t!("arena.lab.preview_over_cap", max = crate::domain::arena::lab::MAX_LAB_REQUESTS)),
            (false, false) => format!("{line} · {left} · {}", tr("arena.lab.preview_over_budget")),
            (false, true) => format!("{line} · {left}"),
        }
    }

    fn render_lab_form(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let problem = self.scenario_problem(cx);
        let run = self.form_preview(cx);
        let ready = problem.is_none() && !self.lab.agents.is_empty() && run.is_some_and(|run| run.fits()) && !self.data.run.is_running();
        let (run_entity, save_entity) = (cx.entity().downgrade(), cx.entity().downgrade());
        let run_button = primary_button(RUN_ID, LucideIcon::Play, tr("arena.lab.run")).disabled(!ready).on_click(move |_, _, cx| {
            let _ = run_entity.update(cx, |panel, cx| panel.submit_lab(true, cx));
        });
        let save_button = ghost_button(SAVE_ID, LucideIcon::Save, tr("arena.lab.save")).disabled(problem.is_some() || self.lab.agents.is_empty()).on_click(move |_, _, cx| {
            let _ = save_entity.update(cx, |panel, cx| panel.submit_lab(false, cx));
        });
        surface(theme)
            .child(block_title(LucideIcon::FlaskConical, theme.accent_brand, tr("arena.lab.form_title"), theme))
            .child(self.render_presets(cx))
            .child(section_label(tr("arena.lab.scenario"), theme))
            .child(line_field(&self.fields.title, theme))
            .child(area_field(&self.fields.premise, theme))
            .child(self.render_variables(cx))
            .child(section_label(tr("arena.lab.question"), theme))
            .child(line_field(&self.fields.question, theme))
            .child(self.render_lab_agents(cx))
            .children(run.filter(|run| run.requests > 0).map(|run| caption(self.preview_line(&run), theme)))
            .children(problem.map(|problem| caption(scenario_problem_text(problem), theme)))
            .child(control_row(theme).child(run_button).child(save_button))
    }
}
