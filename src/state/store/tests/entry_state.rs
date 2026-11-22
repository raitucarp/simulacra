//! State actions from the post menu are patch versions (S7); a parent task is done only
//! once its sub-tasks are settled (S8).

use crate::domain::data::history::Bump;
use crate::domain::symbols::{EntryState, EntrySymbol, PostSymbols};
use crate::domain::{Post, PostId};
use crate::error::AppError;
use crate::state::store::{AppState, ThreadAnchor, ThreadDraft};

fn now() -> jiff::Timestamp {
    crate::time::utc(2026, 11, 2, 9, 0, 0)
}

fn task(state: &mut AppState, text: &str, parent: Option<&PostId>) -> PostId {
    let draft = ThreadDraft { symbols: PostSymbols::entry(EntrySymbol::Task), ..ThreadDraft::text(text) };
    let anchor = ThreadAnchor { reply_to: parent.cloned(), quote_of: None };
    state.submit_thread(&[draft], &anchor).expect("published")[0].clone()
}

fn state_of(state: &AppState, id: &PostId) -> Option<EntryState> {
    state.get_post(id).unwrap().extras.symbols.state
}

#[test]
fn each_state_action_is_a_patch_revision_keeping_the_old_symbols() {
    let mut state = AppState::new_in_memory();
    let id = task(&mut state, "Call the bakery", None);
    let revision = state.set_entry_state(&id, EntryState::Migrated, now()).expect("moved");
    assert_eq!(state.post_head(&id).unwrap().bump, Some(Bump::Patch), "the move is a patch version");
    assert_eq!(revision.content, "Call the bakery", "the text is unchanged");
    assert_eq!(serde_json::from_str::<PostSymbols>(&revision.record_json).unwrap(), PostSymbols::entry(EntrySymbol::Task));
    assert_eq!(state_of(&state, &id), Some(EntryState::Migrated));
    for to in [EntryState::Done, EntryState::Open, EntryState::Scheduled, EntryState::Dropped, EntryState::Open, EntryState::Delegated] {
        state.set_entry_state(&id, to, now()).expect("allowed move");
        assert_eq!(state_of(&state, &id), Some(to));
    }
}

#[test]
fn moves_the_rules_forbid_and_posts_without_entries_are_refused() {
    let mut state = AppState::new_in_memory();
    let id = task(&mut state, "Water the plants", None);
    state.set_entry_state(&id, EntryState::Done, now()).unwrap();
    assert_eq!(state.set_entry_state(&id, EntryState::Migrated, now()), Err(AppError::InvalidSymbols), "done only reopens");
    let plain = state.submit_post("no symbols").unwrap();
    assert_eq!(state.set_entry_state(&plain, EntryState::Done, now()), Err(AppError::InvalidSymbols));
    let agent = state.user_profiles.iter().find(|p| p.is_agent()).cloned().unwrap();
    let mut theirs = Post::create("agent_task", "net1", agent.id.as_str(), "theirs").unwrap();
    theirs.extras.symbols = PostSymbols::entry(EntrySymbol::Task);
    state.commit_post(theirs).unwrap();
    assert_eq!(state.set_entry_state(&PostId::new("agent_task"), EntryState::Done, now()), Err(AppError::NotAPersona(agent.id.to_string())));
}

#[test]
fn a_parent_task_is_done_only_once_every_sub_task_is_done_or_dropped() {
    let mut state = AppState::new_in_memory();
    let parent = task(&mut state, "Move house", None);
    let boxes = task(&mut state, "Pack the boxes", Some(&parent));
    let van = task(&mut state, "Book the van", Some(&parent));
    let note = ThreadDraft { symbols: PostSymbols::entry(EntrySymbol::Note), ..ThreadDraft::text("Ask about the piano") };
    state.submit_thread(&[note], &ThreadAnchor { reply_to: Some(parent.clone()), quote_of: None }).unwrap();
    assert_eq!(state.set_entry_state(&parent, EntryState::Done, now()), Err(AppError::OpenSubtasks(2)));
    state.set_entry_state(&boxes, EntryState::Done, now()).unwrap();
    state.set_entry_state(&van, EntryState::Delegated, now()).unwrap();
    assert_eq!(state.set_entry_state(&parent, EntryState::Done, now()), Err(AppError::OpenSubtasks(1)), "delegated is still pending");
    state.set_entry_state(&van, EntryState::Dropped, now()).unwrap();
    state.set_entry_state(&parent, EntryState::Done, now()).expect("every sub-task settled; notes do not count");
    assert_eq!(state_of(&state, &parent), Some(EntryState::Done));
}
