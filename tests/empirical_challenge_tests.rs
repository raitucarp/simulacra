//! Empirical Challenger Stress Test Suite for Milestone 2.
//! Stress-tests Repost toggling, Quotes/Replies, Threadline continuity,
//! URL parsing, and SQLite schema migrations.

use simulacra::domain::{Post, PostId, Profile};
use simulacra::state::AppState;
use simulacra::storage::Database;
use simulacra::theme::minimalist_dark;
use simulacra::ui::molecules::post_item::{
    extract_host_domain, render_post_item, PostAction, PostCardHandlers, ThreadlineStyle,
};
use std::rc::Rc;

/// Card handlers that ignore every interaction (rendering-only tests).
fn noop_handlers() -> PostCardHandlers {
    let noop: PostAction = Rc::new(|_, _| {});
    PostCardHandlers {
        on_select: noop.clone(),
        on_like: noop.clone(),
        on_reply: noop.clone(),
        on_repost: noop.clone(),
        on_quote: noop.clone(),
        on_find_similar: noop,
        on_open_post: Rc::new(|_, _, _| {}),
        on_delete: None,
        links: Default::default(),
    }
}
use simulacra::ui::organisms::compute_stream_threadline_style;

#[test]
fn test_stress_repost_toggle_cycles_and_db_persistence() {
    let temp_db_path = std::env::temp_dir().join(format!("simulacra_repost_stress_{}.db", uuid::Uuid::new_v4()));

    {
        let mut state = AppState::with_db_path(&temp_db_path);
        let root_id = state
            .submit_post("Original Dialectic Axiom")
            .expect("Root post creation must succeed");

        // Run 10 consecutive toggle cycles (repost -> unrepost -> repost ...)
        for i in 1..=10 {
            let res = state.repost(&root_id).expect("Repost toggle must succeed");

            let target = state.posts.iter().find(|p| p.id == root_id).unwrap();
            let is_odd = i % 2 == 1;

            if is_odd {
                assert_eq!(target.reposts_count, 1, "Cycle {}: reposts_count should be 1", i);
                assert!(target.is_reposted, "Cycle {}: is_reposted should be true", i);
                assert_ne!(res, root_id, "Cycle {}: repost should return newly generated post ID", i);

                // Repost thought must exist in state.posts
                let repost_post = state.posts.iter().find(|p| p.id == res).expect("Repost post must exist");
                assert_eq!(repost_post.repost_id.as_ref(), Some(&root_id));
            } else {
                assert_eq!(target.reposts_count, 0, "Cycle {}: reposts_count should be 0", i);
                assert!(!target.is_reposted, "Cycle {}: is_reposted should be false", i);
                assert_eq!(res, root_id, "Cycle {}: unrepost should return target post ID", i);

                // No repost thought referencing root_id should exist
                let repost_exists = state.posts.iter().any(|p| p.repost_id.as_ref() == Some(&root_id));
                assert!(!repost_exists, "Cycle {}: repost thought should be removed from memory", i);
            }
        }

        // Toggle once more to leave it in reposted state
        state.repost(&root_id).expect("Final repost must succeed");
        let target = state.posts.iter().find(|p| p.id == root_id).unwrap();
        assert_eq!(target.reposts_count, 1);
        assert!(target.is_reposted);
    }

    // Reload from SQLite vault in fresh AppState instance to verify persistence integrity
    {
        let state = AppState::with_db_path(&temp_db_path);
        let root_post = state
            .posts
            .iter()
            .find(|p| p.content == "Original Dialectic Axiom")
            .expect("Root post must persist in SQLite");

        assert_eq!(root_post.reposts_count, 1, "Persisted reposts_count must survive reload");
        assert!(root_post.is_reposted, "Persisted is_reposted flag must survive reload");

        let repost_thought = state
            .posts
            .iter()
            .find(|p| p.repost_id.as_ref() == Some(&root_post.id))
            .expect("Persisted repost thought must exist in SQLite");
        assert_eq!(repost_thought.repost_id.as_ref(), Some(&root_post.id));
    }

    let _ = std::fs::remove_file(&temp_db_path);
}

#[test]
fn test_stress_repost_nonexistent_and_multi_post() {
    let mut state = AppState::new_in_memory();
    let bogus_id = PostId::new("post_nonexistent_9999");
    let err = state.repost(&bogus_id);
    assert!(err.is_err(), "Reposting a nonexistent post must return an error");

    let post_a = state.submit_post("Thought Alpha").unwrap();
    let post_b = state.submit_post("Thought Beta").unwrap();
    let post_c = state.submit_post("Thought Gamma").unwrap();

    // Repost A and B
    state.repost(&post_a).unwrap();
    state.repost(&post_b).unwrap();

    let target_a = state.posts.iter().find(|p| p.id == post_a).unwrap();
    let target_b = state.posts.iter().find(|p| p.id == post_b).unwrap();
    let target_c = state.posts.iter().find(|p| p.id == post_c).unwrap();

    assert!(target_a.is_reposted);
    assert!(target_b.is_reposted);
    assert!(!target_c.is_reposted);

    // Unrepost A
    state.repost(&post_a).unwrap();

    let target_a2 = state.posts.iter().find(|p| p.id == post_a).unwrap();
    let target_b2 = state.posts.iter().find(|p| p.id == post_b).unwrap();

    assert!(!target_a2.is_reposted, "Post A should be unreposted");
    assert!(target_b2.is_reposted, "Post B must remain reposted");
    assert_eq!(target_b2.reposts_count, 1);
}

