//! "Fill the rest" is deterministic, fills only what is blank or untouched, keeps the budget
//! and is one undo step.

use super::*;
use crate::state::forge::attributes::is_balanced_build;
use crate::state::forge::{steps_for, ForgeStep, ScoreField};

fn fill(draft: &AgentDraft, step: ForgeStep) -> Vec<DraftEdit> {
    fill_edits(step, &SuggestContext::of(draft), &[], &keys_as_text)
}

#[test]
fn blank_origin_gets_a_name_and_a_handle_from_it() {
    let draft = agent();
    let edits = fill(&draft, ForgeStep::Origin);
    let name = edits.iter().find_map(|edit| match edit {
        DraftEdit::Text(TextField::Name, name) => Some(name.clone()),
        _ => None,
    });
    let name = name.expect("a name");
    let handle = edits.iter().find_map(|edit| match edit {
        DraftEdit::Text(TextField::Handle, handle) => Some(handle.clone()),
        _ => None,
    });
    let expected = crate::state::forge::suggest_handle(&name, draft.id(), &[]);
    assert_eq!(handle.as_deref(), Some(expected.as_str()));
}

#[test]
fn filling_is_deterministic_idempotent_and_one_undo_step() {
    for step in steps_for(ForgeKind::Agent) {
        let mut draft = agent();
        let edits = fill(&draft, *step);
        assert_eq!(edits, fill(&draft, *step), "{step:?} is deterministic");
        if edits.is_empty() {
            continue;
        }
        let before = draft.profile().clone();
        assert!(draft.edit_all(&edits));
        assert!(fill(&draft, *step).is_empty(), "{step:?} fills only once");
        assert!(is_balanced_build(&draft.profile().config));
        assert!(draft.undo());
        assert_eq!(draft.profile(), &before, "{step:?} undoes in one step");
    }
}

#[test]
fn values_the_user_set_are_kept() {
    let mut draft = agent();
    draft.edit(DraftEdit::Text(TextField::Name, "Hypatia".into()));
    draft.edit(DraftEdit::Score(ScoreField::Openness, 13));
    let edits = fill(&draft, ForgeStep::Origin).into_iter().chain(fill(&draft, ForgeStep::Attributes)).collect::<Vec<_>>();
    assert!(!edits.iter().any(|edit| matches!(edit, DraftEdit::Text(TextField::Name, _))));
    assert!(!edits.iter().any(|edit| matches!(edit, DraftEdit::Score(ScoreField::Openness, _))));
}

#[test]
fn blank_texts_take_their_first_example() {
    let mut draft = agent();
    draft.edit(DraftEdit::Text(TextField::Bio, "  ".into()));
    let edits = fill(&draft, ForgeStep::Origin);
    assert!(edits.contains(&DraftEdit::Text(TextField::Bio, keys::example_key("bio", 1))), "{edits:?}");
}

#[test]
fn personas_fill_only_their_own_steps() {
    let draft = persona();
    for step in steps_for(ForgeKind::Persona) {
        for edit in fill(&draft, *step) {
            assert_eq!(edit.step(), *step);
        }
    }
}
