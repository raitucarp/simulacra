//! [`SuggestedValue`]: one value the AI proposed, already checked against its parameter.

use super::super::choice::{option_keys, OFF, ON};
use super::super::context::SuggestContext;
use super::super::model::{RangeUnit, HUNDREDTHS};
use super::super::param::Param;
use super::super::{suggestions_for, EngineParam, FieldSuggestions};
use serde_json::Value;

/// Longest free text accepted from a suggestion.
pub const MAX_TEXT_CHARS: usize = 1200;
/// Words that mean "on" or "off" for a switch.
const ON_WORDS: [&str; 3] = [ON, "true", "yes"];
const OFF_WORDS: [&str; 3] = [OFF, "false", "no"];

/// A validated value for one parameter.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SuggestedValue {
    Text(String),
    Number(u16),
    /// One option key of a single choice.
    Option(&'static str),
    /// Option keys of a multi-select.
    Options(Vec<&'static str>),
}

/// `raw` as a number on the parameter's stored scale (a temperature may come as `0.7`).
fn number(raw: &Value, unit: RangeUnit, valid: (u16, u16)) -> Option<u16> {
    let parsed = match raw {
        Value::Number(number) => number.as_f64()?,
        Value::String(text) => text.trim().trim_end_matches('%').parse::<f64>().ok()?,
        _ => return None,
    };
    let fraction_limit = f64::from(valid.1) / f64::from(HUNDREDTHS);
    let scaled = match unit {
        RangeUnit::Hundredths if parsed <= fraction_limit => parsed * f64::from(HUNDREDTHS),
        // A fraction above the limit is neither a reading (0.75) nor a stored value (75).
        RangeUnit::Hundredths if parsed.fract() != 0.0 => return None,
        _ => parsed,
    };
    let rounded = scaled.round();
    (rounded >= f64::from(valid.0) && rounded <= f64::from(valid.1)).then_some(rounded as u16)
}

/// The option of `param` that `text` names (key or label, any case), if any.
fn option(param: Param, text: &str) -> Option<&'static str> {
    let wanted = text.trim().to_lowercase().replace([' ', '-'], "_");
    let exact = option_keys(param).into_iter().find(|key| *key == wanted);
    let lenient = || match param {
        Param::Trait(field) => field.parse(text).map(|value| value.key()),
        Param::Toggle(_) if ON_WORDS.contains(&wanted.as_str()) => Some(ON),
        Param::Toggle(_) if OFF_WORDS.contains(&wanted.as_str()) => Some(OFF),
        _ => None,
    };
    exact.or_else(lenient)
}

fn text_of(raw: &Value) -> Option<String> {
    match raw {
        Value::String(text) => Some(text.trim().to_string()),
        Value::Bool(flag) => Some(flag.to_string()),
        Value::Number(number) => Some(number.to_string()),
        _ => None,
    }
}

/// A multi-select alternative: an array of option names (or one name).
fn options(param: Param, raw: &Value) -> Option<SuggestedValue> {
    let names: Vec<String> = match raw {
        Value::Array(items) => items.iter().filter_map(text_of).collect(),
        other => vec![text_of(other)?],
    };
    let keys = names.iter().filter_map(|name| option(param, name)).fold(Vec::new(), |mut keys, key| {
        if !keys.contains(&key) {
            keys.push(key);
        }
        keys
    });
    (!keys.is_empty()).then_some(SuggestedValue::Options(keys))
}

/// `raw` as a valid value of `param`, or `None` when it does not fit the parameter.
pub fn validate(param: Param, raw: &Value, ctx: &SuggestContext) -> Option<SuggestedValue> {
    match suggestions_for(param, ctx) {
        FieldSuggestions::Text(_) => text_of(raw)
            .filter(|text| !text.is_empty() && text.chars().count() <= MAX_TEXT_CHARS)
            .map(SuggestedValue::Text),
        FieldSuggestions::Range(hint) => number(raw, hint.unit, hint.valid).map(SuggestedValue::Number),
        FieldSuggestions::Choice(_) if is_multi(param) => options(param, raw),
        FieldSuggestions::Choice(_) => option(param, &text_of(raw)?).map(SuggestedValue::Option),
        FieldSuggestions::Guide(_) => None,
    }
}

/// Whether `param` takes several options at once.
pub fn is_multi(param: Param) -> bool {
    matches!(param, Param::Set(_)) || param == Param::Engine(EngineParam::Tools)
}
