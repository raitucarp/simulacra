use super::*;
use crate::domain::PromptAssemblyStrategy;
use crate::state::forge::validate::{ARG_MAX, ARG_MIN, ARG_VALUE};

fn kinds(draft: &AgentDraft, existing: &[Profile]) -> Vec<IssueKind> {
    validate(draft, existing).into_iter().map(|issue| issue.kind).collect()
}

fn name_issue(name: &str) -> Vec<IssueKind> {
    let draft = named(agent(), name, "@ok_handle");
    kinds(&draft, &[]).into_iter().filter(|k| k.key().starts_with("name")).collect()
}

fn handle_issue(handle: &str, existing: &[Profile]) -> Vec<IssueKind> {
    let draft = named(agent(), "Theo Vale", handle);
    kinds(&draft, existing).into_iter().filter(|k| k.key().starts_with("handle")).collect()
}

#[test]
fn a_named_default_agent_is_valid() {
    let draft = named(agent(), "Theo Vale", "@theo");
    let issues = validate(&draft, &[]);
    assert!(issues.is_empty(), "{issues:?}");
    assert!(can_save(&issues));
}

#[test]
fn name_rules() {
    assert_eq!(name_issue(""), vec![IssueKind::NameRequired]);
    assert_eq!(name_issue("   "), vec![IssueKind::NameRequired]);
    assert_eq!(name_issue("T"), vec![IssueKind::NameTooShort]);
    assert_eq!(name_issue(&"Theo".repeat(13)), vec![IssueKind::NameTooLong]);
    assert_eq!(name_issue("aaaa"), vec![IssueKind::NameJunk]);
    assert_eq!(name_issue("Z z Z"), vec![IssueKind::NameJunk]);
    assert_eq!(name_issue("1234"), vec![IssueKind::NameJunk]);
    assert!(name_issue("Al").is_empty());
    assert!(name_issue("Ibn Khaldun").is_empty());
    assert!(name_issue("老子").is_empty());
}

#[test]
fn name_issues_carry_their_bounds() {
    let draft = named(agent(), "T", "@ok");
    let issue = validate(&draft, &[]).into_iter().find(|i| i.kind == IssueKind::NameTooShort).expect("short");
    assert_eq!(issue.step, ForgeStep::Origin);
    assert_eq!(issue.field, "name");
    assert_eq!(issue.severity, Severity::Error);
    assert!(issue.args.contains(&(ARG_MIN, NAME_MIN.to_string())));
    assert!(issue.args.iter().any(|(name, _)| *name == ARG_MAX));
    assert_eq!(issue.message_key(), "forge.issue.name_too_short");
}

const NAME_MIN: usize = crate::state::forge::validate::NAME_MIN_CHARS;

#[test]
fn handle_rules_match_the_editor_field() {
    assert_eq!(handle_issue("", &[]), vec![IssueKind::HandleRequired]);
    assert_eq!(handle_issue("@", &[]), vec![IssueKind::HandleRequired]);
    assert!(handle_issue("theo", &[]).is_empty(), "the @ is optional");
    assert!(handle_issue("@theo_2", &[]).is_empty());
    assert_eq!(handle_issue("@@theo", &[]), vec![IssueKind::HandleFormat]);
    assert_eq!(handle_issue("@theo vale", &[]), vec![IssueKind::HandleFormat]);
    assert_eq!(handle_issue("@théo", &[]), vec![IssueKind::HandleFormat]);
    assert_eq!(handle_issue("@t", &[]), vec![IssueKind::HandleLength]);
    assert_eq!(handle_issue(&"t".repeat(crate::state::forge::validate::HANDLE_MAX_CHARS + 1), &[]), vec![IssueKind::HandleLength]);
}

#[test]
fn handles_are_unique_case_insensitively_except_for_yourself() {
    let others = [existing("agent_other", "Other", "@Theo")];
    assert_eq!(handle_issue("theo", &others), vec![IssueKind::HandleTaken]);
    let taken = validate(&named(agent(), "Theo Vale", "THEO"), &others);
    assert!(taken[0].args.contains(&(ARG_VALUE, "@THEO".to_string())));
    let mut draft = AgentDraft::edit_existing(others[0].clone());
    draft.edit(DraftEdit::Text(TextField::Name, "Other Name".into()));
    assert!(validate(&draft, &others).is_empty(), "a profile does not clash with itself");
}

