//! Unit tests for the post item feature.

use super::*;
use crate::domain::{Post, Profile};

#[test]
fn test_resolve_wikilink_excerpt_found() {
    let author = Profile::new_primary("u1", "Agent 1", "@agent1");
    let post = Post::create(
        "post_123456",
        "net1",
        author,
        "Epistemic dialectic simulation enables autonomous debate",
    )
    .unwrap();
    let posts = vec![post];

    let excerpt = resolve_wikilink_excerpt("post_123456", &posts);
    assert_eq!(excerpt, "Epistemic dialectic simulation...");
}

#[test]
fn test_resolve_wikilink_excerpt_not_found_fallback() {
    let posts: Vec<Post> = Vec::new();
    let excerpt = resolve_wikilink_excerpt("post_abcdef123", &posts);
    assert_eq!(excerpt, "#abcd");
}

#[test]
fn test_resolve_wikilink_excerpt_plain_text() {
    let posts: Vec<Post> = Vec::new();
    let excerpt = resolve_wikilink_excerpt("non_post_reference", &posts);
    assert_eq!(excerpt, "non_post_reference");
}

#[test]
fn test_resolve_wikilink_excerpt_unicode_slugs() {
    let posts: Vec<Post> = Vec::new();

    // 1. Accented Latin characters (> 6 chars, non-char-boundary at byte 4)
    let excerpt_accent = resolve_wikilink_excerpt("post_café_latte", &posts);
    assert_eq!(excerpt_accent, "#café");

    let excerpt_uber = resolve_wikilink_excerpt("post_über_mensch", &posts);
    assert_eq!(excerpt_uber, "#über");

    // 2. CJK characters (3 bytes per character)
    // Exactly 6 chars: remains full string
    let excerpt_cjk_6 = resolve_wikilink_excerpt("post_你好世界测试", &posts);
    assert_eq!(excerpt_cjk_6, "post_你好世界测试");

    // 8 chars (> 6 chars): cleanly truncates to first 4 characters
    let excerpt_cjk_long = resolve_wikilink_excerpt("post_你好世界测试哲学", &posts);
    assert_eq!(excerpt_cjk_long, "#你好世界");

    // Japanese
    let excerpt_jp = resolve_wikilink_excerpt("post_東京_tokyo_spark", &posts);
    assert_eq!(excerpt_jp, "#東京_t");

    // Korean (5 chars <= 6)
    let excerpt_kr = resolve_wikilink_excerpt("post_소크拉테斯", &posts);
    assert_eq!(excerpt_kr, "post_소크拉테斯");

    // Cyrillic (2 bytes per character)
    let excerpt_ru = resolve_wikilink_excerpt("post_сократ_диалог", &posts);
    assert_eq!(excerpt_ru, "#сокр");

    // Greek (2 bytes per character)
    let excerpt_el = resolve_wikilink_excerpt("post_σωκράτης_νόμος", &posts);
    assert_eq!(excerpt_el, "#σωκρ");

    // Emoji with leading ASCII
    let excerpt_emoji = resolve_wikilink_excerpt("post_a🚀_spark_node", &posts);
    assert_eq!(excerpt_emoji, "#a🚀_s");
}

#[test]
fn test_resolve_wikilink_excerpt_boundary_cases() {
    let posts: Vec<Post> = Vec::new();

    // Exactly 6 ASCII chars -> remains full
    assert_eq!(resolve_wikilink_excerpt("post_123456", &posts), "post_123456");

    // Exactly 7 ASCII chars -> truncates to first 4
    assert_eq!(resolve_wikilink_excerpt("post_1234567", &posts), "#1234");

    // Exactly 6 CJK chars -> remains full
    assert_eq!(resolve_wikilink_excerpt("post_一二三四五六", &posts), "post_一二三四五六");

    // Exactly 7 CJK chars -> truncates to first 4
    assert_eq!(resolve_wikilink_excerpt("post_一二三四五六七", &posts), "#一二三四");

    // Empty slug
    assert_eq!(resolve_wikilink_excerpt("post_", &posts), "post_");

    // Whitespace trimmed
    assert_eq!(resolve_wikilink_excerpt("  post_abcdefgh  ", &posts), "#abcd");
}

#[test]
fn test_resolve_wikilink_excerpt_cjk_content() {
    let author = Profile::new_primary("u2", "Spinoza", "@spinoza");
    let post = Post::create(
        "post_cjk_1",
        "net1",
        author,
        "知之为知之不知为不知是知也 仁义礼智信",
    )
    .unwrap();
    let posts = vec![post];

    let excerpt = resolve_wikilink_excerpt("post_cjk_1", &posts);
    assert_eq!(excerpt, "知之为知之不知为不知是知也 仁义礼智信...");
}

#[test]
fn test_render_post_body_trailing_punctuation_multibyte() {
    let theme = crate::theme::minimalist_dark();
    let posts = Vec::new();

    // CJK Ideographic period and fullwidth comma
    let text_cjk = "先读[[post_1001]]。然后看[[post_1002]]，再反思。";
    let _el1 = render_post_body(text_cjk, &posts, &theme);

    // Fullwidth exclamation and question marks
    let text_cjk_punct = "惊讶[[post_1001]]！质疑[[post_1002]]？";
    let _el2 = render_post_body(text_cjk_punct, &posts, &theme);

    // Mixed ASCII and multi-byte trailing punctuation
    let text_mixed = "Ref: [[post_1001]]... [[post_1002]]!? [[post_1003]]…";
    let _el3 = render_post_body(text_mixed, &posts, &theme);

    // CJK characters directly following wikilink without whitespace
    let text_direct_cjk = "根据[[post_1001]]的论断可知。";
    let _el4 = render_post_body(text_direct_cjk, &posts, &theme);

    // Hashtags with CJK punctuation
    let text_tags = "#哲学，#virtue。#epistemic！";
    let _el5 = render_post_body(text_tags, &posts, &theme);
}

