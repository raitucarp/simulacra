//! Tolerant reading of a model's JSON reply: code fences and prose around the object are
//! ignored, lists keep only short non-empty strings.

use serde_json::Value;

const OPEN: char = '{';
const CLOSE: char = '}';

/// The outermost JSON object in `text`, if one parses.
pub fn object_in(text: &str) -> Option<Value> {
    let start = text.find(OPEN)?;
    let end = text.rfind(CLOSE)?;
    (end > start).then(|| serde_json::from_str::<Value>(&text[start..=end]).ok()).flatten().filter(Value::is_object)
}

/// A trimmed string field (none when missing or blank).
pub fn text_field(value: &Value, key: &str) -> Option<String> {
    value.get(key).and_then(Value::as_str).map(str::trim).filter(|text| !text.is_empty()).map(str::to_string)
}

/// Up to `cap` trimmed, distinct strings of a list field, each at most `max_chars` long.
pub fn list_field(value: &Value, key: &str, cap: usize, max_chars: usize) -> Vec<String> {
    let items = value.get(key).and_then(Value::as_array).cloned().unwrap_or_default();
    items
        .iter()
        .filter_map(Value::as_str)
        .map(|item| item.trim().chars().take(max_chars).collect::<String>())
        .filter(|item| !item.is_empty())
        .fold(Vec::new(), |mut kept, item| {
            if kept.len() < cap && !kept.contains(&item) {
                kept.push(item);
            }
            kept
        })
}

/// A key field read case-insensitively (`"Analyze"` reads as `analyze`).
pub fn key_field(value: &Value, key: &str) -> Option<String> {
    text_field(value, key).map(|text| text.to_lowercase())
}
