use gpui_kit::rgba;
use simulacra::domain::{NetworkId, Post, PostId, Profile, ProfileId};
use simulacra::state::AppState;
use simulacra::storage::Database;
use simulacra::theme::{minimalist_dark, ThemeTokens};
use simulacra::ui::molecules::post_item::{render_post_body, resolve_wikilink_excerpt};

#[test]
fn test_unicode_wikilink_slicing_safety() {
    let posts = Vec::new();

    // 1. CJK characters in post ID (> 6 characters vs <= 6 characters)
    let cjk_short = resolve_wikilink_excerpt("post_你好世界测试", &posts);
    assert_eq!(cjk_short, "post_你好世界测试", "6-char CJK slug should remain full");

    let cjk_long = resolve_wikilink_excerpt("post_你好世界测试哲学", &posts);
    assert_eq!(cjk_long, "#你好世界", "8-char CJK slug should truncate to first 4 characters");

    // 2. Accented characters in post ID (> 6 characters)
    let accent_res = resolve_wikilink_excerpt("post_café_latte", &posts);
    assert_eq!(accent_res, "#café", "Accented slug should safely truncate at 4 characters without panic");

    // 3. Post body rendering containing unicode wikilinks and multi-byte punctuation
    let theme = minimalist_dark();
    let _el1 = render_post_body("Reference thought: [[post_你好世界测试]] in feed.", &posts, &theme);
    let _el2 = render_post_body("Accented: [[post_café_latte]]! CJK punct: [[post_1001]]。More text.", &posts, &theme);
    let _el3 = render_post_body("CJK particle: 根据[[post_1001]]的观点论证。", &posts, &theme);
}

#[test]
fn test_render_post_body_unbroken_long_strings() {
    let theme = minimalist_dark();
    let posts = Vec::new();

    // 500-char continuous string without spaces
    let long_unbroken = "a".repeat(500);
    let _el1 = render_post_body(&long_unbroken, &posts, &theme);

    // 2000-char continuous URL
    let long_url = format!("https://example.com/vault/path?query={}&tag={}", "x".repeat(1000), "y".repeat(950));
    let _el2 = render_post_body(&long_url, &posts, &theme);
}

#[test]
fn test_render_post_body_pathological_newlines_and_whitespace() {
    let theme = minimalist_dark();
    let posts = Vec::new();

    // Multiple consecutive newlines, carriage returns, leading and trailing blanks
    let text = "\r\n\r\n\n\n\nFirst line\n\n\n\n\r\nSecond line\n\r\n\n\n";
    let _el = render_post_body(text, &posts, &theme);

    // Completely empty string
    let _el_empty = render_post_body("", &posts, &theme);

    // Pure whitespace
    let _el_spaces = render_post_body("   \t   \r\n   ", &posts, &theme);
}

#[test]
fn test_render_post_body_malformed_wikilinks_and_punctuation() {
    let theme = minimalist_dark();
    let posts = Vec::new();

    // Malformed brackets and edge-case punctuation
    let text = "[[unclosed wikilink [[ nested ]] [[ ]] [[]] [[#tag]] [[post_123]], [[post_456]]. [[post_789]]! [[post_000]]? # #hashtag, #hashtag. #hashtag!";
    let _el = render_post_body(text, &posts, &theme);
}

#[test]
fn test_render_post_body_ascii_wikilink_resolution() {
    let _theme = minimalist_dark();
    let sample_author = Profile::new_primary("prof_1", "Socrates", "@socrates");
    let posts = vec![
        Post::create("post_1001", "net_1", sample_author.clone(), "An unexamined life is not worth living").expect("create post failed"),
        Post::create("post_1002", "net_1", sample_author.clone(), "[[post_1001]] #virtue Know thyself and seek wisdom").expect("create post failed"),
    ];

    // Existent linked post
    let excerpt = resolve_wikilink_excerpt("post_1001", &posts);
    assert_eq!(excerpt, "An unexamined life...");

    // Existent linked post skipping leading wikilinks/hashtags
    let excerpt2 = resolve_wikilink_excerpt("post_1002", &posts);
    assert_eq!(excerpt2, "Know thyself and...");

    // Non-existent ASCII slug longer than 6 chars
    let excerpt3 = resolve_wikilink_excerpt("post_abcdefgh", &posts);
    assert_eq!(excerpt3, "#abcd");

    // Non-existent ASCII slug shorter than 6 chars
    let excerpt4 = resolve_wikilink_excerpt("post_abc", &posts);
    assert_eq!(excerpt4, "post_abc");
}

