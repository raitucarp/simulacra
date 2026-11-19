use super::*;
use crate::domain::insights::filter::{Audience, DateRange};

fn at(post: Post, minutes: i64) -> Post {
    Post { created_at: crate::time::utc(2026, 3, 10, 12, 0, 0) - jiff::SignedDuration::from_mins(minutes), ..post }
}

/// A debate: a root by Alice, two replies by the agent Bob, each answered by Alice; a quote,
/// and an old post outside a week.
fn input() -> InsightsInput {
    let alice = Profile::new_primary("alice", "Alice", "@alice");
    let bob = Profile::new_agent("bob", "Bob", "@bob", "Skeptic").with_network("n");
    let posts = vec![
        at(Post::create("r", "n", alice.clone(), "Root #ethics #logic [[Doc]]").unwrap(), 50),
        at(Post::create_reply("a", "n", bob.clone(), "First #ethics", "r", None, None).unwrap(), 40),
        at(Post::create_reply("b", "n", bob.clone(), "Second #ethics", "r", None, None).unwrap(), 30),
        at(Post::create_reply("a1", "n", alice.clone(), "On first", "a", None, None).unwrap(), 20),
        at(Post::create_reply("b1", "n", alice.clone(), "On second", "b", None, None).unwrap(), 10),
        at(Post::create_quote("q", "n", bob.clone(), "Quoting #logic", "r", None, None).unwrap(), 5),
        at(Post::create("old", "n", alice.clone(), "Long ago").unwrap(), 60 * 24 * 40),
    ];
    let mut doc = GroundingDocument::new("d1", "n", "Doc", "Body");
    doc.record.subjects = vec!["s1".into()];
    InsightsInput {
        network: "n".into(),
        posts,
        documents: vec![doc],
        profiles: vec![alice, bob],
        subjects: vec![("s1".into(), "Ethics".into())],
        vectors: vec![("a".into(), vec![1.0, 0.0]), ("b".into(), vec![0.0, 1.0]), ("q".into(), vec![0.6, 0.8])],
        now: crate::time::utc(2026, 3, 10, 12, 0, 0),
        offset: None,
    }
}

#[test]
fn filter_narrows_the_posts() {
    let month = InsightsReport::compute(&input(), &InsightsFilter::default(), GraphScope::Knowledge);
    assert_eq!(month.posts.len(), 6, "the old post is outside the month");
    let agents = InsightsFilter { audience: Audience::Agents, range: DateRange::All, ..InsightsFilter::default() };
    let report = InsightsReport::compute(&input(), &agents, GraphScope::Knowledge);
    assert_eq!(report.posts.len(), 3);
    assert_eq!(report.authors.len(), 1);
    assert!(report.authors[0].is_agent);
    assert_eq!(report.post_index(&"q".into()), Some(2));
}

#[test]
fn thread_symmetry_and_rotation() {
    let report = InsightsReport::compute(&input(), &InsightsFilter::default(), GraphScope::Thoughts);
    let thread = &report.symmetry.threads[0];
    assert_eq!(thread.posts.len(), 5);
    assert_eq!(thread.symmetry.order, Some(2), "the two reply branches can be swapped");
    assert_eq!(thread.rotation.cycles.len(), 1);
    assert_eq!(thread.rotation.order, 2, "Alice and Bob answer each other");
    assert!(!report.symmetry.classes.is_empty());
}

#[test]
fn families_category_and_embedding() {
    let report = InsightsReport::compute(&input(), &InsightsFilter::default(), GraphScope::Concepts);
    let family = |d: SetDimension| report.families.iter().find(|(x, _)| *x == d).map(|(_, s)| s.clone()).unwrap();
    let concepts = family(SetDimension::Concept);
    assert_eq!(concepts[0].key, "ethics");
    assert_eq!(concepts[0].len(), 3);
    assert_eq!(family(SetDimension::Subject)[0].label, "Ethics");
    assert_eq!(family(SetDimension::Kind)[0].key, "thought");
    assert_eq!(report.network.nodes.len(), 2, "two concepts");
    assert_eq!(report.network.graph.edge_count(), 1, "#ethics and #logic share the root");
    assert!(report.category.arrows.iter().any(|a| a.count >= 4));
    assert!(!report.category.composition.composites.is_empty());
    assert_eq!(report.embedding.len(), 3);
    assert_eq!(report.embedding_posts.len(), 3);
    assert_eq!(report.cluster_labels.len(), report.embedding.cluster_count.max(1));
    assert_eq!(report.documents, 1);
    assert_eq!(report.activity.kinds.iter().sum::<usize>(), 6);
}
