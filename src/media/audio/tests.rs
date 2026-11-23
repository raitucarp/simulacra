use super::*;
use crate::domain::post::voice::{wav_bytes, RECORD_CHANNELS, RECORD_SAMPLE_RATE};

fn scratch(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("simulacra-voice-test-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

#[test]
fn a_recording_is_stored_once_by_content_with_its_length() {
    let media = scratch("recording");
    let wav = wav_bytes(&vec![7i16; RECORD_SAMPLE_RATE as usize], RECORD_SAMPLE_RATE, RECORD_CHANNELS);
    let first = store_recording(&wav, &media).expect("stored");
    let again = store_recording(&wav, &media).expect("stored again");
    assert_eq!(first, again, "the same take is one file");
    assert_eq!((first.mime.as_str(), first.duration_ms), ("audio/wav", Some(1000)));
    assert!(first.path.ends_with(".wav") && std::path::Path::new(&first.path).starts_with(media.join("voice")));
    assert_eq!(std::fs::read(&first.path).expect("file"), wav);
    let _ = std::fs::remove_dir_all(&media);
}

#[test]
fn a_chosen_audio_file_is_copied_and_anything_else_is_refused() {
    let media = scratch("file");
    std::fs::create_dir_all(&media).expect("dir");
    let source = media.join("memo.MP3");
    std::fs::write(&source, b"ID3 not really an mp3").expect("source");
    let note = store_voice_file(&source, &media).expect("copied");
    assert_eq!((note.mime.as_str(), note.duration_ms), ("audio/mpeg", None));
    assert!(note.path.ends_with(".mp3"));
    let picture = media.join("photo.png");
    std::fs::write(&picture, b"png").expect("picture");
    assert!(store_voice_file(&picture, &media).is_err());
    assert!(store_voice_file(&media.join("missing.wav"), &media).is_err());
    let _ = std::fs::remove_dir_all(&media);
}
