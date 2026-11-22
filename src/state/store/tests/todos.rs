//! Ticking a box keeps the tick in `extras.todo_state` (D6): saved with the post, never a
//! new version, carried across edits; agents' posts cannot be ticked.

use crate::domain::symbols::EntryState;
use crate::domain::{Post, PostId};
use crate::error::AppError;
use crate::state::store::{toggled, AppState, PostEdit};
use crate::storage::{Database, Vault};

const LIST: &str = "Saturday\n[ ] clean the flat\n[ ] buy bread\n[~] call the plumber";

fn with_vault() -> (AppState, Vault) {
    let vault = Vault::spawn(Database::in_memory().expect("in-memory db")).expect("vault");
    (AppState::fallback_seed(Some(vault.clone())), vault)
}

fn states(state: &AppState, id: &PostId) -> Vec<EntryState> {
    state.post_todos(id).iter().map(|entry| entry.state).collect()
}

#[test]
fn a_click_ticks_and_unticks_and_a_dropped_item_only_reopens() {
    assert_eq!(toggled(EntryState::Open), EntryState::Done);
    assert_eq!(toggled(EntryState::Done), EntryState::Open);
    assert_eq!(toggled(EntryState::Delegated), EntryState::Done);
    assert_eq!(toggled(EntryState::Dropped), EntryState::Open);
    let mut state = AppState::new_in_memory();
    let id = state.submit_post(LIST).unwrap();
    assert_eq!(state.toggle_todo(&id, 1), Ok(EntryState::Done));
    assert_eq!(states(&state, &id), vec![EntryState::Open, EntryState::Done, EntryState::Dropped]);
    assert_eq!(state.toggle_todo(&id, 2), Ok(EntryState::Open));
    assert_eq!(state.toggle_todo(&id, 1), Ok(EntryState::Open));
    let post = state.get_post(&id).unwrap();
    assert_eq!(post.content, LIST, "the text keeps its boxes");
    assert_eq!(post.extras.todo_state.ticks.len(), 1, "a tick saying what the box says is dropped");
}

#[test]
fn ticks_are_saved_with_the_post_and_never_make_a_version() {
    let (mut state, vault) = with_vault();
    let id = state.submit_post(LIST).unwrap();
    state.toggle_todo(&id, 0).unwrap();
    state.toggle_todo(&id, 1).unwrap();
    assert!(state.post_revisions(&id).is_empty(), "no version per tick");
    assert!(state.get_post(&id).unwrap().extras.head.is_none());
    let reloaded = AppState::load_from_db(&vault).expect("reload");
    assert_eq!(states(&reloaded, &id), vec![EntryState::Done, EntryState::Done, EntryState::Dropped]);
}

#[test]
fn an_edit_carries_each_tick_to_its_item() {
    let mut state = AppState::new_in_memory();
    let id = state.submit_post(LIST).unwrap();
    state.toggle_todo(&id, 1).unwrap(); // buy bread
    let edited = "Saturday\n[ ] water the plants\n[ ] clean the flat\n[ ] buy bread\n[~] call the plumber";
    state.edit_post_version(&id, &PostEdit::text(edited), jiff::Timestamp::now()).unwrap();
    assert_eq!(states(&state, &id), vec![EntryState::Open, EntryState::Open, EntryState::Done, EntryState::Dropped]);
    let rewritten = "Saturday\n[ ] water the plants\n[x] buy bread";
    state.edit_post_version(&id, &PostEdit::text(rewritten), jiff::Timestamp::now()).unwrap();
    assert!(state.get_post(&id).unwrap().extras.todo_state.is_empty(), "the box now says it, and a removed item's tick goes");
}

#[test]
fn agents_posts_and_missing_items_cannot_be_ticked() {
    let mut state = AppState::new_in_memory();
    let agent = state.user_profiles.iter().find(|p| p.is_agent()).cloned().unwrap();
    state.commit_post(Post::create("agent_list", "net1", agent.id.as_str(), "[ ] theirs").unwrap()).unwrap();
    let theirs = PostId::new("agent_list");
    assert_eq!(state.toggle_todo(&theirs, 0), Err(AppError::NotAPersona(agent.id.to_string())));
    let id = state.submit_post(LIST).unwrap();
    assert!(state.toggle_todo(&id, 9).is_err());
    assert!(state.set_todo_state(&id, 0, EntryState::Done).is_ok());
}

#[test]
fn the_tasks_view_lists_only_the_users_own_to_dos() {
    use crate::domain::todo::tasks::TaskFilter;
    let mut state = AppState::new_in_memory();
    state.submit_post("[ ] mine &desk").unwrap();
    let agent = state.user_profiles.iter().find(|p| p.is_agent()).cloned().unwrap();
    let network = state.active_network_id.clone();
    state.commit_post(Post::create("agent_list", network.as_str(), agent.id.as_str(), "[ ] theirs &lab").unwrap()).unwrap();
    let rows = state.task_rows(&TaskFilter::default(), jiff::civil::date(2026, 11, 22));
    assert_eq!(rows.iter().map(|row| row.entry.item.text.as_str()).collect::<Vec<_>>(), vec!["mine"]);
    assert_eq!(state.task_contexts(), vec!["desk".to_string()]);
}
