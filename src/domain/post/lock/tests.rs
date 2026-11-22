use super::*;
use crate::domain::crypto::KdfParams;

/// The smallest costs Argon2id accepts, so the tests stay fast.
pub(crate) const FAST: KdfParams = KdfParams { memory_kib: 8, iterations: 1, lanes: 1 };
const SECRET: &str = "Quetzal orchard memo, see [[p-older]] and @someone";

fn password() -> LockPassword {
    LockPassword::new("correct horse battery")
}

fn locked() -> Post {
    let mut post = Post::create("p-locked", "n1", "u1", SECRET).expect("valid");
    post.lock_with(&password(), FAST).expect("locked");
    post
}

#[test]
fn locking_leaves_only_the_placeholder_and_opens_with_the_password() {
    let post = locked();
    assert!(post.is_locked() && post.is_sealed_or_plain());
    assert_eq!(post.content, LOCKED_PLACEHOLDER);
    assert!(post.mentions.is_empty() && post.linked_posts.is_empty(), "nothing read from the text is kept");
    assert_eq!(post.unlock(&password()).expect("opens").as_str(), SECRET);
    assert_eq!(post.unlock(&LockPassword::new("wrong horse battery")).err(), Some(UnlockError::WrongPassword));
}

#[test]
fn the_stored_post_never_holds_the_plaintext() {
    let stored = serde_json::to_string(&locked()).expect("json");
    assert!(!stored.contains("Quetzal") && !stored.contains("orchard"));
    let back: Post = serde_json::from_str(&stored).expect("reads back");
    assert_eq!(back.unlock(&password()).expect("opens").as_str(), SECRET);
}

#[test]
fn a_lock_opens_under_another_post_id_as_an_import_gives_it() {
    let mut imported = Post::create("p-imported", "n1", "u1", "anything").expect("valid");
    imported.extras.lock = locked().extras.lock;
    assert_eq!(imported.unlock(&password()).expect("opens").as_str(), SECRET);
}

#[test]
fn damaged_or_unknown_locks_are_reported_as_damaged() {
    let post = locked();
    let lock = post.extras.lock.clone().expect("lock");
    let damaged = [
        PostLock { format: LOCK_FORMAT + 1, ..lock.clone() },
        PostLock { salt: "not base64!".into(), ..lock.clone() },
        PostLock { nonce: "AAAA".into(), ..lock.clone() },
        PostLock { memory_kib: u32::MAX, ..lock.clone() },
    ];
    for bad in damaged {
        assert_eq!(open_text(&bad, &password()).err(), Some(UnlockError::Damaged), "{bad:?}");
    }
    let plain = Post::create("p-plain", "n1", "u1", "open text").expect("valid");
    assert_eq!(plain.unlock(&password()).err(), Some(UnlockError::Damaged));
}

#[test]
fn attachments_and_double_locks_are_refused() {
    let mut linked = Post::create_with_attachments("p1", "n1", "u1", "see", Some("https://a.org".into()), None, None, None, None).expect("valid");
    assert_eq!(linked.lock_with(&password(), FAST), Err(AppError::LockedWithAttachments));
    assert!(!linked.is_locked());
    assert_eq!(locked().lock_with(&password(), FAST), Err(AppError::PostLocked));
}

#[test]
fn every_lock_gets_a_fresh_salt_and_nonce() {
    let (a, b) = (locked().extras.lock.expect("a"), locked().extras.lock.expect("b"));
    assert_ne!(a.salt, b.salt);
    assert_ne!(a.nonce, b.nonce);
    assert_ne!(a.ciphertext, b.ciphertext);
}

#[test]
fn the_password_is_never_printed_and_has_a_minimum() {
    let secret = LockPassword::new("hunter2hunter2");
    assert!(!format!("{secret:?}").contains("hunter2"));
    assert!(secret.meets_minimum());
    assert!(!LockPassword::new("short").meets_minimum());
}

#[test]
fn a_version_text_seals_and_opens_on_its_own() {
    let lock = seal_text("an older wording", &password(), FAST).expect("sealed");
    assert_eq!(open_text(&lock, &password()).expect("opens").as_str(), "an older wording");
}

#[test]
fn the_composer_locks_only_text_with_a_long_enough_password_typed_twice() {
    assert_eq!(lock_problem("seven tidy lanterns", "seven tidy lanterns", false), None);
    assert_eq!(lock_problem("seven tidy lanterns", "seven tidy lanterns", true), Some(LockProblem::Attachments));
    assert_eq!(lock_problem("short", "short", false), Some(LockProblem::TooShort));
    assert_eq!(lock_problem("", "", false), Some(LockProblem::TooShort));
    assert_eq!(lock_problem("seven tidy lanterns", "seven tidy lantern", false), Some(LockProblem::Mismatch));
    assert_eq!(LockProblem::TooShort.key(), "too_short");
}
