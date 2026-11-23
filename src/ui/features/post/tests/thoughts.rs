//! N16–N20 on the post card: what each line shows, and never past a lock.

use crate::domain::crypto::KdfParams;
use crate::domain::post::lens::{Lens, MAX_LENSES};
use crate::domain::post::lock::LockPassword;
use crate::domain::Post;
use crate::ui::features::data::{lens_name, LensDialog};

const FAST: KdfParams = KdfParams { memory_kib: 8, iterations: 1, lanes: 1 };

/// A locked post that carries `finish`'s marks anyway (as a damaged or imported one might).
pub(super) fn forged(finish: impl FnOnce(&mut Post)) -> Post {
    let mut post = Post::create("p", "n1", "u1", "sealed words").expect("post");
    post.lock_with(&LockPassword::new("seven tidy lanterns"), FAST).expect("locked");
    finish(&mut post);
    post
}

#[test]
fn a_lens_reply_names_its_lens_but_never_past_a_lock() {
    let mut post = Post::create("p", "n1", "u1", "Whose streets?").expect("post");
    post.extras.lens = Some(Lens::Ethics);
    assert!(super::lens_chip::lens_chip_text(&post).is_some_and(|text| text.contains(&lens_name(Lens::Ethics))));
    assert!(super::lens_chip::lens_chip_text(&forged(|post| post.extras.lens = Some(Lens::Ethics))).is_none());
    for lens in Lens::ALL {
        let key = format!("posts.lens.name_{}", lens.key());
        assert_ne!(lens_name(*lens), key, "{key} is translated");
    }
}

#[test]
fn the_lens_sheet_starts_with_the_defaults_and_never_picks_past_the_limit() {
    let mut dialog = LensDialog::new(crate::domain::PostId::new("p"), 3);
    assert_eq!(dialog.chosen, Lens::DEFAULTS.to_vec());
    dialog.toggle(Lens::Ethics);
    assert!(!dialog.chosen.contains(&Lens::Ethics));
    for lens in Lens::ALL {
        dialog.toggle(*lens);
    }
    assert!(dialog.chosen.len() <= MAX_LENSES && dialog.problem.is_some(), "a fifth lens is refused with a reason");
}

#[test]
fn a_defining_post_names_its_term_but_never_past_a_lock() {
    use crate::domain::post::glossary::Definition;
    let definition = || Some(Box::new(Definition { term: "entropy".into(), text: "Disorder, counted.".into(), at: jiff::Timestamp::UNIX_EPOCH }));
    let mut post = Post::create("p", "n1", "u1", "Rooms get messy.").expect("post");
    post.extras.defines = definition();
    assert!(super::term_chip::term_chip_text(&post).is_some_and(|text| text.contains("entropy")));
    assert!(super::term_chip::term_chip_text(&forged(|post| post.extras.defines = definition())).is_none());
}

#[test]
fn a_question_shows_whether_it_is_open_but_never_past_a_lock() {
    use crate::domain::post::open_question::{Answered, QuestionState};
    let mut post = Post::create("p", "n1", "u1", "Why is the sky blue?").expect("post");
    assert_eq!(super::open_chip::shown_question(&post), None);
    post.kind = crate::domain::post::PostKind::Question;
    assert_eq!(super::open_chip::shown_question(&post), Some(QuestionState::Open));
    post.extras.answered = Some(Box::new(Answered { at: jiff::Timestamp::UNIX_EPOCH, answer: None }));
    assert_eq!(super::open_chip::shown_question(&post), Some(QuestionState::Answered));
    assert_eq!(super::open_chip::shown_question(&forged(|post| post.kind = crate::domain::post::PostKind::Question)), None);
}

#[test]
fn a_change_of_mind_shows_the_old_view_but_never_past_a_lock() {
    use crate::domain::post::mind::MindChange;
    let change = || Some(Box::new(MindChange { revises: None, was: "Roads ease traffic.".into(), at: jiff::Timestamp::UNIX_EPOCH }));
    let mut post = Post::create("p", "n1", "u1", "Induced demand.").expect("post");
    post.extras.mind_change = change();
    let posts: [Post; 0] = [];
    assert!(super::mind_banner::mind_lines(&post, &posts).is_some_and(|lines| lines.iter().any(|line| line.contains("Roads ease traffic."))));
    assert!(super::mind_banner::mind_lines(&forged(|post| post.extras.mind_change = change()), &posts).is_none());
    let mut sealed = Post::create("s", "n1", "u1", "Soon sealed.").expect("post");
    sealed.extras.mind_change = change();
    sealed.lock_with(&LockPassword::new("seven tidy lanterns"), FAST).expect("locked");
    assert!(sealed.extras.mind_change.is_none(), "a lock drops what would speak past it");
}

#[test]
fn a_translation_names_its_language_natively_and_a_lock_drops_it() {
    use crate::domain::post::translation::Translation;
    assert_eq!(super::translation_line::language_name("es"), "Español");
    assert_eq!(super::translation_line::language_name("xx"), "xx");
    let mut sealed = Post::create("s", "n1", "u1", "Soon sealed.").expect("post");
    sealed.extras.translation = Some(Box::new(Translation { language: "es".into(), text: "Pronto sellado.".into(), at: jiff::Timestamp::UNIX_EPOCH }));
    sealed.lock_with(&LockPassword::new("seven tidy lanterns"), FAST).expect("locked");
    assert!(sealed.extras.translation.is_none(), "a lock drops the translation");
    for key in ["show", "original", "shown", "words_menu", "review_question"].map(|key| format!("posts.translation.{key}")) {
        assert_ne!(rust_i18n::t!(key.as_str()).to_string(), key, "{key} is translated");
    }
}
