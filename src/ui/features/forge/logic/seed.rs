//! A fresh seed for "roll the dice" and portrait re-rolls.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Mixes the clock so seeds taken in quick succession still differ widely.
const SEED_MIX: u64 = 0x9E37_79B9_7F4A_7C15;

/// Calls so far; keeps two seeds taken within one clock tick apart.
static CALLS: AtomicU64 = AtomicU64::new(0);

/// A seed derived from the current time and a call counter (never the same twice).
pub fn time_seed() -> u64 {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |age| age.as_nanos()) as u64;
    let call = CALLS.fetch_add(1, Ordering::Relaxed);
    nanos.wrapping_add(call).wrapping_mul(SEED_MIX)
}
