//! What the page's buttons do: build a setup or a scenario from the form and ask the shell to
//! run it, fill the form from the open thread, a preset or a saved experiment.

use super::fields::ArenaFields;
use super::ArenaPanel;
use crate::domain::arena::debate::{setup_problem, DebateSetup, SetupProblem};
use crate::domain::arena::lab::{joined_values, preset_scenario, scenario_from, scenario_problem, PresetKey, Scenario, ScenarioProblem};
use crate::domain::arena::ArenaId;
use crate::domain::ProfileId;
use crate::ui::features::arena::events::ArenaEvent;
use gpui_kit::{App, Context, Window};
use rust_i18n::t;

impl ArenaPanel {
    /// The setup the form holds.
    pub(in crate::ui::features::arena) fn current_setup(&self, cx: &App) -> DebateSetup {
        self.draft.setup(&ArenaFields::area_text(&self.fields.motion, cx))
    }

    pub(in crate::ui::features::arena) fn setup_problem(&self, cx: &App) -> Option<SetupProblem> {
        setup_problem(&self.current_setup(cx))
    }

    /// Starts the debate when the setup is complete.
    pub(in crate::ui::features::arena) fn start_debate(&mut self, cx: &mut Context<Self>) {
        let setup = self.current_setup(cx);
        if setup_problem(&setup).is_none() {
            cx.emit(ArenaEvent::StartDebate(setup));
        }
    }

    /// The motion taken from the thread open in the inspector.
    pub(in crate::ui::features::arena) fn motion_from_thread(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(open) = self.data.open_thread.clone() else { return };
        self.draft.source_post = Some(open.post);
        self.fields.motion.update(cx, |field, cx| field.set_value(open.text, window, cx));
        cx.notify();
    }

    /// Back to the setup form (the open debate stays saved).
    pub(in crate::ui::features::arena) fn new_debate(&mut self, cx: &mut Context<Self>) {
        self.open_debate = None;
        cx.notify();
    }

    /// The scenario the form holds.
    pub(in crate::ui::features::arena) fn current_scenario(&self, cx: &App) -> Scenario {
        let fields = &self.fields;
        let title = ArenaFields::text(&fields.title, cx);
        scenario_from(&title, &ArenaFields::area_text(&fields.premise, cx), &ArenaFields::text(&fields.question, cx), &fields.variable_drafts(self.lab.variables, cx))
    }

    pub(in crate::ui::features::arena) fn scenario_problem(&self, cx: &App) -> Option<ScenarioProblem> {
        scenario_problem(&self.current_scenario(cx))
    }

    fn chosen_agents(&self) -> Vec<ProfileId> {
        let order: Vec<ProfileId> = self.data.agents.iter().map(|agent| agent.id.clone()).collect();
        self.lab.present_agents(&order)
    }

    /// Runs the form's scenario as a new experiment, or saves it unrun.
    pub(in crate::ui::features::arena) fn submit_lab(&mut self, run: bool, cx: &mut Context<Self>) {
        let scenario = self.current_scenario(cx);
        let agents = self.chosen_agents();
        if scenario_problem(&scenario).is_some() || agents.is_empty() {
            return;
        }
        let preset = self.lab.preset.map(|preset| preset.key().to_string());
        let event = if run { ArenaEvent::RunExperiment { id: None, scenario, agents, preset } } else { ArenaEvent::SaveExperiment { id: None, scenario, agents, preset } };
        cx.emit(event);
    }

    /// Fills the form with `scenario` (one variable row per variable).
    fn fill_scenario(&mut self, scenario: &Scenario, window: &mut Window, cx: &mut Context<Self>) {
        let fields = &self.fields;
        fields.title.update(cx, |field, cx| field.set_value(scenario.title.clone(), window, cx));
        fields.premise.update(cx, |field, cx| field.set_value(scenario.premise.clone(), window, cx));
        fields.question.update(cx, |field, cx| field.set_value(scenario.question.clone(), window, cx));
        for (index, row) in fields.variables.iter().enumerate() {
            let variable = scenario.variables.get(index);
            let (name, values) = variable.map(|v| (v.name.clone(), joined_values(&v.values))).unwrap_or_default();
            row.name.update(cx, |field, cx| field.set_value(name, window, cx));
            row.values.update(cx, |field, cx| field.set_value(values, window, cx));
        }
        self.lab.variables = scenario.variables.len().max(1);
        cx.notify();
    }

    /// Fills the form with a classic experiment, in the user's language.
    pub(in crate::ui::features::arena) fn load_preset(&mut self, preset: PresetKey, window: &mut Window, cx: &mut Context<Self>) {
        let scenario = preset_scenario(preset, |key| t!(key).to_string());
        self.fill_scenario(&scenario, window, cx);
        self.lab.preset = Some(preset);
        self.open_experiment = None;
    }

    /// Shows a saved experiment and fills the form with its scenario and agents.
    pub(in crate::ui::features::arena) fn load_experiment(&mut self, id: ArenaId, window: &mut Window, cx: &mut Context<Self>) {
        let Some(experiment) = self.data.experiment(&id).cloned() else { return };
        self.fill_scenario(&experiment.scenario, window, cx);
        self.lab.agents = experiment.agents.clone();
        self.lab.preset = experiment.preset.as_deref().and_then(PresetKey::from_key);
        self.open_experiment = Some(id);
    }
}
