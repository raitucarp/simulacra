use super::*;
use crate::domain::data::history::{version_list, Revision, RevisionSubject};
use crate::domain::document::CollectionId;
use crate::domain::PostId;
use jiff::Timestamp;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

fn subject() -> RevisionSubject {
    RevisionSubject::Post(PostId::new("p1"))
}

#[test]
fn history_opens_on_the_latest_earlier_version() {
    let revisions = vec![Revision::of(subject(), "before", Timestamp::now())];
    let sheet = HistorySheet::new(subject(), "title".into(), version_list("", "after", None, &revisions));
    assert_eq!(sheet.selected, 1);
    let alone = HistorySheet::new(subject(), "title".into(), version_list("", "only", None, &[]));
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
        history_open: false,
        on_event: Rc::new(|_, _, _| {}),
    };
    let own = source.controls(&PostId::new("p1"), true);
    assert!(own.own && own.bookmarked);
    assert_eq!((own.revisions, own.member_of.clone()), (2, vec![shelf]));
    let other = source.controls(&PostId::new("p2"), false);
    assert!(!other.own && !other.bookmarked && other.member_of.is_empty() && other.revisions == 0);
    assert!(own.edited_tooltip(), "the mark explains itself while no history is open");
    let open = DataSource { history_open: true, ..source };
    assert!(!open.controls(&PostId::new("p1"), true).edited_tooltip(), "an open history sheet drops the mark's tooltip");
}

#[test]
fn scope_keys_are_distinct() {
    let keys: HashSet<&str> = ScopeKind::ALL.iter().map(|k| k.key()).collect();
    assert_eq!(keys.len(), ScopeKind::ALL.len());
}

#[test]
fn a_branched_history_draws_two_lanes_and_opens_beside_the_current_version() {
    use crate::domain::data::history::{RevisionId, SemVer, VersionHead};
    let at = |hour| crate::time::utc(2026, 11, 20, hour, 0, 0);
    let root = Revision { id: RevisionId::new("a"), version: Some(SemVer::ORIGINAL), edited_at: Some(at(8)), ..Revision::of(subject(), "one", at(9)) };
    let leaf = Revision { id: RevisionId::new("b"), parent_id: Some(RevisionId::new("a")), version: Some(SemVer::new(1, 1, 0)), edited_at: Some(at(9)), ..Revision::of(subject(), "two", at(10)) };
    let head = VersionHead { id: RevisionId::new("h"), version: SemVer::new(1, 2, 0), parent: Some(RevisionId::new("a")), edited_at: Some(at(10)), ..VersionHead::default() };
    let sheet = HistorySheet::new(subject(), "title".into(), version_list("", "three", Some(&head), &[root, leaf]));
    assert_eq!((sheet.lane_count, sheet.lanes.len()), (2, 3));
    assert_eq!(sheet.current().content, "three");
    assert!(!sheet.chose_current(), "the sheet opens on the newest earlier version");
    assert_eq!(sheet.lanes[2].forks, vec![1], "the older line joins the original");
}

#[test]
fn any_two_versions_are_compared_a_to_b() {
    let at = |hour| crate::time::utc(2026, 11, 20, hour, 0, 0);
    let first = Revision::of(subject(), "a mirror", at(9));
    let second = Revision { parent_id: Some(first.id.clone()), ..Revision::of(subject(), "a lamp", at(10)) };
    let mut sheet = HistorySheet::new(subject(), "title".into(), version_list("", "a lamp, not a mirror", None, &[first, second]));
    assert_eq!((sheet.selected, sheet.compare), (1, 0), "A is the newest earlier version, B the current one");
    sheet.pick(2, true);
    assert_eq!(sheet.diff(), Some(crate::domain::data::history::word_diff("a lamp", "a mirror")));
    sheet.pick(2, false);
    assert_eq!(sheet.diff(), None, "a version against itself shows no diff");
    sheet.pick(99, true);
    assert_eq!(sheet.compare, 2, "out-of-range picks clamp to the last version");
}

#[test]
fn a_post_sheet_carries_its_edit_log_and_a_document_sheet_none() {
    use crate::domain::data::history::{Bump, SemVer, VersionHead};
    let original = Revision { version: Some(SemVer::ORIGINAL), ..Revision::of(subject(), "one", Timestamp::now()) };
    let head = VersionHead { version: SemVer::new(2, 0, 0), parent: Some(original.id.clone()), bump: Some(Bump::Major), note: "rewrote it".into(), ..VersionHead::default() };
    let sheet = HistorySheet::new(subject(), "title".into(), version_list("", "two", Some(&head), std::slice::from_ref(&original)));
    let notes: Vec<&str> = sheet.log.iter().map(|e| e.note.as_str()).collect();
    assert_eq!(notes, vec!["rewrote it", ""]);
    assert_eq!(super::history_log::bump_text(&sheet.log[0]), rust_i18n::t!("data.bump.major"));
    let document = RevisionSubject::Document("doc".into());
    assert!(HistorySheet::new(document.clone(), "t".into(), version_list("t", "b", None, &[Revision { subject: document, ..original }])).log.is_empty());
}
