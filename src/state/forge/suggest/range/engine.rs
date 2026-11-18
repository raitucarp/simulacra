//! Sampling and tool bands of the Engine: a lower temperature for structured answers, a higher
//! one for associative minds; an answer budget sized to the post format.

use super::hint;
use super::super::engine_param::EngineParam;
use super::super::model::{RangeHint, RangeUnit};
use super::super::param::Param;
use crate::domain::agent_config::AgentConfig;
use crate::domain::engine::{OutputMode, MAX_TOOL_STEPS_LIMIT};
use crate::domain::traits::{CognitivePace, HumorStyle, PostFormat};

/// Temperature in hundredths (0.00–2.00).
pub const TEMPERATURE_VALID: (u16, u16) = (0, 200);
/// Answer budget in tokens.
pub const MAX_TOKENS_VALID: (u16, u16) = (64, 8192);
/// Image chance in percent.
pub const CHANCE_VALID: (u16, u16) = (5, 100);
/// Tool-loop rounds.
pub const TOOL_STEPS_VALID: (u16, u16) = (1, MAX_TOOL_STEPS_LIMIT as u16);
pub(super) const CHANCE_TYPICAL: (u16, u16) = (10, 40);
const STRUCTURED_TEMPERATURE: (u16, u16) = (20, 50);
const CONVERSATIONAL_TEMPERATURE: (u16, u16) = (60, 90);
const PLAYFUL_TEMPERATURE: (u16, u16) = (80, 110);
const SHORT_ANSWER: (u16, u16) = (192, 448);
const POEM_ANSWER: (u16, u16) = (256, 640);
const LONG_ANSWER: (u16, u16) = (640, 1536);
const FEW_TOOLS: (u16, u16) = (2, 6);
const NO_TOOLS: (u16, u16) = (1, 2);

pub(super) fn temperature(cfg: &AgentConfig) -> RangeHint {
    let playful = matches!(cfg.cognitive_pace, CognitivePace::RapidAssociative) || matches!(cfg.humor_style, HumorStyle::AbsurdistSurreal);
    let band = match (&cfg.engine.output, playful) {
        (OutputMode::JsonSchema(_), _) => STRUCTURED_TEMPERATURE,
        (OutputMode::FreeText, true) => PLAYFUL_TEMPERATURE,
        (OutputMode::FreeText, false) => CONVERSATIONAL_TEMPERATURE,
    };
    hint(Param::Engine(EngineParam::Temperature), TEMPERATURE_VALID, band, RangeUnit::Hundredths)
}

pub(super) fn max_tokens(cfg: &AgentConfig) -> RangeHint {
    use PostFormat::*;
    let band = match cfg.rhetoric.post_format {
        SinglePost | Question | Aphorism => SHORT_ANSWER,
        Poem => POEM_ANSWER,
        Thread | List | Dialogue | FieldNote => LONG_ANSWER,
    };
    hint(Param::Engine(EngineParam::MaxTokens), MAX_TOKENS_VALID, band, RangeUnit::Plain)
}

pub(super) fn tool_steps(cfg: &AgentConfig) -> RangeHint {
    let band = if cfg.engine.tools.is_empty() { NO_TOOLS } else { FEW_TOOLS };
    hint(Param::Engine(EngineParam::MaxToolSteps), TOOL_STEPS_VALID, band, RangeUnit::Plain)
}
