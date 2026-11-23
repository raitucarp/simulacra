use super::*;
use crate::domain::post::lock::LockPassword;
use crate::domain::Post;
use std::path::Path;

#[test]
fn a_recording_round_trips_its_length_through_the_wav_header() {
    let samples = vec![0i16; RECORD_SAMPLE_RATE as usize * 3 / 2];
    let bytes = wav_bytes(&samples, RECORD_SAMPLE_RATE, RECORD_CHANNELS);
    assert_eq!(&bytes[0..4], b"RIFF");
    assert_eq!(bytes.len(), 44 + samples.len() * 2, "the canonical header and the samples");
    assert_eq!(wav_duration_ms(&bytes), Some(1500));
    assert_eq!(wav_duration_ms(b"not audio at all"), None);
    assert_eq!(wav_duration_ms(&bytes[..20]), None, "a cut header is not read");
}

#[test]
fn audio_files_are_known_by_extension_and_lengths_read_as_minutes() {
    assert_eq!(mime_of(Path::new("memo.MP3")), Some("audio/mpeg"));
    assert!(is_audio_path(Path::new("C:/notes/take.wav")));
    assert!(!is_audio_path(Path::new("photo.png")));
    assert!(audio_extensions().contains(&"m4a"));
    assert_eq!(duration_label(65_400), "1:05");
    assert_eq!(duration_label(0), "0:00");
}

#[test]
fn transcripts_are_tidied_and_nothing_said_is_no_transcript() {
    assert_eq!(clean_transcript("  Hello there.  \n\n\n  Second thought. "), Some("Hello there.\nSecond thought.".to_string()));
    assert_eq!(clean_transcript(" \n "), None);
    assert_eq!(clean_transcript(&"a".repeat(MAX_TRANSCRIPT_CHARS + 9)).map(|t| t.chars().count()), Some(MAX_TRANSCRIPT_CHARS));
}

#[test]
fn a_voice_note_is_an_attachment_so_its_post_is_never_locked() {
    let mut post = Post::create_with_files("p1", "n1", "user_main", "", None, None, None, None, None, true).expect("post");
    post.extras.voice = Some(Box::new(VoiceNote { path: "media/voice/a.wav".into(), mime: "audio/wav".into(), duration_ms: Some(900), transcript: Some("secret words".into()) }));
    assert!(post.has_attachments());
    assert!(post.extras.voice.as_ref().is_some_and(|note| note.is_wav() && note.file_name() == "a.wav"));
    let kdf = crate::domain::crypto::KdfParams { memory_kib: 8, iterations: 1, lanes: 1 };
    assert_eq!(post.lock_with(&LockPassword::new("seven tidy lanterns"), kdf), Err(crate::error::AppError::LockedWithAttachments));
    assert!(!post.is_locked());
    let json = serde_json::to_string(&post.extras).expect("json");
    assert!(json.contains("\"voice\""));
    let back: crate::domain::post::PostExtras = serde_json::from_str(&json).expect("back");
    assert_eq!(back, post.extras);
}
