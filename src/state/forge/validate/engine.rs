//! Rules over the agent's engine: keys that are missing, custom requests without a URL or
//! with a body that is not JSON, schemas that do not parse and mappings that miss the schema.
//! Every issue points at its Engine-step section (`engine.<section>`).

use super::engine_parts::{check_image, check_tools, credential, custom_request};
use super::{IssueKind, Severity, ValidationIssue, ARG_VALUE};
use crate::agent::credentials::{CredentialKind, CredentialStore};
use crate::domain::engine::{EngineConfig, OutputMapping, RequestMode, TextModel, REQUEST_PLACEHOLDERS};
use crate::state::forge::engine::{credential_status, EngineSection};
use crate::state::forge::step::ForgeStep;
use serde_json::Value;

const PROPERTIES: &str = "properties";
const PATH_SEPARATOR: char = '.';

/// An issue of `section` on the Engine step.
pub(super) fn issue(section: EngineSection, severity: Severity, kind: IssueKind) -> ValidationIssue {
    ValidationIssue::new(ForgeStep::Engine, section.issue_field(), severity, kind)
}

pub(super) fn check(engine: &EngineConfig, credentials: &CredentialStore, issues: &mut Vec<ValidationIssue>) {
    check_model(engine, credentials, issues);
    if let RequestMode::Custom(request) = &engine.request {
        custom_request(EngineSection::Request, request, &REQUEST_PLACEHOLDERS, issues);
        let status = credential_status(credentials, request.credential.as_deref(), None);
        credential(EngineSection::Request, &status, "", issues);
    }
    check_output(engine, issues);
    check_image(engine, credentials, issues);
    check_tools(engine, credentials, issues);
}

/// The own model's key (only used by native requests).
fn check_model(engine: &EngineConfig, credentials: &CredentialStore, issues: &mut Vec<ValidationIssue>) {
    let (TextModel::Own(own), RequestMode::Native) = (&engine.text, &engine.request) else { return };
    let kind = CredentialKind::from_provider(own.provider);
    let status = credential_status(credentials, own.credential.as_deref(), Some(kind));
    credential(EngineSection::Model, &status, &kind.label(), issues);
}

/// The schema parses, something maps to the post text and every mapped path starts at a
/// schema property.
fn check_output(engine: &EngineConfig, issues: &mut Vec<ValidationIssue>) {
    let Some(structured) = engine.structured() else { return };
    let section = EngineSection::Output;
    let Some(schema) = structured.schema_value() else {
        issues.push(issue(section, Severity::Error, IssueKind::EngineSchemaInvalid));
        return;
    };
    let mapping = &structured.mapping;
    if mapping.content.trim().is_empty() && mapping.thread_parts.trim().is_empty() {
        issues.push(issue(section, Severity::Error, IssueKind::EngineMappingContent));
    }
    let Some(Value::Object(properties)) = schema.get(PROPERTIES) else { return };
    for path in mapped_paths(mapping) {
        let root = path.split(PATH_SEPARATOR).next().unwrap_or_default();
        if !properties.contains_key(root) {
            issues.push(issue(section, Severity::Warning, IssueKind::EngineMappingUnknown).with_arg(ARG_VALUE, path));
        }
    }
}

/// Every non-blank mapping path, trimmed.
fn mapped_paths(mapping: &OutputMapping) -> Vec<&str> {
    [&mapping.content, &mapping.image_prompt, &mapping.hashtags, &mapping.mood, &mapping.thread_parts]
        .into_iter()
        .map(|path| path.trim())
        .filter(|path| !path.is_empty())
        .collect()
}
