//! Spaced repetition and quizzes, pure side (sb C1).
//!
//! Grounded in learning science: retrieval practice (every review is a recall attempt),
//! spacing (FSRS schedules each card for the desired retention), desirable difficulty
//! (plausible distractors, typed recall), interleaving (new and due cards, and cards from
//! different sources, are mixed) and metacognition (self-grading, confidence calibration).
//!
//! - [`fsrs`]: the memory model; [`schedule`]: a card's schedule after a review.
//! - [`card`]: basic, cloze and concept cards with their cited source.
//! - [`queue`] / [`session`]: today's queue within the daily limits and a session's state.
//! - [`log`]: answers applied to cards and logged; [`stats`]: charts' numbers.
//! - [`quiz`]: multiple choice, free recall and ordering; [`ai`]: AI-drafted cards.

pub mod ai;
pub mod card;
pub mod day;
pub mod fsrs;
mod grade;
pub mod log;
pub mod queue;
pub mod quiz;
pub mod schedule;
pub mod session;
pub mod stats;

#[cfg(test)]
mod tests;

pub use card::{Card, CardDraft, CardId, CardKind, CardSource, DraftKind};
pub use grade::{Confidence, Grade};
pub use log::{apply_answer, Answer, ReviewEntry, ReviewMode};
pub use queue::{build_queue, plan_today, DailyLimits, TodayPlan};
pub use schedule::{CardSchedule, Phase, Policy};
pub use session::{Session, Stage};

use crate::domain::NetworkId;

/// Every card and logged answer (all networks), as loaded from the vault.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Deck {
    pub cards: Vec<Card>,
    pub log: Vec<ReviewEntry>,
}

impl Deck {
    /// The deck narrowed to `network`.
    pub fn of_network(&self, network: &NetworkId) -> Deck {
        Deck {
            cards: self.cards.iter().filter(|card| &card.network_id == network).cloned().collect(),
            log: self.log.iter().filter(|entry| &entry.network_id == network).cloned().collect(),
        }
    }

    pub fn card(&self, id: &CardId) -> Option<&Card> {
        self.cards.iter().find(|card| &card.id == id)
    }

    /// The deck with `card` added or replaced.
    pub fn with_card(self, card: Card) -> Deck {
        let mut cards: Vec<Card> = self.cards.into_iter().filter(|c| c.id != card.id).collect();
        cards.push(card);
        Deck { cards, ..self }
    }

    /// The deck without card `id` (its log stays for the statistics).
    pub fn without_card(self, id: &CardId) -> Deck {
        Deck { cards: self.cards.into_iter().filter(|card| &card.id != id).collect(), ..self }
    }

    /// The deck with `entry` logged.
    pub fn with_entry(self, entry: ReviewEntry) -> Deck {
        let mut log = self.log;
        log.push(entry);
        Deck { log, ..self }
    }
}
