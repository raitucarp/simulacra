//! The clap button's gestures (R7): a click claps once, an Alt-click takes one clap back, and
//! holding the button down claps in a burst until it is let go.
//!
//! The timing is pure: [`burst_claps`] says how many claps a press held for some time has
//! given, so the shell only claps the difference on each tick and nothing on release.

use std::time::Duration;

/// How long the button must be held before the burst starts.
pub const HOLD_DELAY: Duration = Duration::from_millis(350);
/// Time between two claps of a burst.
pub const BURST_INTERVAL: Duration = Duration::from_millis(110);

/// What letting go of the button does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClapRelease {
    /// A plain click: one clap.
    Clap,
    /// An Alt-click: one clap taken back.
    Unclap,
    /// The press was a burst, already clapped while held: nothing more.
    Done,
}

/// Claps a press held for `held` has given: none before [`HOLD_DELAY`], then one at once
/// and one more every [`BURST_INTERVAL`]. An Alt press never bursts.
pub fn burst_claps(held: Duration, alt: bool) -> u32 {
    if alt || held < HOLD_DELAY {
        return 0;
    }
    let ticks = (held - HOLD_DELAY).as_millis() / BURST_INTERVAL.as_millis().max(1);
    u32::try_from(ticks).unwrap_or(u32::MAX).saturating_add(1)
}

/// What releasing a press held for `held` does on top of the burst it gave.
pub fn release(held: Duration, alt: bool) -> ClapRelease {
    match (alt, burst_claps(held, alt)) {
        (true, _) => ClapRelease::Unclap,
        (false, 0) => ClapRelease::Clap,
        (false, _) => ClapRelease::Done,
    }
}

/// The claps still owed by a burst that has given `given` so far, after `held`.
pub fn owed(held: Duration, given: u32) -> u32 {
    burst_claps(held, false).saturating_sub(given)
}
