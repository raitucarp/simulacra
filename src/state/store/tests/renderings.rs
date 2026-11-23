//! N5: renderings are versions of the post they render, never new posts, and never of a
//! locked post.

use super::locked::{locked_post, SECRET};
use crate::domain::data::history::{PostRecord, SemVer};
use crate::domain::post::depth::Depth;
use crate::error::AppError;
use crate::state::store::AppState;

fn at(hour: i8) -> jiff::Timestamp {
    crate::time::utc(2026, 11, 23, hour, 0, 0)
}

#[test]
fn a_rendering_is_a_sibling_version_of_the_version_it_renders() {
    let mut state = AppState::new_in_memory();
    let id = state.submit_post("Entropy measures how many arrangements look the same.").expect("posted");
    let posts = state.posts.len();
    let simpler = state.keep_rendering(&id, Depth::Simpler, "Messy rooms have more ways to be messy than tidy.", at(9)).expect("kept");
    let post = state.get_post(&id).expect("post");
    let head = post.extras.head.clone().expect("a stored head");
    assert_eq!((head.version, post.content.as_str()), (SemVer::ORIGINAL, "Entropy measures how many arrangements look the same."), "the post shows what it showed");
    assert_eq!((simpler.parent_id.as_ref(), simpler.version), (Some(&head.id), Some(SemVer::new(1, 0, 1))));
    assert_eq!(PostRecord::parse(&simpler.record_json).and_then(|r| r.rendering), Some(Depth::Simpler));
    let expert = state.keep_rendering(&id, Depth::Expert, "S = k ln W over microstates.", at(10)).expect("kept");
    assert_eq!(expert.version, Some(SemVer::new(1, 0, 2)), "renderings never share a number");
    assert_eq!(state.posts.len(), posts, "no new posts");
    assert_eq!(state.rendering_of(&id, Depth::Simpler).map(|r| r.content), Some("Messy rooms have more ways to be messy than tidy.".to_string()));
    assert_eq!(state.rendering_of(&id, Depth::Expert).map(|r| r.id), Some(expert.id));
}

#[test]
fn renderings_follow_the_version_shown_and_do_not_count_as_edits() {
    let mut state = AppState::new_in_memory();
    let id = state.submit_post("A first take on attention.").expect("posted");
    state.keep_rendering(&id, Depth::Simpler, "Attention, simply.", at(9)).expect("kept");
    let counts = state.db.as_ref().expect("vault").post_revision_counts().expect("counts");
    assert!(counts.iter().all(|(post, _)| post != id.as_str()), "a rendering is no edit: {counts:?}");
    state.edit_post(&id, "A second, rewritten take on attention and where it goes.").expect("edited");
    assert!(state.rendering_of(&id, Depth::Simpler).is_none(), "the new version has no rendering yet");
}

#[test]
fn a_locked_post_is_never_rendered() {
    let mut state = AppState::new_in_memory();
    let id = locked_post(&mut state);
    assert_eq!(state.keep_rendering(&id, Depth::Simpler, "anything", at(9)).map(|_| ()), Err(AppError::PostLocked));
    assert!(state.post_revisions(&id).iter().all(|r| !r.content.contains(SECRET)));
}
