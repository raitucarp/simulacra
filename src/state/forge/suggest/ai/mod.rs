//! "Suggest from context", pure halves: which parameters a request covers ([`targets`]), the
//! prompt that describes the draft ([`suggest_prompt`]), robust parsing of the reply and
//! validation of every proposed value against its parameter's type, range and options
//! ([`parse_suggestions`]), and the edit an accepted value makes ([`value_edit`]). The request
//! itself runs through the post actions runner (`agent::post_actions`).

mod edit;
mod parse;
mod prompt;
mod summary;
mod value;

pub use edit::value_edit;
pub use parse::{parse_suggestions, ParsedSuggestions};
pub use prompt::{answer_tokens, suggest_prompt};
pub use value::SuggestedValue;

use super::engine_param::EngineParam;
use super::param::Param;
use crate::domain::agent_config::AgentConfig;
use crate::domain::engine::{ImageEngine, ImageTrigger, OutputMode, TextModel};
use crate::state::forge::{steps_for, ForgeKind, ForgeStep};

/// What one request asks for: one parameter, or every suggestible parameter of a step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SuggestScope {
    Field(Param),
    Step(ForgeStep),
}

/// Whether an Engine setting matters on the current engine (sampling only for an own model,
/// image settings only with images on, the schema only for structured answers).
fn applies(param: Param, cfg: &AgentConfig) -> bool {
    let engine = &cfg.engine;
    match param {
        Param::Engine(EngineParam::Temperature | EngineParam::MaxTokens) => matches!(engine.text, TextModel::Own(_)),
        Param::Engine(EngineParam::SchemaPreset) => matches!(engine.output, OutputMode::JsonSchema(_)),
        Param::Engine(EngineParam::ImageSize | EngineParam::StylePrefix | EngineParam::Trigger) => engine.image.engine != ImageEngine::None,
        Param::Engine(EngineParam::ImageChance) => matches!(engine.image.trigger, ImageTrigger::Chance { .. }) && engine.image.is_enabled(),
        _ => true,
    }
}

/// The parameters a request of `scope` asks values for, on a draft of `kind`.
pub fn targets(scope: SuggestScope, kind: ForgeKind, cfg: &AgentConfig) -> Vec<Param> {
    let candidates = match scope {
        SuggestScope::Field(param) => vec![param],
        SuggestScope::Step(step) if steps_for(kind).contains(&step) => Param::for_step(step),
        SuggestScope::Step(_) => Vec::new(),
    };
    candidates.into_iter().filter(|param| param.ai_capable() && applies(*param, cfg)).collect()
}
