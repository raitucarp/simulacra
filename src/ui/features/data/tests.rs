use super::*;
use crate::domain::data::history::{version_list, Revision, RevisionSubject};
use crate::domain::document::CollectionId;
use crate::domain::PostId;
use chrono::Utc;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

fn subject() -> RevisionSubject {
    RevisionSubject::Post(PostId::new("p1"))
}

#[test]
fn history_opens_on_the_latest_earlier_version() {
    let revisions = vec![Revision::of(subject(), "before", Utc::now())];
    let sheet = HistorySheet::new(subject(), "title".into(), version_list("", "after", &revisions));
    assert_eq!(sheet.selected, 1);
    let alone = HistorySheet::new(subject(), "title".into(), version_list("", "only", &[]));
    assert_eq!(alone.selected, 0);
}

#[test]
fn card_controls_follow_the_shelf_and_edit_counts() {
    let shelf = CollectionId::new("coll_a");
    let source = DataSource {
        revisions: Rc::new(HashMap::from([("p1".to_string(), 2)])),
        bookmarks: Rc::new(HashSet::from(["p1".to_string()])),
        collections: Rc::new(vec![(shelf.clone(), "Shelf".into())]),
        memberships: Rc::new(HashMap::from([("p1".to_string(), vec![shelf.clone()])])),
        on_event: Rc::new(|_, _, _| {}),
    };
    let own = source.controls(&PostId::new("p1"), true);
    assert!(own.own && own.bookmarked);
    assert_eq!((own.revisions, own.member_of.clone()), (2, vec![shelf]));
    let other = source.controls(&PostId::new("p2"), false);
    assert!(!other.own && !other.bookmarked && other.member_of.is_empty() && other.revisions == 0);
}

#[test]
fn scope_keys_are_distinct() {
    let keys: HashSet<&str> = ScopeKind::ALL.iter().map(|k| k.key()).collect();
    assert_eq!(keys.len(), ScopeKind::ALL.len());
}
