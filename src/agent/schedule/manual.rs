//! Runs the user started from a profile (A4, A5): what each agent's run is doing (queued
//! behind a generation already running for it, writing, done, failed or cancelled), and what
//! a cancel or a finished generation means for it. One run per agent at a time; a run only
//! ever starts from the user's click (`request`).

use crate::domain::post::TriggerMode;
use crate::domain::{PostId, ProfileId};
use std::collections::HashMap;

/// Where one run stands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunStatus {
    /// Waiting for the agent's current generation to finish.
    Queued,
    /// Its request is out.
    Writing,
    /// Its post landed.
    Done { post: Option<PostId> },
    /// The model failed (the shell has said why).
    Failed,
    /// The user cancelled it; a reply still landing is dropped.
    Cancelled,
}

impl RunStatus {
    /// Queued or writing: it can still be cancelled.
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Queued | Self::Writing)
    }
}

/// What a cancel did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CancelOutcome {
    /// It had not started: nothing was sent.
    Dequeued,
    /// Its request is out: the reply is dropped when it lands.
    WillDiscard,
    /// Nothing was running.
    Nothing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ManualRun {
    mode: TriggerMode,
    status: RunStatus,
    /// Cancelled while writing: the reply still owed is dropped.
    discard: bool,
}

/// Every agent's latest run.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ManualRuns {
    runs: HashMap<ProfileId, ManualRun>,
}

impl ManualRuns {
    /// The status of `agent`'s latest run.
    pub fn status(&self, agent: &ProfileId) -> Option<&RunStatus> {
        self.runs.get(agent).map(|run| &run.status)
    }

    /// The user asked `agent` for `mode`; `busy` when a generation of it is already running.
    /// Returns the new status, or `None` when a run of it is already active (nothing changes).
    pub fn request(&mut self, agent: &ProfileId, mode: TriggerMode, busy: bool) -> Option<RunStatus> {
        if self.status(agent).is_some_and(RunStatus::is_active) {
            return None;
        }
        let status = if busy { RunStatus::Queued } else { RunStatus::Writing };
        let discard = self.runs.get(agent).is_some_and(|run| run.discard); // a cancelled reply may still be owed
        self.runs.insert(agent.clone(), ManualRun { mode, status: status.clone(), discard });
        Some(status)
    }

    /// `agent`'s generation ended: a queued run is due now (its mode is returned to start).
    pub fn take_queued(&mut self, agent: &ProfileId) -> Option<TriggerMode> {
        let run = self.runs.get_mut(agent).filter(|run| run.status == RunStatus::Queued)?;
        run.status = RunStatus::Writing;
        Some(run.mode.clone())
    }

    /// The writing run of `agent` ended with `post` (or failed); `false` when the user
    /// cancelled it, so its post must be dropped. Generations the user did not start pass.
    pub fn finished(&mut self, agent: &ProfileId, post: Option<PostId>, failed: bool) -> bool {
        let Some(run) = self.runs.get_mut(agent) else { return true };
        if run.discard {
            run.discard = false;
            return false;
        }
        if run.status == RunStatus::Writing {
            run.status = if failed { RunStatus::Failed } else { RunStatus::Done { post } };
        }
        true
    }

    /// The user cancelled `agent`'s run.
    pub fn cancel(&mut self, agent: &ProfileId) -> CancelOutcome {
        let Some(run) = self.runs.get_mut(agent) else { return CancelOutcome::Nothing };
        let outcome = match run.status {
            RunStatus::Queued => CancelOutcome::Dequeued,
            RunStatus::Writing => CancelOutcome::WillDiscard,
            _ => return CancelOutcome::Nothing,
        };
        run.discard = outcome == CancelOutcome::WillDiscard;
        run.status = RunStatus::Cancelled;
        outcome
    }

    /// Forgets a run that is over (the status line's close button).
    pub fn dismiss(&mut self, agent: &ProfileId) {
        if self.runs.get(agent).is_some_and(|run| !run.status.is_active() && !run.discard) {
            self.runs.remove(agent);
        }
    }
}
