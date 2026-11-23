//! N14: retractions and corrections in the shell. Marking a post asks for the reason; once it
//! is marked, the agents that cited it (and may read it) are told: each answers the post
//! through the usual generation path (its engine, the queue and the spend rules), reading the
//! mark with it, and is remembered as told. A locked post is never marked.

use super::super::view::AppView;
use crate::agent::JobKind;
use crate::domain::post::correction::CorrectionKind;
use crate::domain::spend::SpendEstimate;
use crate::domain::PostId;
use crate::ui::features::composer::publish_error_message;
use crate::ui::features::data::{tr, CorrectionDialog, CorrectionEvent};
use gpui_kit::*;
use rust_i18n::t;

impl AppView {
    /// Routes one retraction or correction event.
    pub(in crate::ui::features::shell) fn on_correction_event(&mut self, event: CorrectionEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            CorrectionEvent::Open(id, kind) => self.open_correction(id, kind, window, cx),
            CorrectionEvent::Withdraw(id) => match self.state.withdraw_correction(&id) {
                Ok(()) => self.notify_data(tr("posts.correction.withdrawn"), window, cx),
                Err(err) => self.notify_data(publish_error_message(&err), window, cx),
            },
            CorrectionEvent::Submit => self.submit_correction(window, cx),
            CorrectionEvent::Close => self.close_correction(window, cx),
        }
    }

    fn open_correction(&mut self, id: PostId, kind: CorrectionKind, window: &mut Window, cx: &mut Context<Self>) {
        if self.state.get_post(&id).is_none_or(|post| post.is_locked()) {
            return;
        }
        let reason = self.data.inputs.ideas.correction_reason.clone();
        reason.update(cx, |field, cx| field.set_value("", window, cx));
        self.data.ideas.correction = Some(CorrectionDialog { post_id: id, kind, reason: reason.clone(), problem: None });
        reason.update(cx, |field, cx| field.focus(window, cx));
    }

    fn close_correction(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.data.inputs.ideas.correction_reason.update(cx, |field, cx| field.set_value("", window, cx));
        self.data.ideas.correction = None;
    }

    fn submit_correction(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(dialog) = self.data.ideas.correction.as_ref() else { return };
        let (id, kind, reason) = (dialog.post_id.clone(), dialog.kind, dialog.reason.read(cx).value().to_string());
        match self.state.mark_correction(&id, kind, &reason, jiff::Timestamp::now()) {
            Ok(_) => {
                self.close_correction(window, cx);
                self.notify_data(tr(&format!("posts.correction.marked_{}", kind.key())), window, cx);
                self.tell_citers(&id, window, cx);
            }
            Err(err) => {
                let problem = if matches!(err, crate::error::AppError::EmptyContent) { tr("posts.correction.reason_first") } else { publish_error_message(&err) };
                if let Some(dialog) = self.data.ideas.correction.as_mut() {
                    dialog.problem = Some(problem);
                }
            }
        }
    }

    /// Tells the agents that cited post `id` of its mark; each may answer it.
    pub(in crate::ui::features::shell) fn tell_citers(&mut self, id: &PostId, window: &mut Window, cx: &mut Context<Self>) {
        let Some(post) = self.state.get_post(id).filter(|post| !post.is_locked() && post.extras.correction.is_some()) else { return };
        let (parent, network) = (self.state.authored(post), post.network_id.clone());
        let agents: Vec<_> = self.state.citers_to_tell(id).into_iter().cloned().collect();
        if agents.is_empty() {
            return;
        }
        let resume = id.clone();
        if self.confirm_spend(SpendEstimate::requests(agents.len() as u32), Box::new(move |this, window, cx| this.tell_citers(&resume, window, cx)), cx) {
            return;
        }
        let told: Vec<_> = agents.iter().map(|agent| agent.id.clone()).collect();
        if self.state.record_told(id, &told).is_err() {
            return;
        }
        let names = agents.iter().map(|agent| agent.name.clone()).collect::<Vec<_>>().join(", ");
        for agent in agents {
            if self.engine_allowed(&agent) && self.scheduler.queue.start_now(&agent.id) {
                self.spawn_generation(agent, JobKind::Reply(parent.clone()), network.clone(), window, cx);
            }
        }
        self.notify_data(t!("posts.correction.told", names = names).to_string(), window, cx);
    }
}
