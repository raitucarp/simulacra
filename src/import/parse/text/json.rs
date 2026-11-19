//! JSON: a top-level array gives one block per element; an object gives one block per key.
//! Nested values are written compactly as `key: value` pairs; a `title` or `name` string
//! becomes the title.

use crate::import::document::{Block, Section};
use crate::import::parse::{Content, ParseError, ParseResult};
use serde_json::Value;

const TITLE_KEYS: [&str; 2] = ["title", "name"];
const PAIR_SEPARATOR: &str = " · ";

/// A value as readable text: strings bare, objects as `key: value` pairs, arrays comma-joined.
pub fn describe(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::String(text) => text.trim().to_string(),
        Value::Bool(_) | Value::Number(_) => value.to_string(),
        Value::Array(items) => items.iter().map(describe).filter(|t| !t.is_empty()).collect::<Vec<_>>().join(", "),
        Value::Object(map) => map
            .iter()
            .map(|(key, value)| (key, describe(value)))
            .filter(|(_, text)| !text.is_empty())
            .map(|(key, text)| format!("{key}: {text}"))
            .collect::<Vec<_>>()
            .join(PAIR_SEPARATOR),
    }
}

/// Parses JSON bytes.
pub fn parse_json(bytes: &[u8]) -> ParseResult {
    let value: Value = serde_json::from_slice(bytes).map_err(|err| ParseError::Unreadable(err.to_string()))?;
    let texts: Vec<String> = match &value {
        Value::Array(items) => items.iter().map(describe).collect(),
        Value::Object(map) => map.iter().map(|(key, value)| format!("{key}: {}", describe(value))).collect(),
        scalar => vec![describe(scalar)],
    };
    let blocks: Vec<Block> = texts.into_iter().filter(|text| !text.trim().is_empty()).map(Block::paragraph).collect();
    let mut content = Content::default();
    content.metadata.title = TITLE_KEYS
        .iter()
        .find_map(|key| value.get(key).and_then(Value::as_str))
        .map(|title| title.trim().to_string())
        .filter(|title| !title.is_empty());
    if !blocks.is_empty() {
        content.sections.push(Section::new(None, blocks));
    }
    Ok(content)
}
