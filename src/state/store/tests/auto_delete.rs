//! Auto-delete (Y7): the heartbeat deletes a post at its moment, with its revisions and
//! its meaning index vectors.

use crate::agent::schedule::FixedClock;
use crate::domain::data::history::RevisionSubject;
use crate::domain::embedding::{EmbeddingOwner, StoredChunk};
use crate::domain::post::auto_delete::DeleteAfter;
use crate::domain::PostId;
use crate::state::store::AppState;
use crate::storage::{Database, Vault};
use jiff::tz::TimeZone;
use jiff::{SignedDuration, Timestamp};

fn tick(state: &mut AppState, at: Timestamp) -> Vec<PostId> {
    state.lifecycle_heartbeat(&FixedClock(at.to_zoned_datetime(TimeZone::UTC))).expect("tick").deleted
}

fn vector(id: &PostId) -> (EmbeddingOwner, StoredChunk) {
    (EmbeddingOwner::post(id), StoredChunk { index: 0, model: "model".into(), content_hash: "hash".into(), vector: vec![1.0, 0.0] })
}

fn kept(vault: &Vault, id: &PostId) -> (usize, usize) {
    let revisions = vault.revisions_of(RevisionSubject::Post(id.clone())).expect("revisions").len();
    let vectors = vault.load_vector_index().expect("vectors").chunks(&EmbeddingOwner::post(id)).len();
    (revisions, vectors)
}

#[test]
fn a_post_deletes_itself_with_its_revisions_and_vectors() {
    let vault = Vault::spawn(Database::in_memory().expect("db")).expect("vault");
    let mut state = AppState::fallback_seed(Some(vault.clone()));
    let (doomed, stays) = (state.submit_post("short lived").expect("post"), state.submit_post("stays around").expect("post"));
    for id in [&doomed, &stays] {
        state.edit_post(id, "edited once").expect("edit");
    }
    vault.save_embeddings(vec![vector(&doomed), vector(&stays)]).expect("vectors");
    let now = Timestamp::now();
    let at = DeleteAfter::Hour.delete_at(now);
    state.set_auto_delete(&doomed, Some(at)).expect("set");
    assert!(tick(&mut state, at - SignedDuration::from_secs(1)).is_empty(), "not before its moment");
    assert_eq!(tick(&mut state, at), vec![doomed.clone()]);
    assert!(state.get_post(&doomed).is_none() && state.get_post(&stays).is_some());
    assert_eq!(kept(&vault, &doomed), (0, 0), "its revisions and vectors are gone");
    assert_eq!(kept(&vault, &stays), (1, 1), "another post keeps its own");
    assert!(AppState::load_from_db(&vault).expect("reload").get_post(&doomed).is_none());
}

#[test]
fn clearing_keeps_the_post_and_agents_posts_are_refused() {
    let mut state = AppState::new_in_memory();
    let id = state.submit_post("changed my mind").expect("post");
    let at = DeleteAfter::Day.delete_at(Timestamp::now());
    state.set_auto_delete(&id, Some(at)).expect("set");
    state.set_auto_delete(&id, None).expect("clear");
    assert!(tick(&mut state, at + SignedDuration::from_hours(1)).is_empty());
    let agent_post = state.posts.iter().find(|p| !state.author_of(p).is_human_persona()).map(|p| p.id.clone()).expect("seeded agent post");
    assert!(state.set_auto_delete(&agent_post, Some(at)).is_err());
}
