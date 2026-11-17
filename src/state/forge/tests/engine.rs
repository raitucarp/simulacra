//! The Engine step: where it sits, how its edits undo, and what its validation reports.

use super::*;
use crate::agent::credentials::{Credential, CredentialKind, CredentialStore};
use crate::domain::engine::{
    CustomRequest, EngineConfig, HttpToolDef, ImageEngine, OutputMode, OwnModel, RequestMode, StructuredOutput, TextModel, ToolGrant,
    WebSearchConfig, SearchBackend,
};
use crate::agent::llm::LlmProvider;

fn engine_issues(engine: EngineConfig, credentials: &CredentialStore) -> Vec<(IssueKind, Severity, &'static str)> {
    let mut draft = named(agent(), "Theo Vale", "@theo");
    draft.edit(DraftEdit::Engine(Box::new(engine), None));
    validate_with(&draft, &[], credentials)
        .into_iter()
        .filter(|issue| issue.step == ForgeStep::Engine)
        .map(|issue| (issue.kind, issue.severity, issue.field))
        .collect()
}

#[test]
fn the_engine_step_follows_the_grimoire_for_agents_only() {
    assert_eq!(ForgeStep::Grimoire.next(ForgeKind::Agent), Some(ForgeStep::Engine));
    assert_eq!(ForgeStep::Engine.next(ForgeKind::Agent), Some(ForgeStep::Review));
    assert!(!steps_for(ForgeKind::Persona).contains(&ForgeStep::Engine));
    assert_eq!(field_step("engine"), Some(ForgeStep::Engine));
    assert_eq!(ForgeStep::Engine.banner(), "illustrations/steps/engine.svg");
}

#[test]
fn typing_in_one_engine_section_merges_into_one_undo_step() {
    let mut draft = agent();
    for url in ["h", "ht", "http"] {
        let engine = EngineConfig { request: RequestMode::Custom(CustomRequest { url: url.into(), ..CustomRequest::default() }), ..EngineConfig::default() };
        draft.edit(DraftEdit::Engine(Box::new(engine), Some(EngineSection::Request)));
    }
    assert!(draft.undo());
    assert!(draft.profile().config.engine.is_classic(), "one undo restores the engine before typing");
    assert!(!draft.can_undo());
}

#[test]
fn the_classic_engine_has_no_issues() {
    assert!(engine_issues(EngineConfig::default(), &CredentialStore::default()).is_empty());
}

#[test]
fn own_models_need_a_key() {
    let engine = EngineConfig { text: TextModel::Own(OwnModel::for_provider(LlmProvider::OpenAi)), ..EngineConfig::default() };
    let issues = engine_issues(engine.clone(), &CredentialStore::default());
    assert_eq!(issues, vec![(IssueKind::EngineCredentialMissing, Severity::Warning, "engine.model")]);
    let store = CredentialStore { items: vec![Credential::new(CredentialKind::OpenAi, "k", "sk-abcdefghijk", "")] };
    assert!(engine_issues(engine, &store).is_empty());
}

#[test]
fn custom_requests_need_a_url_and_a_json_body() {
    let request = CustomRequest { body_template: r#"{"x": {{user}}"#.into(), credential: Some("cred_gone".into()), ..CustomRequest::default() };
    let issues = engine_issues(EngineConfig { request: RequestMode::Custom(request), ..EngineConfig::default() }, &CredentialStore::default());
    let kinds: Vec<IssueKind> = issues.iter().map(|(kind, ..)| *kind).collect();
    assert_eq!(kinds, vec![IssueKind::EngineUrlEmpty, IssueKind::EngineBodyInvalid, IssueKind::EngineCredentialGone]);
    assert!(issues.iter().all(|(_, _, field)| *field == "engine.request"));
}

#[test]
fn schemas_and_mappings_are_checked() {
    let broken = StructuredOutput { schema: "{ not json".into(), ..StructuredOutput::default() };
    let issues = engine_issues(EngineConfig { output: OutputMode::JsonSchema(broken), ..EngineConfig::default() }, &CredentialStore::default());
    assert_eq!(issues, vec![(IssueKind::EngineSchemaInvalid, Severity::Error, "engine.output")]);
    let mut stray = StructuredOutput::default();
    stray.mapping.content = String::new();
    stray.mapping.thread_parts = String::new();
    stray.mapping.mood = "feeling.now".into();
    let issues = engine_issues(EngineConfig { output: OutputMode::JsonSchema(stray), ..EngineConfig::default() }, &CredentialStore::default());
    let kinds: Vec<IssueKind> = issues.iter().map(|(kind, ..)| *kind).collect();
    assert!(kinds.contains(&IssueKind::EngineMappingContent));
    assert!(kinds.contains(&IssueKind::EngineMappingUnknown), "{kinds:?}");
}

#[test]
fn image_engines_and_tools_are_checked() {
    let mut engine = EngineConfig::default();
    engine.image.engine = ImageEngine::LocalSd { flavor: Default::default(), url: " ".into() };
    engine.tools = vec![
        ToolGrant::WebSearch(WebSearchConfig { backend: SearchBackend::Brave, credential: None }),
        ToolGrant::CustomHttp(HttpToolDef { name: "lookup".into(), url_template: "http://x".into(), ..HttpToolDef::default() }),
        ToolGrant::CustomHttp(HttpToolDef { name: "Lookup".into(), params_schema: "[]".into(), ..HttpToolDef::default() }),
    ];
    let kinds: Vec<IssueKind> = engine_issues(engine, &CredentialStore::default()).into_iter().map(|(kind, ..)| kind).collect();
    for wanted in [
        IssueKind::EngineUrlEmpty,
        IssueKind::EngineImageNeverAsked,
        IssueKind::EngineCredentialMissing,
        IssueKind::EngineToolName,
        IssueKind::EngineToolParams,
    ] {
        assert!(kinds.contains(&wanted), "{wanted:?} in {kinds:?}");
    }
    assert!(can_save(&[]));
}
