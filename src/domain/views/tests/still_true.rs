use crate::domain::crypto::KdfParams;
use crate::domain::post::lock::LockPassword;
use crate::domain::post::still_true::FIRST_WAIT_DAYS;
use crate::domain::views::still_true::due_claims;
use crate::domain::Post;
use jiff::{SignedDuration, Timestamp};

fn claim(id: &str, text: &str, days_ago: i64) -> Post {
    let mut post = Post::create(id, "n1", "u1", text).expect("valid");
    post.created_at = Timestamp::now().checked_sub(SignedDuration::from_hours(24 * days_ago)).expect("time");
    post
}

#[test]
fn claims_due_longest_first_and_never_a_locked_one() {
    let mut locked = claim("l", "the secret claim", FIRST_WAIT_DAYS * 3);
    locked.lock_with(&LockPassword::new("seven tidy lanterns"), KdfParams { memory_kib: 8, iterations: 1, lanes: 1 }).expect("locked");
    let posts = vec![claim("new", "Fresh.", 2), claim("old", "Old claim.", FIRST_WAIT_DAYS * 2), claim("mid", "Middle claim.", FIRST_WAIT_DAYS + 1), locked];
    let due = due_claims(&posts, Timestamp::now());
    assert_eq!(due.iter().map(|c| c.post_id.as_str()).collect::<Vec<_>>(), vec!["old", "mid"]);
    assert!(!format!("{due:?}").contains("secret"));
}
