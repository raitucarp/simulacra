//! Long technical examples under short translated chip names: prompt templates, custom request
//! bodies, output schemas and tool argument schemas. Prompts are written in English for the
//! model, like every other prompt the app sends.

use super::super::engine_param::EngineParam;
use super::super::keys::named_examples;
use super::super::model::{Example, Phrase};
use crate::domain::engine::SchemaPreset;
use crate::domain::TraitOption;
use crate::state::forge::TextField;

const PROMPT_TEMPLATES: [&str; 4] = [
    "You are {{name}} ({{handle}}), a member of {{network_context}}.\n\n{{all_agent_info}}\n\nCore Directive:\n{{directive}}",
    "You are {{name}} ({{handle}}).\n\nVoice:\n{{linguistics}}\n{{typing_style}}\n\nRhetoric:\n{{rhetoric}}\n\nCore Directive:\n{{directive}}",
    "You are {{name}}, {{description}}. {{bio}}\n\n{{directive}}",
    "You are {{name}} ({{handle}}) in {{network_context}}.\n\nMind:\n{{psychology}}\n{{intelligence}}\n\nWhen someone mentions you:\n{{mention_attitude}}\n\n{{directive}}",
];
const BODIES: [&str; 4] = [
    r#"{"model": {{model}}, "messages": {{messages}}, "tools": {{tools}}}"#,
    r#"{"model": {{model}}, "messages": {{messages}}}"#,
    r#"{"model": {{model}}, "max_tokens": 1024, "system": {{system}}, "messages": [{"role": "user", "content": {{user}}}]}"#,
    r#"{"model": {{model}}, "messages": {{messages}}, "response_format": {"type": "json_schema", "json_schema": {"name": "post", "schema": {{schema}}}}}"#,
];
const PARAMS_SCHEMAS: [&str; 4] = [
    r#"{"type": "object", "properties": {"query": {"type": "string"}}, "required": ["query"]}"#,
    r#"{"type": "object", "properties": {"query": {"type": "string"}, "limit": {"type": "integer", "minimum": 1, "maximum": 10}}, "required": ["query"]}"#,
    r#"{"type": "object", "properties": {"term": {"type": "string"}, "language": {"type": "string"}}, "required": ["term"]}"#,
    r#"{"type": "object", "properties": {}}"#,
];

pub(super) fn prompt_templates() -> Vec<Example> {
    named_examples(TextField::PromptTemplate.path(), &PROMPT_TEMPLATES)
}

pub(super) fn bodies() -> Vec<Example> {
    named_examples(EngineParam::Body.key(), &BODIES)
}

pub(super) fn params_schemas() -> Vec<Example> {
    named_examples(EngineParam::ParamsSchema.key(), &PARAMS_SCHEMAS)
}

/// Every built-in output schema, named like its preset card.
pub(super) fn schemas() -> Vec<Example> {
    SchemaPreset::ALL
        .iter()
        .filter(|preset| **preset != SchemaPreset::Custom)
        .map(|preset| Example::named(Phrase::key(preset.label_key()), Phrase::literal(preset.schema())))
        .collect()
}
