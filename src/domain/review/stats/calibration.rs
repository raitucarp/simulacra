//! Metacognitive calibration: for each confidence the learner reported before a quiz answer
//! was checked, how often the answer was right. Well calibrated learners are right far more
//! often when sure than when guessing.

use crate::domain::review::grade::Confidence;
use crate::domain::review::log::ReviewEntry;

/// Checked answers given at one confidence.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CalibrationRow {
    pub confidence: Confidence,
    pub answers: usize,
    pub correct: usize,
}

impl CalibrationRow {
    /// Share answered correctly (0–1), when there were answers.
    pub fn accuracy(&self) -> Option<f64> {
        (self.answers > 0).then(|| self.correct as f64 / self.answers as f64)
    }
}

/// One row per confidence, in [`Confidence::ALL`] order.
pub fn calibration(log: &[ReviewEntry]) -> Vec<CalibrationRow> {
    Confidence::ALL
        .into_iter()
        .map(|confidence| {
            let checked: Vec<bool> = log.iter().filter(|e| e.confidence == Some(confidence)).filter_map(|e| e.correct).collect();
            CalibrationRow { confidence, answers: checked.len(), correct: checked.iter().filter(|right| **right).count() }
        })
        .collect()
}
