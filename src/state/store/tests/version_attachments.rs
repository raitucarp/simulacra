//! Attachments are editable and versioned (V11): an edit may change the link and images,
//! each kept version keeps its own, and switching to a version brings them back.

use crate::domain::data::history::{record_changes, Attachments, PostRecord};
use crate::domain::media::ImageAttachment;
use crate::state::store::{AppState, PostEdit, ThreadAnchor, ThreadDraft};

fn at(hour: i8) -> jiff::Timestamp {
    crate::time::utc(2026, 11, 21, hour, 0, 0)
}

fn with(link: Option<&str>, images: &[&str]) -> Option<PostRecord> {
    let images = images.iter().map(|path| ImageAttachment { alt: format!("alt of {path}"), ..ImageAttachment::at(*path) }).collect();
    Some(PostRecord { attachments: Some(Attachments::new(link.map(str::to_string), images)), ..PostRecord::default() })
}

fn published(state: &mut AppState) -> crate::domain::PostId {
    let draft = ThreadDraft { link: Some("https://example.org/a".into()), ..ThreadDraft::text("Read this") };
    state.submit_thread(&[draft], &ThreadAnchor::default()).unwrap().remove(0)
}

#[test]
fn an_edit_replaces_the_link_and_images_and_keeps_the_old_ones() {
    let mut state = AppState::new_in_memory();
    let id = published(&mut state);
    let edit = PostEdit { look: with(None, &["media/one.png", "media/two.png"]), ..PostEdit::text("Read this") };
    let kept = state.edit_post_version(&id, &edit, at(9)).unwrap().expect("a version");
    let post = state.get_post(&id).unwrap();
    assert_eq!((post.link_url.as_deref(), post.image_path.as_deref(), post.extras.images.len()), (None, Some("media/one.png"), 2));
    assert_eq!(post.extras.images[1].alt, "alt of media/two.png");
    let old = PostRecord::parse(&kept.record_json).and_then(|r| r.attachments).expect("the old attachments are kept");
    assert_eq!(old.link_url.as_deref(), Some("https://example.org/a"));
    assert!(record_changes(&kept.record_json, &PostRecord::of(post).to_json()).attachments);
}

#[test]
fn switching_back_restores_the_attachments_of_that_version() {
    let mut state = AppState::new_in_memory();
    let id = published(&mut state);
    state.edit_post_version(&id, &PostEdit { look: with(None, &["media/one.png"]), ..PostEdit::text("Read this") }, at(9)).unwrap();
    let original = state.post_revisions(&id).remove(0);
    state.switch_post_head(&id, &original.id, at(10)).unwrap();
    let post = state.get_post(&id).unwrap();
    assert_eq!((post.link_url.as_deref(), post.image_path.as_deref(), post.extras.images.is_empty()), (Some("https://example.org/a"), None, true));
}

#[test]
fn an_edit_without_attachments_keeps_the_post_ones_and_an_image_carries_an_empty_text() {
    let mut state = AppState::new_in_memory();
    let id = published(&mut state);
    state.edit_post_version(&id, &PostEdit::text("Read this, slowly"), at(9)).unwrap();
    assert_eq!(state.get_post(&id).unwrap().link_url.as_deref(), Some("https://example.org/a"), "a text edit leaves the link");
    let empty = PostEdit { look: with(None, &["media/one.png"]), ..PostEdit::text("") };
    assert!(state.edit_post_version(&id, &empty, at(10)).unwrap().is_some(), "an image may carry a post without text");
    let bare = PostEdit { look: with(None, &[]), ..PostEdit::text("") };
    assert!(state.edit_post_version(&id, &bare, at(11)).is_err(), "nothing at all is refused");
}
