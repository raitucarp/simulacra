//! Due forecast: how many learnt cards come due on each of the next days.

use crate::domain::review::card::Card;
use crate::domain::review::day::days_between;
use chrono::NaiveDate;

/// Days the forecast covers (today first; overdue cards count towards today).
pub const FORECAST_DAYS: usize = 30;

/// Cards due on each of the `days` days from `today` (suspended and new cards left out).
pub fn due_forecast(cards: &[Card], today: NaiveDate, days: usize) -> Vec<usize> {
    cards
        .iter()
        .filter(|card| !card.suspended && !card.schedule.is_new())
        .map(|card| if card.schedule.due <= today { 0 } else { days_between(today, card.schedule.due) as usize })
        .filter(|offset| *offset < days)
        .fold(vec![0; days], |mut counts, offset| {
            counts[offset] += 1;
            counts
        })
}
