//! N6–N10 on the post card: the what-if chip, and nothing past a lock.

use crate::domain::crypto::KdfParams;
use crate::domain::post::branch::BranchPoint;
use crate::domain::post::lock::{LockPassword, LOCKED_PLACEHOLDER};
use crate::domain::Post;

const FAST: KdfParams = KdfParams { memory_kib: 8, iterations: 1, lanes: 1 };

fn locked(mut post: Post) -> Post {
    post.lock_with(&LockPassword::new("seven tidy lanterns"), FAST).expect("locked");
    post
}

#[test]
fn a_branch_wears_its_label_but_never_past_a_lock() {
    let theme = crate::theme::ThemeTokens::default();
    let mut post = Post::create_reply("b", "n1", "u1", "Then every scroll hits the disk.", crate::domain::PostId::new("root"), None, None).expect("valid");
    assert!(super::branch_chip::render_branch_chip(&post, &theme).is_none(), "a plain reply has no chip");
    post.extras.branch = BranchPoint::named("What if we skip the cache?");
    assert!(super::branch_chip::branch_chip_text(&post).is_some_and(|text| text.contains("What if we skip the cache?")));
    assert!(super::branch_chip::render_branch_chip(&post, &theme).is_some());
    let sealed = locked(post);
    assert_eq!(super::branch_chip::branch_chip_text(&sealed).filter(|text| !text.contains("cache")).map(|text| text.contains(LOCKED_PLACEHOLDER)), Some(true));
    for key in ["menu", "title", "subtitle", "explain", "label", "label_placeholder", "first_post", "first_post_placeholder", "send", "name_first", "write_first", "failed", "sent", "chip", "main"].map(|key| format!("posts.branch.{key}")) {
        assert_ne!(rust_i18n::t!(&key, locale = "en"), key);
    }
}
