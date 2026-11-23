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

#[test]
fn an_embed_is_pinned_to_the_version_its_post_shows_when_written() {
    let mut state = AppState::new_in_memory();
    let target = state.submit_post("The first draft of an idea.").expect("target");
    state.edit_post(&target, "The first draft of an idea, with a second thought added to it.").expect("edited");
    let version = state.get_post(&target).and_then(|post| post.extras.head.clone()).expect("a head").version;
    let embeds = state.submit_post(&format!("Look: (({target})) and ((post_unknown))")).expect("embeds");
    assert_eq!(state.get_post(&embeds).expect("post").content, format!("Look: (({target}@{version})) and ((post_unknown))"));
    state.edit_post(&embeds, &format!("Look again: (({target}))")).expect("edited");
    assert_eq!(state.get_post(&embeds).expect("post").content, format!("Look again: (({target}@{version}))"));
    assert_eq!(kinds(&state, &target), vec![(embeds, RefKind::Embed)]);
}

#[test]
fn a_kept_version_s_text_is_read_for_embeds_and_refreshed_after_an_edit() {
    let mut state = AppState::new_in_memory();
    let id = state.submit_post("Draft one of the note.").expect("posted");
    let original = crate::domain::data::history::SemVer::ORIGINAL;
    assert_eq!(state.version_text(id.as_str(), Some(original)).as_deref(), Some("Draft one of the note."));
    state.edit_post(&id, "Draft two of the note, now with a whole new second half to it.").expect("edited");
    assert_eq!(state.version_text(id.as_str(), Some(original)).as_deref(), Some("Draft one of the note."), "kept as a revision");
    state.edit_post(&id, "Draft three, which replaces everything that was written before.").expect("edited");
    let revisions = state.post_revisions(&id);
    assert!(revisions.iter().filter_map(|r| r.version).all(|v| state.version_text(id.as_str(), Some(v)).is_some()), "every kept version is found after the cache was refreshed");
}
