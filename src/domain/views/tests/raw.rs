use super::{locked, post_at};
use crate::domain::views::raw::{raw_blocks, raw_text};

#[test]
fn blocks_keep_the_text_as_written_under_author_and_time() {
    let posts = [post_at("a", "A **bold** idea with [[Link]] and #tag\nsecond line  \n", "2026-11-02T09:00:00Z"), post_at("b", "short", "2026-11-02T10:00:00Z")];
    let blocks = raw_blocks(&posts, |post| format!("@{}", post.id), "Locked post");
    assert_eq!(blocks[0].text, "A **bold** idea with [[Link]] and #tag\nsecond line");
    assert_eq!(raw_text(&blocks), "@a\nA **bold** idea with [[Link]] and #tag\nsecond line\n\n@b\nshort");
}

#[test]
fn a_locked_post_copies_only_its_placeholder() {
    let secret = locked(post_at("s", "the secret plan", "2026-11-02T09:00:00Z"));
    let blocks = raw_blocks([&secret], |_| "me".to_string(), "Locked post");
    assert!(blocks[0].locked);
    let text = raw_text(&blocks);
    assert_eq!(text, "me\nLocked post");
    assert!(!text.contains("secret"));
}

#[test]
fn nothing_to_copy_without_posts() {
    assert_eq!(raw_text(&raw_blocks(std::iter::empty(), |_| String::new(), "")), "");
}
