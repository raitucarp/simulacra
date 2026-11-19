//! Running the tutor's next request (sb E2) through the post actions runner: the shared
//! daily AI budget is checked, the prompt built for the session's phase and sent on the
//! background executor with the configured timeout and retries; the reply moves the session
//! on when it lands (if it is still the latest request). A turn that closes the dialogue asks
//! for the summary at once.

use super::super::view::AppView;
use crate::agent::learning::{run_tutor_blocking, TutorFailure, TutorReply};
use crate::agent::post_actions::{ActionFailure, ActionJob, CancelToken};
use crate::agent::LlmClient;
use crate::domain::learning::tutor::prompt::TutorRequest;
use crate::domain::learning::{SessionId, TutorSession};
use crate::domain::post_actions::{ActionFailureKind, RequestPolicy};
use crate::domain::spend::SpendEstimate;
use crate::ui::features::post_actions::failure_text;
use crate::ui::features::review::learning::TutorStatus;
use jiff::Timestamp;
use gpui_kit::*;

fn failure_message(failure: &TutorFailure) -> String {
    match failure {
        TutorFailure::Request(failure) => failure_text(failure.kind, &failure.detail),
        TutorFailure::Reply(problem) => rust_i18n::t!(problem.message_key()).to_string(),
    }
}

impl AppView {
    /// Sends the request session `id` waits for, if any.
    pub(super) fn run_tutor(&mut self, id: &SessionId, window: &mut Window, cx: &mut Context<Self>) {
        let Some(session) = self.review.learning.records.session(id).cloned() else { return };
        let Some(request) = TutorRequest::for_session(&session) else { return };
        let resume_id = id.clone();
        if self.has_ai_key() && self.confirm_spend(SpendEstimate::one(), Box::new(move |this, window, cx| this.run_tutor(&resume_id, window, cx)), cx) {
            return;
        }
        self.review.learning.cancel_running();
        self.review.learning.tutor = match self.start_tutor_job(&session, request, window, cx) {
            Ok(()) => TutorStatus::Running(id.clone()),
            Err(failure) => TutorStatus::Failed(id.clone(), failure_message(&TutorFailure::Request(failure))),
        };
        self.review_changed(cx);
    }

    /// Checks the key and the budget, then sends the request in the background.
    fn start_tutor_job(&mut self, session: &TutorSession, request: TutorRequest, window: &mut Window, cx: &mut Context<Self>) -> Result<(), ActionFailure> {
        let language = self.state.preferences().general.language.english_label().to_string();
        let prompt = request.prompt(session, &language);
        let ai = self.state.preferences().ai.clone();
        let policy = RequestPolicy::from_prefs(&ai);
        let mut settings = self.state.llm_settings.clone();
        settings.max_tokens = settings.max_tokens.max(request.answer_tokens());
        let client = LlmClient::new(settings).with_timeout(policy.timeout);
        if !client.has_api_key() {
            return Err(ActionFailure::local(ActionFailureKind::NoKey));
        }
        self.charge_ai_request(&prompt, request.answer_tokens())?;
        let cancel = CancelToken::new();
        self.review.learning.cancel = Some(cancel.clone());
        let (generation, id, mastery) = (self.review.learning.generation, session.id.clone(), session.mastery());
        let job = ActionJob { client, prompt, policy, cancel };
        let task = cx.background_executor().spawn(async move { run_tutor_blocking(job, request, mastery) });
        cx.spawn_in(window, async move |this, cx| {
            let outcome = task.await;
            let _ = this.update_in(cx, |this, window, cx| this.finish_tutor(generation, &id, request, outcome, window, cx));
        })
        .detach();
        Ok(())
    }

    /// Moves the session on with a landed reply (or records why there is none).
    fn finish_tutor(&mut self, generation: u64, id: &SessionId, request: TutorRequest, outcome: Result<TutorReply, TutorFailure>, window: &mut Window, cx: &mut Context<Self>) {
        if generation != self.review.learning.generation {
            return;
        }
        self.review.learning.cancel = None;
        let Some(session) = self.review.learning.records.session(id).cloned() else { return };
        let now = Timestamp::now();
        let next = match (request, outcome) {
            (TutorRequest::Opening, Ok(TutorReply::Turn(reply))) => session.with_opening(&reply.question, now),
            (TutorRequest::Turn, Ok(TutorReply::Turn(reply))) => session.with_turn(reply, now),
            (TutorRequest::Summary, Ok(TutorReply::Summary(summary))) => session.with_summary(summary, now),
            (_, Ok(_)) => session,
            (_, Err(TutorFailure::Request(ActionFailure { kind: ActionFailureKind::Cancelled, .. }))) => return,
            (_, Err(failure)) => {
                self.review.learning.tutor = TutorStatus::Failed(id.clone(), failure_message(&failure));
                return self.review_changed(cx);
            }
        };
        self.review.learning.tutor = TutorStatus::Idle;
        self.store_session(next, cx);
        self.run_tutor(id, window, cx);
    }
}
