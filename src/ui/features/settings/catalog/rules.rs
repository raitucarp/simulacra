//! Which rows can be edited right now: dependent rows are disabled while the switch they
//! depend on is off (telemetry is a locked badge, never an input).

use super::rows::SettingRow;
use crate::domain::preferences::{AgentField, EmbeddingField, EmbeddingProvider, LearningField, NotificationField, Preferences, SettingField};

/// Whether `row` accepts input under `prefs`.
pub fn is_enabled(row: SettingRow, prefs: &Preferences) -> bool {
    let SettingRow::Field(field) = row else { return true };
    match field {
        SettingField::Agents(AgentField::QuietStart | AgentField::QuietEnd) => prefs.agents.quiet_hours,
        SettingField::Learning(LearningField::ReminderHour) => prefs.learning.review_reminder,
        SettingField::Learning(LearningField::WeeklyDay | LearningField::WeeklyHour) => prefs.learning.weekly_review_reminder,
        SettingField::Embeddings(EmbeddingField::Endpoint) => {
            matches!(prefs.embeddings.provider, EmbeddingProvider::LocalServer | EmbeddingProvider::Custom)
        }
        SettingField::Notifications(NotificationField::Toasts) => true,
        SettingField::Notifications(_) => prefs.notifications.toasts,
        _ => true,
    }
}
