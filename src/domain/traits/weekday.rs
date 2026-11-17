//! Days of the week, for the days an agent posts on its own.

use super::set::OptionSet;
use crate::domain::trait_option::trait_enum;

trait_enum! {
    /// A day of the week, Monday first.
    pub enum Weekday in "weekday" {
        #[default]
        Monday("monday", "Monday", short: "Mon"),
        Tuesday("tuesday", "Tuesday", short: "Tue"),
        Wednesday("wednesday", "Wednesday", short: "Wed"),
        Thursday("thursday", "Thursday", short: "Thu"),
        Friday("friday", "Friday", short: "Fri"),
        Saturday("saturday", "Saturday", short: "Sat"),
        Sunday("sunday", "Sunday", short: "Sun"),
    }
}

impl Weekday {
    /// The day matching a `chrono` weekday.
    pub const fn from_chrono(day: chrono::Weekday) -> Self {
        match day {
            chrono::Weekday::Mon => Self::Monday,
            chrono::Weekday::Tue => Self::Tuesday,
            chrono::Weekday::Wed => Self::Wednesday,
            chrono::Weekday::Thu => Self::Thursday,
            chrono::Weekday::Fri => Self::Friday,
            chrono::Weekday::Sat => Self::Saturday,
            chrono::Weekday::Sun => Self::Sunday,
        }
    }
}

/// The days an agent posts on (all seven by default).
pub type ActiveDays = OptionSet<Weekday>;

/// Every day of the week.
pub fn every_day() -> ActiveDays {
    OptionSet::all()
}
