use super::layout::{GraphLayout, LAYOUT_CENTER};
use super::*;
use crate::domain::{GroundingDocument, NetworkId, Post, PostId, Profile, ProfileId};
use chrono::{Duration, TimeZone, Utc};

const NET: &str = "net_a";
const OTHER: &str = "net_b";

fn alice() -> Profile {
    Profile::new_primary("alice", "Alice", "@alice")
}

fn bob() -> Profile {
    Profile::new_agent("bob", "Bob", "@bob", "Skeptic").with_network(NET)
}

fn stamp(mut post: Post, minute: i64) -> Post {
    post.created_at = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap() + Duration::minutes(minute);
    post
}

/// p1 (tags, wikilinks to p2 and a doc title, mentions bob), p2 replies to p1,
/// p3 quotes p1, p4 reposts p2, plus a post, doc and agent in another network.
fn fixture() -> (Vec<Post>, Vec<GroundingDocument>, Vec<Profile>) {
    let mut p1 = Post::create("p1", NET, alice(), "Hi #Logic #logic [[p2]] [[Doc Title]] @bob").unwrap();
    p1.mentions = vec!["@BOB".to_string()];
    let posts = vec![
        stamp(p1, 0),
        stamp(Post::create_reply("p2", NET, bob(), "Indeed #Ethics", "p1", None, None).unwrap(), 1),
        stamp(Post::create_quote("p3", NET, alice(), "Quoting", "p1", None, None).unwrap(), 2),
        stamp(Post::create_repost("p4", NET, bob(), "p2").unwrap(), 3),
        stamp(Post::create_reply("x1", OTHER, alice(), "Elsewhere #Logic", "p1", None, None).unwrap(), 4),
    ];
    let docs = vec![
        GroundingDocument::new("d1", NET, "Doc Title", "Body"),
        GroundingDocument::new("d2", OTHER, "Other Doc", "Body"),
    ];
    let carol = Profile::new_agent("carol", "Carol", "@carol", "Cynic").with_network(OTHER);
    (posts, docs, vec![alice(), bob(), carol])
}

fn build(posts: &[Post], docs: &[GroundingDocument], profiles: &[Profile]) -> GraphModel {
    let network = NetworkId::new(NET);
    GraphModel::build(&GraphSource { posts, documents: docs, profiles, network: &network })
}

fn has_edge(model: &GraphModel, from: NodeRef, to: NodeRef, kind: EdgeKind) -> bool {
    let (Some(source), Some(target)) = (model.index_of(&from), model.index_of(&to)) else {
        return false;
    };
    model.edges().contains(&GraphEdge { source, target, kind })
}

fn post(id: &str) -> NodeRef {
    NodeRef::Post(PostId::new(id))
}

#[test]
fn every_edge_kind_is_extracted() {
    let (posts, docs, profiles) = fixture();
    let m = build(&posts, &docs, &profiles);
    let bob = NodeRef::Profile(ProfileId::new("bob"));
    assert!(has_edge(&m, post("p2"), post("p1"), EdgeKind::Reply));
    assert!(has_edge(&m, post("p3"), post("p1"), EdgeKind::Quote));
    assert!(has_edge(&m, post("p4"), post("p2"), EdgeKind::Repost));
    assert!(has_edge(&m, post("p1"), post("p2"), EdgeKind::Wikilink));
    assert!(has_edge(&m, post("p1"), NodeRef::Document("d1".into()), EdgeKind::Wikilink));
    assert!(has_edge(&m, post("p1"), NodeRef::Hashtag("logic".into()), EdgeKind::Hashtag));
    assert!(has_edge(&m, post("p1"), bob.clone(), EdgeKind::Mention));
    assert!(has_edge(&m, bob, post("p2"), EdgeKind::Authored));
}

