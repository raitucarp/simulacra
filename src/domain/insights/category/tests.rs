use super::schema::morphisms_of;
use super::*;
use crate::domain::graph::{EdgeKind, GraphModel, GraphSource};
use crate::domain::{GroundingDocument, Post, Profile};

fn m(source: u32, target: u32, kind: MorphismKind) -> Morphism {
    Morphism { source, target, kind }
}

#[test]
fn wikilinks_cite_documents_and_link_the_rest() {
    assert_eq!(MorphismKind::of(EdgeKind::Wikilink, ObjectKind::Document), MorphismKind::Cites);
    assert_eq!(MorphismKind::of(EdgeKind::Wikilink, ObjectKind::Thought), MorphismKind::Links);
    assert_eq!(MorphismKind::of(EdgeKind::Authored, ObjectKind::Thought), MorphismKind::Writes);
    assert_eq!(MorphismKind::ALL.len(), 8);
    assert_eq!(ObjectKind::ALL.map(ObjectKind::key), ["thought", "document", "concept", "person"]);
}

#[test]
fn composites_count_and_detect_realised_ones() {
    use MorphismKind::*;
    // 0 replies to 1, 1 cites doc 2, and 0 cites doc 2 directly; 0 also replies to 3 which cites 2.
    let arrows = [m(0, 1, RepliesTo), m(1, 2, Cites), m(0, 2, Cites), m(0, 3, RepliesTo), m(3, 2, Cites)];
    let report = compose(4, &arrows);
    let top = report.composites[0];
    assert_eq!((top.first, top.second, top.count, top.realised), (RepliesTo, Cites, 2, 2));
    assert_eq!(report.commuting_squares, 1, "0→1→2 and 0→3→2 carry the same relations");
    assert_eq!(report.mixed_squares, 0);
    assert!(report.examples[0].commutes_in_type());
    assert_eq!((report.examples[0].from, report.examples[0].to), (0, 2));
    assert!(!report.truncated);
}

#[test]
fn mixed_squares_and_no_back_and_forth() {
    use MorphismKind::*;
    let arrows = [m(0, 1, Quotes), m(1, 3, IsAbout), m(0, 2, RepliesTo), m(2, 3, IsAbout), m(1, 0, RepliesTo)];
    let report = compose(4, &arrows);
    assert_eq!((report.commuting_squares, report.mixed_squares), (0, 1));
    assert!(!report.examples[0].commutes_in_type());
    assert!(report.composites.iter().all(|c| c.example.0 != c.example.2), "x → y → x is not a composite here");
}

#[test]
fn schema_groups_morphisms_by_kinds() {
    let alice = Profile::new_primary("alice", "Alice", "@alice");
    let posts = vec![
        Post::create("p1", "n", alice.clone(), "On #ethics [[Doc]]").unwrap(),
        Post::create_reply("p2", "n", alice.clone(), "Yes #ethics", "p1", None, None).unwrap(),
    ];
    let docs = vec![GroundingDocument::new("d1", "n", "Doc", "Body")];
    let model = GraphModel::build(&GraphSource { posts: &posts, documents: &docs, profiles: &[alice], network: &"n".into() });
    let (objects, morphisms) = morphisms_of(&model);
    let arrows = schema_arrows(&objects, &morphisms);
    let find = |kind: MorphismKind| arrows.iter().find(|a| a.kind == kind).map(|a| (a.source, a.target, a.count));
    assert_eq!(find(MorphismKind::IsAbout), Some((ObjectKind::Thought, ObjectKind::Concept, 2)));
    assert_eq!(find(MorphismKind::Writes), Some((ObjectKind::Person, ObjectKind::Thought, 2)));
    assert_eq!(find(MorphismKind::Cites), Some((ObjectKind::Thought, ObjectKind::Document, 1)));
    assert_eq!(find(MorphismKind::RepliesTo), Some((ObjectKind::Thought, ObjectKind::Thought, 1)));
}
