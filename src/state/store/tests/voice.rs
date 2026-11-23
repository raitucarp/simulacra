//! N11: voice posts in the store: published with or without a caption, saved with the post,
//! and never part of a locked thread.

use crate::domain::post::voice::VoiceNote;
use crate::state::store::AppState;

pub(super) fn note() -> VoiceNote {
    VoiceNote { path: "media/voice/take.wav".into(), mime: "audio/wav".into(), duration_ms: Some(4_200), transcript: None }
}

#[test]
fn a_voice_note_is_posted_with_or_without_a_caption() {
    let mut state = AppState::new_in_memory();
    let bare = state.submit_voice_post("", note()).expect("posted without words");
    let captioned = state.submit_voice_post("Walking thoughts", note()).expect("posted with a caption");
    let post = state.get_post(&bare).expect("post");
    assert_eq!(post.content, "");
    assert_eq!(post.extras.voice.as_deref(), Some(&note()));
    assert!(post.has_attachments(), "a voice note is an attachment");
    assert_eq!(state.get_post(&captioned).map(|p| p.content.as_str()), Some("Walking thoughts"));
    assert!(state.submit_voice_post("x", VoiceNote::default()).is_err(), "no file, no voice post");
}