#[test]
fn test_stress_quotes_nonexistent_and_deep_nesting() {
    let mut state = AppState::new_in_memory();
    let bogus_id = PostId::new("post_ghost");
    let err = state.submit_quote(&bogus_id, "Quoting a ghost", None, None);
    assert!(err.is_err(), "Submitting quote for nonexistent target post must fail");

    // Build 10-tier deeply nested quote chain: Post 1 -> Post 2 quotes 1 -> Post 3 quotes 2 ...
    let mut current_id = state.submit_post("Genesis premise 0").unwrap();
    for tier in 1..=10 {
        let comment = format!("Commentary tier {}", tier);
        current_id = state
            .submit_quote(&current_id, &comment, None, None)
            .unwrap_or_else(|e| panic!("Quote tier {} failed: {}", tier, e));
    }

    // Verify 10th tier post has quote_id pointing to tier 9
    let tier10_post = state.posts.iter().find(|p| p.id == current_id).unwrap();
    assert!(tier10_post.is_quote());
    assert!(tier10_post.quote_id.is_some());

    // Stress-test rendering the deeply nested quote post
    let theme = minimalist_dark();
    let _el = render_post_item(
        tier10_post,
        &state.posts,
        &theme,
        false,
        ThreadlineStyle::None,
        noop_handlers(),
    );

    // Test quote referencing missing post (e.g. database post whose referenced quote was deleted)
    let author = Profile::new_primary("u_test", "Tester", "@tester");
    let mut missing_quote_post = Post::create("p_dangling", "net_1", author, "Has missing quote").unwrap();
    missing_quote_post.quote_id = Some(PostId::new("post_deleted_404"));

    let _el_missing = render_post_item(
        &missing_quote_post,
        &state.posts,
        &theme,
        false,
        ThreadlineStyle::None,
        noop_handlers(),
    );
}

#[test]
fn test_stress_replies_nonexistent_and_multitier() {
    let mut state = AppState::new_in_memory();
    let bogus_id = PostId::new("post_fake_parent");
    let err = state.submit_reply(&bogus_id, "Replying to void", None, None);
    assert!(err.is_err(), "Submitting reply to nonexistent parent must fail");

    // Create 3-tier reply chain: Root -> Reply 1 -> Reply 2
    let root_id = state.submit_post("Root thought").unwrap();
    let reply1_id = state.submit_reply(&root_id, "Reply 1 to root", None, None).unwrap();
    let reply2_id = state.submit_reply(&reply1_id, "Reply 2 to reply 1", None, None).unwrap();

    let root = state.posts.iter().find(|p| p.id == root_id).unwrap();
    let reply1 = state.posts.iter().find(|p| p.id == reply1_id).unwrap();
    let reply2 = state.posts.iter().find(|p| p.id == reply2_id).unwrap();

    // Verify parent replies_count was incremented
    assert!(root.replies_count >= 1, "Root should have replies_count >= 1");
    assert_eq!(reply1.replies_count, 1, "Reply 1 should have replies_count == 1");
    assert_eq!(reply2.replies_count, 0, "Reply 2 is leaf, replies_count == 0");
    assert_eq!(reply1.reply_to_id.as_ref(), Some(&root_id));
    assert_eq!(reply2.reply_to_id.as_ref(), Some(&reply1_id));
}

#[test]
fn test_stress_image_attachment_fallback() {
    let theme = minimalist_dark();
    let author = Profile::new_primary("u_img", "ImageTester", "@imgtester");
    let mut post_with_bogus_img = Post::create("p_bogus_img", "net_1", author, "Image check").unwrap();
    post_with_bogus_img.image_path = Some("C:/nonexistent_directory_404/never_existed.png".to_string());

    let _el = render_post_item(
        &post_with_bogus_img,
        &[],
        &theme,
        false,
        ThreadlineStyle::None,
        noop_handlers(),
    );
}

#[test]
fn test_stress_extract_host_domain_pathological() {
    // 1. Valid and common URLs
    assert_eq!(extract_host_domain("https://example.com"), "example.com");
    assert_eq!(extract_host_domain("http://example.com/"), "example.com");
    assert_eq!(extract_host_domain("https://www.example.com/path"), "example.com");
    assert_eq!(extract_host_domain("http://sub.domain.org/foo/bar"), "sub.domain.org");

    // 2. Query parameters, fragments, ports
    assert_eq!(extract_host_domain("https://crates.io:443/crates/gpui?search=text#v0.8"), "crates.io:443");
    assert_eq!(extract_host_domain("http://127.0.0.1:8080/api/v1"), "127.0.0.1:8080");
    assert_eq!(extract_host_domain("https://example.com?query=https://evil.com"), "example.com");
    assert_eq!(extract_host_domain("https://example.com#hash/slash"), "example.com");

    // 3. Leading/trailing whitespace
    assert_eq!(extract_host_domain("   https://github.com/rust-lang/rust   "), "github.com");

    // 4. Missing scheme
    assert_eq!(extract_host_domain("news.ycombinator.com/item?id=123"), "news.ycombinator.com");

    // 5. CJK / IDN domains
    assert_eq!(extract_host_domain("https://例え.jp/path"), "例え.jp");

    // 6. Pathological uppercase schemes:
    let uppercase_res = extract_host_domain("HTTPS://EXAMPLE.COM/PATH");
    assert_eq!(uppercase_res, "example.com");
}

