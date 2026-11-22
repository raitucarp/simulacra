//! The monthly migration review (S16): a guided walk through the user's open tasks, one at
//! a time, each migrated, scheduled, dropped or kept as it is, ending with a tally that can
//! be posted as a summary.

use super::kinds::{EntryState, EntrySymbol};
use super::PostSymbols;
use jiff::civil::Date;

/// What the review does with one task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReviewChoice {
    Migrate,
    Schedule,
    Drop,
    /// Leave it open.
    Keep,
}

impl ReviewChoice {
    pub const ALL: [Self; 4] = [Self::Migrate, Self::Schedule, Self::Drop, Self::Keep];

    /// The state the task moves to (`None` keeps it open).
    pub const fn state(self) -> Option<EntryState> {
        match self {
            Self::Migrate => Some(EntryState::Migrated),
            Self::Schedule => Some(EntryState::Scheduled),
            Self::Drop => Some(EntryState::Dropped),
            Self::Keep => None,
        }
    }

    /// Stable key (locale keys, element ids).
    pub const fn key(self) -> &'static str {
        match self {
            Self::Migrate => "migrate",
            Self::Schedule => "schedule",
            Self::Drop => "drop",
            Self::Keep => "keep",
        }
    }
}

/// Whether `symbols` is an open task (what the review walks through).
pub fn needs_review(symbols: &PostSymbols) -> bool {
    symbols.entry == Some(EntrySymbol::Task) && symbols.state == Some(EntryState::Open)
}

/// Whether the monthly review is due on `today`: never done, or last done in an earlier month.
pub fn review_due(last: Option<Date>, today: Date) -> bool {
    last.is_none_or(|last| (last.year(), last.month()) < (today.year(), today.month()))
}

/// A review in progress: the tasks to walk through (post ids, oldest first) and what was
/// decided so far, in order.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MigrationReview<T> {
    pub queue: Vec<T>,
    pub decided: Vec<(T, ReviewChoice)>,
}

impl<T: Clone + PartialEq> MigrationReview<T> {
    pub fn new(queue: Vec<T>) -> Self {
        Self { queue, decided: Vec::new() }
    }

    /// The task being looked at; `None` once every task is decided.
    pub fn current(&self) -> Option<&T> {
        self.queue.get(self.decided.len())
    }

    /// Records `choice` for the current task and moves on; `false` when none is left.
    pub fn decide(&mut self, choice: ReviewChoice) -> bool {
        let Some(task) = self.current().cloned() else { return false };
        self.decided.push((task, choice));
        true
    }

    pub fn is_done(&self) -> bool {
        self.current().is_none()
    }

    /// How many tasks got `choice`.
    pub fn count(&self, choice: ReviewChoice) -> usize {
        self.decided.iter().filter(|(_, c)| *c == choice).count()
    }
}
