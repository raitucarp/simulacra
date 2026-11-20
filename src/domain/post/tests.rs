use super::*;
use crate::domain::Profile;
use crate::error::AppError;

#[test]
fn test_valid_post_creation() {
    let author = Profile::new_primary("u1", "Architect", "@arch");
    let post = Post::create(
        "p1",
        "net1",
        author,
        "Mind dump: Testing semiotics and [[post_42]] with #AI @friend",
    )
    .expect("Valid post");

    assert_eq!(post.content, "Mind dump: Testing semiotics and [[post_42]] with #AI @friend");
    assert_eq!(post.linked_posts, vec![PostId::new("post_42")]);
    assert_eq!(post.mentions, vec!["@friend".to_string()]);
    assert_eq!(post.reply_to_id, None);
    assert_eq!(post.quote_id, None);
    assert_eq!(post.repost_id, None);
    assert_eq!(post.reposts_count, 0);
    assert!(!post.is_reposted);
}

#[test]
fn test_empty_post_fails() {
    let author = Profile::new_primary("u1", "Architect", "@arch");
    let result = Post::create("p1", "net1", author, "    ");
    assert_eq!(result, Err(AppError::EmptyContent));
}

#[test]
fn test_exceeding_character_limit_fails() {
    let author = Profile::new_primary("u1", "Architect", "@arch");
    let long_text = "a".repeat(281);
    let result = Post::create("p1", "net1", author, &long_text);
    assert_eq!(
        result,
        Err(AppError::ContentTooLong {
            max: 280,
            actual: 281
        })
    );
}

#[test]
fn test_character_limit_ignores_markup() {
    let author = Profile::new_primary("u1", "Architect", "@arch");
    let marked = format!("__{}__ `x`", "a".repeat(276));
    assert!(marked.chars().count() > MAX_POST_CHARS);
    assert!(Post::create("p1", "net1", author.clone(), &marked).is_ok());
    let over = format!("**{}**", "a".repeat(281));
    let result = Post::create("p2", "net1", author, &over);
    assert_eq!(result, Err(AppError::ContentTooLong { max: 280, actual: 281 }));
}

#[test]
fn test_create_with_attachments_and_links() {
    let author = Profile::new_primary("u1", "Architect", "@arch");
    let post = Post::create_with_attachments(
        "p_attach",
        "net1",
        author,
        "Check this link and screenshot [[post_42]]",
        Some("https://example.com/paper.pdf".to_string()),
        Some("/tmp/diagram.png".to_string()),
        Some(PostId::new("post_parent")),
        Some(PostId::new("post_quote")),
        None,
    )
    .expect("Should create post with attachments");

    assert_eq!(post.link_url, Some("https://example.com/paper.pdf".to_string()));
    assert_eq!(post.image_path, Some("/tmp/diagram.png".to_string()));
    assert_eq!(post.reply_to_id, Some(PostId::new("post_parent")));
    assert_eq!(post.quote_id, Some(PostId::new("post_quote")));
    assert!(post.linked_posts.contains(&PostId::new("post_42")));
    assert!(post.linked_posts.contains(&PostId::new("post_parent")));
    assert!(post.linked_posts.contains(&PostId::new("post_quote")));
    assert!(post.has_attachments());
    assert!(post.is_reply());
    assert!(post.is_quote());
    assert!(!post.is_repost());
}

#[test]
fn test_linked_posts_order_is_body_then_quote_reply_repost() {
    let author = Profile::new_primary("u1", "Architect", "@arch");
    let post = Post::create_with_attachments(
        "p_order",
        "net1",
        author,
        "See [[post_b]] and [[post_a]]",
        None,
        None,
        Some(PostId::new("post_reply")),
        Some(PostId::new("post_a")),
        Some(PostId::new("post_repost")),
    )
    .expect("valid post");
    let expected: Vec<PostId> = ["post_b", "post_a", "post_reply", "post_repost"]
        .into_iter()
        .map(PostId::new)
        .collect();
    assert_eq!(post.linked_posts, expected);
}

#[test]
fn test_create_repost_allows_empty_content() {
    let author = Profile::new_primary("u1", "Architect", "@arch");
    let repost = Post::create_repost("p_rep", "net1", author, "post_original")
        .expect("Instant repost allows empty content");

    assert_eq!(repost.content, "");
    assert_eq!(repost.repost_id, Some(PostId::new("post_original")));
    assert!(repost.is_repost());
}

#[test]
fn test_post_serde_backward_compatibility() {
    let legacy_json = r#"{
        "id": "post_old",
        "network_id": "net_philosophy",
        "author": {
            "id": "u1",
            "name": "Architect",
            "handle": "@arch",
            "avatar_initial": "A",
            "role": "UserPrimary"
        },
        "content": "Legacy post content",
        "created_at": "2026-01-01T00:00:00Z",
        "likes": 5,
        "is_liked": true,
        "replies_count": 2,
        "mentions": [],
        "linked_posts": []
    }"#;

    let post: Post = serde_json::from_str(legacy_json).expect("Deserialize legacy JSON");
    assert_eq!(post.author_id.as_str(), "u1", "legacy embedded author keeps only its id");
    assert_eq!(post.reply_to_id, None);
    assert_eq!(post.quote_id, None);
    assert_eq!(post.repost_id, None);
    assert_eq!(post.link_url, None);
    assert_eq!(post.image_path, None);
    assert_eq!(post.reposts_count, 0);
    assert!(!post.is_reposted);
}

#[test]
fn test_post_toggle_repost() {
    let author = Profile::new_primary("u1", "Architect", "@arch");
    let mut post = Post::create("p1", "net1", author, "Test content").unwrap();
    assert_eq!(post.reposts_count, 0);
    assert!(!post.is_reposted);

    post.toggle_repost();
    assert_eq!(post.reposts_count, 1);
    assert!(post.is_reposted);

    post.toggle_repost();
    assert_eq!(post.reposts_count, 0);
    assert!(!post.is_reposted);
}

#[test]
fn test_parse_mentions_and_hashtags() {
    assert_eq!(Post::parse_mentions("hi @a_b, @ and @c!"), vec!["@a_b", "@c"]);
    assert_eq!(Post::parse_hashtags("#AI, # and #Mind."), vec!["#AI", "#Mind"]);
}

#[test]
fn test_action_target_of_instant_repost_is_the_original() {
    let author = Profile::new_primary("u1", "Architect", "@arch");
    let repost = Post::create_repost("p_rep", "net1", author.clone(), "post_original").expect("Repost");
    assert!(repost.is_instant_repost());
    assert_eq!(repost.action_target_id(), &PostId::new("post_original"));

    let quote = Post::create_quote("p_q", "net1", author.clone(), "Commentary", "post_original", None, None)
        .expect("Quote");
    assert!(!quote.is_instant_repost());
    assert_eq!(quote.action_target_id(), &PostId::new("p_q"));

    let plain = Post::create("p_plain", "net1", author, "Plain thought").expect("Plain");
    assert_eq!(plain.action_target_id(), &PostId::new("p_plain"));
}