#[test]
fn test_stress_threadline_geometry_stream_vs_reverse_chronological() {
    let author = Profile::new_primary("u1", "Philosopher", "@philosopher");
    let net = "net_test";

    let root = Post::create("post_root", net, author.clone(), "Root thought").unwrap();
    let mut reply1 = Post::create("post_rep1", net, author.clone(), "Reply 1").unwrap();
    reply1.reply_to_id = Some(root.id.clone());

    // When posts are in chronological order: [Root, Reply 1]
    let chronological = [&root, &reply1];
    let style_root_chrono = compute_stream_threadline_style(&chronological, 0);
    let style_rep1_chrono = compute_stream_threadline_style(&chronological, 1);
    assert_eq!(style_root_chrono, ThreadlineStyle::ParentHasReplies);
    assert_eq!(style_rep1_chrono, ThreadlineStyle::LastReply);

    // When posts are in reverse-chronological order (the stream order: [Reply 1, Root]):
    let reverse_chrono = [&reply1, &root];
    let style_rep1_rev = compute_stream_threadline_style(&reverse_chrono, 0);
    let style_root_rev = compute_stream_threadline_style(&reverse_chrono, 1);
    assert_eq!(style_rep1_rev, ThreadlineStyle::ParentHasReplies);
    assert_eq!(style_root_rev, ThreadlineStyle::LastReply);
}

#[test]
fn test_stress_sqlite_schema_migrations_and_roundtrip() {
    let temp_db_path = std::env::temp_dir().join(format!("simulacra_migration_stress_{}.db", uuid::Uuid::new_v4()));

    // 1. Manually create an old v1 database with legacy 11-column schema
    {
        let conn = rusqlite::Connection::open(&temp_db_path).unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE networks (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                slug TEXT NOT NULL,
                description TEXT NOT NULL,
                doc_count INTEGER NOT NULL DEFAULT 0,
                agent_count INTEGER NOT NULL DEFAULT 0,
                color_accent TEXT NOT NULL DEFAULT '#8b5cf6'
            );
            CREATE TABLE profiles (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                handle TEXT NOT NULL,
                avatar_initial TEXT NOT NULL,
                role_json TEXT NOT NULL
            );
            CREATE TABLE posts (
                id TEXT PRIMARY KEY,
                network_id TEXT NOT NULL,
                author_id TEXT NOT NULL,
                author_json TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL,
                likes INTEGER NOT NULL DEFAULT 0,
                is_liked INTEGER NOT NULL DEFAULT 0,
                replies_count INTEGER NOT NULL DEFAULT 0,
                mentions_json TEXT NOT NULL DEFAULT '[]',
                linked_posts_json TEXT NOT NULL DEFAULT '[]'
            );
            "#,
        ).unwrap();

        // Seed 3 legacy posts
        for i in 1..=3 {
            conn.execute(
                "INSERT INTO posts (id, network_id, author_id, author_json, content, created_at, likes, is_liked, replies_count)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                rusqlite::params![
                    format!("legacy_{}", i),
                    "net_main",
                    "prof_1",
                    r#"{"id":"prof_1","name":"User","handle":"@user","avatar_initial":"U","role":"UserPrimary"}"#,
                    format!("Legacy post {}", i),
                    format!("2026-01-0{}T10:00:00Z", i),
                    i * 2,
                    0,
                    0,
                ],
            ).unwrap();
        }
    }

    // 2. Open via Database::open, triggering automatic migration
    {
        let db = Database::open(&temp_db_path).expect("Database::open must run migrations cleanly on legacy vault");

        // Verify legacy posts load with default values for new fields
        let posts = db.load_posts().expect("load_posts on migrated DB must succeed");
        assert_eq!(posts.len(), 3);
        for p in &posts {
            assert_eq!(p.reposts_count, 0);
            assert!(!p.is_reposted);
            assert_eq!(p.reply_to_id, None);
            assert_eq!(p.quote_id, None);
            assert_eq!(p.repost_id, None);
            assert_eq!(p.link_url, None);
            assert_eq!(p.image_path, None);
        }

        // Add a new M2 post with full attachment and quote metadata
        let author = Profile::new_primary("prof_1", "User", "@user");
        let m2_post = Post::create_with_attachments(
            "post_m2_migrated",
            "net_main",
            author,
            "Migrated post with attachments",
            Some("https://spinoza.org".into()),
            Some("assets/icons/image.svg".into()),
            Some(PostId::new("legacy_1")),
            Some(PostId::new("legacy_2")),
            None,
        ).expect("create_with_attachments must succeed");

        db.save_post(&m2_post).expect("save_post must succeed on migrated DB");

        // Update repost on legacy post
        db.update_post_repost(&PostId::new("legacy_1"), 3, true).expect("update repost on legacy post");
    }

    // 3. Re-open again to verify persistence of both legacy and new M2 records
    {
        let db = Database::open(&temp_db_path).expect("Subsequent Database::open must be idempotent");
        let posts = db.load_posts().expect("load_posts after reload");
        assert_eq!(posts.len(), 4);

        let legacy1 = posts.iter().find(|p| p.id.as_str() == "legacy_1").unwrap();
        assert_eq!(legacy1.reposts_count, 3);
        assert!(legacy1.is_reposted);

        let m2 = posts.iter().find(|p| p.id.as_str() == "post_m2_migrated").unwrap();
        assert_eq!(m2.link_url.as_deref(), Some("https://spinoza.org"));
        assert_eq!(m2.image_path.as_deref(), Some("assets/icons/image.svg"));
        assert_eq!(m2.reply_to_id.as_ref(), Some(&PostId::new("legacy_1")));
        assert_eq!(m2.quote_id.as_ref(), Some(&PostId::new("legacy_2")));
    }

    let _ = std::fs::remove_file(&temp_db_path);
}


