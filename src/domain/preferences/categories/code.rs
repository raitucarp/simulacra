//! Code (P4): running code blocks is off until the user turns it on; each runtime can then be
//! switched off on its own, and every run is stopped after the timeout.

use super::super::bounds::Bounds;
use crate::domain::code::Runtime;
use serde::{Deserialize, Serialize};

/// Seconds one run may take.
pub const RUN_TIMEOUT_SECONDS: Bounds = Bounds::new(1, 300, 1);
const DEFAULT_TIMEOUT_SECONDS: u32 = 10;

/// Code running preferences.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct CodePrefs {
    /// The master switch, off by default: nothing runs while it is off.
    pub enabled: bool,
    /// Seconds a run may take before it is stopped with everything it started.
    pub timeout_seconds: u32,
    /// Keys of the runtimes switched off one by one (every runtime is on otherwise). Keys,
    /// not runtimes, so a key this version does not know is kept and harms nothing.
    pub disabled_runtimes: Vec<String>,
}

impl Default for CodePrefs {
    fn default() -> Self {
        Self { enabled: false, timeout_seconds: DEFAULT_TIMEOUT_SECONDS, disabled_runtimes: Vec::new() }
    }
}

impl CodePrefs {
    /// The timeout clamped into range, each switched-off runtime listed once.
    pub fn validated(self) -> Self {
        let mut disabled: Vec<String> = Vec::new();
        for key in self.disabled_runtimes {
            if !disabled.contains(&key) {
                disabled.push(key);
            }
        }
        Self { timeout_seconds: RUN_TIMEOUT_SECONDS.clamp_u32(self.timeout_seconds), disabled_runtimes: disabled, ..self }
    }

    /// Whether code may run in `runtime`: the master switch is on and the runtime is not off.
    pub fn allows(&self, runtime: Runtime) -> bool {
        self.enabled && !self.is_switched_off(runtime)
    }

    /// Whether `runtime` was switched off on its own.
    pub fn is_switched_off(&self, runtime: Runtime) -> bool {
        self.disabled_runtimes.iter().any(|key| key == runtime.key())
    }

    /// The preferences with `runtime` switched on or off.
    pub fn with_runtime(mut self, runtime: Runtime, on: bool) -> Self {
        self.disabled_runtimes.retain(|key| key != runtime.key());
        if !on {
            self.disabled_runtimes.push(runtime.key().to_string());
        }
        self
    }
}
