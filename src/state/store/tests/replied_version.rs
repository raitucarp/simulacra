//! Replies record the version they answered (V12), and the hint after a major bump.

use crate::domain::data::history::{replied_to_older, Bump, SemVer};
use crate::domain::Post;
use crate::state::store::{AppState, PostEdit, ThreadAnchor, ThreadDraft};

fn at(hour: i8) -> jiff::Timestamp {
    crate::time::utc(2026, 11, 21, hour, 0, 0)
}

fn reply(state: &mut AppState, parent: &crate::domain::PostId, text: &str) -> crate::domain::PostId {
    state.submit_thread(&[ThreadDraft::text(text)], &ThreadAnchor { reply_to: Some(parent.clone()), quote_of: None }).unwrap().remove(0)
}

fn replied(state: &AppState, id: &crate::domain::PostId) -> Option<SemVer> {
    state.get_post(id).unwrap().extras.replied_version
}

#[test]
fn a_reply_notes_the_version_its_parent_shows() {
    let mut state = AppState::new_in_memory();
    let parent = state.submit_post("The mind is a mirror.").unwrap();
    let first = reply(&mut state, &parent, "Is it?");
    assert_eq!(replied(&state, &first), Some(SemVer::ORIGINAL));
    state.edit_post_version(&parent, &PostEdit { bump: Some(Bump::Major), ..PostEdit::text("Attention is the rarest form of generosity.") }, at(9)).unwrap();
    let second = reply(&mut state, &parent, "Agreed.");
    assert_eq!(replied(&state, &second), Some(SemVer::new(2, 0, 0)));
    let root = state.submit_post("No parent").unwrap();
    assert_eq!(replied(&state, &root), None, "a post that replies to nothing notes nothing");
}

#[test]
fn agent_replies_note_the_version_too() {
    let mut state = AppState::new_in_memory();
    let parent = state.submit_post("Habits compound.").unwrap();
    let agent = state.user_profiles.iter().find(|p| p.is_agent()).cloned().expect("a seeded agent");
    let mut answer = Post::create("agent_reply", state.active_network_id.as_str(), agent.id.as_str(), "They do.").unwrap();
    answer.reply_to_id = Some(parent.clone());
    state.insert_reply_after_parent(answer).unwrap();
    assert_eq!(replied(&state, &crate::domain::PostId::new("agent_reply")), Some(SemVer::ORIGINAL));
}

#[test]
fn the_hint_shows_only_after_a_major_bump_of_the_parent() {
    assert_eq!(replied_to_older(Some(SemVer::new(1, 2, 0)), SemVer::new(2, 0, 0)), Some(SemVer::new(1, 2, 0)));
    assert_eq!(replied_to_older(Some(SemVer::new(1, 2, 0)), SemVer::new(1, 9, 3)), None, "minor and patch edits keep the answer current");
    assert_eq!(replied_to_older(Some(SemVer::new(2, 0, 0)), SemVer::new(1, 0, 0)), None, "a parent switched back to an older version");
    assert_eq!(replied_to_older(None, SemVer::new(3, 0, 0)), None, "replies written before versions note none");
}
