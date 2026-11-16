use super::template::insert_placeholder;
use crate::domain::agent_config::TEMPLATE_PLACEHOLDERS;
use crate::domain::PromptAssemblyStrategy;
use crate::state::forge::{validate, AgentDraft, DraftEdit, ForgeKind, TextField, TraitValue};

const NETWORK: &str = "net_test";

#[test]
fn placeholders_are_appended_with_one_separating_space() {
    assert_eq!(insert_placeholder("", "{{name}}"), "{{name}}");
    assert_eq!(insert_placeholder("You are", "{{name}}"), "You are {{name}}");
    assert_eq!(insert_placeholder("You are ", "{{name}}"), "You are {{name}}");
    assert_eq!(insert_placeholder("Bio:\n", "{{bio}}"), "Bio:\n{{bio}}");
}

#[test]
fn a_template_built_from_chips_passes_validation() {
    let mut draft = AgentDraft::new(ForgeKind::Agent, NETWORK);
    let template = TEMPLATE_PLACEHOLDERS.iter().fold(String::new(), |text, chip| insert_placeholder(&text, chip));
    draft.edit(DraftEdit::Trait(TraitValue::AssemblyStrategy(PromptAssemblyStrategy::CustomTemplate)));
    draft.edit(DraftEdit::Text(TextField::PromptTemplate, template));
    let issues = validate(&draft, &[]);
    assert!(issues.iter().all(|issue| issue.field != "prompt_template"), "{issues:?}");
}
