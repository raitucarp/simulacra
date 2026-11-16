use crate::domain::{NetworkId, Post, PostId, Profile};
use crate::storage::db::Database;

#[test]
fn test_db_post_attachments_and_repost_persistence() {
    let db = Database::in_memory().expect("in-memory db");
    let author = Profile::new_primary("u1", "Architect", "@arch");
    let post = Post::create_with_attachments(
        "p_db_test",
        "net_philosophy",
        author.clone(),
        "Persisting attachments and quote",
        Some("https://simulacra.ai".to_string()),
        Some("C:/images/test.png".to_string()),
        Some(PostId::new("p_parent")),
        Some(PostId::new("p_quoted")),
        None,
    )
    .expect("create post");

    db.save_post(&post).expect("save post");

    let loaded = db.load_posts().expect("load posts");
    let retrieved = loaded.iter().find(|p| p.id.as_str() == "p_db_test").expect("found post");

    assert_eq!(retrieved.link_url, Some("https://simulacra.ai".to_string()));
    assert_eq!(retrieved.image_path, Some("C:/images/test.png".to_string()));
    assert_eq!(retrieved.reply_to_id, Some(PostId::new("p_parent")));
    assert_eq!(retrieved.quote_id, Some(PostId::new("p_quoted")));

    // Test update_post_repost
    db.update_post_repost(&retrieved.id, 5, true).expect("update repost");
    let reloaded = db.load_posts().expect("reload posts");
    let reloaded_post = reloaded.iter().find(|p| p.id.as_str() == "p_db_test").unwrap();
    assert_eq!(reloaded_post.reposts_count, 5);
    assert!(reloaded_post.is_reposted);

    // Test update_post_replies_count
    db.update_post_replies_count(&retrieved.id, 9).expect("update replies count");
    let reloaded2 = db.load_posts().expect("reload posts 2");
    let reloaded_post2 = reloaded2.iter().find(|p| p.id.as_str() == "p_db_test").unwrap();
    assert_eq!(reloaded_post2.replies_count, 9);
}

#[test]
fn test_db_delete_repost_by_target_and_author_scoping() {
    let db = Database::in_memory().unwrap();
    let net = NetworkId::new("net_phil");
    let target_id = PostId::new("post_root");
    let author_a = Profile::new_primary("author_a", "Alice", "@alice");
    let author_b = Profile::new_primary("author_b", "Bob", "@bob");

    let post_a = Post::create_with_attachments(
        PostId::new("repost_a"),
        net.clone(),
        author_a.clone(),
        "",
        None,
        None,
        None,
        None,
        Some(target_id.clone()),
    ).unwrap();

    let post_b = Post::create_with_attachments(
        PostId::new("repost_b"),
        net,
        author_b.clone(),
        "",
        None,
        None,
        None,
        None,
        Some(target_id.clone()),
    ).unwrap();

    db.save_post(&post_a).unwrap();
    db.save_post(&post_b).unwrap();

    // Delete Alice's repost only
    db.delete_repost_by_target_and_author(&target_id, &author_a.id).unwrap();

    let remaining = db.load_posts().unwrap();
    assert!(!remaining.iter().any(|p| p.id.as_str() == "repost_a"));
    assert!(remaining.iter().any(|p| p.id.as_str() == "repost_b"));
}
