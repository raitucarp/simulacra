//! Edits are versions (V3): the bump suggested by the changes or picked by the user, the
//! edit note, and the version each kept revision holds.

use crate::domain::data::history::{Bump, SemVer};
use crate::domain::PostId;
use crate::state::store::{AppState, PostEdit};

fn at(hour: i8) -> jiff::Timestamp {
    crate::time::utc(2026, 11, 21, hour, 0, 0)
}

fn own_post(state: &mut AppState, text: &str) -> PostId {
    state.submit_post(text).expect("published")
}

fn head_version(state: &AppState, id: &PostId) -> SemVer {
    state.get_post(id).and_then(|post| post.extras.head.clone()).expect("a head").version
}

#[test]
fn each_edit_is_numbered_by_the_bump_its_changes_suggest() {
    let mut state = AppState::new_in_memory();
    let id = own_post(&mut state, "Teh mind is a mirror.");
    assert!(state.get_post(&id).unwrap().extras.head.is_none(), "an unedited post is its original");
    let steps = [("The mind is a mirror.", SemVer::new(1, 0, 1)), ("The mind is a mirror. It shows what we bring to it, not what is there.", SemVer::new(1, 1, 0)), ("Attention is the rarest form of generosity.", SemVer::new(2, 0, 0))];
    for (hour, (text, expected)) in steps.into_iter().enumerate() {
        state.edit_post_version(&id, &PostEdit::text(text), at(9 + hour as i8)).expect("saved").expect("changed");
        assert_eq!(head_version(&state, &id), expected, "{text}");
    }
    let mut kept = state.post_revisions(&id);
    kept.reverse();
    let numbers: Vec<SemVer> = kept.iter().filter_map(|r| r.version).collect();
    assert_eq!(numbers, vec![SemVer::new(1, 0, 0), SemVer::new(1, 0, 1), SemVer::new(1, 1, 0)], "each revision keeps the number its text had");
    assert_eq!(kept[0].content, "Teh mind is a mirror.");
    assert_eq!((kept[1].bump, kept[2].bump), (Bump::Patch, Bump::Minor));
    assert_eq!(kept[1].parent_id.as_ref(), Some(&kept[0].id), "each version is edited from the one before");
}

#[test]
fn a_picked_bump_and_a_note_are_kept_with_the_version() {
    let mut state = AppState::new_in_memory();
    let id = own_post(&mut state, "Habits compound.");
    let edit = PostEdit { content: "Habits compound!".into(), bump: Some(Bump::Major), note: "  reworded the claim ".into(), base: None };
    state.edit_post_version(&id, &edit, at(9)).unwrap();
    let head = state.post_head(&id).unwrap();
    assert_eq!((head.version, head.bump, head.note.as_str(), head.edited_at), (SemVer::new(2, 0, 0), Some(Bump::Major), "reworded the claim", Some(at(9))));
    state.edit_post_version(&id, &PostEdit::text("Habits compound, slowly."), at(10)).unwrap();
    let previous = state.post_revisions(&id).remove(0);
    assert_eq!((previous.version, previous.note.as_str(), previous.edited_at), (Some(SemVer::new(2, 0, 0)), "reworded the claim", Some(at(9))));
    assert_eq!(state.post_head(&id).unwrap().parent, Some(previous.id));
}

#[test]
fn unchanged_text_makes_no_version_and_the_edit_base_is_the_version_shown() {
    let mut state = AppState::new_in_memory();
    let id = own_post(&mut state, "Habits compound.");
    assert_eq!(state.edit_post_version(&id, &PostEdit::text(" Habits compound. "), at(9)), Ok(None));
    let base = state.edit_base(&id, None).expect("a base");
    assert_eq!((base.text.as_str(), base.version, base.revision, base.taken), ("Habits compound.", SemVer::ORIGINAL, None, vec![SemVer::ORIGINAL]));
    assert!(state.edit_base(&PostId::new("missing"), None).is_none());
}

#[test]
fn editing_an_older_version_starts_a_branch_and_moves_the_head() {
    let mut state = AppState::new_in_memory();
    let id = own_post(&mut state, "Reading slowly is a form of respect.");
    state.edit_post_version(&id, &PostEdit::text("Reading slowly, with a pencil in hand, is a form of respect for the author."), at(9)).unwrap();
    let main_line = state.post_head(&id).unwrap();
    let original = state.post_revisions(&id).remove(0);
    let base = state.edit_base(&id, Some(&original.id)).expect("the original as a base");
    assert_eq!((base.text.as_str(), base.version, base.revision.as_ref()), (original.content.as_str(), SemVer::ORIGINAL, Some(&original.id)));
    assert_eq!(base.next(Bump::Minor), SemVer::new(1, 2, 0), "1.1.0 is taken by the main line");
    let branch = PostEdit { content: "Reading slowly is a form of respect for the reader too.".into(), bump: Some(Bump::Minor), note: "another take".into(), base: Some(original.id.clone()) };
    state.edit_post_version(&id, &branch, at(10)).unwrap().expect("a branch");
    let head = state.post_head(&id).unwrap();
    assert_eq!((head.version, head.parent.as_ref()), (SemVer::new(1, 2, 0), Some(&original.id)), "the head points at the branch, edited from the original");
    assert_eq!(state.get_post(&id).unwrap().content, "Reading slowly is a form of respect for the reader too.");
    let kept = state.post_revisions(&id);
    let leaf = kept.iter().find(|r| r.id == main_line.id).expect("the main line is kept under its own id");
    assert_eq!((leaf.version, leaf.parent_id.as_ref()), (Some(SemVer::new(1, 1, 0)), Some(&original.id)));
    assert_eq!(kept.len(), 2);
    let unknown = PostEdit { base: Some(crate::domain::data::history::RevisionId::new("nope")), ..PostEdit::text("x") };
    assert!(state.edit_post_version(&id, &unknown, at(11)).is_err());
}
