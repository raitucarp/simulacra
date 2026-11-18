//! What the timeline shows for a chosen range of periods and a chosen concept: the lanes
//! active in the range, the concept's mentions in order, and the concepts mentioned with it
//! in each period.

use super::{ConceptTimeline, Mention};
use std::collections::HashMap;

/// An inclusive range of period indices.
pub type PeriodRange = (usize, usize);

/// The concepts mentioned together with one concept in one period.
#[derive(Debug, Clone, PartialEq)]
pub struct PeriodNeighbours {
    pub period: usize,
    /// Sources mentioning the concept in the period.
    pub mentions: u32,
    /// Other lanes and how many of those sources mention them, most first (ties by lane).
    pub with: Vec<(usize, u32)>,
}

impl ConceptTimeline {
    /// The whole span.
    pub fn full_range(&self) -> PeriodRange {
        (0, self.periods.count.saturating_sub(1))
    }

    /// `range` clamped into the span, ends in order.
    pub fn clamp(&self, (a, b): PeriodRange) -> PeriodRange {
        let last = self.periods.count.saturating_sub(1);
        (a.min(b).min(last), a.max(b).min(last))
    }

    /// Mentions of `lane` within `range`.
    pub fn total_in(&self, lane: usize, (from, to): PeriodRange) -> u32 {
        self.lanes.get(lane).map_or(0, |l| l.counts.iter().enumerate().filter(|(i, _)| (from..=to).contains(i)).map(|(_, n)| n).sum())
    }

    /// Lanes mentioned within `range`, most mentioned there first (ties by lane), at most `limit`.
    pub fn active_lanes(&self, range: PeriodRange, limit: usize) -> Vec<usize> {
        let mut active: Vec<(usize, u32)> = (0..self.lanes.len()).map(|lane| (lane, self.total_in(lane, range))).filter(|(_, n)| *n > 0).collect();
        active.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        active.into_iter().take(limit).map(|(lane, _)| lane).collect()
    }

    /// The mentions of `lane` within `range`, oldest first.
    pub fn mentions_of(&self, lane: usize, (from, to): PeriodRange) -> Vec<&Mention> {
        self.mentions.iter().filter(|m| m.period >= from && m.period <= to && m.lanes.contains(&lane)).collect()
    }

    /// Per period of `range` where `lane` is mentioned, the `top` concepts mentioned with it.
    pub fn co_occurring(&self, lane: usize, range: PeriodRange, top: usize) -> Vec<PeriodNeighbours> {
        let mut by_period: Vec<(usize, u32, HashMap<usize, u32>)> = Vec::new();
        for mention in self.mentions_of(lane, range) {
            if by_period.last().is_none_or(|(p, _, _)| *p != mention.period) {
                by_period.push((mention.period, 0, HashMap::new()));
            }
            let Some((_, mentions, with)) = by_period.last_mut() else { continue };
            *mentions += 1;
            mention.lanes.iter().filter(|&&other| other != lane).for_each(|&other| *with.entry(other).or_default() += 1);
        }
        by_period.into_iter().map(|(period, mentions, with)| {
            let mut with: Vec<(usize, u32)> = with.into_iter().collect();
            with.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
            with.truncate(top);
            PeriodNeighbours { period, mentions, with }
        }).collect()
    }
}
