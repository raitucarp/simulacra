//! The Free Spaced Repetition Scheduler (FSRS), as pure functions over a card's memory.
//!
//! The model follows the three-component model of memory (Ye, Su and Cao, "A Stochastic
//! Shortest Path Algorithm for Optimizing Spaced Repetition Scheduling", KDD 2022; and
//! "Optimizing Spaced Repetition Schedule by Capturing the Dynamics of Memory", IEEE TKDE
//! 2023): *stability* S is the number of days until recall probability falls to 90 %,
//! *difficulty* D (1–10) scales how fast stability grows, and *retrievability* R is the
//! probability of recall after `t` days.
//!
//! Formulas and the 21 default weights are those of FSRS-6, published in the algorithm
//! description of the open-spaced-repetition project
//! (<https://github.com/open-spaced-repetition/awesome-fsrs/wiki/The-Algorithm>):
//! - forgetting curve `R(t, S) = (1 + factor · t / S)^(−w20)`, `factor = 0.9^(−1/w20) − 1`
//!   (FSRS-4.5 and FSRS-5 fix the decay at 0.5, which gives their `19/81` factor);
//! - interval for a desired retention `r`: `I = S / factor · (r^(−1/w20) − 1)`;
//! - first review: `S0(G) = w[G−1]`, `D0(G) = w4 − e^(w5·(G−1)) + 1`;
//! - difficulty: `ΔD = −w6·(G−3)`, linear damping `D' = D + ΔD·(10−D)/9`, mean reversion
//!   `D'' = w7·D0(4) + (1−w7)·D'`;
//! - recall: `S' = S·(e^w8·(11−D)·S^(−w9)·(e^(w10·(1−R))−1)·hard(w15)·easy(w16) + 1)`;
//! - lapse: `S' = min(w11·D^(−w12)·((S+1)^w13 − 1)·e^(w14·(1−R)), S / e^(w17·w18))`;
//! - same-day review: `S' = S·e^(w17·(G−3+w18))·S^(−w19)`, never shrinking on Good or Easy.

mod memory;

pub use memory::{
    initial_difficulty, initial_stability, interval_days, lapse_stability, next_difficulty, recall_stability,
    retrievability, same_day_stability,
};

/// Number of FSRS-6 weights.
pub const WEIGHT_COUNT: usize = 21;

/// FSRS-6 default weights `w0..=w20` (trained on the open review-log benchmark).
pub const DEFAULT_WEIGHTS: [f64; WEIGHT_COUNT] = [
    0.212, 1.2931, 2.3065, 8.2956, 6.4133, 0.8334, 3.0194, 0.001, 1.8722, 0.1666, 0.796, 1.4835, 0.0614, 0.2629,
    1.6483, 0.6014, 1.8729, 0.5425, 0.0912, 0.0658, 0.1542,
];

/// Lowest and highest difficulty.
pub const MIN_DIFFICULTY: f64 = 1.0;
pub const MAX_DIFFICULTY: f64 = 10.0;
/// Stability never drops below this many days (keeps the curve defined).
pub const MIN_STABILITY: f64 = 0.01;
/// Retention the stability is defined at (`R(S, S) = 0.9`).
pub const STABILITY_RETENTION: f64 = 0.9;

/// A set of FSRS weights.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Params {
    pub weights: [f64; WEIGHT_COUNT],
}

impl Default for Params {
    fn default() -> Self {
        Self { weights: DEFAULT_WEIGHTS }
    }
}

impl Params {
    /// Weight `w<index>`.
    pub fn w(&self, index: usize) -> f64 {
        self.weights[index]
    }

    /// Power-law decay of the forgetting curve (`w20`).
    pub fn decay(&self) -> f64 {
        self.w(20)
    }

    /// Curve factor that makes `R(S, S)` equal 90 %.
    pub fn factor(&self) -> f64 {
        STABILITY_RETENTION.powf(-1.0 / self.decay()) - 1.0
    }
}

/// What a card's memory holds after at least one review.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Memory {
    /// Days until recall probability falls to 90 %.
    pub stability: f64,
    /// 1 (easiest) to 10 (hardest).
    pub difficulty: f64,
}
