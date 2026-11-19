//! One [`FailureGate`] per engine key (credential, provider or custom endpoint), so a broken
//! key backs off only the agents that use it.

use super::FailureGate;
use jiff::Timestamp;
use std::collections::HashMap;

/// Failure gates by engine key.
#[derive(Debug, Clone, Default)]
pub struct FailureGates {
    gates: HashMap<String, FailureGate>,
}

impl FailureGates {
    /// The gate of `key`, if it ever saw a failure since its last reset.
    pub fn gate(&self, key: &str) -> Option<&FailureGate> {
        self.gates.get(key)
    }

    /// The gate of `key`, created on first use.
    pub fn gate_mut(&mut self, key: &str) -> &mut FailureGate {
        self.gates.entry(key.to_string()).or_default()
    }

    /// Whether agents on `key` may generate at `now`.
    pub fn allows_generation(&self, key: &str, now: Timestamp) -> bool {
        self.gates.get(key).is_none_or(|gate| gate.allows_generation(now))
    }

    /// Forgets `key`'s notices and backoff (a generation on it succeeded).
    pub fn reset(&mut self, key: &str) {
        self.gates.remove(key);
    }

    /// Forgets every gate (settings or credentials changed).
    pub fn reset_all(&mut self) {
        self.gates.clear();
    }
}
