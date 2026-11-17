//! Notifications: the in-app toast switch and which kinds of toast are shown. Errors that
//! put data at risk (a failed vault write) are always shown.

use serde::{Deserialize, Serialize};

/// A family of in-app toasts the user can silence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastKind {
    /// "Imported", "Deleted" and other confirmations.
    Confirmation,
    /// A model request failed or a key is missing.
    AiFailure,
    /// An agent posted or replied.
    AgentActivity,
}

/// Toast preferences.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct NotificationPrefs {
    /// Master switch for every toast that can be silenced.
    pub toasts: bool,
    pub confirmations: bool,
    pub ai_failures: bool,
    /// Consumed by a later wave.
    pub agent_activity: bool,
}

impl Default for NotificationPrefs {
    fn default() -> Self {
        Self { toasts: true, confirmations: true, ai_failures: true, agent_activity: false }
    }
}

impl NotificationPrefs {
    /// Nothing to clamp.
    pub fn validated(self) -> Self {
        self
    }

    /// Whether a toast of `kind` may be shown.
    pub fn allows(&self, kind: ToastKind) -> bool {
        self.toasts
            && match kind {
                ToastKind::Confirmation => self.confirmations,
                ToastKind::AiFailure => self.ai_failures,
                ToastKind::AgentActivity => self.agent_activity,
            }
    }
}
