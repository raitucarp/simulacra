//! Active hours around the chronotype's peak, and an age that suits the body.

use super::hint;
use super::super::model::{RangeHint, RangeUnit};
use super::super::param::Param;
use crate::domain::traits::{BodyKind, HumanChronotype};
use crate::state::forge::fields::{MAX_AGE, MAX_HOUR};
use crate::state::forge::{CountField, HourField};

/// `(start band, end band)` of the active hours per chronotype: online a little before the
/// peak and offline a little after it.
const fn hour_bands(chronotype: HumanChronotype) -> ((u16, u16), (u16, u16)) {
    match chronotype {
        HumanChronotype::EarlyBirdLark => ((4, 7), (12, 16)),
        HumanChronotype::DaytimeDiurnal => ((7, 10), (17, 21)),
        HumanChronotype::TwilightContemplative => ((13, 17), (21, 23)),
        HumanChronotype::NightOwlInsomniac => ((18, 22), (2, 6)),
    }
}

/// The band of one end of the active hours.
pub(super) fn hour_hint(field: HourField, chronotype: HumanChronotype) -> RangeHint {
    let (start, end) = hour_bands(chronotype);
    let band = match field {
        HourField::ActiveHoursStart => start,
        HourField::ActiveHoursEnd => end,
    };
    hint(Param::Hour(field), (0, u16::from(MAX_HOUR)), band, RangeUnit::Hour)
}

/// Typical ages per body: an adult thinker, a creature's lifespan, a machine's service years,
/// and long-lived places, plants, elements and ideas.
const fn age_band(kind: BodyKind) -> (u16, u16) {
    match kind {
        BodyKind::Human => (24, 70),
        BodyKind::Creature => (2, 15),
        BodyKind::Machine => (1, 30),
        BodyKind::Object => (5, 120),
        BodyKind::Plant | BodyKind::Element | BodyKind::Place | BodyKind::Abstraction => (40, 150),
    }
}

pub(super) fn age_hint(kind: BodyKind) -> RangeHint {
    hint(Param::Count(CountField::Age), (0, MAX_AGE), age_band(kind), RangeUnit::Plain)
}
