//! To-dos in the shell (D6): clicking a box of one of the user's own posts ticks it, kept
//! with the post without a new version; completing a repeating item says when it comes
//! round next (D8); the composer is handed what completion offers (D11); agents handed a
//! to-do are asked for a progress reply (D12).

use super::super::view::AppView;
use crate::agent::JobKind;
use crate::domain::preferences::ToastKind;
use crate::domain::PostId;
use crate::ui::features::composer::publish_error_message;
use crate::ui::features::post::todo_due_label;
use gpui_kit::component::notification::Notification;
use gpui_kit::*;
use rust_i18n::t;

impl AppView {
    /// Ticks (or unticks) the `index`-th to-do of `id`, saying why when it cannot.
    pub(in crate::ui::features::shell) fn toggle_todo(&mut self, id: &PostId, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let rounds = |this: &Self| this.state.post_todos(id).get(index).map(|entry| entry.completions);
        let before = rounds(self);
        match self.state.toggle_todo(id, index) {
            Ok(_) if rounds(self) > before => self.say_next_round(id, index, window, cx),
            Ok(_) => {}
            Err(err) => self.notify_data(publish_error_message(&err), window, cx),
        }
        cx.notify();
    }

    /// "Next: Thursday, 2026-11-26" once a repeating item came round again.
    fn say_next_round(&mut self, id: &PostId, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(post) = self.state.get_post(id) else { return };
        let written = crate::time::local_date(post.created_at);
        let Some(due) = self.state.post_todos(id).get(index).and_then(|entry| entry.due(written)) else { return };
        let text = t!("todo.repeat.next", when = todo_due_label(due, crate::time::local_now().date())).to_string();
        self.toast(ToastKind::Confirmation, Notification::success(text), window, cx);
    }
}

impl AppView {
    /// Hands the composer what to-do completion may offer (D11): the contexts the user's to-dos
    /// use and the active network's agents.
    pub(in crate::ui::features::shell) fn hand_completion_sources(&self, cx: &mut Context<Self>) {
        let agents = self.state.current_network_agents().iter().map(|agent| agent.handle.trim_start_matches('@').to_string()).collect();
        let sources = crate::domain::todo::complete::CompletionSources { contexts: self.state.task_contexts(), agents };
        self.composer_modal.update(cx, |panel, _| panel.set_completion_sources(sources));
    }
}

impl AppView {
    /// Asks each agent handed a to-do (D12) for a progress reply to its post, once per item;
    /// an agent already busy is asked on a later heartbeat.
    pub(in crate::ui::features::shell) fn dispatch_delegations(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        for job in self.state.pending_delegations() {
            if !self.scheduler.queue.start_now(&job.agent.id) {
                continue;
            }
            let parent = self.state.get_post(&job.post_id).map(|post| self.state.authored(post));
            match (parent, self.state.mark_progress_asked(&job.post_id, job.index)) {
                (Some(parent), Ok(())) => {
                    let network = parent.network_id.clone();
                    self.spawn_generation(job.agent, JobKind::Progress { parent, item: job.item }, network, window, cx);
                }
                (_, result) => {
                    self.scheduler.queue.complete(&job.agent.id);
                    if let Err(err) = result {
                        self.report_storage_failure(&err, window, cx);
                    }
                }
            }
        }
    }
}
