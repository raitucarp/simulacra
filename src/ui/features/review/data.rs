//! What the shell hands the Review page: the active network's cards and log, today's review
//! day, the limits and retention of Settings › Learning, and a label per card source.

use crate::domain::review::queue::{plan_today, DailyLimits, TodayPlan};
use crate::domain::review::schedule::Policy;
use crate::domain::review::{Card, CardId, CardSource, ReviewEntry};
use crate::domain::NetworkId;
use chrono::NaiveDate;
use std::collections::HashMap;

/// The Review page's data.
#[derive(Debug, Clone)]
pub struct ReviewData {
    pub network_id: NetworkId,
    pub cards: Vec<Card>,
    pub log: Vec<ReviewEntry>,
    pub today: NaiveDate,
    pub limits: DailyLimits,
    pub policy: Policy,
    /// Display label of each card source, by [`CardSource::group_key`].
    pub source_labels: HashMap<String, String>,
}

impl ReviewData {
    /// An empty page for `network_id`.
    pub fn empty(network_id: NetworkId, today: NaiveDate, limits: DailyLimits, policy: Policy) -> Self {
        Self { network_id, cards: Vec::new(), log: Vec::new(), today, limits, policy, source_labels: HashMap::new() }
    }

    pub fn card(&self, id: &CardId) -> Option<&Card> {
        self.cards.iter().find(|card| &card.id == id)
    }

    /// Today's counts within the limits.
    pub fn plan(&self) -> TodayPlan {
        plan_today(&self.cards, &self.log, self.today, self.limits)
    }

    /// The label of `source` (the shell's, else a generic one).
    pub fn source_label(&self, source: &CardSource) -> Option<String> {
        self.source_labels.get(&source.group_key()).cloned()
    }

    /// Cards that have been reviewed at least once (quizzes draw from these first).
    pub fn learnt_cards(&self) -> impl Iterator<Item = &Card> {
        self.cards.iter().filter(|card| !card.suspended && !card.schedule.is_new())
    }
}
