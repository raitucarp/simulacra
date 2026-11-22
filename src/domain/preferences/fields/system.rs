//! Fields of the Privacy & Data, Notifications and Advanced categories.

use super::macros::category_fields;
use super::value::{choice, choice_value, choices, toggle, FieldKind, SettingValue};
use crate::domain::preferences::{AdvancedPrefs, LogLevel, NotificationPrefs, PrivacyPrefs};

category_fields! {
    /// A Privacy & Data preference.
    PrivacyField { LinkPreviews => "link_previews", JournalInSearch => "journal_in_search", Telemetry => "telemetry" }
}

impl PrivacyField {
    pub fn kind(self) -> FieldKind {
        match self {
            Self::LinkPreviews | Self::JournalInSearch => FieldKind::Toggle,
            Self::Telemetry => FieldKind::Locked,
        }
    }

    pub fn get(self, prefs: &PrivacyPrefs) -> SettingValue {
        SettingValue::Toggle(match self {
            Self::LinkPreviews => prefs.link_previews,
            Self::JournalInSearch => prefs.journal_in_search,
            Self::Telemetry => prefs.telemetry,
        })
    }

    /// Telemetry is never switched on.
    pub fn set(self, prefs: PrivacyPrefs, value: &SettingValue) -> PrivacyPrefs {
        match self {
            Self::LinkPreviews => PrivacyPrefs { link_previews: toggle(value, prefs.link_previews), ..prefs },
            Self::JournalInSearch => PrivacyPrefs { journal_in_search: toggle(value, prefs.journal_in_search), ..prefs },
            Self::Telemetry => prefs,
        }
    }
}

category_fields! {
    /// A Notifications preference.
    NotificationField {
        Toasts => "toasts", Confirmations => "confirmations", AiFailures => "ai_failures", AgentActivity => "agent_activity", DueReminders => "due_reminders",
    }
}

impl NotificationField {
    pub fn kind(self) -> FieldKind {
        FieldKind::Toggle
    }

    pub fn get(self, prefs: &NotificationPrefs) -> SettingValue {
        SettingValue::Toggle(match self {
            Self::Toasts => prefs.toasts,
            Self::Confirmations => prefs.confirmations,
            Self::AiFailures => prefs.ai_failures,
            Self::AgentActivity => prefs.agent_activity,
            Self::DueReminders => prefs.due_reminders,
        })
    }

    pub fn set(self, prefs: NotificationPrefs, value: &SettingValue) -> NotificationPrefs {
        match self {
            Self::Toasts => NotificationPrefs { toasts: toggle(value, prefs.toasts), ..prefs },
            Self::Confirmations => NotificationPrefs { confirmations: toggle(value, prefs.confirmations), ..prefs },
            Self::AiFailures => NotificationPrefs { ai_failures: toggle(value, prefs.ai_failures), ..prefs },
            Self::AgentActivity => NotificationPrefs { agent_activity: toggle(value, prefs.agent_activity), ..prefs },
            Self::DueReminders => NotificationPrefs { due_reminders: toggle(value, prefs.due_reminders), ..prefs },
        }
    }
}

category_fields! {
    /// An Advanced preference.
    AdvancedField { LogLevel => "log_level", DeveloperOptions => "developer_options" }
}

impl AdvancedField {
    pub fn kind(self) -> FieldKind {
        match self {
            Self::LogLevel => choices::<LogLevel>(),
            Self::DeveloperOptions => FieldKind::Toggle,
        }
    }

    pub fn get(self, prefs: &AdvancedPrefs) -> SettingValue {
        match self {
            Self::LogLevel => choice_value(prefs.log_level),
            Self::DeveloperOptions => SettingValue::Toggle(prefs.developer_options),
        }
    }

    pub fn set(self, prefs: AdvancedPrefs, value: &SettingValue) -> AdvancedPrefs {
        match self {
            Self::LogLevel => AdvancedPrefs { log_level: choice(value, prefs.log_level), ..prefs },
            Self::DeveloperOptions => AdvancedPrefs { developer_options: toggle(value, prefs.developer_options), ..prefs },
        }
    }
}
