//! `{{name}}` placeholder expansion shared by custom requests, custom tools and image
//! requests.

use serde_json::Value;

const OPEN: &str = "{{";
const CLOSE: &str = "}}";

/// Replaces every `{{name}}` whose `lookup(name)` is `Some`; unknown placeholders are kept
/// verbatim. Returns the text and the names that were replaced.
pub fn fill(template: &str, lookup: impl Fn(&str) -> Option<String>) -> (String, Vec<String>) {
    let mut out = String::with_capacity(template.len());
    let mut used = Vec::new();
    let mut rest = template;
    while let Some(start) = rest.find(OPEN) {
        out.push_str(&rest[..start]);
        let after = &rest[start + OPEN.len()..];
        let Some(end) = after.find(CLOSE) else {
            out.push_str(&rest[start..]);
            return (out, used);
        };
        let name = after[..end].trim();
        match lookup(name) {
            Some(value) => {
                out.push_str(&value);
                used.push(name.to_string());
            }
            None => out.push_str(&rest[start..start + OPEN.len() + end + CLOSE.len()]),
        }
        rest = &after[end + CLOSE.len()..];
    }
    out.push_str(rest);
    (out, used)
}

/// A JSON argument as plain text: strings unquoted, everything else as JSON.
pub fn plain(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}
