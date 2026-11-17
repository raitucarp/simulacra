//! The live preview of a structured answer: a sample JSON answer built from the schema (with
//! localized sample texts) and the post that answer maps to.

use crate::agent::runtime::PostDraft;
use crate::domain::engine::{OutputMapping, StructuredOutput};
use serde_json::{json, Map, Value};

const PROPERTIES: &str = "properties";
const TYPE: &str = "type";
const ITEMS: &str = "items";
const ARRAY: &str = "array";
const NUMBER: &str = "number";
const INTEGER: &str = "integer";
const BOOLEAN: &str = "boolean";
const OBJECT: &str = "object";

/// Localized sample values for the well-known answer fields.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SampleTexts {
    pub content: String,
    pub image_prompt: String,
    pub mood: String,
    pub hashtags: Vec<String>,
    pub thread_parts: Vec<String>,
    /// Any other string field.
    pub other: String,
}

/// Why the preview could not be drawn.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreviewError {
    /// The sample is not JSON (the parser's message).
    InvalidJson(String),
    /// The mapping finds no post text in the sample.
    NothingToPost,
}

/// A pretty-printed sample answer with one value per schema property (the well-known fields
/// get the localized samples). An invalid schema yields an empty object.
pub fn sample_answer(structured: &StructuredOutput, texts: &SampleTexts) -> String {
    let properties = structured.schema_value().and_then(|schema| schema.get(PROPERTIES).cloned());
    let mut answer = Map::new();
    if let Some(Value::Object(properties)) = properties {
        for (name, property) in properties {
            let value = sample_value(&name, &property, texts);
            answer.insert(name, value);
        }
    }
    serde_json::to_string_pretty(&Value::Object(answer)).unwrap_or_default()
}

/// The localized sample of a well-known answer field (the top-level mapping's names).
fn well_known(name: &str, texts: &SampleTexts) -> Option<Value> {
    let keys = OutputMapping::top_level();
    let known = [
        (keys.content, json!(texts.content)),
        (keys.image_prompt, json!(texts.image_prompt)),
        (keys.mood, json!(texts.mood)),
        (keys.hashtags, json!(texts.hashtags)),
        (keys.thread_parts, json!(texts.thread_parts)),
    ];
    known.into_iter().find(|(key, _)| key == name).map(|(_, value)| value)
}

/// A sample for one property, by its well-known name first, then by its JSON type.
fn sample_value(name: &str, property: &Value, texts: &SampleTexts) -> Value {
    if let Some(value) = well_known(name, texts) {
        return value;
    }
    match property.get(TYPE).and_then(Value::as_str).unwrap_or_default() {
        ARRAY => {
            let item = property.get(ITEMS).map_or(Value::Null, |items| sample_value("", items, texts));
            Value::Array(vec![item])
        }
        NUMBER | INTEGER => json!(1),
        BOOLEAN => json!(true),
        OBJECT => json!({}),
        _ => json!(texts.other),
    }
}

/// The post `sample` maps to through the structured output's mapping.
pub fn preview(structured: &StructuredOutput, sample: &str) -> Result<PostDraft, PreviewError> {
    let answer: Value = serde_json::from_str(sample).map_err(|e| PreviewError::InvalidJson(e.to_string()))?;
    PostDraft::from_structured(&answer, &structured.mapping).ok_or(PreviewError::NothingToPost)
}
