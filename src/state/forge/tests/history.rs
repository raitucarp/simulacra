use super::*;

fn type_into(draft: &mut AgentDraft, field: TextField, text: &str) {
    for end in 1..=text.len() {
        draft.edit(DraftEdit::Text(field, text[..end].to_string()));
    }
}

#[test]
fn undo_and_redo_walk_snapshots() {
    let mut draft = agent();
    draft.edit(DraftEdit::Score(ScoreField::Openness, 10));
    draft.edit(DraftEdit::Score(ScoreField::Openness, 20));
    assert!(draft.undo());
    assert_eq!(draft.profile().config.openness, 10);
    assert!(draft.can_redo());
    assert!(draft.redo());
    assert_eq!(draft.profile().config.openness, 20);
    assert!(!draft.redo());
    assert!(draft.undo() && draft.undo());
    assert!(!draft.is_dirty());
    assert!(!draft.undo());
}

#[test]
fn consecutive_typing_in_one_field_is_one_undo_step() {
    let mut draft = agent();
    type_into(&mut draft, TextField::Name, "Theo");
    assert_eq!(draft.profile().name, "Theo");
    assert!(draft.undo());
    assert_eq!(draft.profile().name, "");
    assert!(!draft.can_undo());
}

#[test]
fn switching_fields_or_sealing_starts_a_new_step() {
    let mut draft = agent();
    type_into(&mut draft, TextField::Name, "Theo");
    type_into(&mut draft, TextField::Handle, "th");
    type_into(&mut draft, TextField::Name, "Theo V");
    draft.seal_history();
    type_into(&mut draft, TextField::Name, "Theo Va");
    assert!(draft.undo());
    assert_eq!(draft.profile().name, "Theo V");
    assert!(draft.undo());
    assert_eq!(draft.profile().name, "Theo");
    assert_eq!(draft.profile().handle, "th");
    assert!(draft.undo());
    assert_eq!(draft.profile().handle, "");
    assert!(draft.undo());
    assert_eq!(draft.profile().name, "");
}

#[test]
fn a_non_text_edit_breaks_the_typing_run() {
    let mut draft = agent();
    type_into(&mut draft, TextField::Bio, "ab");
    draft.edit(DraftEdit::Score(ScoreField::Openness, 1));
    type_into(&mut draft, TextField::Bio, "abc");
    assert!(draft.undo());
    assert_eq!(draft.profile().config.bio, "ab");
}

#[test]
fn undo_after_redo_point_discards_the_redo_branch() {
    let mut draft = agent();
    draft.edit(DraftEdit::Score(ScoreField::Openness, 10));
    draft.undo();
    draft.edit(DraftEdit::Score(ScoreField::Openness, 30));
    assert!(!draft.can_redo());
}

#[test]
fn history_is_bounded() {
    let mut draft = agent();
    let edits = HISTORY_LIMIT + 10;
    for i in 0..edits {
        draft.edit(DraftEdit::Score(ScoreField::Openness, (i % 2) as u8 * 50 + 1));
    }
    let mut undone = 0;
    while draft.undo() {
        undone += 1;
    }
    assert_eq!(undone, HISTORY_LIMIT);
}

#[test]
fn edit_all_is_a_single_undo_step() {
    let mut draft = agent();
    let edits = [DraftEdit::Score(ScoreField::Openness, 1), DraftEdit::Score(ScoreField::Extraversion, 2)];
    assert!(draft.edit_all(&edits));
    assert!(draft.undo());
    assert!(!draft.is_dirty());
    assert!(!draft.edit_all(&[]));
}
