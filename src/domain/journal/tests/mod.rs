//! Unit tests of the journal core.

mod capsule;
mod entries;
mod focus;
mod mood;
mod feeling_checkin; // M6
mod privacy;
mod prompts;

use super::entry::JournalEntry;
use super::ids::RecordId;
use jiff::Timestamp;
use jiff::civil::Date;

/// A fixed journal day.
pub(super) fn day(offset: i64) -> Date {
    jiff::civil::date(2026, 3, 2) + crate::time::days(offset)
}

/// Noon UTC of `day(offset)`.
pub(super) fn noon(offset: i64) -> Timestamp {
    let d = day(offset);
    crate::time::at_utc(d, 12, 0, 0)
}

/// An entry `id` on `day(offset)` reading `body`.
pub(super) fn entry(id: &str, offset: i64, body: &str) -> JournalEntry {
    JournalEntry::new(RecordId::new(id), day(offset), noon(offset), body)
}
