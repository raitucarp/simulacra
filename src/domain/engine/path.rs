//! Dotted-path lookups into JSON values (`choices.0.message.content`).

use serde_json::Value;

/// Separator between path segments.
const PATH_SEPARATOR: char = '.';

/// The value at `path`: object keys by name, array items by index. A blank path is the root.
pub fn value_at<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let path = path.trim();
    if path.is_empty() {
        return Some(value);
    }
    path.split(PATH_SEPARATOR).try_fold(value, |current, segment| match current {
        Value::Array(items) => segment.parse::<usize>().ok().and_then(|idx| items.get(idx)),
        Value::Object(map) => map.get(segment),
        _ => None,
    })
}

/// The text at `path`: strings as-is, numbers and booleans printed, `null` and absent as
/// `None`, arrays and objects as compact JSON.
pub fn value_text(value: &Value, path: &str) -> Option<String> {
    match value_at(value, path)? {
        Value::Null => None,
        Value::String(text) => Some(text.clone()),
        other => Some(other.to_string()),
    }
}
