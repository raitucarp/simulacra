//! N5: the depth dial in the shell. Picking a depth only changes what the dial shows;
//! asking for a rendering checks the spend, sends the post's rendering prompt through the
//! post actions runner (the configured client, timeout and retries) and keeps the answer as
//! a version of the post. A locked post is never sent: the dial is not offered and the
//! request is refused here too.

use super::super::view::AppView;
use super::thinking_ai::failure_message;
use crate::agent::post_actions::{ActionFailure, CancelToken};
use crate::domain::post::depth::prompt::{can_render, clean_rendering, rendering_prompt};
use crate::domain::post::depth::Depth;
use crate::domain::post_actions::ActionFailureKind;
use crate::domain::spend::SpendEstimate;
use crate::domain::PostId;
use crate::ui::features::composer::publish_error_message;
use crate::ui::features::data::DataEvent;
use crate::ui::features::post_actions::failure_text;
use gpui_kit::*;

/// The answer budget of a rendering (a post's length, with room to spare).
const RENDERING_TOKENS: u32 = 400;

impl AppView {
    /// Routes one depth dial event.
    pub(in crate::ui::features::shell) fn on_depth_event(&mut self, event: DataEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            DataEvent::ShowDepth(id, Some(depth)) => {
                self.data.depth.chosen.insert(id, depth);
            }
            DataEvent::ShowDepth(id, None) => {
                self.data.depth.chosen.remove(&id);
            }
            DataEvent::RenderDepth(id, depth) => self.ask_rendering(id, depth, window, cx),
            _ => {}
        }
        cx.notify();
    }

    /// Sends the post `id`'s `depth` rendering request, after the spend check.
    fn ask_rendering(&mut self, id: PostId, depth: Depth, _: &mut Window, cx: &mut Context<Self>) {
        let renders = self.state.get_post(&id).is_some_and(can_render); // X2: never a locked post
        if !renders || self.data.depth.running.is_some() {
            return;
        }
        let resume = id.clone();
        if self.confirm_spend(SpendEstimate::one(), Box::new(move |this, window, cx| this.on_depth_event(DataEvent::RenderDepth(resume, depth), window, cx)), cx) {
            return;
        }
        let Some(context) = self.action_context(&id) else { return };
        match self.thinking_job(rendering_prompt(depth, &context), RENDERING_TOKENS, CancelToken::new()) {
            Ok(job) => {
                self.data.depth_ticket += 1;
                self.data.depth.running = Some((id.clone(), depth));
                self.data.depth.problems.remove(&id);
                self.spawn_thinking_job(job, self.data.depth_ticket, Self::finish_rendering, cx);
            }
            Err(failure) => {
                self.data.depth.problems.insert(id, failure_message(&failure));
            }
        }
    }

    /// Keeps a finished rendering as a version of its post, or says why there is none.
    fn finish_rendering(&mut self, ticket: u64, outcome: Result<String, ActionFailure>, cx: &mut Context<Self>) {
        if ticket != self.data.depth_ticket {
            return;
        }
        let Some((id, depth)) = self.data.depth.running.take() else { return };
        let text = outcome.map_err(|failure| failure_message(&failure)).and_then(|answer| clean_rendering(&answer).ok_or_else(|| failure_text(ActionFailureKind::Empty, "")));
        let kept = text.and_then(|text| self.state.keep_rendering(&id, depth, &text, jiff::Timestamp::now()).map_err(|err| publish_error_message(&err)));
        if let Err(problem) = kept {
            self.data.depth.problems.insert(id, problem);
        }
        cx.notify();
    }
}
