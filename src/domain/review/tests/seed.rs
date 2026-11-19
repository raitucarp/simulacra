use crate::domain::post::{SourceKind, SourceRef};
use crate::domain::review::connections::seed::{map_from_document, map_from_posts, thread_posts, DocumentNeighbourhood, SHARED_CONCEPTS};
use crate::domain::review::connections::{MapEdge, MapPoint, MAX_POINTS};
use crate::domain::{NetworkId, Post, PostId, ProfileId};

fn post(id: &str, reply_to: Option<&str>, minute: u32, text: &str) -> Post {
    let mut post = Post::create(PostId::new(id), NetworkId::new("n"), ProfileId::new("p"), text).unwrap();
    post.reply_to_id = reply_to.map(PostId::new);
    post.created_at = chrono::TimeZone::with_ymd_and_hms(&chrono::Utc, 2026, 1, 1, 0, minute, 0).unwrap();
    post
}

fn pairs(edges: &[MapEdge]) -> Vec<(usize, usize)> {
    let mut pairs: Vec<(usize, usize)> = edges.iter().map(MapEdge::pair).collect();
    pairs.sort();
    pairs
}

#[test]
fn a_thread_runs_from_its_first_post_breadth_first() {
    let posts = vec![
        post("root", None, 0, "a"),
        post("b", Some("root"), 2, "b"),
        post("a", Some("root"), 1, "c"),
        post("a1", Some("a"), 3, "d"),
        post("other", None, 4, "e"),
    ];
    let ids: Vec<&str> = thread_posts(&posts, &PostId::new("a1")).iter().map(|p| p.id.as_str()).collect();
    assert_eq!(ids, vec!["root", "a", "b", "a1"]);
    let many: Vec<Post> = std::iter::once(post("r", None, 0, "r")).chain((0..20).map(|i| post(&format!("x{i}"), Some("r"), i + 1, "x"))).collect();
    assert_eq!(thread_posts(&many, &PostId::new("r")).len(), MAX_POINTS);
}

#[test]
fn a_thread_map_is_seeded_with_its_replies() {
    let posts = vec![post("root", None, 0, "a"), post("a", Some("root"), 1, "b"), post("b", Some("a"), 2, "c")];
    let line = thread_posts(&posts, &PostId::new("b"));
    let map = map_from_posts("Thread".into(), &line);
    assert_eq!(map.points[0], MapPoint::Post { id: "root".into() });
    assert_eq!(pairs(&map.edges), vec![(0, 1), (1, 2)]);
    assert!(map.edges.iter().all(|e| !e.directed && e.label.is_none()), "the learner names relations, the seed does not");
}

#[test]
fn wikilinks_quotes_cited_posts_and_shared_tags_join_a_selection() {
    let mut quoting = post("q", None, 1, "Against [[w]] and #Conatus");
    quoting.quote_id = Some(PostId::new("w"));
    let mut citing = post("c", None, 2, "Same #conatus here, also #ethics");
    citing.extras.sources.push(SourceRef::new(SourceKind::Post, "w", "w"));
    let linked = post("w", None, 0, "The striving #ethics");
    let lonely = post("l", None, 3, "Unrelated");
    let selection = [&quoting, &citing, &linked, &lonely, &quoting];
    let map = map_from_posts("Mine".into(), &selection);
    assert_eq!(map.points.len(), 4 + 2, "four posts once each, two shared tags as concepts");
    assert_eq!(map.points[4], MapPoint::Concept { label: "#conatus".into() });
    assert_eq!(pairs(&map.edges), vec![(0, 2), (0, 4), (1, 2), (1, 4), (1, 5), (2, 5)]);
    assert!(SHARED_CONCEPTS >= 2);
}

#[test]
fn a_document_map_joins_related_entries_and_linking_posts() {
    let linking = post("p", None, 0, "[[doc_a]]");
    let around = DocumentNeighbourhood { document_id: "doc_a", related: vec!["doc_b", "doc_c"], linking_posts: vec![&linking] };
    let related_pair = |a: &str, b: &str| (a, b) == ("doc_b", "doc_c");
    let map = map_from_document("Ethics".into(), &around, &related_pair);
    assert_eq!(map.points.len(), 4);
    assert!(matches!(&map.points[0], MapPoint::Passage { document_id, .. } if document_id == "doc_a"));
    assert_eq!(map.points[3], MapPoint::Post { id: "p".into() });
    assert_eq!(pairs(&map.edges), vec![(0, 1), (0, 2), (0, 3), (1, 2)]);
}