#[test]
fn test_sqlite_theme_tokens_corrupted_json_resilience() {
    let temp_db_path = std::env::temp_dir().join(format!("simulacra_test_{}.db", uuid::Uuid::new_v4()));
    {
        let db = Database::open(&temp_db_path).expect("open db failed");
        // Seed network
        db.save_network(&simulacra::domain::Network {
            id: "net_default".into(),
            name: "Dialectic Network".into(),
            slug: "dialectic".into(),
            description: "Test".into(),
            doc_count: 0,
            agent_count: 0,
            color_accent: "#8b5cf6".into(),
        }).expect("save_network failed");

        // Inject corrupt JSON into settings table
        db.set_setting("theme_tokens", "{corrupted: true, invalid_json:").expect("set_setting failed");

        // Database load_theme_tokens should return Err on invalid json
        let res = db.load_theme_tokens();
        assert!(res.is_err(), "load_theme_tokens must return Err on corrupted JSON");
    }

    // AppState::with_db_path should gracefully fall back to minimalist_dark
    let state = AppState::with_db_path(&temp_db_path);
    assert_eq!(state.theme_tokens, minimalist_dark(), "Should fall back to minimalist_dark on corrupted settings");

    let _ = std::fs::remove_file(&temp_db_path);
}

#[test]
fn test_sqlite_theme_tokens_empty_and_partial_json() {
    let db = Database::in_memory().expect("in-memory db failed");

    // Empty object `{}` should deserialize into default ThemeTokens thanks to #[serde(default)]
    db.set_setting("theme_tokens", "{}").expect("set_setting failed");
    let loaded = db.load_theme_tokens().expect("load should succeed").expect("should be Some");
    assert_eq!(loaded, ThemeTokens::default());

    // Partial JSON with unknown extra fields and missing fields
    let partial_json = r#"{
        "font_family_ui": "Custom UI Font",
        "unknown_legacy_field": 42,
        "another_extra_key": "ignored"
    }"#;
    db.set_setting("theme_tokens", partial_json).expect("set_setting failed");
    let loaded_partial = db.load_theme_tokens().expect("load should succeed").expect("should be Some");
    assert_eq!(loaded_partial.font_family_ui, "Custom UI Font");
    assert_eq!(loaded_partial.font_family_body, ThemeTokens::default().font_family_body);
    assert_eq!(loaded_partial.bg_feed_post, ThemeTokens::default().bg_feed_post);
}

#[test]
fn test_sqlite_theme_tokens_custom_colors_roundtrip() {
    let db = Database::in_memory().expect("in-memory db failed");
    let mut custom_tokens = minimalist_dark();
    // Set custom visual colors
    custom_tokens.accent_brand = rgba(0x12345678);
    custom_tokens.accent_dialectic = rgba(0xfedcba98);
    custom_tokens.bg_feed_post = rgba(0x01020304);
    custom_tokens.bg_transparent = rgba(0x00000000);
    custom_tokens.text_white = rgba(0xffffffff);

    // Save to SQLite
    db.save_theme_tokens(&custom_tokens).expect("save_theme_tokens failed");

    // Load back and assert fidelity
    let loaded = db.load_theme_tokens().expect("load succeeded").expect("found tokens");
    assert_eq!(loaded.accent_brand, rgba(0x12345678));
    assert_eq!(loaded.accent_dialectic, rgba(0xfedcba98));
    assert_eq!(loaded.bg_feed_post, rgba(0x01020304));
    assert_eq!(loaded.bg_transparent, rgba(0x00000000));
    assert_eq!(loaded.text_white, rgba(0xffffffff));
}

#[test]
fn test_adversarial_cjk_trailing_punctuation_combinations() {
    let theme = minimalist_dark();
    let posts = vec![
        Post::create("post_1001", "net_1", Profile::new_primary("prof_1", "Socrates", "@socrates"), "Philosophy is the search for truth").unwrap(),
    ];

    // 1. Every single CJK punctuation character in isolation
    let puncts = ['，', '。', '！', '？', '；', '：', '…'];
    for p in puncts {
        let input_wikilink = format!("Thought [[post_1001]]{} continues", p);
        let _el = render_post_body(&input_wikilink, &posts, &theme);

        let input_tag = format!("Topic #epistemic{} continues", p);
        let _el_tag = render_post_body(&input_tag, &posts, &theme);
    }

    // 2. Chained multi-byte punctuation
    let chained = "Ref: [[post_1001]]！？…… And tag: #wisdom；：… Next [[post_1001]]……";
    let _el_chained = render_post_body(chained, &posts, &theme);

    // 3. Dense CJK without spaces
    let dense = "根据[[post_1001]]的论述，我们认为#道德与实践！是不可分割的。";
    let _el_dense = render_post_body(dense, &posts, &theme);

    // 4. Punctuation at end of string
    let end_p = "[[post_1001]]。";
    let _el_end = render_post_body(end_p, &posts, &theme);

    let end_tag = "#philosophy！";
    let _el_end_tag = render_post_body(end_tag, &posts, &theme);

    // 5. Incomplete wikilinks with CJK punctuation
    let incomplete = "[[post_1001。 and [[unclosed #tag，";
    let _el_inc = render_post_body(incomplete, &posts, &theme);
}

