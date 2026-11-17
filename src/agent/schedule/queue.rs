//! Bounded generation queue: caps pending work and concurrent generations, and never
//! lets the same agent be queued twice or run twice at once.

use crate::domain::ProfileId;
use std::collections::{HashSet, VecDeque};

/// Default maximum number of agents waiting for a generation slot.
pub const DEFAULT_QUEUE_CAPACITY: usize = 4;
/// Default maximum number of generations running at once.
pub const DEFAULT_MAX_IN_FLIGHT: usize = 1;

/// Result of [`GenerationQueue::enqueue`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnqueueOutcome {
    /// Accepted into the pending queue.
    Queued,
    /// Already waiting in the queue; not added again.
    AlreadyQueued,
    /// Already generating; not added.
    InFlight,
    /// The queue is at capacity; dropped.
    Full,
}

/// FIFO of agents awaiting generation plus the set currently generating.
#[derive(Debug, Clone)]
pub struct GenerationQueue {
    pending: VecDeque<ProfileId>,
    in_flight: HashSet<ProfileId>,
    capacity: usize,
    max_in_flight: usize,
}

impl Default for GenerationQueue {
    fn default() -> Self {
        Self::new(DEFAULT_QUEUE_CAPACITY, DEFAULT_MAX_IN_FLIGHT)
    }
}

impl GenerationQueue {
    /// A queue holding at most `capacity` pending agents and running `max_in_flight` at once.
    pub fn new(capacity: usize, max_in_flight: usize) -> Self {
        Self { pending: VecDeque::new(), in_flight: HashSet::new(), capacity, max_in_flight }
    }

    /// Changes how many generations may run at once (running ones finish either way).
    pub fn set_max_in_flight(&mut self, max_in_flight: usize) {
        self.max_in_flight = max_in_flight.max(1);
    }

    /// Adds `id` to the back of the queue unless it is already queued, running, or full.
    pub fn enqueue(&mut self, id: ProfileId) -> EnqueueOutcome {
        if self.in_flight.contains(&id) {
            return EnqueueOutcome::InFlight;
        }
        if self.pending.contains(&id) {
            return EnqueueOutcome::AlreadyQueued;
        }
        if self.pending.len() >= self.capacity {
            return EnqueueOutcome::Full;
        }
        self.pending.push_back(id);
        EnqueueOutcome::Queued
    }

    /// Moves the next pending agent in flight when a concurrency slot is free.
    pub fn start_next(&mut self) -> Option<ProfileId> {
        if self.in_flight.len() >= self.max_in_flight {
            return None;
        }
        let id = self.pending.pop_front()?;
        self.in_flight.insert(id.clone());
        Some(id)
    }

    /// Starts `id` immediately (a user-requested run, which bypasses the concurrency cap).
    /// Returns `false` when it is already generating.
    pub fn start_now(&mut self, id: &ProfileId) -> bool {
        self.pending.retain(|queued| queued != id);
        self.in_flight.insert(id.clone())
    }

    /// Marks the generation of `id` finished, freeing its slot.
    pub fn complete(&mut self, id: &ProfileId) {
        self.in_flight.remove(id);
    }

    /// Drops every waiting agent (running generations are left to finish).
    pub fn clear_pending(&mut self) {
        self.pending.clear();
    }

    /// Whether `id` is currently generating.
    pub fn is_in_flight(&self, id: &ProfileId) -> bool {
        self.in_flight.contains(id)
    }

    /// Number of agents waiting for a slot.
    pub fn pending_len(&self) -> usize {
        self.pending.len()
    }

    /// Number of generations running.
    pub fn in_flight_len(&self) -> usize {
        self.in_flight.len()
    }
}
