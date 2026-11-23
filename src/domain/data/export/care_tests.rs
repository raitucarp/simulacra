//! N11, N12, N14: a voice note, a drawing and a retraction travel with the post, its files
//! bundled; never those of a locked post.

use super::*;
use crate::domain::post::correction::{Correction, CorrectionKind};
use crate::domain::post::ink::{Ink, InkDrawing, Stroke};
use crate::domain::post::voice::VoiceNote;

fn media(post: &mut Post, tag: &str) {
    post.extras.voice = Some(Box::new(VoiceNote { path: format!("C:/media/voice/{tag}.wav"), mime: "audio/wav".into(), duration_ms: Some(65_000), transcript: Some(format!("{tag} spoken words")) }));
    post.extras.ink = Some(Box::new(InkDrawing { strokes: vec![Stroke { points: vec![1, 1, 9, 9], width: 2, ink: Ink::Text }], svg_path: Some(format!("C:/media/ink/{tag}.svg")), ..InkDrawing::blank() }));
    post.extras.correction = Some(Box::new(Correction { kind: CorrectionKind::Retracted, reason: format!("{tag} was wrong"), at: jiff::Timestamp::UNIX_EPOCH, told: Vec::new() }));
}

#[test]
fn voice_notes_drawings_and_retractions_travel_but_never_past_a_lock() {
    let mut open = Post::create("p1", "n1", "u1", "Out loud.").expect("post");
    media(&mut open, "Open");
    let block = care::care_block(&open, &|path| format!("media/{}", path.rsplit('/').next().unwrap_or_default()));
    assert!(block.contains("> **Retracted:** Open was wrong"));
    assert!(block.contains("[Voice note (1:05)](media/Open.wav)\n\n> Open spoken words"));
    assert!(block.contains("![Drawing](media/Open.svg)"));
    assert_eq!(care::post_media(&open).len(), 2, "the audio and the SVG are bundled");
    let mut locked = Post::create("p2", "n1", "u1", "Tamarind").expect("post");
    locked.lock_with(&crate::domain::post::lock::LockPassword::new("seven tidy lanterns"), crate::domain::crypto::KdfParams { memory_kib: 8, iterations: 1, lanes: 1 }).expect("locked");
    media(&mut locked, "Quetzal");
    assert_eq!(care::care_block(&locked, &|path| path.to_string()), "", "a locked post travels as its seal only");
    assert!(care::post_media(&locked).is_empty());
}
