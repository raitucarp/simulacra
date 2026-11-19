//! Serde for a moment kept with its UTC offset (`#[serde(with = "crate::time::serde_offset")]`).
//!
//! Writes RFC 3339 with the offset (`Z` for UTC), as earlier versions of the app did, and reads
//! that form, any other RFC 3339 offset, and `jiff`'s bracketed form.

use super::rfc3339::{parse_offset, write};
use jiff::ZonedDateTime;
use serde::{de, Deserialize, Deserializer, Serializer};

/// Serializes `at` as `2026-11-18T17:00:00+07:00`.
pub fn serialize<S: Serializer>(at: &ZonedDateTime, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&write(at.timestamp(), at.offset(), true))
}

/// Deserializes an RFC 3339 instant, keeping its offset.
pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<ZonedDateTime, D::Error> {
    let raw = String::deserialize(deserializer)?;
    parse_offset(&raw).ok_or_else(|| de::Error::custom(format!("invalid RFC 3339 date-time: {raw}")))
}
