//! `read_network` over a fixed snapshot, and the snapshot capture itself.

use super::context;
use crate::agent::tools::network;
use crate::agent::tools::ToolContext;
use crate::domain::post::SourceKind;
use crate::domain::{NetworkId, Post, Profile};
use serde_json::json;

#[test]
fn recent_posts_filter_by_handle_and_limit() {
    let ctx = context();
    let all = network::run(&ctx, &json!({ "action": "recent", "limit": 2 }));
    assert_eq!(all.content.lines().count(), 2);
    assert!(all.content.starts_with("[p3] weil @weil at 2026-11-17 09:30: Unrelated"), "{}", all.content);
    let by = network::run(&ctx, &json!({ "action": "recent", "handle": "SPINOZA" }));
    assert_eq!(by.content, "[p2] spinoza @spinoza at 2026-11-17 09:20 (reply to p1): God or Nature, one substance");
    assert_eq!((by.sources[0].kind, by.sources[0].id.as_str()), (SourceKind::Post, "p2"));
}

#[test]
fn profiles_threads_and_search() {
    let ctx = context();
    let profile = network::run(&ctx, &json!({ "action": "profile", "handle": "@spinoza" }));
    assert_eq!(profile.content, "spinoza (@spinoza), agent. Occupation: Philosopher. Bio: Thinks.");
    assert_eq!(profile.sources[0].kind, SourceKind::Profile);
    assert!(network::run(&ctx, &json!({ "action": "profile", "handle": "@nobody" })).failed);
    let thread = network::run(&ctx, &json!({ "action": "thread", "post_id": "p1" }));
    let ids: Vec<&str> = thread.sources.iter().map(|s| s.id.as_str()).collect();
    assert_eq!(ids, vec!["p1", "p2"], "root first, then replies");
    assert!(network::run(&ctx, &json!({ "action": "thread", "post_id": "nope" })).failed);
    let search = network::run(&ctx, &json!({ "action": "search", "query": "SUBSTANCE" }));
    assert_eq!(search.sources.len(), 1);
    assert!(network::run(&ctx, &json!({ "action": "search" })).failed);
    assert!(network::run(&ctx, &json!({ "action": "dance" })).failed);
    assert_eq!(network::run(&ctx, &json!({ "action": "recent", "handle": "@ghost" })).content, "No matching posts.");
}

#[test]
fn capture_keeps_only_the_agents_network() {
    let net = NetworkId::new("net_a");
    let other = NetworkId::new("net_b");
    let mut member = Profile::new_agent("a1", "Ada", "@ada", "Engineer");
    member.network_id = Some(net.clone());
    let mut outsider = Profile::new_agent("a2", "Bob", "@bob", "Baker");
    outsider.network_id = Some(other.clone());
    let global = Profile::new_primary("u1", "Me", "@me");
    let posts = vec![
        Post::create("p1", net.clone(), member.id.clone(), "hello").expect("post"),
        Post::create("p2", other.clone(), outsider.id.clone(), "elsewhere").expect("post"),
    ];
    let docs = vec![
        crate::domain::GroundingDocument::new("d1", net.clone(), "Mine", "text"),
        crate::domain::GroundingDocument::new("d2", other, "Theirs", "text"),
    ];
    let ctx = ToolContext::capture(&net, &posts, &[member, outsider, global], &docs);
    assert_eq!(ctx.posts.len(), 1);
    assert_eq!((ctx.posts[0].author_handle.as_str(), ctx.posts[0].author_name.as_str()), ("@ada", "Ada"));
    let handles: Vec<&str> = ctx.profiles.iter().map(|p| p.handle.as_str()).collect();
    assert_eq!(handles, vec!["@ada", "@me"]);
    assert_eq!(ctx.documents.len(), 1);
    assert_eq!(ctx.documents[0].title, "Mine");
    assert!(!ctx.fetch.allow_private_hosts);
}
