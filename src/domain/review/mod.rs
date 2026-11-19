//! Spaced repetition and quizzes on the posts and passages themselves, pure side (sb C1,
//! anchored on posts in sb C1b).
//!
//! Grounded in learning science: retrieval practice (every review is a recall attempt),
//! spacing (FSRS schedules each item for the desired retention), desirable difficulty
//! (plausible distractors, typed recall), interleaving (new and due items, and items from
//! different sources, are mixed), elaboration (recalling how ideas connect) and
//! metacognition (self-grading, confidence calibration).
//!
//! - [`fsrs`]: the memory model; [`schedule`]: an item's schedule after a review.
//! - [`item`]: items anchored on a post or a vault passage, the learner writing one face.
//! - [`connections`]: connection maps and their drill (connection recall).
//! - [`queue`] / [`session`]: today's queue within the daily limits and a session's state.
//! - [`log`]: answers applied to items and logged; [`stats`]: charts' numbers.
//! - [`quiz`]: multiple choice, free recall and ordering; [`ai`]: AI-suggested faces.
//! - [`legacy`]: the retired standalone flashcards carried over to anchored items.

pub mod ai;
pub mod connections;
pub mod day;
pub mod fsrs;
mod grade;
pub mod item;
pub mod legacy;
pub mod log;
pub mod queue;
pub mod quiz;
pub mod schedule;
pub mod session;
pub mod stats;

#[cfg(test)]
mod tests;

pub use connections::ConnectionMap;
pub use grade::{Confidence, Grade};
pub use item::{Anchor, AnchorSide, ItemDraft, ItemId, ReviewItem, Subject};
pub use log::{apply_answer, Answer, ReviewEntry, ReviewMode};
pub use queue::{build_queue, plan_today, DailyLimits, TodayPlan};
pub use schedule::{ItemSchedule, Phase, Policy};
pub use session::{Session, Stage};

use crate::domain::NetworkId;

/// Every item and logged answer (all networks), as loaded from the vault.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Deck {
    pub items: Vec<ReviewItem>,
    pub log: Vec<ReviewEntry>,
}

impl Deck {
    /// The deck narrowed to `network`.
    pub fn of_network(&self, network: &NetworkId) -> Deck {
        Deck {
            items: self.items.iter().filter(|item| &item.network_id == network).cloned().collect(),
            log: self.log.iter().filter(|entry| &entry.network_id == network).cloned().collect(),
        }
    }

    pub fn item(&self, id: &ItemId) -> Option<&ReviewItem> {
        self.items.iter().find(|item| &item.id == id)
    }

    /// The active (not retired) item anchored on post `id` in `network`, if any.
    pub fn on_post(&self, network: &NetworkId, id: &str) -> Option<&ReviewItem> {
        self.items.iter().find(|item| &item.network_id == network && !item.retired && item.is_on_post(id))
    }

    /// The deck with `item` added or replaced.
    pub fn with_item(self, item: ReviewItem) -> Deck {
        let mut items: Vec<ReviewItem> = self.items.into_iter().filter(|i| i.id != item.id).collect();
        items.push(item);
        Deck { items, ..self }
    }

    /// The deck without item `id` (its log stays for the statistics).
    pub fn without_item(self, id: &ItemId) -> Deck {
        Deck { items: self.items.into_iter().filter(|item| &item.id != id).collect(), ..self }
    }

    /// The deck with `entry` logged.
    pub fn with_entry(self, entry: ReviewEntry) -> Deck {
        let mut log = self.log;
        log.push(entry);
        Deck { log, ..self }
    }
}
