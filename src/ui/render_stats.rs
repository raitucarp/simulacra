//! Dev-only render counter: with `SIMULACRA_RENDER_STATS` set in a debug build, every view
//! render is counted per view name and a summary line is logged every
//! [`LOG_EVERY_RENDERS`] renders. Release builds compile it to nothing.

/// Environment variable switching the counter on (any value).
pub const RENDER_STATS_ENV: &str = "SIMULACRA_RENDER_STATS";
/// Renders (of all views) between two summary lines.
pub const LOG_EVERY_RENDERS: usize = 100;

#[cfg(debug_assertions)]
mod imp {
    use super::{LOG_EVERY_RENDERS, RENDER_STATS_ENV};
    use std::cell::RefCell;
    use std::collections::BTreeMap;
    use std::sync::OnceLock;

    thread_local! {
        static COUNTS: RefCell<BTreeMap<&'static str, usize>> = const { RefCell::new(BTreeMap::new()) };
    }

    fn enabled() -> bool {
        static ENABLED: OnceLock<bool> = OnceLock::new();
        *ENABLED.get_or_init(|| std::env::var_os(RENDER_STATS_ENV).is_some())
    }

    /// Counts one render of `view` (and logs a summary periodically).
    pub fn record(view: &'static str) {
        if !enabled() {
            return;
        }
        COUNTS.with(|counts| {
            let mut counts = counts.borrow_mut();
            *counts.entry(view).or_default() += 1;
            let total: usize = counts.values().sum();
            if total.is_multiple_of(LOG_EVERY_RENDERS) {
                let line: Vec<String> = counts.iter().map(|(name, n)| format!("{name}={n}")).collect();
                eprintln!("simulacra[render-stats] {}", line.join(" "));
            }
        });
    }

    /// Render count of `view` so far on this thread.
    pub fn count(view: &'static str) -> usize {
        COUNTS.with(|counts| counts.borrow().get(view).copied().unwrap_or_default())
    }
}

#[cfg(debug_assertions)]
pub use imp::{count, record};

/// Counts one render of `view` (no-op in release builds).
#[cfg(not(debug_assertions))]
#[inline(always)]
pub fn record(_view: &'static str) {}

/// Render count of `view` (always zero in release builds).
#[cfg(not(debug_assertions))]
#[inline(always)]
pub fn count(_view: &'static str) -> usize {
    0
}