#[test]
fn test_render_post_body_long_continuous_strings() {
    let theme = crate::theme::minimalist_dark();
    let posts = Vec::new();

    // 1000-char continuous ASCII without spaces
    let long_ascii = "z".repeat(1000);
    let _el1 = render_post_body(&long_ascii, &posts, &theme);

    // 600-char continuous CJK without spaces
    let long_cjk = "道可道非常道名可名非常名".repeat(50);
    let _el2 = render_post_body(&long_cjk, &posts, &theme);

    // 3000-char continuous URL
    let long_url = format!("https://simulacra.vault/query?key={}", "payload_".repeat(300));
    let _el3 = render_post_body(&long_url, &posts, &theme);

    // Long emoji chain
    let long_emojis = "🧠💡⚡️".repeat(100);
    let _el4 = render_post_body(&long_emojis, &posts, &theme);
}

#[test]
fn test_extract_host_domain_variations() {
    assert_eq!(extract_host_domain("https://crates.io/crates/gpui"), "crates.io");
    assert_eq!(extract_host_domain("http://www.nature.com/articles/s41586-020-2649-2"), "nature.com");
    assert_eq!(extract_host_domain("plato.stanford.edu/entries/ethics/"), "plato.stanford.edu");
    assert_eq!(extract_host_domain("https://sub.domain.org:8080/path?arg=1#hash"), "sub.domain.org:8080");
    assert_eq!(extract_host_domain("   https://example.com   "), "example.com");
    assert_eq!(extract_host_domain("HTTPS://PHILPAPERS.ORG/rec/xyz"), "philpapers.org");
    assert_eq!(extract_host_domain("HTTP://WWW.EXAMPLE.COM:8080/foo"), "example.com:8080");
}

#[test]
fn test_threadline_style_booleans() {
    assert!(!ThreadlineStyle::None.has_top_line());
    assert!(!ThreadlineStyle::None.has_bottom_line());

    assert!(!ThreadlineStyle::ParentHasReplies.has_top_line());
    assert!(ThreadlineStyle::ParentHasReplies.has_bottom_line());

    assert!(ThreadlineStyle::MiddleReply.has_top_line());
    assert!(ThreadlineStyle::MiddleReply.has_bottom_line());

    assert!(ThreadlineStyle::LastReply.has_top_line());
    assert!(!ThreadlineStyle::LastReply.has_bottom_line());
}

#[test]
fn test_tokenize_post_body_segments() {
    use crate::domain::post::{tokenize_post_body, BodyToken};
    let tokens = tokenize_post_body("See [[ post_1 ]]， and #virtue! end [[open");
    assert_eq!(
        tokens,
        vec![
            BodyToken::Text("See ".to_string()),
            BodyToken::Wikilink { target: "post_1".to_string(), trailing: "，".to_string() },
            BodyToken::Text(" and ".to_string()),
            BodyToken::Hashtag { tag: "#virtue".to_string(), trailing: "!".to_string() },
            BodyToken::Text(" end [[open".to_string()),
        ]
    );
}

#[test]
fn test_tokenize_post_body_ignores_inline_and_bare_hash() {
    use crate::domain::post::{tokenize_post_body, BodyToken};
    assert_eq!(tokenize_post_body("a#b # c"), vec![BodyToken::Text("a#b # c".to_string())]);
}

#[test]
fn test_slice_of_refs_lookup_matches_owned_excerpt() {
    let author = Profile::new_primary("u3", "Kant", "@kant");
    let post = Post::create("post_ref_1", "net1", author, "Sapere aude dare to know").unwrap();
    let owned = vec![post.clone()];
    let borrowed: Vec<&Post> = owned.iter().collect();
    let refs: &[&Post] = &borrowed;
    assert_eq!(refs.resolve_wikilink("post_ref_1"), resolve_wikilink_excerpt("post_ref_1", &owned));
    assert_eq!(refs.resolve_wikilink("post_missing_slug"), "#miss");
    assert!(refs.find_post("post_ref_1").is_some());
}

#[test]
fn a_margin_reply_quotes_its_passage_but_never_a_locked_parent() {
    use crate::domain::post::anchor::SpanAnchor;
    use crate::domain::post::lock::{LockPassword, LOCKED_PLACEHOLDER};
    let mut parent = Post::create("p", "n1", "u1", "Habits compound. Small steps add up.").expect("valid");
    let anchor = SpanAnchor::of(&parent.content, 0..16, None).expect("anchor");
    assert_eq!(super::anchor_line::anchor_quote(&anchor, Some(&parent)), ("Habits compound.".to_string(), false));
    let edited = Post { content: "Rewritten entirely.".into(), ..parent.clone() };
    assert_eq!(super::anchor_line::anchor_quote(&anchor, Some(&edited)), ("Habits compound.".to_string(), true));
    let fast = crate::domain::crypto::KdfParams { memory_kib: 8, iterations: 1, lanes: 1 };
    parent.lock_with(&LockPassword::new("seven tidy lanterns"), fast).expect("locked");
    assert_eq!(super::anchor_line::anchor_quote(&anchor, Some(&parent)), (LOCKED_PLACEHOLDER.to_string(), false));
    for key in ["post.margin.quote", "post.margin.changed", "feed.margin.title_one", "feed.margin.title_other"] {
        assert_ne!(rust_i18n::t!(key, locale = "en"), key);
    }
}
