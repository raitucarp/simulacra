use crate::state::store::AppState;

#[test]
fn test_store_submit_with_attachments_and_reply_quote() {
    let mut state = AppState::new_in_memory();
    let root_id = state.submit_post("Root thought").expect("Submit root");

    // Submit reply
    let reply_id = state
        .submit_reply(&root_id, "Reply to root thought", Some("https://example.com".into()), None)
        .expect("Submit reply");

    let reply_post = state.posts.iter().find(|p| p.id == reply_id).unwrap();
    assert_eq!(reply_post.reply_to_id, Some(root_id.clone()));
    assert_eq!(reply_post.link_url, Some("https://example.com".into()));

    // Root post replies_count should have incremented
    let root_post = state.posts.iter().find(|p| p.id == root_id).unwrap();
    assert!(root_post.replies_count >= 1);

    // Submit quote
    let quote_id = state
        .submit_quote(&root_id, "Quoting thought with commentary", None, Some("/path/image.png".into()))
        .expect("Submit quote");

    let quote_post = state.posts.iter().find(|p| p.id == quote_id).unwrap();
    assert_eq!(quote_post.quote_id, Some(root_id.clone()));
    assert_eq!(quote_post.image_path, Some("/path/image.png".into()));
}

#[test]
fn test_store_repost_flow() {
    let mut state = AppState::new_in_memory();
    let root_id = state.submit_post("Thought to repost").expect("Submit root");

    let repost_id = state.repost(&root_id).expect("Repost action");
    let repost_post = state.posts.iter().find(|p| p.id == repost_id).unwrap();
    assert_eq!(repost_post.repost_id, Some(root_id.clone()));
    assert_eq!(repost_post.content, "");

    let target_post = state.posts.iter().find(|p| p.id == root_id).unwrap();
    assert_eq!(target_post.reposts_count, 1);
    assert!(target_post.is_reposted);

    // Toggle un-repost
    let un_repost_id = state.repost(&root_id).expect("Un-repost action");
    assert_eq!(un_repost_id, root_id);
    let target_post_after = state.posts.iter().find(|p| p.id == root_id).unwrap();
    assert_eq!(target_post_after.reposts_count, 0);
    assert!(!target_post_after.is_reposted);
    assert!(!state.posts.iter().any(|p| p.repost_id.as_ref() == Some(&root_id)));
}

#[test]
fn test_store_post_indexing_and_fast_lookup() {
    let state = AppState::new_in_memory();
    assert!(!state.posts.is_empty());
    assert_eq!(state.post_index_by_id.len(), state.posts.len());

    for post in &state.posts {
        let found_by_ref = state.get_post(&post.id);
        assert!(found_by_ref.is_some());
        assert_eq!(found_by_ref.unwrap().id, post.id);

        let found_by_str = state.get_post_by_id(post.id.as_str());
        assert!(found_by_str.is_some());
        assert_eq!(found_by_str.unwrap().id, post.id);
    }

    assert!(state.get_post_by_id("nonexistent_post_xyz").is_none());
}

#[test]
fn test_store_wikilink_cache_and_resolution() {
    let state = AppState::new_in_memory();
    assert!(!state.wikilink_cache.is_empty());

    let _post_1 = state.get_post_by_id("post_1").expect("post_1 exists");
    let excerpt = state.resolve_wikilink_fast("post_1");
    assert!(excerpt.ends_with("..."));

    // Fallback for non-existent post
    let fallback_short = state.resolve_wikilink_fast("post_12345678");
    assert_eq!(fallback_short, "#1234");

    // Fallback for non-post plain string
    let plain = state.resolve_wikilink_fast("Ontology");
    assert_eq!(plain, "Ontology");
}

#[test]
fn test_store_indices_sync_on_mutations() {
    let mut state = AppState::new_in_memory();
    let initial_count = state.posts.len();

    // 1. Submit post (human top-level thought is placed directly at index 0 without auto-spawning agent post above it)
    let post_id = state.submit_post("Brand new indexed thought").unwrap();
    assert_eq!(state.posts.len(), initial_count + 1);
    assert_eq!(state.post_index_by_id.get(&post_id), Some(&0));
    assert!(state.wikilink_cache.contains_key(post_id.as_str()));
    assert_eq!(state.post_index_by_id.len(), state.posts.len());

    // 2. Submit reply (1 post added)
    let reply_id = state.submit_reply(&post_id, "Reply to indexed thought", None, None).unwrap();
    assert_eq!(state.post_index_by_id.get(&reply_id), Some(&0));
    assert_eq!(state.post_index_by_id.get(&post_id), Some(&1));
    assert_eq!(state.post_index_by_id.len(), state.posts.len());

    // 3. Submit quote (1 post added at index 0)
    let quote_id = state.submit_quote(&post_id, "Quote indexed thought", None, None).unwrap();
    assert_eq!(state.post_index_by_id.get(&quote_id), Some(&0));
    assert_eq!(state.post_index_by_id.len(), state.posts.len());

    // 4. Repost & un-repost
    let repost_id = state.repost(&post_id).unwrap();
    assert_eq!(state.post_index_by_id.get(&repost_id), Some(&0));
    assert_eq!(state.post_index_by_id.len(), state.posts.len());

    let _ = state.repost(&post_id).unwrap(); // un-repost
    assert_eq!(state.post_index_by_id.len(), state.posts.len());
    assert!(!state.post_index_by_id.contains_key(&repost_id));
}

#[test]
fn test_submit_post_does_not_auto_spawn_agent_above_user_post() {
    let mut state = AppState::new_in_memory();
    let initial_len = state.posts.len();

    let post_id = state.submit_post("My independent epistemic reflection").unwrap();

    assert_eq!(state.posts.len(), initial_len + 1);
    assert_eq!(state.posts[0].id, post_id);
    assert_eq!(state.posts[0].content, "My independent epistemic reflection");
    assert_eq!(state.posts[0].author_id, state.active_profile_id);
    assert_eq!(state.posts[0].replies_count, 0);
}
