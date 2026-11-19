//! Localized wording of the Journal page: days, region names, outcomes, capsule states and
//! the careful sentences that describe an association (never a cause).

use crate::domain::journal::capsule::CapsuleState;
use crate::domain::journal::mood::correlate::{Association, Strength, MIN_DAYS};
use crate::domain::journal::mood::AffectRegion;
use crate::i18n::{t_count, COUNT_SEPARATOR};
use jiff::civil::Date;
use rust_i18n::t;

/// Correlations are shown with two decimals.
const R_DECIMALS: usize = 2;

/// A translated key.
pub fn tr(key: &str) -> String {
    t!(key).to_string()
}

/// A day as "Mon 2 Mar 2026" in the user's language (numbers and month as figures).
pub fn day_label(day: Date) -> String {
    let weekday = tr(&format!("journal.weekday.{}", day.weekday().to_monday_zero_offset()));
    t!("journal.day_format", weekday = weekday, date = day.strftime("%Y-%m-%d").to_string()).to_string()
}

pub fn region_label(region: AffectRegion) -> String {
    tr(&format!("journal.region.{}", region.key()))
}

/// What a capsule says on its card.
pub fn capsule_state_label(state: CapsuleState, opens_on: Date) -> String {
    match state {
        CapsuleState::Sealed { days_left } => t_count("journal.capsule.days_left", days_left as usize) + COUNT_SEPARATOR + &t!("journal.capsule.opens_on", date = day_label(opens_on)),
        CapsuleState::Ready => tr("journal.capsule.ready"),
        CapsuleState::Opened => tr("journal.capsule.opened"),
    }
}

fn r_text(r: f64) -> String {
    format!("{r:.R_DECIMALS$}")
}

/// One association as a sentence that says how many days it rests on.
pub fn association_sentence(link: &Association) -> String {
    let activity = tr(&format!("journal.activity.{}", link.activity.key()));
    let measure = tr(&format!("journal.measure.{}", link.measure.key()));
    let (days, r) = (link.days, link.r.unwrap_or_default());
    match link.strength {
        Strength::TooFewDays => t!("journal.assoc.too_few", activity = activity, measure = measure, days = days, min = MIN_DAYS).to_string(),
        Strength::NoVariation => t!("journal.assoc.no_variation", activity = activity, measure = measure, days = days).to_string(),
        Strength::Negligible => t!("journal.assoc.none", activity = activity, measure = measure, r = r_text(r), days = days).to_string(),
        strength => {
            let level = tr(&format!("journal.strength.{}", strength_key(strength)));
            let key = if link.is_positive() { "journal.assoc.higher" } else { "journal.assoc.lower" };
            t!(key, activity = activity, measure = measure, strength = level, r = r_text(r), days = days).to_string()
        }
    }
}

fn strength_key(strength: Strength) -> &'static str {
    match strength {
        Strength::Weak => "weak",
        Strength::Moderate => "moderate",
        _ => "strong",
    }
}

/// Minutes as "1 h 20 min" or "25 min".
pub fn minutes_label(minutes: u32) -> String {
    const MINUTES_PER_HOUR: u32 = 60;
    match (minutes / MINUTES_PER_HOUR, minutes % MINUTES_PER_HOUR) {
        (0, m) => t!("journal.minutes", count = m).to_string(),
        (h, m) => t!("journal.hours_minutes", hours = h, minutes = m).to_string(),
    }
}
