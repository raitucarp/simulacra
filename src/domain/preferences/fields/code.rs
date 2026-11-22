//! Fields of the Code category (P4); the per-runtime switches live in the Runtimes pane.

use super::macros::category_fields;
use super::value::{count, number, number_value, toggle, FieldKind, NumberUnit, SettingValue};
use crate::domain::preferences::{CodePrefs, RUN_TIMEOUT_SECONDS};

category_fields! {
    /// A Code preference.
    CodeField { Enabled => "enabled", Timeout => "timeout_seconds" }
}

impl CodeField {
    pub fn kind(self) -> FieldKind {
        match self {
            Self::Enabled => FieldKind::Toggle,
            Self::Timeout => number(RUN_TIMEOUT_SECONDS, NumberUnit::Seconds),
        }
    }

    pub fn get(self, prefs: &CodePrefs) -> SettingValue {
        match self {
            Self::Enabled => SettingValue::Toggle(prefs.enabled),
            Self::Timeout => number_value(prefs.timeout_seconds),
        }
    }

    pub fn set(self, prefs: CodePrefs, value: &SettingValue) -> CodePrefs {
        match self {
            Self::Enabled => CodePrefs { enabled: toggle(value, prefs.enabled), ..prefs },
            Self::Timeout => CodePrefs { timeout_seconds: count(value, prefs.timeout_seconds), ..prefs },
        }
    }
}