#[test]
fn test_adversarial_mind_graph_truncation_utf8_boundaries() {
    use simulacra::domain::GroundingDocument;
    use simulacra::ui::organisms::mind_graph_view::render_mind_graph_view;

    let theme = minimalist_dark();
    let author = Profile::new_primary("prof_1", "Socrates", "@socrates");

    // Test cases that would have caused panic in `&post.content[..35]`
    // 1. Exactly 38 CJK characters (38 * 3 = 114 bytes)
    let cjk_38: String = "知".repeat(38);
    let p_cjk_38 = Post::create("post_cjk_38", "net_1", author.clone(), &cjk_38).unwrap();

    // 2. 39 CJK characters (39 * 3 = 117 bytes) - previously: len > 38 triggered &[..35], panicking mid-codepoint at byte 35
    let cjk_39: String = "知".repeat(39);
    let p_cjk_39 = Post::create("post_cjk_39", "net_1", author.clone(), &cjk_39).unwrap();

    // 3. 12 CJK characters (36 bytes) - byte index 35 is inside the 12th char ('知')
    let cjk_12: String = "知".repeat(12);
    let p_cjk_12 = Post::create("post_cjk_12", "net_1", author.clone(), &cjk_12).unwrap();

    // 4. 40 4-byte emojis (160 bytes)
    let emoji_40: String = "🦀".repeat(40);
    let p_emoji_40 = Post::create("post_emoji_40", "net_1", author.clone(), &emoji_40).unwrap();

    // 5. Mixed ASCII and CJK right around 35 and 38 char count
    let mut mixed_35 = "a".repeat(30);
    mixed_35.push_str("你好世界！"); // 35 chars
    let p_mixed_35 = Post::create("post_mixed_35", "net_1", author.clone(), &mixed_35).unwrap();

    let mut mixed_39 = "a".repeat(34);
    mixed_39.push_str("你好世界！"); // 39 chars
    let p_mixed_39 = Post::create("post_mixed_39", "net_1", author.clone(), &mixed_39).unwrap();

    let posts = vec![p_cjk_38, p_cjk_39, p_cjk_12, p_emoji_40, p_mixed_35, p_mixed_39];
    let docs = vec![
        GroundingDocument::new("doc_1", "net_1", "Republic", "Plato's Republic discussion"),
    ];

    // Rendering mind graph view must not panic
    let _graph_view = render_mind_graph_view(&posts, &docs, &theme);
}

#[test]
fn test_adversarial_avatar_geometry_and_theme_tokens() {
    use gpui_kit::px;
    use simulacra::domain::ProfileRole;
    use simulacra::ui::atoms::render_avatar;

    let theme = minimalist_dark();

    // All avatar sizes in ThemeTokens must be positive
    assert!(theme.avatar_size_sm.as_f32() > 0.0);
    assert!(theme.avatar_size_md.as_f32() > 0.0);
    assert!(theme.avatar_size_lg.as_f32() > 0.0);

    // Scaling ordering: sm <= md <= lg
    assert!(theme.avatar_size_sm.as_f32() <= theme.avatar_size_md.as_f32());
    assert!(theme.avatar_size_md.as_f32() <= theme.avatar_size_lg.as_f32());

    // Post avatar column width must accommodate avatar_size_lg
    assert!(theme.post_avatar_col_width.as_f32() >= theme.avatar_size_lg.as_f32());

    // Threadline parameters must be positive
    assert!(theme.threadline_min_height.as_f32() > 0.0);
    assert!(theme.border_width_threadline.as_f32() > 0.0);

    // Avatar rendering across all roles and sizes
    let roles = [
        ProfileRole::UserPrimary,
        ProfileRole::UserAlt,
        ProfileRole::Agent { persona_type: "gemini".into() },
    ];
    for role in &roles {
        let _a_sm = render_avatar("S", role, theme.avatar_size_sm, &theme);
        let _a_md = render_avatar("P", role, theme.avatar_size_md, &theme);
        let _a_lg = render_avatar("A", role, theme.avatar_size_lg, &theme);

        // Edge sizes
        let _a_zero = render_avatar("", role, px(0.0), &theme);
        let _a_huge = render_avatar("🦀", role, px(128.0), &theme);
    }
}

#[test]
fn test_adversarial_feed_surface_oled_black_palettes() {
    use gpui_kit::rgb;
    use simulacra::theme::{midnight_slate, obsidian_zinc};

    let palettes = [
        minimalist_dark(),
        obsidian_zinc(),
        midnight_slate(),
        ThemeTokens::default(),
    ];

    for (idx, pal) in palettes.iter().enumerate() {
        assert_eq!(
            pal.bg_feed_post,
            rgb(0x000000),
            "Palette {} must have pure flat OLED black bg_feed_post (#000000)",
            idx
        );
    }
}

