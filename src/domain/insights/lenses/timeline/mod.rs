//! The concept timeline: how often each concept is mentioned per period, from its first
//! appearance to today, with its peak and where it stands now. Built in one pass over the
//! sources (linear in sources × concepts per source), so thousands of posts stay fast.

pub mod detail;
pub mod lifecycle;
#[cfg(test)]
mod tests;

pub use lifecycle::Phase;

use super::periods::Periods;
use super::sources::{Source, SourceKind};
use chrono::NaiveDate;
use std::collections::HashMap;

/// Periods drawn at most.
pub const MAX_PERIODS: usize = 48;
/// Concepts kept (the most mentioned).
pub const MAX_LANES: usize = 40;
/// Mentions a concept needs to get a lane.
pub const MIN_MENTIONS: u32 = 2;

/// One concept over time.
#[derive(Debug, Clone, PartialEq)]
pub struct ConceptLane {
    pub key: String,
    /// Sources mentioning it, per period.
    pub counts: Vec<u32>,
    pub total: u32,
    pub first_day: NaiveDate,
    pub last_day: NaiveDate,
    /// The busiest period (the earliest of equals).
    pub peak: usize,
    pub phase: Phase,
}

/// A source that mentions kept concepts.
#[derive(Debug, Clone, PartialEq)]
pub struct Mention {
    pub kind: SourceKind,
    pub id: String,
    pub day: NaiveDate,
    pub period: usize,
    pub excerpt: String,
    /// Lane indices.
    pub lanes: Vec<usize>,
}

/// Every lane, busiest concept first, and the mentions oldest first.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConceptTimeline {
    pub periods: Periods,
    pub lanes: Vec<ConceptLane>,
    pub mentions: Vec<Mention>,
}

impl ConceptTimeline {
    /// The timeline of `sources` (oldest first) up to `today`.
    pub fn of(sources: Vec<Source>, today: NaiveDate) -> Self {
        let Some(first) = sources.iter().map(|s| s.day).min() else { return Self::default() };
        let periods = Periods::spanning(first, today, MAX_PERIODS);
        let keys = kept_concepts(&sources);
        let lane_of: HashMap<&str, usize> = keys.iter().enumerate().map(|(i, k)| (k.as_str(), i)).collect();
        let mut counts = vec![vec![0u32; periods.count]; keys.len()];
        let mut days: Vec<Option<(NaiveDate, NaiveDate)>> = vec![None; keys.len()];
        let mut mentions = Vec::new();
        for source in sources.iter().filter(|s| s.day <= today) {
            let Some(period) = periods.index_of(source.day) else { continue };
            let lanes: Vec<usize> = source.concepts.iter().filter_map(|c| lane_of.get(c.as_str()).copied()).collect();
            for &lane in &lanes {
                counts[lane][period] += 1;
                days[lane] = Some(days[lane].map_or((source.day, source.day), |(a, b)| (a.min(source.day), b.max(source.day))));
            }
            if !lanes.is_empty() {
                mentions.push(Mention { kind: source.kind, id: source.id.clone(), day: source.day, period, excerpt: source.excerpt.clone(), lanes });
            }
        }
        let lanes = keys.into_iter().zip(counts).zip(days).filter_map(|((key, counts), days)| days.map(|days| lane(key, counts, days, &periods, today))).collect();
        Self { periods, lanes, mentions }
    }

    pub fn is_empty(&self) -> bool {
        self.lanes.is_empty()
    }
}

/// The concepts mentioned by at least [`MIN_MENTIONS`] sources, most mentioned first (ties by
/// key), at most [`MAX_LANES`].
fn kept_concepts(sources: &[Source]) -> Vec<String> {
    let mut totals: HashMap<&str, u32> = HashMap::new();
    sources.iter().flat_map(|s| s.concepts.iter()).for_each(|c| *totals.entry(c.as_str()).or_default() += 1);
    let mut ranked: Vec<(&str, u32)> = totals.into_iter().filter(|(_, n)| *n >= MIN_MENTIONS).collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));
    ranked.into_iter().take(MAX_LANES).map(|(key, _)| key.to_string()).collect()
}

fn lane(key: String, counts: Vec<u32>, (first_day, last_day): (NaiveDate, NaiveDate), periods: &Periods, today: NaiveDate) -> ConceptLane {
    let total = counts.iter().sum();
    let peak = counts.iter().enumerate().fold(0, |best, (i, &n)| if n > counts[best] { i } else { best });
    let quiet_periods = periods.index_of(today).unwrap_or(periods.count.saturating_sub(1)).saturating_sub(periods.index_of(last_day).unwrap_or_default());
    let phase = lifecycle::phase(&counts, quiet_periods);
    ConceptLane { key, counts, total, first_day, last_day, peak, phase }
}
