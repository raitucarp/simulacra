//! The mood and energy log: quick check-ins on the affect grid with an energy level, optional
//! sleep, tags and a note; summaries over time, weekday and hour; and careful associations
//! with activity (see `correlate`).

pub mod affect;
pub mod correlate;
pub mod summary;

pub use affect::{Affect, AffectRegion};

use super::ids::RecordId;
use super::records::Record;
use chrono::{DateTime, FixedOffset, NaiveDate};
use serde::{Deserialize, Serialize};

/// Lowest and highest energy level.
pub const ENERGY_MIN: u8 = 1;
pub const ENERGY_MAX: u8 = 5;
/// Hours of sleep accepted.
pub const SLEEP_MAX_HOURS: f32 = 24.0;

/// One check-in.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoodCheckin {
    pub id: RecordId,
    /// When, with the local offset it was made at (weekday and hour summaries use it).
    pub at: DateTime<FixedOffset>,
    /// Its journal day.
    pub day: NaiveDate,
    pub affect: Affect,
    /// 1 (drained) to 5 (energised).
    pub energy: u8,
    #[serde(default)]
    pub sleep_hours: Option<f32>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub note: String,
    /// The user ticked "I'd like pointers to support" on this check-in (never inferred).
    #[serde(default)]
    pub wants_support: bool,
}

impl Record for MoodCheckin {
    fn id(&self) -> &RecordId {
        &self.id
    }
}

/// What the check-in form holds before it is saved.
#[derive(Debug, Clone, PartialEq)]
pub struct CheckinDraft {
    pub affect: Option<Affect>,
    pub energy: u8,
    pub sleep_hours: Option<f32>,
    pub tags: Vec<String>,
    pub note: String,
    pub wants_support: bool,
}

impl Default for CheckinDraft {
    fn default() -> Self {
        Self { affect: None, energy: (ENERGY_MIN + ENERGY_MAX) / 2, sleep_hours: None, tags: Vec::new(), note: String::new(), wants_support: false }
    }
}

impl CheckinDraft {
    /// The check-in, once a point on the grid is chosen.
    pub fn into_checkin(self, id: RecordId, at: DateTime<FixedOffset>, day: NaiveDate) -> Option<MoodCheckin> {
        let affect = self.affect?;
        let sleep_hours = self.sleep_hours.filter(|h| h.is_finite() && *h >= 0.0).map(|h| h.min(SLEEP_MAX_HOURS));
        let tags = parse_tags(&self.tags.join(","));
        let energy = self.energy.clamp(ENERGY_MIN, ENERGY_MAX);
        Some(MoodCheckin { id, at, day, affect, energy, sleep_hours, tags, note: self.note.trim().to_string(), wants_support: self.wants_support })
    }
}

/// Comma or space separated tags, trimmed of `#`, lowercased, without repeats.
pub fn parse_tags(text: &str) -> Vec<String> {
    let mut tags: Vec<String> = Vec::new();
    for tag in text.split([',', ' ']).map(|t| t.trim().trim_start_matches('#').to_lowercase()).filter(|t| !t.is_empty()) {
        if !tags.contains(&tag) {
            tags.push(tag);
        }
    }
    tags
}
