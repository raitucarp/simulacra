//! N1: backlinks follow every insert, edit and removal, and a locked post's sealed embeds
//! never refer to anything.

use super::locked::{locked_post, password, FAST};
use crate::domain::post::refs::{Backlink, RefKind};
use crate::domain::PostId;
use crate::state::store::{AppState, ThreadAnchor, ThreadDraft};

fn kinds(state: &AppState, id: &PostId) -> Vec<(PostId, RefKind)> {
    state.backlinks(id).into_iter().map(|Backlink { from, kind }| (from, kind)).collect()
}

#[test]
fn backlinks_follow_writes_edits_and_deletes_newest_first() {
    let mut state = AppState::new_in_memory();
    let target = state.submit_post("The map is not the territory.").expect("target");
    let embeds = state.submit_post(&format!("Worth keeping: (({target}))")).expect("embeds");
    let links = state.submit_post(&format!("Compare [[{target}]]")).expect("links");
    assert_eq!(kinds(&state, &target), vec![(links.clone(), RefKind::Link), (embeds.clone(), RefKind::Embed)]);
    state.edit_post(&links, "No link any more.").expect("edited");
    assert_eq!(kinds(&state, &target), vec![(embeds.clone(), RefKind::Embed)]);
    state.delete_post(&embeds).expect("deleted");
    assert!(state.backlinks(&target).is_empty());
}

#[test]
fn a_locked_post_embedding_another_is_no_backlink() {
    let mut state = AppState::new_in_memory();
    let target = state.submit_post("Visible thought.").expect("target");
    let draft = ThreadDraft::text(&format!("hidden (({target}))"));
    state.submit_locked_thread(&[draft], &ThreadAnchor::default(), &password(), FAST).expect("locked");
    let _other = locked_post(&mut state);
    assert!(state.backlinks(&target).is_empty(), "the sealed text is never read for references");
}
