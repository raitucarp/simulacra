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

#[test]
fn a_transcript_lands_on_the_note_and_agents_read_it_but_never_past_a_lock() {
    let mut state = AppState::new_in_memory();
    let id = state.submit_voice_post("Walk", note()).expect("posted");
    assert!(state.set_transcript(&id, "  \n ").is_err(), "nothing said, nothing kept");
    state.set_transcript(&id, "  I think entropy is about counting.  ").expect("kept");
    let post = state.get_post(&id).expect("post");
    assert_eq!(post.extras.voice.as_ref().and_then(|v| v.transcript.as_deref()), Some("I think entropy is about counting."));
    let read = post.agent_text();
    assert!(read.starts_with("Walk") && read.contains("[Voice note, 0:04] Transcript: I think entropy is about counting."), "{read}");
    let locked = super::locked::locked_post(&mut state);
    let at = state.posts.iter().position(|p| p.id == locked).expect("locked post");
    state.posts[at].extras.voice = Some(Box::new(VoiceNote { transcript: Some("Quetzalcoatl confession".into()), ..note() }));
    assert!(state.set_transcript(&locked, "more secrets").is_err(), "a locked post is never transcribed");
    let sealed = state.get_post(&locked).expect("post").agent_text();
    assert!(!sealed.contains("Quetzalcoatl") && !sealed.contains("Voice note"), "agents read only the placeholder: {sealed}");
}
