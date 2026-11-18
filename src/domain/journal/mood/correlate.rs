//! Associations between mood and activity, never causes: Pearson's r between a mood measure
//! (daily mean valence or energy) and an activity count over the days with a check-in, with
//! the number of days. Fewer than [`MIN_DAYS`] days, or an activity that never varies, gives
//! no figure at all. Strength follows Cohen's conventional bands.

use super::summary::MoodMean;
use crate::domain::journal::activity::{ActivityByDay, ActivityKind};
use chrono::NaiveDate;
use std::collections::BTreeMap;

/// Days with a check-in needed before an association is described.
pub const MIN_DAYS: usize = 7;
/// Cohen's bands of |r|: below these a link is negligible, weak, moderate; above, strong.
const NEGLIGIBLE_BELOW: f64 = 0.1;
const WEAK_BELOW: f64 = 0.3;
const MODERATE_BELOW: f64 = 0.5;

/// A mood measure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoodMeasure {
    Valence,
    Energy,
}

impl MoodMeasure {
    pub const ALL: [Self; 2] = [Self::Valence, Self::Energy];

    pub const fn key(self) -> &'static str {
        match self {
            Self::Valence => "valence",
            Self::Energy => "energy",
        }
    }

    fn of(self, mean: &MoodMean) -> f64 {
        f64::from(match self {
            Self::Valence => mean.valence,
            Self::Energy => mean.energy,
        })
    }
}

/// How strong a link looks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strength {
    /// Fewer than [`MIN_DAYS`] days.
    TooFewDays,
    /// The activity (or the mood) was the same every day.
    NoVariation,
    Negligible,
    Weak,
    Moderate,
    Strong,
}

/// One association.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Association {
    pub activity: ActivityKind,
    pub measure: MoodMeasure,
    /// Days compared.
    pub days: usize,
    pub r: Option<f64>,
    pub strength: Strength,
}

impl Association {
    /// Whether more activity went with a higher mood measure.
    pub fn is_positive(&self) -> bool {
        self.r.is_some_and(|r| r > 0.0)
    }
}

/// Pearson's r of paired samples (`None` when either side has no variance).
pub fn pearson(pairs: &[(f64, f64)]) -> Option<f64> {
    let n = pairs.len() as f64;
    if pairs.len() < 2 {
        return None;
    }
    let (mx, my) = (pairs.iter().map(|p| p.0).sum::<f64>() / n, pairs.iter().map(|p| p.1).sum::<f64>() / n);
    let (sxy, sxx, syy) = pairs.iter().fold((0.0, 0.0, 0.0), |(xy, xx, yy), (x, y)| (xy + (x - mx) * (y - my), xx + (x - mx).powi(2), yy + (y - my).powi(2)));
    (sxx > f64::EPSILON && syy > f64::EPSILON).then(|| (sxy / (sxx.sqrt() * syy.sqrt())).clamp(-1.0, 1.0))
}

fn strength_of(r: f64) -> Strength {
    match r.abs() {
        a if a < NEGLIGIBLE_BELOW => Strength::Negligible,
        a if a < WEAK_BELOW => Strength::Weak,
        a if a < MODERATE_BELOW => Strength::Moderate,
        _ => Strength::Strong,
    }
}

/// The association of `activity` with `measure` over the days with a check-in.
pub fn associate(mood: &BTreeMap<NaiveDate, MoodMean>, activity: &ActivityByDay, kind: ActivityKind, measure: MoodMeasure) -> Association {
    let pairs: Vec<(f64, f64)> = mood.iter().map(|(day, mean)| (activity.get(day).map(|a| kind.of(a)).unwrap_or(0.0), measure.of(mean))).collect();
    let days = pairs.len();
    let r = (days >= MIN_DAYS).then(|| pearson(&pairs)).flatten();
    let strength = match r {
        _ if days < MIN_DAYS => Strength::TooFewDays,
        None => Strength::NoVariation,
        Some(r) => strength_of(r),
    };
    Association { activity: kind, measure, days, r, strength }
}

/// Every activity against every mood measure.
pub fn associations(mood: &BTreeMap<NaiveDate, MoodMean>, activity: &ActivityByDay) -> Vec<Association> {
    ActivityKind::ALL.iter().flat_map(|kind| MoodMeasure::ALL.iter().map(move |measure| associate(mood, activity, *kind, *measure))).collect()
}
