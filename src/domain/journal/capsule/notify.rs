//! Which capsules to announce: those whose day has come, not opened and not yet announced.
//! The shell shows one notice for them and marks them announced, so each is told once.

use super::{CapsuleState, TimeCapsule};
use jiff::civil::Date;

/// Capsules ready on `today` that were never announced.
pub fn to_announce(capsules: &[TimeCapsule], today: Date) -> Vec<&TimeCapsule> {
    capsules.iter().filter(|c| !c.announced && c.state(today) == CapsuleState::Ready).collect()
}

/// `capsule` marked announced.
pub fn announced(capsule: &TimeCapsule) -> TimeCapsule {
    TimeCapsule { announced: true, ..capsule.clone() }
}

/// Capsules in the list order of the Capsules tab: ready first, then sealed by opening day,
/// then opened (most recently opened first).
pub fn in_list_order(capsules: &[TimeCapsule], today: Date) -> Vec<&TimeCapsule> {
    let rank = |c: &TimeCapsule| match c.state(today) {
        CapsuleState::Ready => 0,
        CapsuleState::Sealed { .. } => 1,
        CapsuleState::Opened => 2,
    };
    let mut sorted: Vec<&TimeCapsule> = capsules.iter().collect();
    sorted.sort_by(|a, b| rank(a).cmp(&rank(b)).then(a.opens_on.cmp(&b.opens_on)).then(b.opened_at.cmp(&a.opened_at)));
    sorted
}
