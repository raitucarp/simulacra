//! Unit tests of the journal core.

mod capsule;
mod entries;
mod focus;
mod mood;
mod privacy;
mod prompts;

use super::entry::JournalEntry;
use super::ids::RecordId;
use chrono::{DateTime, NaiveDate, TimeZone, Utc};

/// A fixed journal day.
pub(super) fn day(offset: i64) -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 3, 2).unwrap() + chrono::Duration::days(offset)
}

/// Noon UTC of `day(offset)`.
pub(super) fn noon(offset: i64) -> DateTime<Utc> {
    let d = day(offset);
    Utc.from_utc_datetime(&d.and_hms_opt(12, 0, 0).unwrap())
}

/// An entry `id` on `day(offset)` reading `body`.
pub(super) fn entry(id: &str, offset: i64, body: &str) -> JournalEntry {
    JournalEntry::new(RecordId::new(id), day(offset), noon(offset), body)
}