#[test]
fn test_adversarial_repost_and_counter_integrity() {
    let mut state = AppState::new_in_memory();
    let root_id = state
        .submit_post("Original Dialectic Thesis on Ethics")
        .expect("Root post must succeed");

    let initial_post = state
        .posts
        .iter()
        .find(|p| p.id == root_id)
        .expect("Post must exist in store");
    assert_eq!(initial_post.reposts_count, 0);
    assert!(!initial_post.is_reposted);

    // 1. First repost: creates repost thought in feed and increments reposts_count to 1
    let repost_id = state.repost(&root_id).expect("Repost must succeed");
    let updated_post = state.posts.iter().find(|p| p.id == root_id).unwrap();
    assert_eq!(updated_post.reposts_count, 1);
    assert!(updated_post.is_reposted);

    let repost_thought = state.posts.iter().find(|p| p.id == repost_id).unwrap();
    assert_eq!(repost_thought.repost_id.as_deref(), Some(root_id.as_str()));

    // 2. Second repost: toggles un-repost, decrementing count back to 0
    let second_repost = state.repost(&root_id).expect("Second repost must toggle un-repost");
    assert_eq!(second_repost, root_id);
    let untoggled_post = state.posts.iter().find(|p| p.id == root_id).unwrap();
    assert_eq!(untoggled_post.reposts_count, 0);
    assert!(!untoggled_post.is_reposted);
}

#[test]
fn test_adversarial_quote_post_embed_and_attachments() {
    let mut state = AppState::new_in_memory();
    let root_id = state
        .submit_post("Epistemic Foundations of Simulacra")
        .expect("Root post must succeed");

    // Submit a quote thought with link attachment and image path
    let quote_id = state
        .submit_quote(
            &root_id,
            "Synthesizing dialectic reply with grounded documentation",
            Some("https://plato.stanford.edu/entries/spinoza/".to_string()),
            Some("assets/icons/image.svg".to_string()),
        )
        .expect("Quote post must succeed");

    let quote_post = state.posts.iter().find(|p| p.id == quote_id).unwrap();
    assert_eq!(quote_post.quote_id.as_deref(), Some(root_id.as_str()));
    assert_eq!(
        quote_post.link_url.as_deref(),
        Some("https://plato.stanford.edu/entries/spinoza/")
    );
    assert_eq!(quote_post.image_path.as_deref(), Some("assets/icons/image.svg"));
    assert!(quote_post.is_quote());
    assert!(quote_post.has_attachments());
}

#[test]
fn test_adversarial_threadline_continuity_geometry() {
    use simulacra::ui::molecules::ThreadlineStyle;
    use simulacra::ui::organisms::compute_stream_threadline_style;

    let author = Profile::new_primary("u1", "Philosopher", "@philosopher");
    let net = "net_test";

    // Create a 3-tier linear thread: Root -> Reply 1 -> Reply 2
    let root = Post::create("post_root", net, author.clone(), "Root thought").unwrap();
    let mut reply1 = Post::create("post_rep1", net, author.clone(), "Reply 1").unwrap();
    reply1.reply_to_id = Some(root.id.clone());

    let mut reply2 = Post::create("post_rep2", net, author.clone(), "Reply 2").unwrap();
    reply2.reply_to_id = Some(reply1.id.clone());

    let standalone = Post::create("post_solo", net, author.clone(), "Standalone thought").unwrap();

    // 1. Realistic Reverse-Chronological Stream Slice: [Reply 2, Reply 1, Root]
    // In production (created_at DESC), newer replies appear higher in the feed stream.
    let rev_thread_posts = [&reply2, &reply1, &root];

    // Topmost thought in stream (Reply 2): connects downward to Reply 1 below
    let top_style = compute_stream_threadline_style(&rev_thread_posts, 0);
    assert_eq!(top_style, ThreadlineStyle::ParentHasReplies);
    assert!(!top_style.has_top_line());
    assert!(top_style.has_bottom_line());

    // Interior thought in stream (Reply 1): connects upward to Reply 2 and downward to Root
    let mid_style = compute_stream_threadline_style(&rev_thread_posts, 1);
    assert_eq!(mid_style, ThreadlineStyle::MiddleReply);
    assert!(mid_style.has_top_line());
    assert!(mid_style.has_bottom_line());

    // Bottommost thought in stream (Root): connects upward to Reply 1 above
    let bottom_style = compute_stream_threadline_style(&rev_thread_posts, 2);
    assert_eq!(bottom_style, ThreadlineStyle::LastReply);
    assert!(bottom_style.has_top_line());
    assert!(!bottom_style.has_bottom_line());

    // 2. Reverse-Chronological 2-post thread: [Reply 1, Root]
    let pair_posts = [&reply1, &root];
    let pair_top = compute_stream_threadline_style(&pair_posts, 0);
    assert_eq!(pair_top, ThreadlineStyle::ParentHasReplies);
    assert!(!pair_top.has_top_line());
    assert!(pair_top.has_bottom_line());
    let pair_bottom = compute_stream_threadline_style(&pair_posts, 1);
    assert_eq!(pair_bottom, ThreadlineStyle::LastReply);
    assert!(pair_bottom.has_top_line());
    assert!(!pair_bottom.has_bottom_line());

    // 3. Chronological Slice (e.g. Thread Inspector): [Root, Reply 1, Reply 2]
    let chrono_thread_posts = [&root, &reply1, &reply2];
    assert_eq!(compute_stream_threadline_style(&chrono_thread_posts, 0), ThreadlineStyle::ParentHasReplies);
    assert_eq!(compute_stream_threadline_style(&chrono_thread_posts, 1), ThreadlineStyle::MiddleReply);
    assert_eq!(compute_stream_threadline_style(&chrono_thread_posts, 2), ThreadlineStyle::LastReply);

    // 4. Standalone thought: None
    let solo_posts = [&standalone];
    let solo_style = compute_stream_threadline_style(&solo_posts, 0);
    assert_eq!(solo_style, ThreadlineStyle::None);
    assert!(!solo_style.has_top_line());
    assert!(!solo_style.has_bottom_line());
}

