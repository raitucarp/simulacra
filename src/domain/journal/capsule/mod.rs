//! Time capsules: a note (and images from the media folder) to the future self, sealed until
//! a day. While sealed its contents are not readable through this module at all; from its
//! day on it can be opened, once, and answered with a reply comparing then and now.

pub mod notify;

use super::ids::RecordId;
use super::records::Record;
use crate::domain::media::ImageAttachment;
use jiff::Timestamp;
use jiff::civil::Date;
use serde::{Deserialize, Serialize};

/// Fewest and most days a capsule can be sealed for (about twenty years).
pub const MIN_SEAL_DAYS: u32 = 1;
pub const MAX_SEAL_DAYS: u32 = 7_305;

/// When a capsule opens, as chosen in the form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SealUntil {
    Date(Date),
    Days(u32),
}

/// Why a capsule cannot be sealed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SealProblem {
    Empty,
    /// The day is today or earlier, or too far ahead.
    OutOfRange,
}

/// What a capsule shows on a day.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapsuleState {
    Sealed { days_left: u32 },
    /// Its day has come; not opened yet.
    Ready,
    Opened,
}

/// A sealed note to the future self.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeCapsule {
    pub id: RecordId,
    pub written_at: Timestamp,
    pub written_on: Date,
    pub opens_on: Date,
    body: String,
    #[serde(default)]
    attachments: Vec<ImageAttachment>,
    #[serde(default)]
    pub opened_at: Option<Timestamp>,
    /// The reply written after opening it.
    #[serde(default)]
    pub reply: Option<String>,
    /// Set once the "a capsule has opened" notice was shown.
    #[serde(default)]
    pub announced: bool,
}

impl Record for TimeCapsule {
    fn id(&self) -> &RecordId {
        &self.id
    }
}

/// The day a capsule written on `today` opens, when it is in range.
pub fn opening_day(until: SealUntil, today: Date) -> Option<Date> {
    let day = match until {
        SealUntil::Date(day) => day,
        SealUntil::Days(days) => today.checked_add(crate::time::days(i64::from(days))).ok()?,
    };
    let days = i64::from((day - today).get_days());
    (i64::from(MIN_SEAL_DAYS)..=i64::from(MAX_SEAL_DAYS)).contains(&days).then_some(day)
}

/// The ISO day format the opening day may be typed in.
pub const DAY_FORMAT: &str = "%Y-%m-%d";

/// What the "opens in" field says: a number of days, or a day (`2027-01-31`).
pub fn parse_until(text: &str) -> Option<SealUntil> {
    let text = text.trim();
    text.parse::<u32>().ok().map(SealUntil::Days).or_else(|| Date::strptime(DAY_FORMAT, text).ok().map(SealUntil::Date))
}

/// Seals `body` (and `attachments`) on `today` until `until`.
pub fn seal(id: RecordId, body: &str, attachments: Vec<ImageAttachment>, until: SealUntil, now: Timestamp, today: Date) -> Result<TimeCapsule, SealProblem> {
    if body.trim().is_empty() && attachments.is_empty() {
        return Err(SealProblem::Empty);
    }
    let opens_on = opening_day(until, today).ok_or(SealProblem::OutOfRange)?;
    Ok(TimeCapsule { id, written_at: now, written_on: today, opens_on, body: body.trim().to_string(), attachments, opened_at: None, reply: None, announced: false })
}

impl TimeCapsule {
    pub fn state(&self, today: Date) -> CapsuleState {
        match (self.opened_at, today >= self.opens_on) {
            (Some(_), _) => CapsuleState::Opened,
            (None, true) => CapsuleState::Ready,
            (None, false) => CapsuleState::Sealed { days_left: (self.opens_on - today).get_days().max(0) as u32 },
        }
    }

    /// The note, only once opened.
    pub fn body(&self) -> Option<&str> {
        self.opened_at.map(|_| self.body.as_str())
    }

    /// The images, only once opened.
    pub fn attachments(&self) -> Option<&[ImageAttachment]> {
        self.opened_at.map(|_| self.attachments.as_slice())
    }

    /// How many images it holds (shown on a sealed capsule; their content stays hidden).
    pub fn attachment_count(&self) -> usize {
        self.attachments.len()
    }

    /// Opens it at `now` when its day has come (`None` while sealed; unchanged if open).
    pub fn opened(&self, now: Timestamp, today: Date) -> Option<Self> {
        match self.state(today) {
            CapsuleState::Sealed { .. } => None,
            CapsuleState::Opened => Some(self.clone()),
            CapsuleState::Ready => Some(Self { opened_at: Some(now), announced: true, ..self.clone() }),
        }
    }

    /// The capsule with `reply` stored (only once opened).
    pub fn replied(&self, reply: &str) -> Option<Self> {
        self.opened_at.map(|_| Self { reply: Some(reply.trim().to_string()).filter(|r| !r.is_empty()), ..self.clone() })
    }

    /// Whole days between writing and opening (for "then and now").
    pub fn days_sealed(&self) -> u32 {
        (self.opens_on - self.written_on).get_days().max(0) as u32
    }
}
