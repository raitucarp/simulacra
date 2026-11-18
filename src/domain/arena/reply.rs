//! Lenient readers of a model's JSON reply, shared by the debate judge, the lab and the
//! belief extraction: the object may sit in a code fence or among prose, a field may go by a
//! common synonym, and numbers may come as text ("0.7", "70%").

use serde_json::Value;

/// Percent sign accepted after a number.
const PERCENT_SIGN: char = '%';
/// Numbers above this are read as percent points.
const UNIT_MAX: f64 = 1.0;
const PERCENT_DIVISOR: f64 = 100.0;

/// The outermost `{…}` of `raw`, parsed.
pub fn json_object(raw: &str) -> Option<Value> {
    let (start, end) = (raw.find('{')?, raw.rfind('}')?);
    (start < end).then(|| serde_json::from_str::<Value>(&raw[start..=end]).ok()).flatten().filter(Value::is_object)
}

/// The first of `names` present in `value`.
pub fn field<'a>(value: &'a Value, names: &[&str]) -> Option<&'a Value> {
    names.iter().find_map(|name| value.get(*name))
}

/// A string (or number) field as trimmed text; `None` when absent or blank.
pub fn text(value: &Value, names: &[&str]) -> Option<String> {
    let text = match field(value, names)? {
        Value::String(text) => text.trim().to_string(),
        Value::Number(number) => number.to_string(),
        _ => return None,
    };
    (!text.is_empty()).then_some(text)
}

/// A number field, given as a number or as text.
pub fn number(value: &Value, names: &[&str]) -> Option<f64> {
    match field(value, names)? {
        Value::Number(number) => number.as_f64(),
        Value::String(text) => text.trim().trim_end_matches(PERCENT_SIGN).trim().parse().ok(),
        _ => None,
    }
}

/// A probability: a unit value, or percent points when above 1 (clamped to 0–1).
pub fn unit_value(value: &Value, names: &[&str]) -> Option<f64> {
    let raw = number(value, names)?;
    let unit = if raw > UNIT_MAX { raw / PERCENT_DIVISOR } else { raw };
    unit.is_finite().then(|| unit.clamp(0.0, UNIT_MAX))
}

/// The array under the first of `names`, or the reply itself when it is an array.
pub fn items<'a>(value: &'a Value, names: &[&str]) -> Vec<&'a Value> {
    match field(value, names) {
        Some(Value::Array(items)) => items.iter().collect(),
        _ => Vec::new(),
    }
}

/// `text` with angle brackets neutralised, so quoted data cannot close the block around it.
pub fn quoted(text: &str) -> String {
    text.replace('<', "‹").replace('>', "›")
}
