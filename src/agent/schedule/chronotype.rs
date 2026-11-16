//! Chronotype peak hours and the spontaneity-driven per-tick posting chance.

use super::window::HourWindow;
use super::PERCENT_MAX;
use crate::domain::agent_config::HumanChronotype;

/// Early-bird lark peak (05:00–11:59).
const LARK_PEAK: HourWindow = HourWindow::new(5, 12);
/// Daytime diurnal peak (09:00–17:59).
const DIURNAL_PEAK: HourWindow = HourWindow::new(9, 18);
/// Twilight contemplative peak (17:00–23:59).
const TWILIGHT_PEAK: HourWindow = HourWindow::new(17, 0);
/// Night-owl insomniac peak (22:00–04:59).
const NIGHT_OWL_PEAK: HourWindow = HourWindow::new(22, 5);

/// Lowest per-tick chance (percent) of a maximally spontaneous agent at its peak.
const MIN_PEAK_CHANCE: u8 = 10;
/// Off-peak hours divide the posting chance by this factor.
const OFF_PEAK_DIVISOR: u8 = 2;
/// Floor of the chance so an eligible agent is never starved entirely.
const MIN_CHANCE: u8 = 1;

/// Hours in which the chronotype feels most alert and posts most readily.
pub fn peak_window(chronotype: HumanChronotype) -> HourWindow {
    match chronotype {
        HumanChronotype::EarlyBirdLark => LARK_PEAK,
        HumanChronotype::DaytimeDiurnal => DIURNAL_PEAK,
        HumanChronotype::TwilightContemplative => TWILIGHT_PEAK,
        HumanChronotype::NightOwlInsomniac => NIGHT_OWL_PEAK,
    }
}

/// Percent chance (`1..=100`) that an otherwise-due agent posts on this heartbeat tick.
///
/// Spontaneity is timing jitter: `0` posts exactly when the cadence interval elapses
/// (100%), `100` posts with only a [`MIN_PEAK_CHANCE`]% chance per tick, spreading posts
/// out geometrically. Outside the chronotype's peak hours the chance shrinks further,
/// by up to [`OFF_PEAK_DIVISOR`]× at full spontaneity (a clockwork agent, spontaneity
/// `0`, ignores its chronotype).
pub fn posting_chance(spontaneity: u8, chronotype: HumanChronotype, hour: u8) -> u8 {
    let percent = u16::from(PERCENT_MAX);
    let spontaneity = u16::from(spontaneity.min(PERCENT_MAX));
    let span = u16::from(PERCENT_MAX - MIN_PEAK_CHANCE);
    let peak_chance = percent - spontaneity * span / percent;
    let chance = if peak_window(chronotype).contains(hour) {
        peak_chance
    } else {
        let off_peak_factor = percent - spontaneity / u16::from(OFF_PEAK_DIVISOR);
        peak_chance * off_peak_factor / percent
    };
    (chance as u8).max(MIN_CHANCE)
}