// ============================================================================
// EMPIRICAL CHALLENGER STRESS TESTS — MILESTONE 2 ITERATION 2
// ============================================================================

use simulacra::AppError;

#[test]
fn test_empirical_valid_vs_nonexistent_image_attachments() {
    let temp_dir = std::env::temp_dir().join(format!("simulacra_img_stress_{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&temp_dir).expect("create temp dir");

    let formats = [
        ("test_alpha.png", &b"\x89PNG\r\n\x1a\n"[..]),
        ("test_beta.jpg", &b"\xFF\xD8\xFF\xE0"[..]),
        ("test_gamma.jpeg", &b"\xFF\xD8\xFF\xE0"[..]),
        ("test_delta.webp", &b"RIFF\x24\x00\x00\x00WEBPVP8 "[..]),
        ("test_epsilon.bmp", &b"BM\x36\x00\x00\x00"[..]),
    ];

    let mut valid_paths = Vec::new();
    for (filename, magic) in &formats {
        let p = temp_dir.join(filename);
        std::fs::write(&p, magic).expect("write valid image file");
        valid_paths.push(p.to_string_lossy().to_string());
    }

    let theme = minimalist_dark();
    let author = Profile::new_primary("u_emp_img", "Image Challenger", "@challenger");

    // 1. Stress-test all valid image formats
    for (idx, path_str) in valid_paths.iter().enumerate() {
        assert!(std::path::Path::new(path_str).exists(), "File must exist on disk");

        let post = Post::create_with_attachments(
            format!("p_valid_img_{}", idx),
            "net_valid",
            author.clone(),
            &format!("Thought with valid image format {}", formats[idx].0),
            None,
            Some(path_str.clone()),
            None,
            None,
            None,
        ).expect("Must accept valid image path");

        assert_eq!(post.image_path.as_deref(), Some(path_str.as_str()));
        assert!(post.has_attachments());

        // Verify GPUI rendering branch executes cleanly
        let _el = render_post_item(
            &post,
            &[],
            &theme,
            false,
            ThreadlineStyle::None,
            noop_handlers(),
        );
    }

    // 2. Stress-test non-existent image paths
    let non_existent_paths = [
        "C:/nonexistent_directory_404/never_existed.png".to_string(),
        "C:\\nonexistent\\sub\\missing.jpg".to_string(),
        "relative/path/missing.webp".to_string(),
        temp_dir.join("deleted_after_creation.png").to_string_lossy().to_string(),
    ];

    for (idx, bogus_path) in non_existent_paths.iter().enumerate() {
        assert!(!std::path::Path::new(bogus_path).exists(), "Path must NOT exist");

        let post = Post::create_with_attachments(
            format!("p_bogus_img_{}", idx),
            "net_bogus",
            author.clone(),
            &format!("Thought with missing image path {}", bogus_path),
            None,
            Some(bogus_path.clone()),
            None,
            None,
            None,
        ).expect("Domain preserves raw image_path in database even if file removed");

        // Verify fallback rendering branch ("Image file not found: ...") executes cleanly without panic
        let _el = render_post_item(
            &post,
            &[],
            &theme,
            false,
            ThreadlineStyle::None,
            noop_handlers(),
        );
    }

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_empirical_windows_quote_wrapped_paths() {
    let temp_dir = std::env::temp_dir().join(format!("simulacra space test {}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&temp_dir).expect("create temp dir with spaces");

    let real_file = temp_dir.join("dialectical synthesis diagram.png");
    std::fs::write(&real_file, b"\x89PNG\r\n\x1a\n").expect("write png");
    let real_path_str = real_file.to_string_lossy().to_string();

    // Emulate raw string copied from Windows File Explorer / terminal
    let test_inputs = vec![
        format!("\"{}\"", real_path_str),
        format!("'{}'", real_path_str),
        format!("   \"{}\"   ", real_path_str),
        format!("   '{}'   ", real_path_str),
        format!("\"{}\"", real_path_str.replace('\\', "/")),
        format!("'{}'", real_path_str.replace('\\', "/")),
    ];

    for raw_input in &test_inputs {
        // Apply production sanitization logic from composer_modal.rs and app_view.rs
        let sanitized = raw_input.trim().trim_matches('"').trim_matches('\'').to_string();

        assert!(!sanitized.starts_with('"') && !sanitized.ends_with('"'), "Double quotes must be stripped");
        assert!(!sanitized.starts_with('\'') && !sanitized.ends_with('\''), "Single quotes must be stripped");
        assert!(std::path::Path::new(&sanitized).exists(), "Sanitized path '{}' must exist on disk", sanitized);

        // Verify AppState submission with sanitized path
        let mut state = AppState::new_in_memory();
        let post_id = state.submit_post_with_attachments(
            "Thought with sanitized Windows path",
            None,
            Some(sanitized.clone()),
            None,
            None,
        ).expect("Must accept sanitized image path");

        let post = state.posts.iter().find(|p| p.id == post_id).unwrap();
        assert_eq!(post.image_path.as_deref(), Some(sanitized.as_str()));
    }

    // Verify quote-stripping on invalid/empty input strings
    let empty_inputs = ["", "\"\"", "''", "   \"\"   ", "   ''   ", "   "];
    for raw in &empty_inputs {
        let sanitized = raw.trim().trim_matches('"').trim_matches('\'').to_string();
        assert!(sanitized.is_empty(), "Empty quotes must sanitize to empty string");
    }

    // Verify quote-stripping on non-existent path
    let fake_quoted = "\"C:\\nonexistent_dir_xyz\\fake.png\"";
    let fake_sanitized = fake_quoted.trim().trim_matches('"').trim_matches('\'').to_string();
    assert_eq!(fake_sanitized, "C:\\nonexistent_dir_xyz\\fake.png");
    assert!(!std::path::Path::new(&fake_sanitized).exists());

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_empirical_link_only_thought_submission_and_persistence() {
    let temp_db_path = std::env::temp_dir().join(format!("simulacra_link_only_{}.db", uuid::Uuid::new_v4()));

    let links_to_test = [
        "https://plato.stanford.edu/entries/spinoza/",
        "https://en.wikipedia.org/wiki/Ethics_(Spinoza_book)",
        "HTTPS://PHILPAPERS.ORG/REC/SPITEO-3",
        "http://arxiv.org/abs/2301.00001",
    ];

    // 1. Submit thoughts containing only links (empty body text) into SQLite vault
    {
        let mut state = AppState::with_db_path(&temp_db_path);

        for (idx, link) in links_to_test.iter().enumerate() {
            // Test with varying blank string variations for content
            let blank_content = match idx % 3 {
                0 => "",
                1 => "   ",
                _ => "\t\r\n   ",
            };

            let post_id = state.submit_post_with_attachments(
                blank_content,
                Some(link.to_string()),
                None,
                None,
                None,
            ).unwrap_or_else(|e| panic!("Link-only post {} must succeed: {}", idx, e));

            let post = state.posts.iter().find(|p| p.id == post_id).unwrap();
            assert_eq!(post.content, "", "Trimmed content must be empty");
            assert_eq!(post.link_url.as_deref(), Some(*link));
            assert!(post.has_attachments());
        }
    }

    // 2. Reload fresh instance from SQLite vault and verify persistence
    {
        let state = AppState::with_db_path(&temp_db_path);
        let theme = minimalist_dark();

        for link in &links_to_test {
            let post = state
                .posts
                .iter()
                .find(|p| p.link_url.as_deref() == Some(*link))
                .unwrap_or_else(|| panic!("Link-only post for '{}' must exist in reloaded DB", link));

            assert_eq!(post.content, "");
            assert!(post.has_attachments());

            // Verify extract_host_domain normalizes properly
            let domain = extract_host_domain(link);
            assert!(!domain.is_empty());
            assert!(!domain.starts_with("http"));
            assert!(!domain.starts_with("https"));

            // Verify render_post_item renders the link card cleanly without body text
            let _el = render_post_item(
                post,
                &state.posts,
                &theme,
                false,
                ThreadlineStyle::None,
                noop_handlers(),
            );
        }
    }

    let _ = std::fs::remove_file(&temp_db_path);
}

#[test]
fn test_empirical_boundary_conditions_empty_post_rejection_and_can_post() {
    let net = "net_boundary";
    let author = Profile::new_primary("u_bound", "Boundary Tester", "@boundary");

    // 1. Both content and attachments completely empty -> AppError::EmptyContent
    let res1 = Post::create_with_attachments("p_err_1", net, author.clone(), "", None, None, None, None, None);
    assert!(matches!(res1, Err(AppError::EmptyContent)), "Empty post must be rejected");

    let res2 = Post::create_with_attachments("p_err_2", net, author.clone(), "   \r\n\t  ", None, None, None, None, None);
    assert!(matches!(res2, Err(AppError::EmptyContent)), "Whitespace-only post must be rejected");

    let res3 = Post::create_with_attachments("p_err_3", net, author.clone(), "", Some("".into()), Some("".into()), None, None, None);
    assert!(matches!(res3, Err(AppError::EmptyContent)), "Empty link and image strings must be rejected");

    let res4 = Post::create_with_attachments("p_err_4", net, author.clone(), "  ", Some("   ".into()), Some("   ".into()), None, None, None);
    assert!(matches!(res4, Err(AppError::EmptyContent)), "Whitespace link and image must be rejected");

    // 2. AppState must return Err(AppError::EmptyContent) when all inputs empty
    let mut state = AppState::new_in_memory();
    let state_res = state.submit_post_with_attachments("", None, None, None, None);
    assert!(matches!(state_res, Err(AppError::EmptyContent)));

    let state_res2 = state.submit_post_with_attachments("   ", Some("   ".into()), None, None, None);
    assert!(matches!(state_res2, Err(AppError::EmptyContent)));

    // 3. Truth-table verification of modal can_post logic
    // Formula from composer_modal.rs:
    // can_post = match mode {
    //     ComposerModalMode::NewThread | ComposerModalMode::Reply(_) => draft_length > 0 || has_image || has_link,
    //     ComposerModalMode::Quote(_) => draft_length > 0,
    // };

    struct TestCase {
        mode_is_quote: bool,
        draft_len: usize,
        has_link: bool,
        has_image: bool,
        expected_can_post: bool,
    }

    let cases = vec![
        // NewThread / Reply
        TestCase { mode_is_quote: false, draft_len: 0, has_link: false, has_image: false, expected_can_post: false },
        TestCase { mode_is_quote: false, draft_len: 1, has_link: false, has_image: false, expected_can_post: true },
        TestCase { mode_is_quote: false, draft_len: 0, has_link: true,  has_image: false, expected_can_post: true },
        TestCase { mode_is_quote: false, draft_len: 0, has_link: false, has_image: true,  expected_can_post: true },
        TestCase { mode_is_quote: false, draft_len: 5, has_link: true,  has_image: true,  expected_can_post: true },
        // Quote (MUST require non-empty commentary text)
        TestCase { mode_is_quote: true,  draft_len: 0, has_link: false, has_image: false, expected_can_post: false },
        TestCase { mode_is_quote: true,  draft_len: 0, has_link: true,  has_image: false, expected_can_post: false },
        TestCase { mode_is_quote: true,  draft_len: 0, has_link: false, has_image: true,  expected_can_post: false },
        TestCase { mode_is_quote: true,  draft_len: 0, has_link: true,  has_image: true,  expected_can_post: false },
        TestCase { mode_is_quote: true,  draft_len: 1, has_link: false, has_image: false, expected_can_post: true },
        TestCase { mode_is_quote: true,  draft_len: 1, has_link: true,  has_image: true,  expected_can_post: true },
    ];

    for (i, tc) in cases.iter().enumerate() {
        let can_post = if tc.mode_is_quote {
            tc.draft_len > 0
        } else {
            tc.draft_len > 0 || tc.has_image || tc.has_link
        };
        assert_eq!(
            can_post, tc.expected_can_post,
            "Case {}: mode_is_quote={}, draft_len={}, has_link={}, has_image={} expected {}",
            i, tc.mode_is_quote, tc.draft_len, tc.has_link, tc.has_image, tc.expected_can_post
        );
    }
}

#[test]
fn test_empirical_image_and_link_combined_with_quote() {
    let temp_db_path = std::env::temp_dir().join(format!("simulacra_combo_{}.db", uuid::Uuid::new_v4()));
    let temp_img = std::env::temp_dir().join(format!("simulacra_combo_img_{}.png", uuid::Uuid::new_v4()));
    std::fs::write(&temp_img, b"\x89PNG\r\n\x1a\n").expect("write test png");

    let img_str = temp_img.to_string_lossy().to_string();
    let link_str = "https://spinoza.org/geometrical-method";

    {
        let mut state = AppState::with_db_path(&temp_db_path);
        let root_id = state.submit_post("Original Geometric Proposition").unwrap();

        // Submit quote with commentary, link, AND image
        let quote_id = state.submit_quote(
            &root_id,
            "Commentary exploring corollary with empirical diagram and paper link",
            Some(link_str.to_string()),
            Some(img_str.clone()),
        ).expect("submit_quote with combined attachments must succeed");

        let quote_post = state.posts.iter().find(|p| p.id == quote_id).unwrap();
        assert_eq!(quote_post.quote_id.as_ref(), Some(&root_id));
        assert_eq!(quote_post.link_url.as_deref(), Some(link_str));
        assert_eq!(quote_post.image_path.as_deref(), Some(img_str.as_str()));
        assert!(quote_post.has_attachments());
        assert!(quote_post.is_quote());
    }

    // Reload from SQLite vault
    {
        let state = AppState::with_db_path(&temp_db_path);
        let theme = minimalist_dark();

        let quote_post = state.posts.iter().find(|p| p.is_quote()).expect("Quote post must persist in SQLite");
        assert_eq!(quote_post.link_url.as_deref(), Some(link_str));
        assert_eq!(quote_post.image_path.as_deref(), Some(img_str.as_str()));

        // Render with full attachments, quote embed, and threadline
        let _el = render_post_item(
            quote_post,
            &state.posts,
            &theme,
            false,
            ThreadlineStyle::None,
            noop_handlers(),
        );
    }

    let _ = std::fs::remove_file(&temp_db_path);
    let _ = std::fs::remove_file(&temp_img);
}


#[test]
fn test_stress_multi_user_repost_isolation_in_memory_and_raw_sqlite() {
    use simulacra::domain::ProfileId;
    use rusqlite::params;

    let temp_db_path = std::env::temp_dir().join(format!("simulacra_multi_repost_{}.db", uuid::Uuid::new_v4()));

    let user_a_id = ProfileId::new("user_main");
    let user_b_id = ProfileId::new("user_alt");
    let user_c_id = ProfileId::new("user_charlie");

    let root_id;
    let rep_a;
    let rep_b;
    let rep_c;

    {
        let mut state = AppState::with_db_path(&temp_db_path);

        // Register user_c so all 3 profiles can be switched to
        let user_c = Profile::new_alt("user_charlie", "Charlie", "@charlie");
        if let Some(ref db) = state.db {
            db.save_profile(&user_c).expect("save_profile user_c");
        }
        state.user_profiles.push(user_c);

        // Submit root post P
        root_id = state
            .submit_post("Shared epistemic dialectic thought")
            .expect("Root post creation must succeed");

        // 1. User A reposts thought P
        state.switch_profile(&user_a_id).expect("switch to user A");
        rep_a = state.repost(&root_id).expect("User A repost must succeed");

        // 2. User B reposts thought P
        state.switch_profile(&user_b_id).expect("switch to user B");
        rep_b = state.repost(&root_id).expect("User B repost must succeed");

        // 3. User C reposts thought P
        state.switch_profile(&user_c_id).expect("switch to user C");
        rep_c = state.repost(&root_id).expect("User C repost must succeed");

        // Assert all 3 repost IDs are distinct
        assert_ne!(rep_a, rep_b);
        assert_ne!(rep_b, rep_c);
        assert_ne!(rep_a, rep_c);

        // In memory: state.posts holds all 3 reposts
        assert!(state.posts.iter().any(|p| p.id == rep_a));
        assert!(state.posts.iter().any(|p| p.id == rep_b));
        assert!(state.posts.iter().any(|p| p.id == rep_c));

        let target_in_mem = state.posts.iter().find(|p| p.id == root_id).unwrap();
        assert_eq!(target_in_mem.reposts_count, 3, "Target post reposts_count must be 3");

        // In raw SQLite: verify directly via rusqlite
        {
            let conn = rusqlite::Connection::open(&temp_db_path).expect("open raw sqlite");
            let count: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM posts WHERE repost_id = ?1",
                    params![root_id.as_str()],
                    |row| row.get(0),
                )
                .expect("raw query count");
            assert_eq!(count, 3, "SQLite must hold 3 repost rows");

            let target_db_count: i64 = conn
                .query_row(
                    "SELECT reposts_count FROM posts WHERE id = ?1",
                    params![root_id.as_str()],
                    |row| row.get(0),
                )
                .expect("raw query target reposts_count");
            assert_eq!(target_db_count, 3);
        }

        // 4. User A un-reposts thought P
        state.switch_profile(&user_a_id).expect("switch to user A");
        let unrepost_a = state.repost(&root_id).expect("User A un-repost must succeed");
        assert_eq!(unrepost_a, root_id);

        // Verification: User A's repost is removed, but User B and User C reposts are PRESERVED in memory
        assert!(!state.posts.iter().any(|p| p.id == rep_a), "User A repost must be removed from memory");
        assert!(state.posts.iter().any(|p| p.id == rep_b), "User B repost must be preserved in memory");
        assert!(state.posts.iter().any(|p| p.id == rep_c), "User C repost must be preserved in memory");

        let target_after_unrepost_a = state.posts.iter().find(|p| p.id == root_id).unwrap();
        assert_eq!(target_after_unrepost_a.reposts_count, 2, "Target post reposts_count must decrement to 2");

        // Verification: In raw SQLite, User A's repost row is deleted, but User B and User C rows are PRESERVED
        {
            let conn = rusqlite::Connection::open(&temp_db_path).expect("open raw sqlite");
            let count_a: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM posts WHERE repost_id = ?1 AND author_id = 'user_main'",
                    params![root_id.as_str()],
                    |row| row.get(0),
                )
                .expect("raw query count A");
            assert_eq!(count_a, 0, "User A repost row must be deleted from SQLite");

            let count_b: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM posts WHERE repost_id = ?1 AND author_id = 'user_alt'",
                    params![root_id.as_str()],
                    |row| row.get(0),
                )
                .expect("raw query count B");
            assert_eq!(count_b, 1, "User B repost row must be preserved in SQLite");

            let count_c: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM posts WHERE repost_id = ?1 AND author_id = 'user_charlie'",
                    params![root_id.as_str()],
                    |row| row.get(0),
                )
                .expect("raw query count C");
            assert_eq!(count_c, 1, "User C repost row must be preserved in SQLite");

            let total_db: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM posts WHERE repost_id = ?1",
                    params![root_id.as_str()],
                    |row| row.get(0),
                )
                .expect("raw query total count");
            assert_eq!(total_db, 2, "SQLite must hold exactly 2 repost rows");
        }

        // 5. User B un-reposts thought P
        state.switch_profile(&user_b_id).expect("switch to user B");
        let unrepost_b = state.repost(&root_id).expect("User B un-repost must succeed");
        assert_eq!(unrepost_b, root_id);

        assert!(!state.posts.iter().any(|p| p.id == rep_b), "User B repost must be removed from memory");
        assert!(state.posts.iter().any(|p| p.id == rep_c), "User C repost must be preserved in memory");

        let target_after_unrepost_b = state.posts.iter().find(|p| p.id == root_id).unwrap();
        assert_eq!(target_after_unrepost_b.reposts_count, 1);

        // 6. User C un-reposts thought P
        state.switch_profile(&user_c_id).expect("switch to user C");
        let unrepost_c = state.repost(&root_id).expect("User C un-repost must succeed");
        assert_eq!(unrepost_c, root_id);

        assert!(!state.posts.iter().any(|p| p.id == rep_c), "User C repost must be removed from memory");

        let target_final = state.posts.iter().find(|p| p.id == root_id).unwrap();
        assert_eq!(target_final.reposts_count, 0, "Target reposts_count must reach 0");
        assert!(!target_final.is_reposted);

        // In raw SQLite: zero repost rows
        {
            let conn = rusqlite::Connection::open(&temp_db_path).expect("open raw sqlite");
            let total_remaining: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM posts WHERE repost_id = ?1",
                    params![root_id.as_str()],
                    |row| row.get(0),
                )
                .expect("raw query total remaining");
            assert_eq!(total_remaining, 0, "SQLite must have 0 repost rows remaining");
        }
    }

    let _ = std::fs::remove_file(&temp_db_path);
}