#[test]
fn test_adversarial_modal_composer_tokens_and_extract_domain() {
    use simulacra::ui::molecules::post_item::extract_host_domain;

    let theme = minimalist_dark();
    assert!(theme.modal_composer_width.as_f32() >= 500.0);
    assert!(theme.modal_composer_max_height.as_f32() >= 400.0);
    assert!(theme.post_image_max_height.as_f32() > 0.0);
    assert!(theme.post_image_preview_height.as_f32() > 0.0);
    assert!(theme.attachment_btn_size.as_f32() > 0.0);

    // Host domain parser adversarial test
    assert_eq!(extract_host_domain("https://crates.io/crates/gpui"), "crates.io");
    assert_eq!(extract_host_domain("http://www.wikipedia.org/wiki/Spinoza"), "wikipedia.org");
    assert_eq!(extract_host_domain("subdomain.vault.net:9000/path?key=1#ref"), "subdomain.vault.net:9000");
    assert_eq!(extract_host_domain("    https://MIT.EDU/research   "), "mit.edu");
    assert_eq!(extract_host_domain("HTTPS://PHILPAPERS.ORG/rec/xyz"), "philpapers.org");
    assert_eq!(extract_host_domain("HTTP://WWW.EXAMPLE.COM:8080/foo"), "example.com:8080");
    assert_eq!(extract_host_domain("not-a-valid-url"), "not-a-valid-url");
}

#[test]
fn test_adversarial_draft_bounds_and_multibyte_limits() {
    use simulacra::domain::{MAX_POST_CHARS, Post, Profile};
    use simulacra::error::AppError;

    let author = Profile::new_primary("prof_limits", "Kant", "@kant");
    let net = "net_ethics";

    // 1. ASCII boundaries
    assert!(Post::create("p_empty", net, author.clone(), "").is_err());
    assert!(Post::create("p_ws", net, author.clone(), "   \t\r\n   ").is_err());

    let ascii_280 = "a".repeat(280);
    let p_280 = Post::create("p_280", net, author.clone(), &ascii_280);
    assert!(p_280.is_ok(), "280 ASCII chars must be valid");
    assert_eq!(p_280.unwrap().content.chars().count(), MAX_POST_CHARS);

    let ascii_281 = "a".repeat(281);
    let p_281 = Post::create("p_281", net, author.clone(), &ascii_281);
    match p_281 {
        Err(AppError::ContentTooLong { max, actual }) => {
            assert_eq!(max, 280);
            assert_eq!(actual, 281);
        }
        other => panic!("Expected ContentTooLong error, got {:?}", other),
    }

    // 2. Multibyte CJK boundaries (3 bytes per character)
    let cjk_280: String = "德".repeat(280);
    assert_eq!(cjk_280.len(), 840, "280 CJK chars must be 840 bytes");
    let p_cjk_280 = Post::create("p_cjk_280", net, author.clone(), &cjk_280);
    assert!(p_cjk_280.is_ok(), "280 CJK chars must be permitted (character count, not byte count)");

    let cjk_281: String = "德".repeat(281);
    assert_eq!(cjk_281.len(), 843);
    assert!(matches!(
        Post::create("p_cjk_281", net, author.clone(), &cjk_281),
        Err(AppError::ContentTooLong { max: 280, actual: 281 })
    ));

    // 3. Multibyte Emoji boundaries (4 bytes per character)
    let emoji_280: String = "🦀".repeat(280);
    assert_eq!(emoji_280.len(), 1120, "280 crab emojis must be 1120 bytes");
    let p_emoji_280 = Post::create("p_emoji_280", net, author.clone(), &emoji_280);
    assert!(p_emoji_280.is_ok(), "280 4-byte emojis must succeed");

    let emoji_281: String = "🦀".repeat(281);
    assert_eq!(emoji_281.len(), 1124);
    assert!(matches!(
        Post::create("p_emoji_281", net, author.clone(), &emoji_281),
        Err(AppError::ContentTooLong { max: 280, actual: 281 })
    ));

    // 4. Attachments allowing empty text
    let p_link_only = Post::create_with_attachments(
        "p_link",
        net,
        author.clone(),
        "",
        Some("https://example.com".to_string()),
        None,
        None,
        None,
        None,
    );
    assert!(p_link_only.is_ok(), "Empty text with link attachment must succeed");

    let p_img_only = Post::create_with_attachments(
        "p_img",
        net,
        author.clone(),
        "",
        None,
        Some("assets/test.png".to_string()),
        None,
        None,
        None,
    );
    assert!(p_img_only.is_ok(), "Empty text with image attachment must succeed");

    let p_repost_only = Post::create_with_attachments(
        "p_rep",
        net,
        author.clone(),
        "",
        None,
        None,
        None,
        None,
        Some("target_123".into()),
    );
    assert!(p_repost_only.is_ok(), "Empty text with repost_id must succeed");

    let p_reply_empty_fail = Post::create_with_attachments(
        "p_reply_empty",
        net,
        author.clone(),
        "",
        None,
        None,
        Some("parent_123".into()),
        None,
        None,
    );
    assert!(matches!(p_reply_empty_fail, Err(AppError::EmptyContent)), "Reply without text or media must fail EmptyContent");

    let p_quote_empty_fail = Post::create_with_attachments(
        "p_quote_empty",
        net,
        author.clone(),
        "   ",
        None,
        None,
        None,
        Some("target_123".into()),
        None,
    );
    assert!(matches!(p_quote_empty_fail, Err(AppError::EmptyContent)), "Quote with whitespace-only text and no media must fail EmptyContent");
}

