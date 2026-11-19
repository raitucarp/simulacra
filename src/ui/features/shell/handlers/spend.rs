//! The one AI budget path and "Ask before spending" (sb P16). Every AI action the user starts
//! calls [`AppView::confirm_spend`] first: with the preference on it opens a confirm stating
//! how many requests the action sends and resumes the action once it is accepted. Every
//! request then goes through [`AppView::charge_ai_request`], which charges only what the
//! session allowed and what today's budget still has room for.

use super::super::view::AppView;
use super::spend_dialog::{open_spend_dialog, SpendAsk};
use crate::agent::post_actions::ActionFailure;
use crate::domain::embedding::{estimate_tokens, BudgetLimits};
use crate::domain::post_actions::{ActionFailureKind, ActionPrompt};
use crate::domain::spend::SpendEstimate;
use gpui_kit::*;

/// Local calendar day key of the AI budget (shared with embedding requests).
const DAY_FORMAT: &str = "%Y-%m-%d";

/// The action a confirm was asked for, run once it is accepted.
pub(in crate::ui::features::shell) type SpendResume = Box<dyn FnOnce(&mut AppView, &mut Window, &mut Context<AppView>) + 'static>;

/// Today's budget key.
pub(in crate::ui::features::shell) fn budget_day() -> String {
    chrono::Local::now().format(DAY_FORMAT).to_string()
}

impl AppView {
    /// Whether the action of `estimate` waits for a confirm. When it does (true), the confirm
    /// opens and `resume` runs once it is accepted, so the caller stops; otherwise it goes on.
    pub(in crate::ui::features::shell) fn confirm_spend(&mut self, estimate: SpendEstimate, resume: SpendResume, cx: &mut Context<Self>) -> bool {
        self.confirm_spend_or(estimate, resume, None, cx)
    }

    /// [`Self::confirm_spend`] for an action already shown as running: `decline` runs when the
    /// confirm closes without sending, so it can stand down.
    pub(in crate::ui::features::shell) fn confirm_spend_or(&mut self, estimate: SpendEstimate, resume: SpendResume, decline: Option<SpendResume>, cx: &mut Context<Self>) -> bool {
        let ask = self.state.preferences().ai.ask_before_spending;
        if !self.spend.consent.must_ask(ask) {
            self.spend.consent = self.spend.consent.started();
            return false;
        }
        if !self.spend.asking {
            self.spend.asking = true;
            let ask = SpendAsk { estimate, left: self.requests_left(), gap: self.theme.stack_gap, resume, decline };
            let (view, handle) = (cx.entity().downgrade(), self.spend.window);
            cx.defer(move |cx| {
                let _ = handle.update(cx, |_, window, cx| open_spend_dialog(view, ask, window, cx));
            });
        }
        true
    }

    /// The confirm was accepted: the session allows `estimate` and the action runs.
    pub(in crate::ui::features::shell) fn accept_spend(&mut self, estimate: SpendEstimate, remember: bool, resume: SpendResume, window: &mut Window, cx: &mut Context<Self>) {
        self.spend.asking = false;
        self.spend.consent = self.spend.consent.granted(estimate, remember);
        resume(self, window, cx);
        cx.notify();
    }

    /// The confirm closed; `declined` is set when it closed without sending.
    pub(in crate::ui::features::shell) fn close_spend(&mut self, declined: Option<SpendResume>, window: &mut Window, cx: &mut Context<Self>) {
        self.spend.asking = false;
        if let Some(decline) = declined {
            decline(self, window, cx);
        }
    }

    /// Requests today's budget still allows (`None` without a daily limit).
    fn requests_left(&self) -> Option<u32> {
        let cap = BudgetLimits::from_prefs(&self.state.preferences().ai).daily_requests;
        (cap > 0).then(|| cap.saturating_sub(self.semantic.usage.on(&budget_day()).requests))
    }

    /// Charges one request of `prompt` (with an answer of up to `answer_tokens`) to today's AI
    /// budget, shared by post actions, embeddings, the thinking tools, the tutor, the Arena and
    /// belief updates, after "Ask before spending" allowed it.
    pub(in crate::ui::features::shell) fn charge_ai_request(&mut self, prompt: &ActionPrompt, answer_tokens: u32) -> Result<(), ActionFailure> {
        let ai = self.state.preferences().ai.clone();
        if !self.spend.consent.allows(ai.ask_before_spending) {
            return Err(ActionFailure::local(ActionFailureKind::Unapproved));
        }
        let today = budget_day();
        let tokens = estimate_tokens([prompt.system.as_str(), prompt.user.as_str()]).saturating_add(answer_tokens);
        if !self.semantic.usage.allows(BudgetLimits::from_prefs(&ai), &today, 1, tokens) {
            return Err(ActionFailure::local(ActionFailureKind::Budget));
        }
        self.semantic.usage = self.semantic.usage.recorded(&today, 1, tokens);
        self.spend.consent = self.spend.consent.spent();
        if let Some(vault) = self.state.db.as_ref() {
            let _ = vault.save_embedding_usage(&self.semantic.usage);
        }
        Ok(())
    }
}