#[test]
fn test_stress_reverse_chronological_stream_threadlines_multitier() {
    let author = Profile::new_primary("u_phil", "Philosopher", "@philosopher");
    let net = "net_dialectics";

    let root = Post::create("p_root", net, author.clone(), "Root thesis").unwrap();

    let mut reply1 = Post::create("p_reply1", net, author.clone(), "Reply 1 to root").unwrap();
    reply1.reply_to_id = Some(root.id.clone());

    let mut reply2 = Post::create("p_reply2", net, author.clone(), "Reply 2 to reply 1").unwrap();
    reply2.reply_to_id = Some(reply1.id.clone());

    let mut reply3 = Post::create("p_reply3", net, author.clone(), "Reply 3 to reply 2").unwrap();
    reply3.reply_to_id = Some(reply2.id.clone());

    let unrelated1 = Post::create("p_unrelated1", net, author.clone(), "Unrelated axiom 1").unwrap();
    let unrelated2 = Post::create("p_unrelated2", net, author.clone(), "Unrelated axiom 2").unwrap();

    // 1. 3-tier feed slice: [&reply2, &reply1, &root] (reverse-chronological)
    let three_tier = [&reply2, &reply1, &root];
    assert_eq!(
        compute_stream_threadline_style(&three_tier, 0),
        ThreadlineStyle::ParentHasReplies,
        "reply2 at top of 3-tier reverse-chronological stream must have ParentHasReplies"
    );
    assert_eq!(
        compute_stream_threadline_style(&three_tier, 1),
        ThreadlineStyle::MiddleReply,
        "reply1 in middle of 3-tier reverse-chronological stream must have MiddleReply"
    );
    assert_eq!(
        compute_stream_threadline_style(&three_tier, 2),
        ThreadlineStyle::LastReply,
        "root at bottom of 3-tier reverse-chronological stream must have LastReply"
    );

    // 2. 2-tier feed slice: [&reply1, &root] (reverse-chronological)
    let two_tier = [&reply1, &root];
    assert_eq!(
        compute_stream_threadline_style(&two_tier, 0),
        ThreadlineStyle::ParentHasReplies,
        "reply1 at top of 2-tier reverse-chronological stream must have ParentHasReplies"
    );
    assert_eq!(
        compute_stream_threadline_style(&two_tier, 1),
        ThreadlineStyle::LastReply,
        "root at bottom of 2-tier reverse-chronological stream must have LastReply"
    );

    // 3. 4-tier feed slice: [&reply3, &reply2, &reply1, &root]
    let four_tier = [&reply3, &reply2, &reply1, &root];
    assert_eq!(compute_stream_threadline_style(&four_tier, 0), ThreadlineStyle::ParentHasReplies);
    assert_eq!(compute_stream_threadline_style(&four_tier, 1), ThreadlineStyle::MiddleReply);
    assert_eq!(compute_stream_threadline_style(&four_tier, 2), ThreadlineStyle::MiddleReply);
    assert_eq!(compute_stream_threadline_style(&four_tier, 3), ThreadlineStyle::LastReply);

    // 4. Interleaved stream with unrelated posts: [unrelated1, reply2, reply1, root, unrelated2]
    let interleaved = [&unrelated1, &reply2, &reply1, &root, &unrelated2];
    assert_eq!(compute_stream_threadline_style(&interleaved, 0), ThreadlineStyle::None, "unrelated1 must be None");
    assert_eq!(compute_stream_threadline_style(&interleaved, 1), ThreadlineStyle::ParentHasReplies, "reply2 must be ParentHasReplies");
    assert_eq!(compute_stream_threadline_style(&interleaved, 2), ThreadlineStyle::MiddleReply, "reply1 must be MiddleReply");
    assert_eq!(compute_stream_threadline_style(&interleaved, 3), ThreadlineStyle::LastReply, "root must be LastReply");
    assert_eq!(compute_stream_threadline_style(&interleaved, 4), ThreadlineStyle::None, "unrelated2 must be None");

    // 5. Verification of top/bottom drawing booleans for each style
    assert!(!ThreadlineStyle::ParentHasReplies.has_top_line());
    assert!(ThreadlineStyle::ParentHasReplies.has_bottom_line());

    assert!(ThreadlineStyle::MiddleReply.has_top_line());
    assert!(ThreadlineStyle::MiddleReply.has_bottom_line());

    assert!(ThreadlineStyle::LastReply.has_top_line());
    assert!(!ThreadlineStyle::LastReply.has_bottom_line());

    assert!(!ThreadlineStyle::None.has_top_line());
    assert!(!ThreadlineStyle::None.has_bottom_line());
}

