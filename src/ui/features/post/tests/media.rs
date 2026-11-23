//! N11, N12 on the post card: a voice note and a drawing show, but never past a lock.

use crate::domain::crypto::KdfParams;
use crate::domain::post::lock::LockPassword;
use crate::domain::post::voice::VoiceNote;
use crate::domain::Post;

const FAST: KdfParams = KdfParams { memory_kib: 8, iterations: 1, lanes: 1 };

fn note() -> VoiceNote {
    VoiceNote { path: "media/voice/take.wav".into(), mime: "audio/wav".into(), duration_ms: Some(4_200), transcript: Some("the secret plan".into()) }
}

/// A locked post that carries `finish`'s media anyway (as a damaged or imported one might).
fn forged(finish: impl FnOnce(&mut Post)) -> Post {
    let mut post = Post::create_with_attachments("p", "n1", "u1", "sealed words", None, None, None, None, None).expect("post");
    post.lock_with(&LockPassword::new("seven tidy lanterns"), FAST).expect("locked");
    finish(&mut post);
    post
}

#[test]
fn a_voice_note_shows_its_name_and_length_but_never_past_a_lock() {
    let mut post = Post::create_with_files("p", "n1", "u1", "", None, None, None, None, None, true).expect("post");
    post.extras.voice = Some(Box::new(note()));
    let shown = super::voice_card::shown_voice(&post).expect("shown");
    assert_eq!(super::voice_card::voice_label(shown), "take.wav · 0:04");
    let sealed = forged(|post| post.extras.voice = Some(Box::new(note())));
    assert!(super::voice_card::shown_voice(&sealed).is_none(), "a locked post shows only its placeholder");
    for key in ["play", "stop_playing", "play_failed"].map(|key| format!("posts.voice.{key}")) {
        assert_ne!(rust_i18n::t!(key.as_str()).to_string(), key, "{key} is translated");
    }
}