#[test]
fn suggest_handle_finds_a_free_valid_handle() {
    let own = crate::domain::ProfileId::new("agent_me");
    let others = [existing("a", "A", "@theo_vale"), existing("b", "B", "@theo_vale_2")];
    assert_eq!(suggest_handle("Theo Vale", &own, &[]), "@theo_vale");
    assert_eq!(suggest_handle("Theo Vale", &own, &others), "@theo_vale_3");
    assert_eq!(suggest_handle("Jean-Luc O.", &own, &[]), "@jean_luc_o");
    assert_eq!(suggest_handle("老子", &own, &[]), "@thinker");
    let long = suggest_handle(&"x".repeat(80), &own, &[]);
    assert_eq!(long.len(), crate::state::forge::validate::HANDLE_MAX_CHARS + 1);
}

#[test]
fn language_is_required() {
    let mut draft = named(agent(), "Theo Vale", "@theo");
    draft.edit(DraftEdit::Text(TextField::PrimaryLanguage, "  ".into()));
    let issues = validate(&draft, &[]);
    assert_eq!(issues.len(), 1);
    assert_eq!((issues[0].kind, issues[0].step), (IssueKind::LanguageRequired, ForgeStep::Voice));
    assert!(!can_save(&issues));
}

#[test]
fn schedule_checks_apply_to_agents_only() {
    let mut profile = existing("agent_h", "Night Owl", "@owl");
    profile.config.active_hours_start = 40;
    let draft = AgentDraft::edit_existing(profile.clone());
    assert!(kinds(&draft, &[]).contains(&IssueKind::HoursOutOfRange));
    let mut same = named(agent(), "Theo Vale", "@theo");
    same.edit(DraftEdit::Hour(HourField::ActiveHoursStart, 9));
    same.edit(DraftEdit::Hour(HourField::ActiveHoursEnd, 9));
    let issues = validate(&same, &[]);
    assert_eq!(issues[0].kind, IssueKind::HoursAroundTheClock);
    assert!(can_save(&issues), "warnings never block saving");
    let mut alt = Profile::new_alt("user_h", "Night Owl", "@owl");
    alt.config.active_hours_start = 40;
    assert!(validate(&AgentDraft::edit_existing(alt), &[]).is_empty(), "personas skip Daily Life");
}

#[test]
fn custom_templates_must_use_known_placeholders() {
    let mut draft = named(agent(), "Theo Vale", "@theo");
    draft.edit(DraftEdit::Trait(TraitValue::AssemblyStrategy(PromptAssemblyStrategy::CustomTemplate)));
    assert!(validate(&draft, &[]).is_empty(), "the default template is valid");
    draft.edit(DraftEdit::Text(TextField::PromptTemplate, "Hi {{name}} {{nmae}} {{mood".into()));
    let issues = validate(&draft, &[]);
    let unknown = issues.iter().find(|i| i.kind == IssueKind::TemplateUnknownPlaceholder).expect("unknown");
    assert_eq!(unknown.args, vec![(ARG_VALUE, "{{nmae}}".to_string())]);
    assert_eq!(unknown.step, ForgeStep::Grimoire);
    assert!(issues.iter().any(|i| i.kind == IssueKind::TemplateUnclosed && i.severity == Severity::Warning));
    draft.edit(DraftEdit::Text(TextField::PromptTemplate, " ".into()));
    assert!(kinds(&draft, &[]).contains(&IssueKind::TemplateEmpty));
    draft.edit(DraftEdit::Trait(TraitValue::AssemblyStrategy(PromptAssemblyStrategy::FullHumanSynthesis)));
    assert!(validate(&draft, &[]).is_empty(), "templates only matter for the custom strategy");
}

#[test]
fn soft_warnings_for_bio_and_age() {
    let mut draft = named(agent(), "Theo Vale", "@theo");
    draft.edit(DraftEdit::Text(TextField::Bio, String::new()));
    draft.edit(DraftEdit::Count(CountField::Age, 5));
    let issues = validate(&draft, &[]);
    assert_eq!(kinds(&draft, &[]), vec![IssueKind::BioEmpty, IssueKind::AgeUnusual]);
    assert!(issues.iter().all(|i| i.severity == Severity::Warning));
    assert_eq!(issues_for_step(&issues, ForgeStep::Origin).len(), 2);
    assert!(issues_for_step(&issues, ForgeStep::Voice).is_empty());
}

#[test]
fn every_issue_kind_has_a_distinct_key() {
    let keys: std::collections::HashSet<_> = IssueKind::ALL.iter().map(|k| k.key()).collect();
    assert_eq!(keys.len(), IssueKind::ALL.len());
}
