//! Today's study queue: the due cards and a ration of new ones within the daily limits of
//! Settings › Learning, interleaved so new material is spread between reviews and cards from
//! the same source rarely follow each other (interleaving improves discrimination).

use super::card::{Card, CardId};
use super::log::ReviewEntry;
use super::schedule::Phase;
use chrono::NaiveDate;
use std::collections::{HashMap, HashSet};

/// The daily limits (Settings › Learning).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DailyLimits {
    /// Most cards answered per day, new ones included.
    pub reviews: u32,
    pub new_cards: u32,
}

/// What today holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TodayPlan {
    /// Learnt cards due today (overdue included), before the limit.
    pub due: usize,
    /// New cards waiting, before the limit.
    pub new_waiting: usize,
    /// Cards answered today so far.
    pub answered: usize,
    /// New cards introduced today so far.
    pub introduced: usize,
    /// Due cards in today's queue (after the limits).
    pub queued_due: usize,
    /// New cards in today's queue (after the limits).
    pub queued_new: usize,
}

impl TodayPlan {
    pub fn queued(&self) -> usize {
        self.queued_due + self.queued_new
    }
}

fn is_active(card: &Card) -> bool {
    !card.suspended
}

/// Distinct cards answered today and how many of them were new.
fn answered_today(log: &[ReviewEntry], today: NaiveDate) -> (usize, usize) {
    let today_entries: Vec<&ReviewEntry> = log.iter().filter(|entry| entry.day == today).collect();
    let answered: HashSet<&CardId> = today_entries.iter().map(|entry| &entry.card_id).collect();
    let introduced = today_entries.iter().filter(|entry| entry.phase_before == Phase::New).count();
    (answered.len(), introduced)
}

/// Due learnt cards, most overdue first, and new cards, oldest first.
fn candidates(cards: &[Card], today: NaiveDate) -> (Vec<&Card>, Vec<&Card>) {
    let mut due: Vec<&Card> = cards.iter().filter(|c| is_active(c) && c.schedule.is_due(today)).collect();
    due.sort_by(|a, b| a.schedule.due.cmp(&b.schedule.due).then_with(|| a.id.cmp(&b.id)));
    let mut new: Vec<&Card> = cards.iter().filter(|c| is_active(c) && c.schedule.is_new()).collect();
    new.sort_by(|a, b| a.created_at.cmp(&b.created_at).then_with(|| a.id.cmp(&b.id)));
    (due, new)
}

/// Counts for the overview and the sizes of today's queue.
pub fn plan_today(cards: &[Card], log: &[ReviewEntry], today: NaiveDate, limits: DailyLimits) -> TodayPlan {
    let (due, new) = candidates(cards, today);
    let (answered, introduced) = answered_today(log, today);
    let room = (limits.reviews as usize).saturating_sub(answered);
    let queued_due = due.len().min(room);
    let new_room = (limits.new_cards as usize).saturating_sub(introduced).min(room - queued_due);
    TodayPlan { due: due.len(), new_waiting: new.len(), answered, introduced, queued_due, queued_new: new.len().min(new_room) }
}

/// Today's queue: due and new cards within the limits, new cards spread evenly between the
/// reviews, then cards from one source moved apart where possible.
pub fn build_queue(cards: &[Card], log: &[ReviewEntry], today: NaiveDate, limits: DailyLimits) -> Vec<CardId> {
    let plan = plan_today(cards, log, today, limits);
    let (due, new) = candidates(cards, today);
    let (due, new) = (&due[..plan.queued_due], &new[..plan.queued_new]);
    let stride = (due.len() + new.len()).checked_div(new.len().max(1)).unwrap_or(1).max(1);
    let mut mixed: Vec<&Card> = Vec::with_capacity(due.len() + new.len());
    let (mut due_iter, mut new_iter) = (due.iter(), new.iter());
    for slot in 0..due.len() + new.len() {
        let take_new = slot % stride == stride - 1 || due_iter.len() == 0;
        let next = if take_new { new_iter.next().or_else(|| due_iter.next()) } else { due_iter.next().or_else(|| new_iter.next()) };
        mixed.extend(next.copied());
    }
    spread_sources(mixed).into_iter().map(|card| card.id.clone()).collect()
}

/// The source group with the most waiting cards, if it holds more than half of them (it
/// must be placed now, or two of its cards would end up side by side).
fn crowding_group(waiting: &[&Card]) -> Option<String> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    waiting.iter().for_each(|card| *counts.entry(card.source.group_key()).or_default() += 1);
    counts.into_iter().max_by(|a, b| a.1.cmp(&b.1).then_with(|| b.0.cmp(&a.0))).filter(|(_, count)| count * 2 > waiting.len()).map(|(group, _)| group)
}

/// Reorders `cards` so neighbours come from different sources wherever that is possible,
/// otherwise keeping their order.
fn spread_sources(cards: Vec<&Card>) -> Vec<&Card> {
    let mut waiting = cards;
    let mut out: Vec<&Card> = Vec::with_capacity(waiting.len());
    while !waiting.is_empty() {
        let last = out.last().map(|card| card.source.group_key());
        let crowded = crowding_group(&waiting).filter(|group| Some(group) != last.as_ref());
        let pick = match crowded {
            Some(group) => waiting.iter().position(|card| card.source.group_key() == group),
            None => waiting.iter().position(|card| Some(card.source.group_key()) != last),
        };
        out.push(waiting.remove(pick.unwrap_or(0)));
    }
    out
}
