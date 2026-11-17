//! JSON-schema adaptation for Gemini, whose schemas are an OpenAPI subset.

use serde_json::{Map, Value};

/// Schema keywords Gemini accepts; everything else (`$schema`, `additionalProperties`,
/// `title`, …) is dropped.
const GEMINI_SCHEMA_KEYS: [&str; 10] =
    ["type", "format", "description", "nullable", "enum", "properties", "required", "items", "minItems", "maxItems"];

/// `schema` restricted to the keywords Gemini understands, recursively.
pub(crate) fn gemini_schema(schema: &Value) -> Value {
    let Value::Object(map) = schema else { return schema.clone() };
    let mut out = Map::new();
    for (key, value) in map.iter().filter(|(key, _)| GEMINI_SCHEMA_KEYS.contains(&key.as_str())) {
        let adapted = match key.as_str() {
            "properties" => properties(value),
            "items" => gemini_schema(value),
            _ => value.clone(),
        };
        out.insert(key.clone(), adapted);
    }
    Value::Object(out)
}

fn properties(value: &Value) -> Value {
    let Value::Object(props) = value else { return value.clone() };
    Value::Object(props.iter().map(|(name, prop)| (name.clone(), gemini_schema(prop))).collect())
}
