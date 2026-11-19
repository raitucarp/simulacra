use super::*;
use crate::domain::{Post, PostId, Profile};
use crate::state::{AppState, ThreadAnchor, ThreadDraft};

/// A post created `minute` minutes after a fixed epoch, optionally replying to `parent`.
fn post_at(id: &str, minute: i64, parent: Option<&str>) -> Post {
    let author = Profile::new_primary("u1", "Architect", "@arch");
    let mut post = Post::create(id, "net", author, id).expect("valid post");
    post.created_at = crate::time::utc(2026, 1, 1, 0, 0, 0) + jiff::SignedDuration::from_mins(minute);
    post.reply_to_id = parent.map(PostId::new);
    post
}

fn ids(posts: &[&Post]) -> Vec<String> {
    posts.iter().map(|p| p.id.to_string()).collect()
}

/// Store order is newest first.
fn newest_first(posts: &[Post]) -> Vec<&Post> {
    let mut refs: Vec<&Post> = posts.iter().collect();
    refs.sort_by_key(|p| std::cmp::Reverse(p.created_at));
    refs
}

#[test]
fn replies_follow_their_parent_and_bump_the_conversation() {
    let posts = [post_at("a", 1, None), post_at("b", 2, None), post_at("a1", 3, Some("a"))];
    let groups = group_conversations(&newest_first(&posts));
    let grouped: Vec<Vec<String>> = groups.iter().map(|g| ids(g)).collect();
    assert_eq!(grouped, vec![vec!["a", "a1"], vec!["b"]]);
}

#[test]
fn siblings_are_oldest_first_and_walked_depth_first() {
    let posts = [
        post_at("root", 0, None),
        post_at("r1", 1, Some("root")),
        post_at("r2", 2, Some("root")),
        post_at("r1a", 3, Some("r1")),
    ];
    let groups = group_conversations(&newest_first(&posts));
    assert_eq!(groups.len(), 1);
    assert_eq!(ids(&groups[0]), vec!["root", "r1", "r1a", "r2"]);
}

#[test]
fn missing_parents_and_cycles_keep_every_post() {
    let posts = [
        post_at("orphan", 1, Some("gone")),
        post_at("x", 2, Some("y")),
        post_at("y", 3, Some("x")),
    ];
    let groups = group_conversations(&newest_first(&posts));
    let total: usize = groups.iter().map(Vec::len).sum();
    assert_eq!(total, 3);
    assert!(groups.iter().any(|g| ids(g) == vec!["orphan"]));
}

#[test]
fn dialectic_tab_keeps_debates_only() {
    let solo = post_at("solo", 0, None);
    let (root, reply) = (post_at("root", 1, None), post_at("reply", 2, Some("root")));
    let mut quote = post_at("quote", 3, None);
    quote.quote_id = Some(PostId::new("solo"));
    assert!(FeedTab::ForYou.includes(&[&solo]));
    assert!(!FeedTab::Dialectic.includes(&[&solo]));
    assert!(FeedTab::Dialectic.includes(&[&root, &reply]));
    assert!(FeedTab::Dialectic.includes(&[&quote]));
}

#[test]
fn new_reply_is_shown_right_under_its_old_parent() {
    let mut state = AppState::new_in_memory();
    let root = state.submit_post("old root").expect("root");
    let other = state.submit_post("newer standalone").expect("other");
    let reply = state.submit_reply(&root, "late reply", None, None).expect("reply");
    let entries = feed_entries(&state, FeedTab::ForYou);
    assert_eq!(entries[0].id, root);
    assert_eq!(entries[1].id, reply);
    assert_eq!(feed_position(&state, FeedTab::ForYou, &other), Some(2));
    assert_eq!(feed_position(&state, FeedTab::Dialectic, &reply), Some(1));
    assert_eq!(feed_position(&state, FeedTab::Dialectic, &other), None);
}

#[test]
fn conversation_shows_ancestors_focus_and_replies() {
    let mut state = AppState::new_in_memory();
    let drafts: Vec<ThreadDraft> = ["one", "two", "three"].map(ThreadDraft::text).to_vec();
    let ids = state.submit_thread(&drafts, &ThreadAnchor::default()).expect("thread");
    let focused = state.get_post(&ids[1]).expect("focused");
    let conversation = build_conversation(&state, focused, ReplyLimits::UNLIMITED);
    assert_eq!(ids_of(&conversation.ancestors), vec![ids[0].clone()]);
    assert_eq!(conversation.focused.id, ids[1]);
    assert_eq!(conversation.branches.len(), 1);
    assert_eq!(conversation.branches[0].reply.id, ids[2]);
    assert_eq!((conversation.direct_total, conversation.hidden), (1, 0));
}

fn ids_of(posts: &[&Post]) -> Vec<PostId> {
    posts.iter().map(|p| p.id.clone()).collect()
}

#[test]
fn reply_limits_hide_and_count_the_rest() {
    let mut state = AppState::new_in_memory();
    let root = state.submit_post("root").expect("root");
    let direct: Vec<PostId> = (0..3)
        .map(|i| state.submit_reply(&root, &format!("reply {i}"), None, None).expect("reply"))
        .collect();
    for i in 0..3 {
        state.submit_reply(&direct[0], &format!("nested {i}"), None, None).expect("nested");
    }
    let focused = state.get_post(&root).expect("root");
    let limits = ReplyLimits { direct: 2, nested: 1 };
    let conversation = build_conversation(&state, focused, limits);
    assert_eq!(conversation.direct_total, 3);
    let shown: Vec<PostId> = conversation.branches.iter().map(|b| b.reply.id.clone()).collect();
    assert_eq!(shown, direct[..2].to_vec(), "oldest replies first");
    assert_eq!(conversation.branches[0].nested.len(), 1);
    assert_eq!(conversation.hidden, 1 + 2, "one direct + two nested hidden");
    let full = build_conversation(&state, focused, ReplyLimits::UNLIMITED);
    assert_eq!((full.branches.len(), full.branches[0].nested.len(), full.hidden), (3, 3, 0));
}
