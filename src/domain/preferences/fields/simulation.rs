//! Fields of the Agents & Simulation and Learning categories.

use super::macros::category_fields;
use super::value::{choice, choice_value, choices, count, number, number_value, toggle, FieldKind, NumberUnit, SettingValue};
use crate::domain::preferences::{AgentPrefs, MAX_CONCURRENT, POSTS_PER_HOUR, REPLY_DEPTH};
use crate::domain::preferences::{LearningPrefs, ReviewWeekday, DAILY_REVIEWS, DESIRED_RETENTION, HOUR_OF_DAY, NEW_CARDS};

category_fields! {
    /// An Agents & Simulation preference.
    AgentField {
        AutonomousPosting => "autonomous_posting", PostsPerHour => "posts_per_hour", QuietHours => "quiet_hours",
        QuietStart => "quiet_start", QuietEnd => "quiet_end", MaxConcurrent => "max_concurrent",
        ReplyDepthLimit => "reply_depth_limit",
    }
}

impl AgentField {
    pub fn kind(self) -> FieldKind {
        match self {
            Self::AutonomousPosting | Self::QuietHours => FieldKind::Toggle,
            Self::PostsPerHour => number(POSTS_PER_HOUR, NumberUnit::Posts),
            Self::QuietStart | Self::QuietEnd => number(HOUR_OF_DAY, NumberUnit::Hour),
            Self::MaxConcurrent => number(MAX_CONCURRENT, NumberUnit::Generations),
            Self::ReplyDepthLimit => number(REPLY_DEPTH, NumberUnit::Replies),
        }
    }

    pub fn get(self, prefs: &AgentPrefs) -> SettingValue {
        match self {
            Self::AutonomousPosting => SettingValue::Toggle(prefs.autonomous_posting),
            Self::PostsPerHour => number_value(prefs.posts_per_hour),
            Self::QuietHours => SettingValue::Toggle(prefs.quiet_hours),
            Self::QuietStart => number_value(prefs.quiet_start),
            Self::QuietEnd => number_value(prefs.quiet_end),
            Self::MaxConcurrent => number_value(prefs.max_concurrent),
            Self::ReplyDepthLimit => number_value(prefs.reply_depth_limit),
        }
    }

    pub fn set(self, prefs: AgentPrefs, value: &SettingValue) -> AgentPrefs {
        match self {
            Self::AutonomousPosting => AgentPrefs { autonomous_posting: toggle(value, prefs.autonomous_posting), ..prefs },
            Self::PostsPerHour => AgentPrefs { posts_per_hour: count(value, prefs.posts_per_hour), ..prefs },
            Self::QuietHours => AgentPrefs { quiet_hours: toggle(value, prefs.quiet_hours), ..prefs },
            Self::QuietStart => AgentPrefs { quiet_start: count(value, prefs.quiet_start), ..prefs },
            Self::QuietEnd => AgentPrefs { quiet_end: count(value, prefs.quiet_end), ..prefs },
            Self::MaxConcurrent => AgentPrefs { max_concurrent: count(value, prefs.max_concurrent), ..prefs },
            Self::ReplyDepthLimit => AgentPrefs { reply_depth_limit: count(value, prefs.reply_depth_limit), ..prefs },
        }
    }
}

category_fields! {
    /// A Learning (spaced repetition) preference.
    LearningField {
        DailyReviewLimit => "daily_review_limit", NewCardsPerDay => "new_cards_per_day",
        DesiredRetention => "desired_retention", ReviewReminder => "review_reminder", ReminderHour => "reminder_hour",
        WeeklyReminder => "weekly_review_reminder", WeeklyDay => "weekly_review_day", WeeklyHour => "weekly_review_hour",
    }
}

impl LearningField {
    pub fn kind(self) -> FieldKind {
        match self {
            Self::DailyReviewLimit => number(DAILY_REVIEWS, NumberUnit::Cards),
            Self::NewCardsPerDay => number(NEW_CARDS, NumberUnit::Cards),
            Self::DesiredRetention => number(DESIRED_RETENTION, NumberUnit::Percent),
            Self::ReviewReminder => FieldKind::Toggle,
            Self::ReminderHour | Self::WeeklyHour => number(HOUR_OF_DAY, NumberUnit::Hour),
            Self::WeeklyReminder => FieldKind::Toggle,
            Self::WeeklyDay => choices::<ReviewWeekday>(),
        }
    }

    pub fn get(self, prefs: &LearningPrefs) -> SettingValue {
        match self {
            Self::DailyReviewLimit => number_value(prefs.daily_review_limit),
            Self::NewCardsPerDay => number_value(prefs.new_cards_per_day),
            Self::DesiredRetention => number_value(prefs.desired_retention),
            Self::ReviewReminder => SettingValue::Toggle(prefs.review_reminder),
            Self::ReminderHour => number_value(prefs.reminder_hour),
            Self::WeeklyReminder => SettingValue::Toggle(prefs.weekly_review_reminder),
            Self::WeeklyDay => choice_value(prefs.weekly_review_day),
            Self::WeeklyHour => number_value(prefs.weekly_review_hour),
        }
    }

    pub fn set(self, prefs: LearningPrefs, value: &SettingValue) -> LearningPrefs {
        match self {
            Self::DailyReviewLimit => LearningPrefs { daily_review_limit: count(value, prefs.daily_review_limit), ..prefs },
            Self::NewCardsPerDay => LearningPrefs { new_cards_per_day: count(value, prefs.new_cards_per_day), ..prefs },
            Self::DesiredRetention => LearningPrefs { desired_retention: count(value, prefs.desired_retention), ..prefs },
            Self::ReviewReminder => LearningPrefs { review_reminder: toggle(value, prefs.review_reminder), ..prefs },
            Self::ReminderHour => LearningPrefs { reminder_hour: count(value, prefs.reminder_hour), ..prefs },
            Self::WeeklyReminder => LearningPrefs { weekly_review_reminder: toggle(value, prefs.weekly_review_reminder), ..prefs },
            Self::WeeklyDay => LearningPrefs { weekly_review_day: choice(value, prefs.weekly_review_day), ..prefs },
            Self::WeeklyHour => LearningPrefs { weekly_review_hour: count(value, prefs.weekly_review_hour), ..prefs },
        }
    }
}
