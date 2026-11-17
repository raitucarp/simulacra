//! Advanced: log level and developer options.

use super::super::choices::LogLevel;
use serde::{Deserialize, Serialize};

/// Diagnostics preferences.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct AdvancedPrefs {
    /// Consumed by a later wave.
    pub log_level: LogLevel,
    /// Extra diagnostics in the interface. Consumed by a later wave.
    pub developer_options: bool,
}

impl AdvancedPrefs {
    /// Nothing to clamp.
    pub fn validated(self) -> Self {
        self
    }
}
