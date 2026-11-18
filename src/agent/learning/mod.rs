//! The tutor's requests (sb E2), effectful side: the prompt goes through the post actions
//! runner (timeout, retries with backoff, cancel); the reply is parsed and validated, and an
//! unusable reply (a lecture, no question, no JSON) is asked for once more with a nudge.
//! Prompts and parsing are pure (`domain::learning::tutor`).

#[cfg(test)]
mod tests;

use crate::agent::post_actions::{run_action_blocking, ActionFailure, ActionJob};
use crate::domain::learning::tutor::prompt::{TutorRequest, RETRY_NUDGE};
use crate::domain::learning::tutor::{parse_summary, parse_turn_reply, ReplyProblem, TurnReply, TutorSummary};
use crate::domain::learning::BloomLevel;
use crate::domain::post_actions::ActionPrompt;

/// A parsed reply.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TutorReply {
    /// An opening question or a turn (assessment and next question).
    Turn(TurnReply),
    Summary(TutorSummary),
}

/// Why a request gave no usable reply.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TutorFailure {
    Request(ActionFailure),
    Reply(ReplyProblem),
}

fn parse(request: TutorRequest, text: &str, mastery: Option<BloomLevel>) -> Result<TutorReply, ReplyProblem> {
    match request {
        TutorRequest::Opening | TutorRequest::Turn => parse_turn_reply(text).map(TutorReply::Turn),
        TutorRequest::Summary => parse_summary(text, mastery).map(TutorReply::Summary),
    }
}

/// Runs `job` (its prompt built for `request`) and parses the reply; an unusable reply is
/// retried once with [`RETRY_NUDGE`] appended. `mastery` goes into a summary.
pub fn run_tutor_blocking(job: ActionJob, request: TutorRequest, mastery: Option<BloomLevel>) -> Result<TutorReply, TutorFailure> {
    let text = run_action_blocking(&job).map_err(TutorFailure::Request)?;
    match parse(request, &text, mastery) {
        Ok(reply) => Ok(reply),
        Err(_) => {
            let prompt = ActionPrompt { user: format!("{}\n\n{RETRY_NUDGE}", job.prompt.user), ..job.prompt.clone() };
            let retry = ActionJob { prompt, ..job };
            let text = run_action_blocking(&retry).map_err(TutorFailure::Request)?;
            parse(request, &text, mastery).map_err(TutorFailure::Reply)
        }
    }
}
