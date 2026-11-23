//! A4: the trigger dialog of an agent's profile ("Write a post…"): a new post on an optional
//! topic or vault document, or a reply to the network's newest post, with the run's likely
//! cost. Nothing is sent until the user clicks "Write".

use super::super::view::AppView;
use crate::agent::runtime::EnginePlan;
use crate::agent::{JobKind, PromptContext};
use crate::domain::document::DocumentId;
use crate::domain::post::TriggerMode;
use crate::domain::spend::{trigger_estimate, TriggerEstimate};
use crate::domain::ProfileId;
use crate::ui::features::roster::{render_trigger_body, ChangeChoice, TriggerBody, TriggerChoice};
use gpui_kit::component::button::ButtonVariant;
use gpui_kit::component::input::InputState;
use gpui_kit::component::WindowExt;
use gpui_kit::*;
use rust_i18n::t;
use std::cell::RefCell;
use std::rc::Rc;

/// Vault documents offered as a seed at most.
const MAX_DOCUMENTS: usize = 8;

impl AppView {
    /// What `mode` would likely cost for `agent`; `None` when it cannot start.
    pub(crate) fn trigger_estimate(&self, agent: &ProfileId, mode: &TriggerMode) -> Option<TriggerEstimate> {
        let profile = self.simulation_agent(Some(agent))?;
        let kind: JobKind = self.trigger_job_kind(mode.clone())?;
        let network = self.state.active_network_id.clone();
        let context = PromptContext { beliefs: self.belief_dossier(agent), ..self.prompt_context(&network) };
        let system = crate::agent::DialecticEngine::system_prompt_in(&profile, &context);
        let user = kind.user_prompt(&profile);
        let engine = &profile.config.engine;
        let ready = EnginePlan::gate_of(engine, &self.state.llm_settings, &self.state.credentials).1;
        let steps = if engine.tools.is_empty() { 0 } else { engine.max_tool_steps };
        Some(trigger_estimate(&[system.as_str(), user.as_str()], self.state.llm_settings.max_tokens, steps, ready))
    }

    /// The network's vault documents a new post may start from.
    fn seed_documents(&self) -> Vec<(DocumentId, String)> {
        let network = &self.state.active_network_id;
        self.state.documents.iter().filter(|doc| &doc.network_id == network).take(MAX_DOCUMENTS).map(|doc| (DocumentId::new(doc.id.as_str()), doc.title.clone())).collect()
    }

    /// Opens the dialog for `agent`.
    pub(crate) fn open_trigger_dialog(&mut self, agent: ProfileId, window: &mut Window, cx: &mut Context<Self>) {
        let Some(profile) = self.simulation_agent(Some(&agent)) else { return };
        let topic = cx.new(|cx| InputState::new(window, cx).placeholder(t!("trigger.topic_placeholder").to_string()));
        let (documents, theme, view) = (self.seed_documents(), self.theme.clone(), cx.entity().downgrade());
        let choice = Rc::new(RefCell::new(TriggerChoice::default()));
        let title = t!("trigger.title", name = profile.name).to_string();
        window.open_alert_dialog(cx, move |alert, _, cx| {
            let current = choice.borrow().clone();
            let mode = current.mode(&topic.read(cx).value());
            let estimate = view.upgrade().and_then(|view| view.read(cx).trigger_estimate(&agent, &mode));
            let store = choice.clone();
            let on_change: ChangeChoice = Rc::new(move |next, window, _| {
                *store.borrow_mut() = next;
                window.refresh();
            });
            let body = render_trigger_body(TriggerBody { choice: &current, topic: &topic, documents: &documents, estimate, on_change }, &theme);
            let (ok_view, ok_choice, ok_topic, ok_agent) = (view.clone(), choice.clone(), topic.clone(), agent.clone());
            alert
                .confirm()
                .title(title.clone())
                .description(body)
                .ok_text(t!("trigger.write").to_string())
                .ok_variant(ButtonVariant::Primary)
                .cancel_text(t!("composer.cancel").to_string())
                .on_ok(move |_, window, cx| {
                    let mode = ok_choice.borrow().mode(&ok_topic.read(cx).value());
                    let _ = ok_view.update(cx, |this, cx| this.start_manual_trigger(ok_agent.clone(), mode, window, cx));
                    true
                })
        });
    }
}
