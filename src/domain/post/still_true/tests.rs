use super::*;
use crate::domain::data::history::VersionHead;
use crate::domain::post::correction::{Correction, CorrectionKind};
use crate::domain::post::lock::LockPassword;

fn day(n: i64) -> Timestamp {
    Timestamp::from_second(1_780_000_000 + n * SECONDS_PER_DAY).expect("time")
}

fn claim(written: i64) -> Post {
    let mut post = Post::create_with_attachments("p1", "n1", "user_main", "Sourdough needs a 1:5 starter ratio.", None, None, None, None, None).expect("post");
    post.created_at = day(written);
    post
}

#[test]
fn a_claim_is_due_after_a_month_and_each_confirmation_doubles_the_wait() {
    let mut post = claim(0);
    assert_eq!(due_at(&post), Some(day(FIRST_WAIT_DAYS)));
    assert!(!is_due(&post, day(29)) && is_due(&post, day(30)));
    post.extras.check = Some(Box::new(confirmed(&post, day(31))));
    assert_eq!(post.extras.check.as_ref().map(|c| c.streak), Some(1));
    assert_eq!(due_at(&post), Some(day(31 + 60)));
    post.extras.check = Some(Box::new(confirmed(&post, day(91))));
    assert_eq!(due_at(&post), Some(day(91 + 120)));
    assert_eq!((wait_days(4), wait_days(9)), (LONGEST_WAIT_DAYS, LONGEST_WAIT_DAYS), "never more than a year");
}

#[test]
fn a_revision_starts_the_wait_again() {
    let mut post = claim(0);
    post.extras.check = Some(Box::new(StillTrue { confirmed_at: day(40), streak: 3 }));
    post.extras.head = Some(VersionHead { edited_at: Some(day(50)), ..VersionHead::original(day(0)) });
    assert_eq!(due_at(&post), Some(day(50 + FIRST_WAIT_DAYS)), "the edit made a new claim");
    assert_eq!(confirmed(&post, day(80)).streak, 1);
}

#[test]
fn retracted_locked_and_empty_posts_are_never_asked() {
    let mut retracted = claim(0);
    retracted.extras.correction = Some(Box::new(Correction { kind: CorrectionKind::Retracted, reason: "Wrong.".into(), at: day(1), told: Vec::new() }));
    assert_eq!(due_at(&retracted), None);
    let mut corrected = retracted.clone();
    corrected.extras.correction.as_mut().expect("mark").kind = CorrectionKind::Corrected;
    assert!(due_at(&corrected).is_some(), "a corrected claim still stands, corrected");
    let mut locked = claim(0);
    locked.lock_with(&LockPassword::new("seven tidy lanterns"), crate::domain::crypto::KdfParams { memory_kib: 8, iterations: 1, lanes: 1 }).expect("locked");
    assert!(!checkable(&locked) && !is_due(&locked, day(400)));
    let mut empty = claim(0);
    empty.content.clear();
    assert!(!checkable(&empty));
}
