//! The background indexing queue as a pure state machine: a plan fills it, batches are taken
//! one at a time, and a batch result is applied only if it belongs to the current
//! generation (a cancel starts a new generation, so late results are dropped).

use super::failure::FailureKind;
use super::plan::ChunkJob;
use std::collections::{HashSet, VecDeque};

/// What the queue is doing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueStatus {
    /// Nothing to do.
    Idle,
    Running,
    /// Stopped by the user; new plans wait until [`IndexQueue::resumed`].
    Cancelled,
    /// Stopped by a failure; new plans wait until [`IndexQueue::resumed`].
    Paused(FailureKind),
}

/// A batch handed to the provider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Batch {
    pub generation: u64,
    pub jobs: Vec<ChunkJob>,
}

/// "Indexing `done` of `total`".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Progress {
    pub done: usize,
    pub total: usize,
}

/// The indexing queue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexQueue {
    generation: u64,
    pending: VecDeque<ChunkJob>,
    in_flight: Vec<ChunkJob>,
    done: usize,
    status: QueueStatus,
}

impl Default for IndexQueue {
    fn default() -> Self {
        Self { generation: 0, pending: VecDeque::new(), in_flight: Vec::new(), done: 0, status: QueueStatus::Idle }
    }
}

impl IndexQueue {
    pub fn status(&self) -> QueueStatus {
        self.status
    }

    /// Whether a batch is out at the provider.
    pub fn is_busy(&self) -> bool {
        !self.in_flight.is_empty()
    }

    /// Chunks still waiting.
    pub fn pending_len(&self) -> usize {
        self.pending.len()
    }

    /// Progress while running.
    pub fn progress(&self) -> Option<Progress> {
        let total = self.done + self.in_flight.len() + self.pending.len();
        (self.status == QueueStatus::Running && total > 0).then_some(Progress { done: self.done, total })
    }

    /// The queue waiting on `jobs` (replacing what was pending; chunks already out at the
    /// provider are not queued twice). A stopped queue keeps the jobs but stays stopped.
    pub fn with_plan(self, jobs: Vec<ChunkJob>) -> Self {
        let out: HashSet<_> = self.in_flight.iter().map(ChunkJob::key).collect();
        let pending: VecDeque<ChunkJob> = jobs.into_iter().filter(|job| !out.contains(&job.key())).collect();
        let fresh_run = self.status == QueueStatus::Idle;
        let done = if fresh_run { 0 } else { self.done };
        let has_work = !pending.is_empty() || !self.in_flight.is_empty();
        let status = match self.status {
            QueueStatus::Idle | QueueStatus::Running if has_work => QueueStatus::Running,
            QueueStatus::Idle | QueueStatus::Running => QueueStatus::Idle,
            stopped => stopped,
        };
        Self { pending, done, status, ..self }
    }

    /// A stopped queue running again (after a rebuild, or a provider or key change).
    pub fn resumed(self) -> Self {
        let has_work = !self.pending.is_empty() || !self.in_flight.is_empty();
        let status = if has_work { QueueStatus::Running } else { QueueStatus::Idle };
        Self { status, done: 0, ..self }
    }

    /// Takes the next batch of at most `max` chunks (none while one is out or stopped).
    pub fn take_batch(mut self, max: usize) -> (Self, Option<Batch>) {
        if self.status != QueueStatus::Running || self.is_busy() || self.pending.is_empty() {
            return (self, None);
        }
        let count = max.max(1).min(self.pending.len());
        self.in_flight = self.pending.drain(..count).collect();
        let batch = Batch { generation: self.generation, jobs: self.in_flight.clone() };
        (self, Some(batch))
    }

    /// The queue after batch `generation` was embedded and stored.
    pub fn completed(mut self, generation: u64) -> Self {
        if generation != self.generation {
            return self;
        }
        self.done += std::mem::take(&mut self.in_flight).len();
        if self.pending.is_empty() {
            return Self { done: 0, status: QueueStatus::Idle, ..self };
        }
        self
    }

    /// The queue after batch `generation` failed: its chunks wait again, and the queue pauses.
    pub fn failed(mut self, generation: u64, kind: FailureKind) -> Self {
        if generation != self.generation {
            return self;
        }
        let returned = std::mem::take(&mut self.in_flight);
        returned.into_iter().rev().for_each(|job| self.pending.push_front(job));
        Self { status: QueueStatus::Paused(kind), ..self }
    }

    /// The queue emptied by the user; any batch still out is ignored when it returns.
    pub fn cancelled(self) -> Self {
        Self { generation: self.generation + 1, status: QueueStatus::Cancelled, ..Self::default() }
    }

    /// Whether `generation` is still current (a late batch of a cancelled run is not).
    pub fn is_current(&self, generation: u64) -> bool {
        generation == self.generation
    }
}
