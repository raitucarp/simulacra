//! What is remembered about a pair, by its content hash: the AI's verdict and reason (once
//! classified) and what the user did with it. A pair whose texts change gets a new key, so
//! neither a stale verdict nor a stale dismissal carries over to edited texts.

use super::classify::{Classification, Verdict};
use super::status::TensionStatus;

/// One remembered pair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TensionRecord {
    /// [`super::pair_key`] of the pair.
    pub pair: String,
    /// Side keys ([`super::TensionSide::key`]) of both texts.
    pub left: String,
    pub right: String,
    pub verdict: Option<Verdict>,
    pub reason: String,
    pub status: TensionStatus,
}

impl TensionRecord {
    /// A fresh, unclassified, open record.
    pub fn new(pair: &str, left: &str, right: &str) -> Self {
        Self { pair: pair.into(), left: left.into(), right: right.into(), verdict: None, reason: String::new(), status: TensionStatus::Open }
    }

    /// This record with the model's verdict.
    pub fn classified(self, classification: &Classification) -> Self {
        Self { verdict: Some(classification.verdict), reason: classification.reason.clone(), ..self }
    }

    pub fn with_status(self, status: TensionStatus) -> Self {
        Self { status, ..self }
    }

    /// Whether the pair still belongs on the Tensions view: open, and not judged compatible
    /// or unrelated by the model.
    pub fn is_shown(&self) -> bool {
        self.status == TensionStatus::Open && matches!(self.verdict, None | Some(Verdict::Contradicts))
    }
}
