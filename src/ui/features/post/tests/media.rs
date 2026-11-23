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

#[test]
fn a_drawing_shows_on_its_card_but_never_past_a_lock() {
    use crate::domain::post::ink::{Ink, InkDrawing, Stroke};
    let drawing = InkDrawing { strokes: vec![Stroke { points: vec![1, 1, 40, 40], width: 4, ink: Ink::Text }], ..InkDrawing::blank() };
    let mut post = Post::create_with_files("p", "n1", "u1", "", None, None, None, None, None, true).expect("post");
    post.extras.ink = Some(Box::new(drawing.clone()));
    assert_eq!(super::ink_card::shown_ink(&post), Some(&drawing));
    let theme = crate::theme::ThemeTokens::default();
    assert!(super::ink_card::render_ink_card(&post, &theme).is_some());
    let sealed = forged(|post| post.extras.ink = Some(Box::new(drawing.clone())));
    assert!(super::ink_card::shown_ink(&sealed).is_none() && super::ink_card::render_ink_card(&sealed, &theme).is_none(), "a locked post shows only its placeholder");
    assert_eq!(crate::ui::components::ink_figure::hex(theme.accent_brand).len(), 7);
}

#[test]
fn a_retraction_wears_its_banner_but_never_past_a_lock() {
    use crate::domain::post::correction::{Correction, CorrectionKind};
    let mark = Correction { kind: CorrectionKind::Retracted, reason: "The source was a parody.".into(), at: jiff::Timestamp::UNIX_EPOCH, told: Vec::new() };
    let mut post = Post::create_with_attachments("p", "n1", "u1", "Big news.", None, None, None, None, None).expect("post");
    post.extras.correction = Some(Box::new(mark.clone()));
    let shown = super::correction_banner::shown_correction(&post).expect("shown");
    assert!(super::correction_banner::correction_text(shown).contains("The source was a parody."));
    let theme = crate::theme::ThemeTokens::default();
    assert!(super::correction_banner::render_correction_banner(&post, &theme).is_some());
    let sealed = forged(|post| post.extras.correction = Some(Box::new(mark.clone())));
    assert!(super::correction_banner::render_correction_banner(&sealed, &theme).is_none(), "a locked post shows only its placeholder");
}

#[test]
fn only_the_users_due_claims_ask_still_true_and_never_a_locked_one() {
    use crate::domain::post::still_true::FIRST_WAIT_DAYS;
    let later = jiff::Timestamp::now().checked_add(jiff::SignedDuration::from_hours(24 * (FIRST_WAIT_DAYS + 1))).expect("time");
    let post = Post::create_with_attachments("p", "n1", "u1", "Tea is best at 80 degrees.", None, None, None, None, None).expect("post");
    assert!(super::still_true_strip::asks_still_true(&post, true, later));
    assert!(!super::still_true_strip::asks_still_true(&post, false, later), "never on someone else's post");
    assert!(!super::still_true_strip::asks_still_true(&post, true, jiff::Timestamp::now()), "not before it is due");
    let sealed = forged(|_| {});
    assert!(!super::still_true_strip::asks_still_true(&sealed, true, later), "a locked post shows only its placeholder");
}

#[test]
fn a_distilled_root_names_its_document_but_never_past_a_lock() {
    use crate::domain::post::distill::Distilled;
    let distilled = Distilled { doc_id: "doc_1".into(), title: "Why bridges hum".into() };
    let mut post = Post::create_with_attachments("p", "n1", "u1", "Why do bridges hum?", None, None, None, None, None).expect("post");
    post.extras.distilled = Some(Box::new(distilled.clone()));
    assert!(super::distill_chip::distill_text(&post).is_some_and(|text| text.contains("Why bridges hum")));
    let sealed = forged(|post| post.extras.distilled = Some(Box::new(distilled.clone())));
    assert_eq!(super::distill_chip::distill_text(&sealed), None, "a locked post shows only its placeholder");
}
