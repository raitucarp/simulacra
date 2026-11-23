//! What the version probes found (P1), cached as one JSON row of the settings table so the
//! probes run only when the user asks (Settings › Code › Refresh).

use super::runtime::Runtime;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

/// One runtime found on this computer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DetectedRuntime {
    pub runtime: Runtime,
    /// The executable's full path.
    pub path: String,
    pub version: String,
}

/// Every runtime found by the last detection, and when it ran (`None`: never).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct RuntimeCache {
    pub found: Vec<DetectedRuntime>,
    pub probed_at: Option<Timestamp>,
}

impl RuntimeCache {
    /// Reads the stored JSON; anything unreadable (a runtime a newer version knows) reads as
    /// never detected, so a refresh is offered.
    pub fn from_json(raw: &str) -> Self {
        serde_json::from_str(raw).unwrap_or_default()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    /// What was found of `runtime`.
    pub fn get(&self, runtime: Runtime) -> Option<&DetectedRuntime> {
        self.found.iter().find(|found| found.runtime == runtime)
    }
}
