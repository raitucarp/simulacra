//! JSON checks of the Engine step's editors: request bodies with placeholders, schemas and
//! sample answers, and inserting a placeholder chip into a template.

use crate::agent::template::fill;
use serde_json::Value;

const OPEN: &str = "{{";
const CLOSE: &str = "}}";
/// What every known placeholder becomes when a body is checked.
const PLACEHOLDER_STAND_IN: &str = "null";
const GAP: char = ' ';

/// The parser's complaint about body `template` once every known placeholder (full
/// `{{name}}` tokens) is filled in; unknown placeholders stay and make it invalid.
pub fn body_error(template: &str, placeholders: &[&str]) -> Option<String> {
    let known = |name: &str| placeholders.iter().any(|token| token.trim_start_matches(OPEN).trim_end_matches(CLOSE) == name);
    let (filled, _) = fill(template, |name| known(name).then(|| PLACEHOLDER_STAND_IN.to_string()));
    serde_json::from_str::<Value>(&filled).err().map(|e| e.to_string())
}

/// What is wrong with a JSON text that must hold an object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsonProblem {
    /// It does not parse (the parser's message).
    Syntax(String),
    /// It parses, but not to an object.
    NotAnObject,
}

/// Why `text` is not a JSON object, if it is not.
pub fn object_problem(text: &str) -> Option<JsonProblem> {
    match serde_json::from_str::<Value>(text) {
        Ok(value) if value.is_object() => None,
        Ok(_) => Some(JsonProblem::NotAnObject),
        Err(e) => Some(JsonProblem::Syntax(e.to_string())),
    }
}

/// `template` with `token` appended, separated by a space unless the template is empty or
/// already ends in whitespace.
pub fn insert_token(template: &str, token: &str) -> String {
    if template.is_empty() || template.ends_with(char::is_whitespace) {
        format!("{template}{token}")
    } else {
        format!("{template}{GAP}{token}")
    }
}
