//! Engine rules shared by sections (credentials, raw requests) and those of the image engine
//! and the tools.

use super::engine::issue;
use super::{IssueKind, Severity, ValidationIssue, ARG_VALUE};
use crate::agent::credentials::CredentialStore;
use crate::domain::engine::{CustomRequest, EngineConfig, HttpToolDef, ImageEngine, ImageTrigger, ToolGrant, IMAGE_PLACEHOLDERS};
use crate::state::forge::engine::{body_error, credential_status, image_credential_kind, search_credential_kind, CredentialStatus, EngineSection};
use serde_json::Value;
use std::collections::HashSet;


/// A warning when the key for `service` is missing or the chosen one was deleted.
pub(super) fn credential(section: EngineSection, status: &CredentialStatus, service: &str, issues: &mut Vec<ValidationIssue>) {
    let kind = match status {
        CredentialStatus::Missing => IssueKind::EngineCredentialMissing,
        CredentialStatus::Gone => IssueKind::EngineCredentialGone,
        CredentialStatus::Ready { .. } | CredentialStatus::NotNeeded => return,
    };
    issues.push(issue(section, Severity::Warning, kind).with_arg(ARG_VALUE, service));
}

/// A raw request has a URL and (when it sends one) a JSON body.
pub(super) fn custom_request(section: EngineSection, request: &CustomRequest, placeholders: &[&str], issues: &mut Vec<ValidationIssue>) {
    if request.url.trim().is_empty() {
        issues.push(issue(section, Severity::Error, IssueKind::EngineUrlEmpty));
    }
    if request.method.has_body() {
        if let Some(error) = body_error(&request.body_template, placeholders) {
            issues.push(issue(section, Severity::Error, IssueKind::EngineBodyInvalid).with_arg(ARG_VALUE, error));
        }
    }
}

/// The image engine can be reached and its trigger can fire.
pub(super) fn check_image(engine: &EngineConfig, credentials: &CredentialStore, issues: &mut Vec<ValidationIssue>) {
    let (image, section) = (&engine.image, EngineSection::Image);
    match &image.engine {
        ImageEngine::None => return,
        ImageEngine::LocalSd { url, .. } if url.trim().is_empty() => {
            issues.push(issue(section, Severity::Error, IssueKind::EngineUrlEmpty));
        }
        ImageEngine::Custom(request) => custom_request(section, request, &IMAGE_PLACEHOLDERS, issues),
        _ => {}
    }
    let kind = image_credential_kind(&image.engine);
    let status = credential_status(credentials, image.credential.as_deref(), kind);
    credential(section, &status, &kind.map(|k| k.label()).unwrap_or_default(), issues);
    let asks = engine.structured().is_some_and(|s| !s.mapping.image_prompt.trim().is_empty());
    if image.trigger == ImageTrigger::WhenModelAsks && !asks {
        issues.push(issue(section, Severity::Warning, IssueKind::EngineImageNeverAsked));
    }
}

/// Search keys exist and every custom tool is complete, with a unique name.
pub(super) fn check_tools(engine: &EngineConfig, credentials: &CredentialStore, issues: &mut Vec<ValidationIssue>) {
    let mut names = HashSet::new();
    for grant in &engine.tools {
        match grant {
            ToolGrant::WebSearch(search) => {
                if let Some(kind) = search_credential_kind(search.backend) {
                    let status = credential_status(credentials, search.credential.as_deref(), Some(kind));
                    credential(EngineSection::Tools, &status, &kind.label(), issues);
                }
            }
            ToolGrant::CustomHttp(def) => check_custom_tool(def, credentials, &mut names, issues),
            _ => {}
        }
    }
}

fn check_custom_tool(def: &HttpToolDef, credentials: &CredentialStore, names: &mut HashSet<String>, issues: &mut Vec<ValidationIssue>) {
    let section = EngineSection::Tools;
    let name = def.name.trim();
    if name.is_empty() || !names.insert(name.to_lowercase()) {
        issues.push(issue(section, Severity::Error, IssueKind::EngineToolName).with_arg(ARG_VALUE, name));
    }
    if def.url_template.trim().is_empty() {
        issues.push(issue(section, Severity::Error, IssueKind::EngineUrlEmpty));
    }
    if serde_json::from_str::<Value>(&def.params_schema).ok().filter(Value::is_object).is_none() {
        issues.push(issue(section, Severity::Error, IssueKind::EngineToolParams).with_arg(ARG_VALUE, name));
    }
    credential(section, &credential_status(credentials, def.credential.as_deref(), None), "", issues);
}
