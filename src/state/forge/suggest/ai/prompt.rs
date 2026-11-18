//! The "suggest from context" prompt: the draft so far, then each target with its type and
//! rules (text, a number range, or the allowed option keys) and the JSON shape to answer in.
//! Prompts are written in English; free text is asked for in the interface language.

use super::summary::draft_lines;
use super::value::is_multi;
use super::super::choice::option_keys;
use super::super::context::SuggestContext;
use super::super::param::Param;
use super::super::{suggestions_for, FieldSuggestions, RangeUnit};
use crate::domain::post_actions::ActionPrompt;

const SYSTEM: &str = "You help people configure a character who posts in a small social network of thinkers. \
You propose values that fit everything already chosen and make the character coherent and distinct. \
Reply with one JSON object only, no prose.";
/// Alternatives asked per parameter for one field, and per parameter for a whole step.
const FIELD_ALTERNATIVES: usize = 3;
const STEP_ALTERNATIVES: usize = 2;
/// Answer budget per target parameter, and the least budget of any request.
const TOKENS_PER_TARGET: u32 = 160;
const MIN_ANSWER_TOKENS: u32 = 400;
const KEY_SEPARATOR: &str = ", ";

/// How one target may be answered.
fn rule(param: Param, ctx: &SuggestContext) -> String {
    match suggestions_for(param, ctx) {
        FieldSuggestions::Text(_) => "free text (one value is one complete field value, at most a few sentences)".to_string(),
        FieldSuggestions::Range(hint) => {
            let scale = if hint.unit == RangeUnit::Hundredths { " (hundredths: 70 means 0.70)" } else { "" };
            format!("a whole number from {} to {}{scale}; typical {}–{}", hint.valid.0, hint.valid.1, hint.typical.0, hint.typical.1)
        }
        FieldSuggestions::Choice(_) if is_multi(param) => format!("an array of several of: {}", option_keys(param).join(KEY_SEPARATOR)),
        FieldSuggestions::Choice(_) => format!("exactly one of: {}", option_keys(param).join(KEY_SEPARATOR)),
        FieldSuggestions::Guide(_) => String::new(),
    }
}

/// The prompt asking for values of `targets` on the draft of `ctx`; free text in `language`
/// (an English language name).
pub fn suggest_prompt(targets: &[Param], ctx: &SuggestContext, language: &str) -> ActionPrompt {
    let count = if targets.len() == 1 { FIELD_ALTERNATIVES } else { STEP_ALTERNATIVES };
    let wanted: Vec<String> = targets.iter().map(|param| format!("- {}: {}", param.path(), rule(*param, ctx))).collect();
    let example = targets.first().map_or_else(String::new, |param| format!("{{\"{}\": [ ... ]}}", param.path()));
    let user = format!(
        "The character so far:\n{}\n\nSuggest {count} different values for each of these settings:\n{}\n\n\
Answer as a JSON object mapping each setting to an array of {count} values, like {example}. \
For a multi-choice setting, each value is itself an array. Use option keys exactly as listed. \
Write free text in {language}.",
        draft_lines(ctx.profile, targets).join("\n"),
        wanted.join("\n"),
    );
    ActionPrompt { system: SYSTEM.to_string(), user }
}

/// The answer budget of a request for `targets`.
pub fn answer_tokens(targets: &[Param]) -> u32 {
    (u32::try_from(targets.len()).unwrap_or(u32::MAX).saturating_mul(TOKENS_PER_TARGET)).max(MIN_ANSWER_TOKENS)
}
