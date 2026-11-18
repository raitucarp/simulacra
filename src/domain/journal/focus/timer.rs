//! The focus timer as a pure state machine over wall-clock milliseconds: start, pause,
//! resume, stop, and a tick that finishes the session at its planned length.
//!
//! Drift: time is measured as the difference of wall-clock readings, never by counting
//! ticks, so a late or skipped repaint loses nothing. A clock that moves backwards counts as
//! no time. A gap between ticks longer than [`SUSPEND_GAP_MS`] is taken as the computer
//! sleeping: the timer pauses at the last tick instead of counting the sleep as focus.

/// A gap between ticks after which the computer is taken to have slept.
pub const SUSPEND_GAP_MS: i64 = 120_000;
pub const MS_PER_MINUTE: i64 = 60_000;
const MS_PER_SECOND: i64 = 1_000;
const SECONDS_PER_MINUTE: i64 = 60;
const SECONDS_PER_HOUR: i64 = 3_600;

/// Where the timer is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    /// Counting since `since`; `last_tick` is the latest reading seen.
    Running { since: i64, last_tick: i64 },
    /// Paused by the user, or because the computer slept (`suspended`).
    Paused { suspended: bool },
    /// Reached its planned length, or stopped early.
    Finished { early: bool },
}

/// A running or finished focus timer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FocusTimer {
    pub planned_ms: i64,
    /// Focus counted before the current run.
    pub banked_ms: i64,
    pub started_at: i64,
    pub phase: Phase,
}

impl FocusTimer {
    /// A timer of `minutes` started at `now`.
    pub fn start(minutes: u32, now: i64) -> Self {
        Self { planned_ms: i64::from(minutes.max(1)) * MS_PER_MINUTE, banked_ms: 0, started_at: now, phase: Phase::Running { since: now, last_tick: now } }
    }

    /// Focus time so far at `now` (never more than planned).
    pub fn elapsed(&self, now: i64) -> i64 {
        let run = match self.phase {
            Phase::Running { since, .. } => (now - since).max(0),
            _ => 0,
        };
        (self.banked_ms + run).min(self.planned_ms)
    }

    pub fn remaining(&self, now: i64) -> i64 {
        (self.planned_ms - self.elapsed(now)).max(0)
    }

    /// Share of the planned time done, 0..=1.
    pub fn progress(&self, now: i64) -> f32 {
        (self.elapsed(now) as f32 / self.planned_ms.max(1) as f32).clamp(0.0, 1.0)
    }

    pub fn is_running(&self) -> bool {
        matches!(self.phase, Phase::Running { .. })
    }

    pub fn is_finished(&self) -> bool {
        matches!(self.phase, Phase::Finished { .. })
    }

    fn banked(&self, now: i64, phase: Phase) -> Self {
        Self { banked_ms: self.elapsed(now), phase, ..*self }
    }

    pub fn pause(self, now: i64) -> Self {
        if self.is_running() { self.banked(now, Phase::Paused { suspended: false }) } else { self }
    }

    pub fn resume(self, now: i64) -> Self {
        match self.phase {
            Phase::Paused { .. } => Self { phase: Phase::Running { since: now, last_tick: now }, ..self },
            _ => self,
        }
    }

    /// Ends the session now (early unless the planned time was reached).
    pub fn stop(self, now: i64) -> Self {
        let early = self.elapsed(now) < self.planned_ms;
        if self.is_finished() { self } else { self.banked(now, Phase::Finished { early }) }
    }

    /// A wall-clock reading: finishes at the planned length, pauses after a sleep, and
    /// re-bases after the clock moved backwards.
    pub fn tick(self, now: i64) -> Self {
        let Phase::Running { since, last_tick } = self.phase else { return self };
        if now < last_tick {
            let banked = self.banked_ms + (last_tick - since).max(0);
            return Self { banked_ms: banked.min(self.planned_ms), phase: Phase::Running { since: now, last_tick: now }, ..self };
        }
        if now - last_tick > SUSPEND_GAP_MS {
            return self.banked(last_tick, Phase::Paused { suspended: true });
        }
        if self.elapsed(now) >= self.planned_ms {
            return self.banked(now, Phase::Finished { early: false });
        }
        Self { phase: Phase::Running { since, last_tick: now }, ..self }
    }
}

/// Whole minutes of `ms`, rounded to the nearest minute.
pub fn whole_minutes(ms: i64) -> u32 {
    ((ms.max(0) + MS_PER_MINUTE / 2) / MS_PER_MINUTE) as u32
}

/// `ms` as `m:ss` (or `h:mm:ss`), counting a started second as whole.
pub fn clock_text(ms: i64) -> String {
    let seconds = (ms.max(0) + MS_PER_SECOND - 1) / MS_PER_SECOND;
    let (h, m, s) = (seconds / SECONDS_PER_HOUR, seconds / SECONDS_PER_MINUTE % SECONDS_PER_MINUTE, seconds % SECONDS_PER_MINUTE);
    if h > 0 { format!("{h}:{m:02}:{s:02}") } else { format!("{m}:{s:02}") }
}
