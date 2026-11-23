//! N7: a post pins a vault passage verbatim, is flagged when the document changes, can be
//! re-pinned while the words are there, and a locked post never shows its quote.

use super::locked::locked_post;
use crate::domain::post::pinned::{PinnedQuote, QuoteDrift};
use crate::error::AppError;
use crate::state::store::AppState;

const DOC: &str = "Memory is reconstructive. Each recall rewrites the trace a little.";

fn with_doc() -> (AppState, String) {
    let mut state = AppState::new_in_memory();
    let doc = state.add_document("On memory", DOC).expect("document");
    (state, doc)
}

fn set_text(state: &mut AppState, doc: &str, text: &str) {
    let found = state.documents.iter_mut().find(|d| d.id == doc).expect("doc");
    found.content = text.to_string();
}

#[test]
fn a_pinned_quote_is_intact_until_its_document_changes() {
    let (mut state, doc) = with_doc();
    let id = state.pin_quote(&doc, 25..66, "Worth remembering when I trust old notes.").expect("posted");
    let post = state.get_post(&id).expect("post").clone();
    let quote = post.extras.pinned.clone().expect("quote");
    assert_eq!((quote.passage.as_str(), quote.title.as_str(), quote.doc_id.as_str()), ("Each recall rewrites the trace a little.", "On memory", doc.as_str()));
    assert_eq!(state.quote_drift(&post), Some(QuoteDrift::Intact));
    assert!(state.drifted_quotes().is_empty());
    set_text(&mut state, &doc, &format!("Preface. {DOC}"));
    assert_eq!(state.quote_drift(&post), Some(QuoteDrift::SourceChanged));
    assert_eq!(state.drifted_quotes(), vec![(id.clone(), QuoteDrift::SourceChanged)]);
    assert_eq!(state.repin_quote(&id), Ok(true));
    let post = state.get_post(&id).expect("post").clone();
    assert_eq!((state.quote_drift(&post), post.extras.pinned.as_ref().map(|q| q.start)), (Some(QuoteDrift::Intact), Some(35)));
    assert_eq!(state.repin_quote(&id), Ok(false), "nothing to re-pin");
}

#[test]
fn a_lost_passage_or_document_is_flagged_and_cannot_be_repinned() {
    let (mut state, doc) = with_doc();
    let id = state.pin_quote(&doc, 0..25, "A strong claim.").expect("posted");
    set_text(&mut state, &doc, "Memory is a faithful recording.");
    let post = state.get_post(&id).expect("post").clone();
    assert_eq!(state.quote_drift(&post), Some(QuoteDrift::PassageGone));
    assert!(state.repin_quote(&id).is_err());
    state.documents.clear();
    assert_eq!(state.quote_drift(&post), Some(QuoteDrift::SourceMissing));
    assert!(state.pin_quote(&doc, 0..5, "Gone.").is_err());
}

#[test]
fn a_locked_post_never_shows_its_quote_or_drift() {
    let (mut state, doc) = with_doc();
    let locked = locked_post(&mut state);
    let at = state.post_position(&locked).expect("stored");
    state.posts[at].extras.pinned = PinnedQuote::of(&doc, "On memory", DOC, 0..25).map(Box::new);
    set_text(&mut state, &doc, "Changed.");
    let post = state.get_post(&locked).expect("locked").clone();
    assert_eq!(state.quote_drift(&post), None, "a sealed post shows only its placeholder");
    assert!(state.drifted_quotes().is_empty());
    assert_eq!(state.repin_quote(&locked), Err(AppError::PostLocked));
}
