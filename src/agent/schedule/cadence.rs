//! Maps a [`PostingCadence`] to a minimum posting interval and the hours it may post in.

use super::window::HourWindow;
use crate::domain::agent_config::PostingCadence;
use chrono::Duration;

/// Minimum gap for the 15-minute cadences.
const QUARTER_HOUR_MINUTES: i64 = 15;
/// Minimum gap for the 30-minute cadence.
const HALF_HOUR_MINUTES: i64 = 30;
/// Minimum gap for the hourly and night-owl cadences.
const HOUR_MINUTES: i64 = 60;
/// Minimum gap between the morning and evening reflections (one post per slot).
const TWICE_DAILY_GAP_MINUTES: i64 = 360;

/// Morning reflection slot (07:00–09:59).
const MORNING_SLOT: HourWindow = HourWindow::new(7, 10);
/// Evening reflection slot (18:00–21:59).
const EVENING_SLOT: HourWindow = HourWindow::new(18, 22);
/// Late-night contemplation slot (22:00–03:59).
const NIGHT_OWL_SLOT: HourWindow = HourWindow::new(22, 4);

/// Slots of the morning-and-evening cadence.
const MORNING_AND_EVENING: &[HourWindow] = &[MORNING_SLOT, EVENING_SLOT];
/// Slots of the night-owl cadence.
const NIGHT_OWL_ONLY: &[HourWindow] = &[NIGHT_OWL_SLOT];
/// No extra hour restriction beyond the agent's active hours.
const ANY_HOUR: &[HourWindow] = &[];

/// Scheduling rule derived from a cadence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CadenceRule {
    /// Minimum minutes between two autonomous posts by the same agent.
    pub min_interval_minutes: i64,
    /// Hours the cadence may post in; empty means any hour.
    pub slots: &'static [HourWindow],
}

impl CadenceRule {
    /// Minimum interval between two posts.
    pub fn min_interval(&self) -> Duration {
        Duration::minutes(self.min_interval_minutes)
    }

    /// Whether the cadence permits posting at local `hour`.
    pub fn allows_hour(&self, hour: u8) -> bool {
        self.slots.is_empty() || self.slots.iter().any(|slot| slot.contains(hour))
    }
}

/// The scheduling rule for `cadence`, or `None` when it is manual-only.
pub fn cadence_rule(cadence: PostingCadence) -> Option<CadenceRule> {
    let (min_interval_minutes, slots) = match cadence {
        PostingCadence::ManualOnly => return None,
        PostingCadence::OnEveryNewPost | PostingCadence::Every15Minutes => {
            (QUARTER_HOUR_MINUTES, ANY_HOUR)
        }
        PostingCadence::Every30Minutes => (HALF_HOUR_MINUTES, ANY_HOUR),
        PostingCadence::Hourly => (HOUR_MINUTES, ANY_HOUR),
        PostingCadence::MorningAndEvening => (TWICE_DAILY_GAP_MINUTES, MORNING_AND_EVENING),
        PostingCadence::NightOwlHours => (HOUR_MINUTES, NIGHT_OWL_ONLY),
    };
    Some(CadenceRule { min_interval_minutes, slots })
}