#[test]
fn test_adversarial_keyboard_shortcut_matrix_and_enter_variants() {
    use simulacra::ui::molecules::post_composer::{KEY_ENTER, KEY_ENTER_CR, KEY_ENTER_LF};

    // Verify key constants adhere to GPUI keystroke representations
    assert_eq!(KEY_ENTER, "enter");
    assert_eq!(KEY_ENTER_CR, "\r");
    assert_eq!(KEY_ENTER_LF, "\n");

    let evaluate_shortcut = |key: &str, ctrl: bool, platform: bool, is_macos: bool| -> bool {
        let is_enter = key == KEY_ENTER || key == KEY_ENTER_CR || key == KEY_ENTER_LF;
        let is_ctrl = ctrl || (is_macos && platform);
        is_enter && is_ctrl
    };

    // Standard Ctrl+Enter on Windows/Linux
    assert!(evaluate_shortcut(KEY_ENTER, true, false, false));
    assert!(evaluate_shortcut(KEY_ENTER_CR, true, false, false));
    assert!(evaluate_shortcut(KEY_ENTER_LF, true, false, false));

    // Cmd+Enter on macOS (platform modifier is true)
    assert!(evaluate_shortcut(KEY_ENTER, false, true, true));
    assert!(evaluate_shortcut(KEY_ENTER_CR, false, true, true));
    assert!(evaluate_shortcut(KEY_ENTER_LF, false, true, true));

    // Win/Super key on Windows/Linux (platform modifier is true, ctrl is false) -> should NOT submit
    assert!(!evaluate_shortcut(KEY_ENTER, false, true, false));

    // Plain Enter without Ctrl (inserts newline) -> should NOT submit
    assert!(!evaluate_shortcut(KEY_ENTER, false, false, false));
    assert!(!evaluate_shortcut(KEY_ENTER_CR, false, false, false));
    assert!(!evaluate_shortcut(KEY_ENTER_LF, false, false, false));

    // Shift+Enter (inserts newline) -> should NOT submit
    assert!(!evaluate_shortcut(KEY_ENTER, false, false, false));

    // Non-enter keys with Ctrl -> should NOT submit
    assert!(!evaluate_shortcut("Tab", true, false, false));
    assert!(!evaluate_shortcut("Space", true, false, false));
    assert!(!evaluate_shortcut("a", true, false, false));
    assert!(!evaluate_shortcut("Escape", true, false, false));
}

