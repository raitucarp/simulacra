//! Privacy & Data: Simulacra sends no telemetry, and the stored flag can never turn it on.

use serde::{Deserialize, Serialize};

/// Privacy preferences.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PrivacyPrefs {
    /// Always `false`: shown read-only and forced off by [`Self::validated`].
    pub telemetry: bool,
}

impl PrivacyPrefs {
    /// Telemetry stays off whatever was stored.
    pub fn validated(self) -> Self {
        Self { telemetry: false }
    }
}
