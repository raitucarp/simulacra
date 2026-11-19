//! Mood over time: a daily series (mean valence, arousal and energy), and means by weekday
//! and by hour of the day, each with its number of check-ins.

use super::MoodCheckin;
use chrono::{Datelike, NaiveDate, Timelike};
use std::collections::BTreeMap;

pub const WEEKDAYS: usize = 7;
pub const HOURS: usize = 24;

/// Means of a group of check-ins.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MoodMean {
    pub valence: f32,
    pub arousal: f32,
    pub energy: f32,
    pub count: usize,
}

fn mean_of<'a>(checkins: impl Iterator<Item = &'a MoodCheckin>) -> MoodMean {
    let (sum, count) = checkins.fold(((0.0, 0.0, 0.0), 0usize), |((v, a, e), n), c| ((v + c.affect.valence, a + c.affect.arousal, e + f32::from(c.energy)), n + 1));
    if count == 0 {
        return MoodMean::default();
    }
    let n = count as f32;
    MoodMean { valence: sum.0 / n, arousal: sum.1 / n, energy: sum.2 / n, count }
}

/// The mean of each journal day with check-ins.
pub fn daily_means(checkins: &[MoodCheckin]) -> BTreeMap<NaiveDate, MoodMean> {
    let days: BTreeMap<NaiveDate, Vec<&MoodCheckin>> = checkins.iter().fold(BTreeMap::new(), |mut days, c| {
        days.entry(c.day).or_insert_with(Vec::new).push(c);
        days
    });
    days.into_iter().map(|(day, list)| (day, mean_of(list.into_iter()))).collect()
}

/// Means by weekday, Monday first (local time of each check-in).
pub fn by_weekday(checkins: &[MoodCheckin]) -> [MoodMean; WEEKDAYS] {
    std::array::from_fn(|weekday| mean_of(checkins.iter().filter(|c| c.at.weekday().num_days_from_monday() as usize == weekday)))
}

/// Means by local hour of the day.
pub fn by_hour(checkins: &[MoodCheckin]) -> [MoodMean; HOURS] {
    std::array::from_fn(|hour| mean_of(checkins.iter().filter(|c| c.at.hour() as usize == hour)))
}

/// Check-ins newest first.
pub fn newest_checkins(checkins: &[MoodCheckin]) -> Vec<&MoodCheckin> {
    let mut sorted: Vec<&MoodCheckin> = checkins.iter().collect();
    sorted.sort_by_key(|a| std::cmp::Reverse(a.at));
    sorted
}
