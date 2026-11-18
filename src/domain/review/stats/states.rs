//! Cards by state: new, learning (incl. relearning), young and mature, and suspended.

use crate::domain::review::item::ReviewItem;
use crate::domain::review::schedule::Phase;

/// An interval of at least this many days makes a card mature.
pub const MATURE_INTERVAL: u32 = 21;

/// The state a card is counted under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemState {
    New,
    Learning,
    Young,
    Mature,
    Suspended,
}

impl ItemState {
    pub const ALL: [Self; 5] = [Self::New, Self::Learning, Self::Young, Self::Mature, Self::Suspended];

    pub const fn key(self) -> &'static str {
        match self {
            Self::New => "new",
            Self::Learning => "learning",
            Self::Young => "young",
            Self::Mature => "mature",
            Self::Suspended => "suspended",
        }
    }

    pub fn of(card: &ReviewItem) -> Self {
        if !card.is_active() {
            return Self::Suspended;
        }
        match card.schedule.phase {
            Phase::New => Self::New,
            Phase::Learning | Phase::Relearning => Self::Learning,
            Phase::Review if card.schedule.interval >= MATURE_INTERVAL => Self::Mature,
            Phase::Review => Self::Young,
        }
    }
}

/// Number of cards in each state, in [`ItemState::ALL`] order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StateCounts(pub [usize; 5]);

impl StateCounts {
    pub fn get(&self, state: ItemState) -> usize {
        self.0[ItemState::ALL.iter().position(|s| *s == state).unwrap_or_default()]
    }

    pub fn total(&self) -> usize {
        self.0.iter().sum()
    }
}

pub fn state_counts(cards: &[ReviewItem]) -> StateCounts {
    StateCounts(ItemState::ALL.map(|state| cards.iter().filter(|card| ItemState::of(card) == state).count()))
}
