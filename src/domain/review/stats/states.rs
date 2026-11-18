//! Cards by state: new, learning (incl. relearning), young and mature, and suspended.

use crate::domain::review::card::Card;
use crate::domain::review::schedule::Phase;

/// An interval of at least this many days makes a card mature.
pub const MATURE_INTERVAL: u32 = 21;

/// The state a card is counted under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardState {
    New,
    Learning,
    Young,
    Mature,
    Suspended,
}

impl CardState {
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

    pub fn of(card: &Card) -> Self {
        if card.suspended {
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

/// Number of cards in each state, in [`CardState::ALL`] order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StateCounts(pub [usize; 5]);

impl StateCounts {
    pub fn get(&self, state: CardState) -> usize {
        self.0[CardState::ALL.iter().position(|s| *s == state).unwrap_or_default()]
    }

    pub fn total(&self) -> usize {
        self.0.iter().sum()
    }
}

pub fn state_counts(cards: &[Card]) -> StateCounts {
    StateCounts(CardState::ALL.map(|state| cards.iter().filter(|card| CardState::of(card) == state).count()))
}
