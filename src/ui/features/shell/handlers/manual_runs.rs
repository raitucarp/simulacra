//! A5: the run the user started from an agent's profile: starting it (at once, or queued
//! behind the agent's current generation), following it to its post, and cancelling it.

use super::super::view::AppView;
use crate::agent::schedule::RunStatus;
use crate::domain::post::TriggerMode;
use crate::domain::ProfileId;
use crate::ui::features::roster::RunEvent;
use gpui_kit::*;

impl AppView {
    /// A4: the dialog's "Write": runs `mode` for `agent` now, or queues it while the agent
    /// is already writing.
    pub(crate) fn start_manual_trigger(&mut self, agent: ProfileId, mode: TriggerMode, window: &mut Window, cx: &mut Context<Self>) {
        let busy = self.scheduler.queue.is_in_flight(&agent);
        if self.lab.runs.request(&agent, mode.clone(), busy) == Some(RunStatus::Writing) && !self.trigger_simulation_for_agent(Some(agent.clone()), mode, window, cx) {
            self.lab.runs.finished(&agent, None, true);
        }
        cx.notify();
    }

    /// Starts `agent`'s queued run once its generation has ended.
    pub(super) fn pump_manual_run(&mut self, agent: &ProfileId, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(mode) = self.lab.runs.take_queued(agent) {
            if !self.trigger_simulation_for_agent(Some(agent.clone()), mode, window, cx) {
                self.lab.runs.finished(agent, None, true);
            }
        }
    }

    /// The status line's Cancel and Dismiss.
    pub(crate) fn run_status_event(&mut self, event: RunEvent, _: &mut Window, cx: &mut Context<Self>) {
        match event {
            RunEvent::Cancel(agent) => {
                self.lab.runs.cancel(&agent);
            }
            RunEvent::Dismiss(agent) => self.lab.runs.dismiss(&agent),
        }
        cx.notify();
    }
}
