//! Suggestions for every agent and persona parameter (sb D1), as pure data: example values for
//! texts, a typical band and default for numbers, starting picks and known combinations for
//! choices — each a function of the rest of the draft ([`suggestions_for`]). Also the
//! deterministic "fill the rest" ([`fill_edits`]) and the pure halves of "suggest from context"
//! with the AI ([`ai`]: prompt, parsing, validation).
//!
//! - [`Param`] / [`EngineParam`]: one id per parameter over the field registries.
//! - [`FieldSuggestions`]: what the catalog says about one parameter.
//! - [`choice`]: option keys, labels, current values and combinations of choices.

pub mod ai;
pub mod choice;
mod context;
mod engine_param;
mod fill;
pub mod keys;
mod model;
mod param;
mod phrase;
mod range;
mod text;

#[cfg(test)]
mod tests;

pub use context::SuggestContext;
pub use engine_param::{EngineParam, GROUP_CAPTIONS};
pub use fill::fill_edits;
pub use model::{ChoiceHints, Combo, Example, FieldSuggestions, Joiner, OptionRef, Phrase, RangeHint, RangeUnit, TextHints, Verdict};
pub use param::Param;
pub use range::{CHANCE_VALID, MAX_TOKENS_VALID, TEMPERATURE_VALID, TOOL_STEPS_VALID};

use keys::why_key;

/// Everything the catalog suggests for `param` on the draft described by `ctx`.
pub fn suggestions_for(param: Param, ctx: &SuggestContext) -> FieldSuggestions {
    text::text_hints(param, ctx)
        .map(FieldSuggestions::Text)
        .or_else(|| range::range_hint(param, ctx).map(FieldSuggestions::Range))
        .or_else(|| choice::choice_hints(param, ctx).map(FieldSuggestions::Choice))
        .unwrap_or_else(|| FieldSuggestions::Guide(Phrase::key(why_key(param.path()))))
}

/// The range hint of `param`, if it is numeric.
pub fn range_hint(param: Param, ctx: &SuggestContext) -> Option<RangeHint> {
    match suggestions_for(param, ctx) {
        FieldSuggestions::Range(hint) => Some(hint),
        _ => None,
    }
}

/// The starting picks of `param`, if it is a choice.
pub fn picks_for(param: Param, ctx: &SuggestContext) -> Vec<&'static str> {
    match suggestions_for(param, ctx) {
        FieldSuggestions::Choice(hints) => hints.picks,
        _ => Vec::new(),
    }
}
