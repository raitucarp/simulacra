//! Tests of the views of a list of posts (epic W).

mod rapid_log;
mod registry;

use crate::domain::crypto::KdfParams;
use crate::domain::post::lock::LockPassword;
use crate::domain::Post;
use jiff::civil::Date;
use jiff::Timestamp;

/// The smallest costs Argon2id accepts, so the tests stay fast.
const FAST: KdfParams = KdfParams { memory_kib: 8, iterations: 1, lanes: 1 };

/// A post `id` with `text`, written at `at` (UTC).
pub(super) fn post_at(id: &str, text: &str, at: &str) -> Post {
    let mut post = Post::create(id, "net", "author", text).expect("valid post");
    post.created_at = at.parse::<Timestamp>().expect("timestamp");
    post
}

/// `post`, locked.
pub(super) fn locked(mut post: Post) -> Post {
    post.lock_with(&LockPassword::new("correct horse battery"), FAST).expect("locked");
    post
}

/// The UTC day of `at` (the tests' time zone).
pub(super) fn utc_day(at: Timestamp) -> Date {
    crate::time::utc_date(at)
}

pub(super) fn date(raw: &str) -> Date {
    raw.parse().expect("date")
}
