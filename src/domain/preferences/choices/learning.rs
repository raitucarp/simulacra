//! Learning choices (sb E2): the weekday of the weekly review.

use super::super::choice::choice_enum;
use jiff::civil::Weekday;

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
            Self::Monday => Weekday::Monday,
            Self::Tuesday => Weekday::Tuesday,
            Self::Wednesday => Weekday::Wednesday,
            Self::Thursday => Weekday::Thursday,
            Self::Friday => Weekday::Friday,
            Self::Saturday => Weekday::Saturday,
            Self::Sunday => Weekday::Sunday,
        }
    }
}
