//! "Ask before spending" (Settings › AI & Models): what an AI action the user starts is about
//! to send, and what this session has allowed so far. The shell asks through a confirm before
//! an action runs; the budget path charges a request only while [`SpendConsent::allows`] it.

mod estimate;
mod trigger; // A4: a manual agent run
#[cfg(test)]
mod trigger_tests;

pub use estimate::{debate_requests, experiment_requests, SpendEstimate};
pub use trigger::{trigger_estimate, TriggerEstimate};

/// What this session allowed (kept by the shell for as long as the window lives).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SpendConsent {
    /// "Don't ask again this session" was ticked once.
    remembered: bool,
    /// Requests the last confirm allowed that are not sent yet.
    granted: u32,
    /// A confirm was just accepted; the action it was asked for has not started yet.
    approved: bool,
}

impl SpendConsent {
    /// Whether starting an action must ask first (`ask` is the preference).
    pub const fn must_ask(self, ask: bool) -> bool {
        ask && !self.remembered && !self.approved
    }

    /// The action an accepted confirm was for starts (its approval is used up).
    pub const fn started(self) -> Self {
        Self { approved: false, ..self }
    }

    /// The user accepted a confirm for `estimate`; `remember` skips every later confirm.
    pub const fn granted(self, estimate: SpendEstimate, remember: bool) -> Self {
        Self { remembered: self.remembered || remember, granted: estimate.requests, approved: true }
    }

    /// Whether one request may be charged now without asking.
    pub const fn allows(self, ask: bool) -> bool {
        !ask || self.remembered || self.granted > 0
    }

    /// After one request was charged.
    pub const fn spent(self) -> Self {
        Self { granted: self.granted.saturating_sub(1), ..self }
    }

    /// Whether the user chose not to be asked again this session.
    pub const fn is_remembered(self) -> bool {
        self.remembered
    }
}

#[cfg(test)]
mod tests;