#[test]
fn graph_is_scoped_to_the_network() {
    let (posts, docs, profiles) = fixture();
    let m = build(&posts, &docs, &profiles);
    assert_eq!(m.count_of(NodeKind::Post), 4);
    assert_eq!(m.count_of(NodeKind::Document), 1);
    assert_eq!(m.count_of(NodeKind::Hashtag), 2, "#Logic and #logic merge; #Ethics");
    assert_eq!(m.count_of(NodeKind::Profile), 2, "carol belongs to another network");
    assert!(m.index_of(&post("x1")).is_none());
    assert!(m.index_of(&NodeRef::Document("d2".into())).is_none());
    assert_eq!(m.node_count(), NODE_KINDS.iter().map(|k| m.count_of(*k)).sum::<usize>());
    assert!(m.edges().iter().all(|e| e.source != e.target && e.target < m.node_count()));
}

#[test]
fn order_is_deterministic_regardless_of_input_order() {
    let (posts, docs, profiles) = fixture();
    let first = build(&posts, &docs, &profiles);
    let mut reversed = posts.clone();
    reversed.reverse();
    let second = build(&reversed, &docs, &profiles);
    assert_eq!(first, second);
    assert_eq!(first.nodes()[0].node, post("p1"), "posts come oldest first");
    assert!(first.edges().windows(2).all(|w| w[0] < w[1]), "edges are sorted and unique");
}

#[test]
fn labels_are_char_safe_excerpts() {
    let long = "知".repeat(80);
    let label = excerpt(&long, 10);
    assert_eq!(label.chars().count(), 10);
    assert!(label.ends_with('…'));
    assert_eq!(excerpt("  short   text ", 10), "short text");
}

#[test]
fn layout_is_deterministic_finite_and_bounded() {
    let (posts, docs, profiles) = fixture();
    let m = build(&posts, &docs, &profiles);
    let layout = GraphLayout::compute(&m);
    assert_eq!(layout, GraphLayout::compute(&m));
    assert_eq!(layout.positions().len(), m.node_count());
    for p in layout.positions() {
        assert!(p.x.is_finite() && p.y.is_finite());
        assert!((0.0..=1.0).contains(&p.x) && (0.0..=1.0).contains(&p.y), "{p:?}");
    }
}

#[test]
fn layout_handles_degenerate_graphs() {
    assert!(GraphLayout::compute(&GraphModel::default()).positions().is_empty());
    let single = vec![Post::create("solo", NET, alice(), "alone").unwrap()];
    let m = build(&single, &[], &[]);
    let layout = GraphLayout::compute(&m);
    assert_eq!(m.node_count(), 2, "the post and its author");
    assert!(layout.positions().iter().all(|p| p.x.is_finite() && p.y.is_finite()));
    let docs = vec![GroundingDocument::new("only", NET, "Only doc", "Body")];
    let one = GraphLayout::compute(&build(&[], &docs, &[]));
    assert_eq!(one.positions(), &[layout::LayoutPoint { x: LAYOUT_CENTER, y: LAYOUT_CENTER }]);
}

#[test]
fn linked_nodes_end_up_closer_than_unlinked_ones() {
    let posts: Vec<Post> = (0..12)
        .map(|i| {
            let parent = if i % 6 == 0 { None } else { Some(PostId::new(format!("c{}", i - 1))) };
            let content = format!("thought {i}");
            let mut p = Post::create(format!("c{i}"), NET, alice(), &content).unwrap();
            p.reply_to_id = parent;
            stamp(p, i)
        })
        .collect();
    let m = build(&posts, &[], &[]);
    let layout = GraphLayout::compute(&m);
    let dist = |a: usize, b: usize| {
        let (p, q) = (layout.position(a).unwrap(), layout.position(b).unwrap());
        ((p.x - q.x).powi(2) + (p.y - q.y).powi(2)).sqrt()
    };
    let (mut linked, mut unlinked) = (Vec::new(), Vec::new());
    for a in 0..m.node_count() {
        for b in (a + 1)..m.node_count() {
            if m.are_adjacent(a, b) { linked.push(dist(a, b)) } else { unlinked.push(dist(a, b)) }
        }
    }
    let mean = |v: &[f32]| v.iter().sum::<f32>() / v.len() as f32;
    assert!(mean(&linked) < mean(&unlinked));
}