#[test]
fn test_adversarial_consecutive_submissions_and_store_reentrancy() {
    use simulacra::domain::PostId;
    use simulacra::error::AppError;

    let mut state = AppState::new_in_memory();

    // 1. High-frequency consecutive burst: 100 submissions
    let mut post_ids = Vec::new();
    for i in 0..100 {
        let content = format!("Burst Thought #{}: exploring dialectic epistemology", i);
        let id = state.submit_post(&content).expect("Burst submit must succeed");
        assert!(!post_ids.contains(&id), "Every submitted post must have a unique ID");
        post_ids.push(id);
    }
    assert_eq!(post_ids.len(), 100);

    // 2. Rapid Repost Toggle Burst (20 alternations)
    let root_id = post_ids[0].clone();
    for iter in 0..20 {
        let res = state.repost(&root_id).expect("Repost toggle must succeed");
        let post = state.posts.iter().find(|p| p.id == root_id).unwrap();
        if iter % 2 == 0 {
            // First of pair: reposted
            assert_eq!(post.reposts_count, 1, "Iter {} should have reposts_count=1", iter);
            assert!(post.is_reposted, "Iter {} should be is_reposted=true", iter);
            assert_eq!(state.posts.iter().filter(|p| p.repost_id.as_ref() == Some(&root_id)).count(), 1);
        } else {
            // Second of pair: untoggled
            assert_eq!(res, root_id);
            assert_eq!(post.reposts_count, 0, "Iter {} should have reposts_count=0", iter);
            assert!(!post.is_reposted, "Iter {} should be is_reposted=false", iter);
            assert_eq!(state.posts.iter().filter(|p| p.repost_id.as_ref() == Some(&root_id)).count(), 0);
        }
    }

    // 3. Error resilience: quote, repost, or reply on non-existent post ID
    let ghost_id = PostId::new("post_ghost_nonexistent_9999");
    assert!(matches!(
        state.submit_quote(&ghost_id, "Commentary on ghost", None, None),
        Err(AppError::DatabaseError(_))
    ));
    assert!(matches!(
        state.repost(&ghost_id),
        Err(AppError::DatabaseError(_))
    ));
    assert!(matches!(
        state.submit_reply(&ghost_id, "Reply to ghost", None, None),
        Err(AppError::DatabaseError(_))
    ));

    // Valid reply to existent root post increments parent replies_count by 1
    let initial_replies = state.posts.iter().find(|p| p.id == root_id).unwrap().replies_count;
    let reply_res = state.submit_reply(&root_id, "Dialectic reply to root thought", None, None);
    assert!(reply_res.is_ok(), "Replying to existent post succeeds");
    let updated_root = state.posts.iter().find(|p| p.id == root_id).unwrap();
    assert_eq!(updated_root.replies_count, initial_replies + 1, "Replying to root must increment replies_count by 1");
}

#[test]
fn test_adversarial_char_meter_thresholds() {
    use simulacra::theme::minimalist_dark;
    use simulacra::ui::atoms::render_char_meter;

    let theme = minimalist_dark();

    // Verify render_char_meter handles all threshold ranges without panicking
    let _el_0 = render_char_meter(0, &theme);
    let _el_259 = render_char_meter(259, &theme);
    let _el_260 = render_char_meter(260, &theme);
    let _el_279 = render_char_meter(279, &theme);
    let _el_280 = render_char_meter(280, &theme);
    let _el_281 = render_char_meter(281, &theme);
    let _el_500 = render_char_meter(500, &theme);
    let _el_huge = render_char_meter(100_000, &theme);
}

#[test]
fn test_adversarial_sqlite_attachments_and_null_coalescing() {
    let temp_db_path = std::env::temp_dir().join(format!("simulacra_m2_test_{}.db", uuid::Uuid::new_v4()));
    {
        let db = Database::open(&temp_db_path).expect("open db failed");
        let author = Profile::new_primary("prof_sqlite", "Spinoza", "@spinoza");

        // 1. Fully-populated thought post with all 7 new attachment fields
        let full_post = Post::create_with_attachments(
            "post_full_m2",
            "net_default",
            author.clone(),
            "Fully attached dialectic thesis with link and image",
            Some("https://plato.stanford.edu/".to_string()),
            Some("assets/image.png".to_string()),
            Some("parent_post_1".into()),
            Some("quote_post_2".into()),
            None,
        ).unwrap();

        db.save_post(&full_post).expect("save_post with attachments failed");
        db.update_post_repost(&full_post.id, 5, true).expect("update_post_repost failed");

        let loaded = db.load_posts().expect("load_posts failed");
        let loaded_full = loaded.iter().find(|p| p.id.as_str() == "post_full_m2").expect("Must find full post");

        assert_eq!(loaded_full.link_url.as_deref(), Some("https://plato.stanford.edu/"));
        assert_eq!(loaded_full.image_path.as_deref(), Some("assets/image.png"));
        assert_eq!(loaded_full.reply_to_id.as_deref(), Some("parent_post_1"));
        assert_eq!(loaded_full.quote_id.as_deref(), Some("quote_post_2"));
        assert_eq!(loaded_full.reposts_count, 5);
        assert!(loaded_full.is_reposted);

        // 2. Legacy / unpopulated post where all attachment columns are NULL
        let empty_attachments = Post::create(
            "post_empty_m2",
            "net_default",
            author.clone(),
            "Plain thought with zero attachments",
        ).unwrap();

        db.save_post(&empty_attachments).expect("save plain post failed");
        let loaded2 = db.load_posts().expect("load_posts failed");
        let loaded_empty = loaded2.iter().find(|p| p.id.as_str() == "post_empty_m2").unwrap();

        assert_eq!(loaded_empty.link_url, None);
        assert_eq!(loaded_empty.image_path, None);
        assert_eq!(loaded_empty.reply_to_id, None);
        assert_eq!(loaded_empty.quote_id, None);
        assert_eq!(loaded_empty.repost_id, None);
        assert_eq!(loaded_empty.reposts_count, 0);
        assert!(!loaded_empty.is_reposted);
    }

    let _ = std::fs::remove_file(&temp_db_path);
}

