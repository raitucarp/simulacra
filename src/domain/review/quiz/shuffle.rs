//! Deterministic shuffling from a seed (SplitMix64), so quizzes are pure and testable; the
//! caller picks the seed (the clock) at the edge.

const GOLDEN_GAMMA: u64 = 0x9E37_79B9_7F4A_7C15;
const MIX_1: u64 = 0xBF58_476D_1CE4_E5B9;
const MIX_2: u64 = 0x94D0_49BB_1331_11EB;

/// The next state and output of SplitMix64.
fn splitmix(state: u64) -> (u64, u64) {
    let state = state.wrapping_add(GOLDEN_GAMMA);
    let mut z = state;
    z = (z ^ (z >> 30)).wrapping_mul(MIX_1);
    z = (z ^ (z >> 27)).wrapping_mul(MIX_2);
    (state, z ^ (z >> 31))
}

/// `items` in a seeded random order (Fisher–Yates).
pub fn shuffled<T>(items: Vec<T>, seed: u64) -> Vec<T> {
    let mut items = items;
    let mut state = seed;
    for upper in (1..items.len()).rev() {
        let (next, value) = splitmix(state);
        state = next;
        items.swap(upper, (value % (upper as u64 + 1)) as usize);
    }
    items
}

/// A seeded order of `0..len` that differs from the identity whenever `len > 1`.
pub fn derangement_order(len: usize, seed: u64) -> Vec<usize> {
    let order = shuffled((0..len).collect::<Vec<usize>>(), seed);
    if len > 1 && order.iter().enumerate().all(|(i, item)| i == *item) {
        let mut rotated = order;
        rotated.rotate_left(1);
        return rotated;
    }
    order
}

/// A seed derived from `seed` and `salt` (one per question of a quiz).
pub fn mix_seed(seed: u64, salt: u64) -> u64 {
    splitmix(seed ^ salt.wrapping_mul(GOLDEN_GAMMA)).1
}
