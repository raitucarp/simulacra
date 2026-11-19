//! Reading the AI's reply: the outermost JSON object (code fences and chatter around it are
//! skipped, a `suggestions` wrapper is unwrapped, keys may be paths or field labels), each
//! entry checked against its parameter; whatever does not fit is counted and dropped.

use super::value::{is_multi, validate, SuggestedValue};
use super::super::context::SuggestContext;
use super::super::choice::current_keys;
use super::super::param::Param;
use serde_json::{Map, Value};

const OBJECT_OPEN: char = '{';
const OBJECT_CLOSE: char = '}';
/// A wrapper some models put around the answer.
const WRAPPER: &str = "suggestions";
/// Most values kept per parameter.
pub const MAX_VALUES: usize = 3;

/// The usable values of a reply, per parameter, and how many proposals were dropped.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ParsedSuggestions {
    pub values: Vec<(Param, Vec<SuggestedValue>)>,
    pub discarded: usize,
}

impl ParsedSuggestions {
    pub fn is_empty(&self) -> bool {
        self.values.iter().all(|(_, values)| values.is_empty())
    }
}

/// The outermost `{…}` of `raw`, parsed.
fn object(raw: &str) -> Option<Map<String, Value>> {
    let (start, end) = (raw.find(OBJECT_OPEN)?, raw.rfind(OBJECT_CLOSE)?);
    let parsed: Value = serde_json::from_str(raw.get(start..=end)?).ok()?;
    match parsed {
        Value::Object(map) => match map.get(WRAPPER) {
            Some(Value::Object(inner)) => Some(inner.clone()),
            _ => Some(map),
        },
        _ => None,
    }
}

/// The proposals for one parameter: an array of alternatives, or one bare value. For a
/// multi-select a flat array of names is one alternative.
fn alternatives(param: Param, raw: &Value) -> Vec<Value> {
    match raw {
        Value::Array(items) if is_multi(param) && !items.iter().any(Value::is_array) => vec![raw.clone()],
        Value::Array(items) => items.clone(),
        other => vec![other.clone()],
    }
}

/// The entry of `param`: by path, or by a key equal to it ignoring case and separators.
fn entry<'a>(map: &'a Map<String, Value>, param: Param) -> Option<&'a Value> {
    let squash = |text: &str| text.to_lowercase().replace(['_', ' ', '-', '.'], "");
    map.get(param.path()).or_else(|| map.iter().find(|(key, _)| squash(key) == squash(param.path())).map(|(_, value)| value))
}

/// Whether `value` just repeats what the draft already has.
fn is_current(param: Param, value: &SuggestedValue, ctx: &SuggestContext) -> bool {
    match value {
        SuggestedValue::Option(key) => current_keys(param, ctx.profile) == vec![*key],
        SuggestedValue::Options(keys) => current_keys(param, ctx.profile) == *keys,
        SuggestedValue::Text(_) | SuggestedValue::Number(_) => false,
    }
}

/// Valid, distinct, new values of one parameter, plus how many proposals were dropped.
fn checked(param: Param, raw: &Value, ctx: &SuggestContext) -> (Vec<SuggestedValue>, usize) {
    let proposals = alternatives(param, raw);
    let total = proposals.len();
    let kept = proposals.iter().filter_map(|proposal| validate(param, proposal, ctx)).fold(Vec::new(), |mut kept, value| {
        if !kept.contains(&value) && !is_current(param, &value, ctx) && kept.len() < MAX_VALUES {
            kept.push(value);
        }
        kept
    });
    let discarded = total.saturating_sub(kept.len());
    (kept, discarded)
}

/// The suggestions in `reply` for `targets`; `None` when the reply holds no JSON object.
pub fn parse_suggestions(reply: &str, targets: &[Param], ctx: &SuggestContext) -> Option<ParsedSuggestions> {
    let map = object(reply)?;
    let parsed = targets.iter().filter_map(|param| entry(&map, *param).map(|raw| (*param, checked(*param, raw, ctx)))).fold(
        ParsedSuggestions::default(),
        |mut parsed, (param, (values, discarded))| {
            parsed.discarded += discarded;
            if !values.is_empty() {
                parsed.values.push((param, values));
            }
            parsed
        },
    );
    Some(parsed)
}
