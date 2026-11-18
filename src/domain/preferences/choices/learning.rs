//! Learning choices (sb E2): the weekday of the weekly review.

use super::super::choice::choice_enum;
use chrono::Weekday;

choice_enum! {
    /// The day the weekly review reminder shows.
    ReviewWeekday, default: Sunday,
    [
        Monday => "monday", Tuesday => "tuesday", Wednesday => "wednesday", Thursday => "thursday",
        Friday => "friday", Saturday => "saturday", Sunday => "sunday",
    ]
}

impl ReviewWeekday {
    /// The calendar weekday.
    pub const fn weekday(self) -> Weekday {
        match self {
            Self::Monday => Weekday::Mon,
            Self::Tuesday => Weekday::Tue,
            Self::Wednesday => Weekday::Wed,
            Self::Thursday => Weekday::Thu,
            Self::Friday => Weekday::Fri,
            Self::Saturday => Weekday::Sat,
            Self::Sunday => Weekday::Sun,
        }
    }
}