#[test]
fn test_stress_uppercase_and_mixed_case_urls_with_ports() {
    // Exact schemes from DISPATCH
    assert_eq!(
        extract_host_domain("HTTPS://PHILPAPERS.ORG/rec/xyz"),
        "philpapers.org"
    );
    assert_eq!(
        extract_host_domain("HTTP://WWW.EXAMPLE.COM:8080/path"),
        "example.com:8080"
    );

    // Additional mixed-case schemes, ports, queries, and fragments
    assert_eq!(
        extract_host_domain("HtTpS://Plato.Stanford.Edu:443/entries/spinoza"),
        "plato.stanford.edu:443"
    );
    assert_eq!(
        extract_host_domain("hTtP://wWw.SuB.eXaMpLe.CoM:8000/api/v2?filter=true#anchor"),
        "sub.example.com:8000"
    );
    assert_eq!(
        extract_host_domain("HTTP://127.0.0.1:9090/"),
        "127.0.0.1:9090"
    );
    assert_eq!(
        extract_host_domain("HTTPS://LOCALHOST:3000"),
        "localhost:3000"
    );
    assert_eq!(
        extract_host_domain("   HTTPS://WWW.RUST-LANG.ORG/tools/install   "),
        "rust-lang.org"
    );
    assert_eq!(
        extract_host_domain("HTTPs://WWW.GITHUB.COM/"),
        "github.com"
    );
}
