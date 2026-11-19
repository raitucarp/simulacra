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
    /// The day matching a `jiff` weekday.
    pub const fn from_jiff(day: jiff::civil::Weekday) -> Self {
        match day {
            jiff::civil::Weekday::Monday => Self::Monday,
            jiff::civil::Weekday::Tuesday => Self::Tuesday,
            jiff::civil::Weekday::Wednesday => Self::Wednesday,
            jiff::civil::Weekday::Thursday => Self::Thursday,
            jiff::civil::Weekday::Friday => Self::Friday,
            jiff::civil::Weekday::Saturday => Self::Saturday,
            jiff::civil::Weekday::Sunday => Self::Sunday,
        }
    }
}

/// The days an agent posts on (all seven by default).
pub type ActiveDays = OptionSet<Weekday>;

/// Every day of the week.
pub fn every_day() -> ActiveDays {
    OptionSet::all()
}