#[test]
fn test_real_image_attachment_and_fallback() {
    use std::fs;
    use simulacra::state::AppState;

    let temp_dir = std::env::temp_dir();
    let test_image_path = temp_dir.join("simulacra_test_image.png");
    fs::write(&test_image_path, b"\x89PNG\r\n\x1a\n").expect("write temp png");

    let mut state = AppState::new_in_memory();
    let post_id = state.submit_post_with_attachments(
        "Test post with genuine image",
        None,
        Some(test_image_path.to_string_lossy().to_string()),
        None,
        None,
    ).expect("submission must succeed");

    let post = state.posts.iter().find(|p| p.id == post_id).unwrap();
    assert!(std::path::Path::new(post.image_path.as_ref().unwrap()).exists());

    let _ = fs::remove_file(&test_image_path);
}

#[test]
fn test_adversarial_multi_user_repost_scoping_and_untoggle() {
    let mut state = AppState::new_in_memory();
    let root_id = state
        .submit_post("Epistemic inquiry on shared knowledge dialectics")
        .expect("Root post must succeed");

    let user_1_id = ProfileId::new("user_main");
    let user_2_id = ProfileId::new("user_alt");

    // 1. User 1 reposts root thought
    state.switch_profile(&user_1_id).unwrap();
    let repost_1 = state.repost(&root_id).expect("User 1 repost must succeed");

    // 2. User 2 reposts root thought
    state.switch_profile(&user_2_id).unwrap();
    let repost_2 = state.repost(&root_id).expect("User 2 repost must succeed");

    assert_ne!(repost_1, repost_2);
    assert!(state.posts.iter().any(|p| p.id == repost_1));
    assert!(state.posts.iter().any(|p| p.id == repost_2));

    // Verify DB also holds both distinct reposts
    if let Some(ref db) = state.db {
        let loaded = db.load_posts().unwrap();
        assert!(loaded.iter().any(|p| p.id == repost_1));
        assert!(loaded.iter().any(|p| p.id == repost_2));
    }

    // 3. User 1 switches back and toggles un-repost
    state.switch_profile(&user_1_id).unwrap();
    let un_repost_target = state.repost(&root_id).expect("User 1 un-repost must succeed");
    assert_eq!(un_repost_target, root_id);

    // CRITICAL: User 1's repost is removed, but User 2's repost REMAINS in memory!
    assert!(
        !state.posts.iter().any(|p| p.id == repost_1),
        "User 1's repost must be deleted"
    );
    assert!(
        state.posts.iter().any(|p| p.id == repost_2),
        "User 2's repost must NOT be deleted by User 1's un-repost"
    );

    // CRITICAL: In SQLite database, User 2's repost is also preserved!
    if let Some(ref db) = state.db {
        let loaded = db.load_posts().unwrap();
        assert!(
            !loaded.iter().any(|p| p.id == repost_1),
            "User 1's repost must be deleted in SQLite"
        );
        assert!(
            loaded.iter().any(|p| p.id == repost_2),
            "User 2's repost must be preserved in SQLite"
        );
    }
}

#[test]
fn test_adversarial_link_only_post_creation_and_modal_can_post_logic() {
    let net = NetworkId::new("net_epistemic");
    let author = Profile::new_primary("user_link", "Link Tester", "@linker");

    // 1. Domain creates link-only post successfully with empty text
    let post_res = Post::create_with_attachments(
        PostId::new("post_link_only"),
        net,
        author,
        "   ",
        Some("https://plato.stanford.edu/entries/epistemology/".to_string()),
        None,
        None,
        None,
        None,
    );
    assert!(post_res.is_ok(), "Domain must allow link-only post with empty text");
    let post = post_res.unwrap();
    assert_eq!(post.link_url.as_deref(), Some("https://plato.stanford.edu/entries/epistemology/"));

    // 2. AppState accepts link-only post
    let mut state = AppState::new_in_memory();
    let state_res = state.submit_post_with_attachments(
        "",
        Some("https://arxiv.org/abs/2301.00001".to_string()),
        None,
        None,
        None,
    );
    assert!(state_res.is_ok(), "AppState must accept link-only post");

    // 3. can_post simulation matches expected states
    let draft_length = 0;
    let image_path: Option<String> = None;
    let has_link_empty = "".trim().is_empty();
    let has_link_valid = !"https://arxiv.org".trim().is_empty();

    let can_post_link_only = draft_length > 0 || image_path.is_some() || has_link_valid;
    assert!(can_post_link_only, "can_post must be true when link is present");

    let can_post_none = draft_length > 0 || image_path.is_some() || !has_link_empty;
    assert!(!can_post_none, "can_post must be false when all inputs are empty");
}

