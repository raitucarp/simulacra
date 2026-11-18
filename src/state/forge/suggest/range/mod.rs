//! Typical bands and defaults of the numeric parameters, each following the rest of the draft
//! (personality bands follow the type letters, hours the chronotype, sampling the post format).

mod engine;
mod schedule;
mod scores;

pub use engine::{CHANCE_VALID, MAX_TOKENS_VALID, TEMPERATURE_VALID, TOOL_STEPS_VALID};

use super::context::SuggestContext;
use super::engine_param::EngineParam;
use super::keys::why_key;
use super::model::{Phrase, RangeHint, RangeUnit};
use super::param::Param;
use crate::domain::portrait::{ZOOM_MAX, ZOOM_MIN};

/// A typical photo zoom: a little closer than the whole picture.
const ZOOM_TYPICAL: (u16, u16) = (1000, 1800);
const ZOOM_RECOMMENDED: u16 = 1200;

/// A hint over `valid` with band `typical`, default at its middle unless `recommended` is given.
pub(super) fn hint(param: Param, valid: (u16, u16), typical: (u16, u16), unit: RangeUnit) -> RangeHint {
    let (low, high) = (typical.0.clamp(valid.0, valid.1), typical.1.clamp(valid.0, valid.1));
    let typical = (low.min(high), low.max(high));
    let recommended = typical.0 + (typical.1 - typical.0) / 2;
    RangeHint { valid, typical, recommended, unit, why: Phrase::key(why_key(param.path())) }
}

/// The range suggestions of `param`, when it is numeric.
pub(super) fn range_hint(param: Param, ctx: &SuggestContext) -> Option<RangeHint> {
    let cfg = ctx.config();
    match param {
        Param::Score(field) => Some(scores::score_hint(field, cfg)),
        Param::Hour(field) => Some(schedule::hour_hint(field, cfg.chronotype)),
        Param::Count(_) => Some(schedule::age_hint(cfg.embodiment.body_kind)),
        Param::Engine(EngineParam::Temperature) => Some(engine::temperature(cfg)),
        Param::Engine(EngineParam::MaxTokens) => Some(engine::max_tokens(cfg)),
        Param::Engine(EngineParam::ImageChance) => Some(hint(param, CHANCE_VALID, engine::CHANCE_TYPICAL, RangeUnit::Percent)),
        Param::Engine(EngineParam::MaxToolSteps) => Some(engine::tool_steps(cfg)),
        Param::PhotoZoom => Some(RangeHint { recommended: ZOOM_RECOMMENDED, ..hint(param, (ZOOM_MIN, ZOOM_MAX), ZOOM_TYPICAL, RangeUnit::PerMille) }),
        _ => None,
    }
}
