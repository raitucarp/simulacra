use super::{backlinks, link_counts, related_entries, RelatedReason, ScoredDocument};
use crate::domain::document::fixtures::{documents, library, NET};
use crate::domain::document::record::Discipline;
use crate::domain::document::DocumentId;
use crate::domain::post::{SourceKind, SourceRef};
use crate::domain::{Post, PostId, Profile};

fn post(id: &str, network: &str, content: &str) -> Post {
    let author = Profile::new_primary("u1", "Ada", "@ada");
    let mut post = Post::create(id, network, &author, content).expect("valid post");
    post.linked_posts = Post::parse_linked_posts(content);
    post
}

fn posts() -> Vec<Post> {
    let mut sourced = post("p3", NET, "drawing on the vault");
    sourced.extras.add_source(SourceRef::new(SourceKind::VaultDoc, "doc_ethics", "Ethics"));
    vec![
        post("p1", NET, "see [[doc_ethics]] and [[ethics]]"),
        post("p2", NET, "about [[Tractatus Logico-Philosophicus]]"),
        sourced,
        post("p4", "net_other", "[[doc_ethics]] elsewhere"),
    ]
}

#[test]
fn backlinks_come_from_wikilinks_titles_and_sources_in_the_same_network() {
    let docs = documents();
    let posts = posts();
    let ethics: Vec<&str> = backlinks(&docs[0], &posts).iter().map(|p| p.id.as_str()).collect();
    assert_eq!(ethics, ["p1", "p3"]);
    let tractatus: Vec<&PostId> = backlinks(&docs[1], &posts).iter().map(|p| &p.id).collect();
    assert_eq!(tractatus, [&PostId::new("p2")]);
}

#[test]
fn link_counts_count_each_post_once_per_document() {
    let counts = link_counts(&documents(), &posts());
    assert_eq!(counts.get("doc_ethics"), Some(&2));
    assert_eq!(counts.get("doc_tractatus"), Some(&1));
    assert_eq!(counts.get("doc_note"), None);
}

#[test]
fn related_entries_rank_shared_subjects_then_discipline_and_merge_semantic_scores() {
    let docs = documents();
    let tree = library().tree();
    let related = related_entries(&docs[0], &docs, &tree, &[], 5);
    assert_eq!(related[0].id, DocumentId::new("doc_note"));
    assert_eq!(related[0].reason, RelatedReason::SharedSubjects(2));
    assert_eq!(related[1].reason, RelatedReason::SharedSubjects(1), "Philosophy is shared through broader terms");
    let semantic = [ScoredDocument { id: DocumentId::new("doc_tractatus"), score: 0.9 }];
    let merged = related_entries(&docs[0], &docs, &tree, &semantic, 1);
    assert_eq!((merged[0].id.as_str(), merged[0].reason), ("doc_tractatus", RelatedReason::Semantic));
}

#[test]
fn a_shared_discipline_alone_is_a_weak_relation() {
    let mut docs = documents();
    docs.iter_mut().for_each(|d| d.record.subjects.clear());
    let related = related_entries(&docs[0], &docs, &library().tree(), &[], 5);
    assert!(related.iter().all(|r| r.reason == RelatedReason::SameDiscipline));
    docs[1].record.classification.discipline = Some(Discipline::Language);
    assert_eq!(related_entries(&docs[0], &docs, &library().tree(), &[], 5).len(), 1);
}
