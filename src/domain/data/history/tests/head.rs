//! Version heads (V3) and the numbers of branches (V4).

use super::super::*;
use crate::domain::post::PostExtras;

fn at(hour: i8) -> jiff::Timestamp {
    crate::time::utc(2026, 11, 20, hour, 0, 0)
}

fn v(major: u32, minor: u32, patch: u32) -> SemVer {
    SemVer::new(major, minor, patch)
}

#[test]
fn a_free_version_skips_the_numbers_other_branches_took() {
    assert_eq!(free_version(v(1, 0, 0), Bump::Minor, &[]), v(1, 1, 0));
    assert_eq!(free_version(v(1, 0, 0), Bump::Minor, &[v(1, 1, 0), v(1, 2, 0)]), v(1, 3, 0));
    assert_eq!(free_version(v(1, 1, 0), Bump::Patch, &[v(1, 1, 1)]), v(1, 1, 2));
    assert_eq!(free_version(v(1, 4, 2), Bump::Major, &[v(2, 0, 0)]), v(3, 0, 0));
    assert_eq!(free_version(v(1, 1, u32::MAX), Bump::Patch, &[v(1, 1, u32::MAX)]), v(1, 1, u32::MAX), "a saturated number stops");
}

#[test]
fn the_bump_between_two_numbers_is_the_highest_part_that_changed() {
    assert_eq!(step_bump(v(1, 0, 0), v(1, 0, 1)), Bump::Patch);
    assert_eq!(step_bump(v(1, 0, 3), v(1, 1, 0)), Bump::Minor);
    assert_eq!(step_bump(v(1, 9, 3), v(2, 0, 0)), Bump::Major);
    assert_eq!(step_bump(v(1, 2, 0), v(1, 2, 0)), Bump::Patch);
}

#[test]
fn a_post_without_a_stored_head_is_its_original_or_follows_its_newest_revision() {
    let subject = RevisionSubject::Post(crate::domain::PostId::new("p"));
    let original = VersionHead::of(None, &[], at(8));
    assert_eq!((original.version, original.parent.clone(), original.bump, original.edited_at), (SemVer::ORIGINAL, None, None, Some(at(8))));
    let older = Revision { version: Some(v(1, 0, 0)), ..Revision::of(subject.clone(), "one", at(9)) };
    let newer = Revision { version: Some(v(1, 1, 0)), ..Revision::of(subject, "two", at(10)) };
    let head = VersionHead::of(None, &[older, newer.clone()], at(8));
    assert_eq!((head.version, head.parent, head.bump, head.edited_at), (v(1, 2, 0), Some(newer.id), Some(Bump::Minor), Some(at(10))));
    let stored = VersionHead { version: v(4, 0, 0), ..VersionHead::default() };
    assert_eq!(VersionHead::of(Some(&stored), &[], at(8)), stored);
}

#[test]
fn a_head_kept_as_a_revision_comes_back_unchanged() {
    let head = VersionHead { id: RevisionId::new("h"), version: v(1, 3, 1), parent: Some(RevisionId::new("p")), bump: Some(Bump::Patch), note: "typo".into(), edited_at: Some(at(7)) };
    let kept = head.clone().into_revision(RevisionSubject::Post(crate::domain::PostId::new("x")), "text", at(9));
    assert_eq!((kept.id.as_str(), kept.version, kept.bump, kept.note.as_str(), kept.saved_at), ("h", Some(v(1, 3, 1)), Bump::Patch, "typo", at(9)));
    assert_eq!(VersionHead::from_revision(&kept), head);
    let original = VersionHead::original(at(1));
    assert_eq!(VersionHead::from_revision(&original.clone().into_revision(kept.subject.clone(), "t", at(2))).bump, None, "the original bumped nothing");
}

#[test]
fn heads_and_bumps_are_stored_as_text_and_old_extras_read_without_one() {
    let head = VersionHead { id: RevisionId::new("rev_1"), version: v(2, 1, 0), parent: None, bump: Some(Bump::Major), note: String::new(), edited_at: None };
    let extras = PostExtras { head: Some(head.clone()), ..PostExtras::default() };
    let json = serde_json::to_string(&extras).unwrap();
    assert_eq!(json, r#"{"head":{"id":"rev_1","version":"2.1.0","bump":"major"}}"#);
    assert_eq!(serde_json::from_str::<PostExtras>(&json).unwrap().head, Some(head));
    assert_eq!(serde_json::from_str::<PostExtras>("{}").unwrap().head, None);
    assert!(!extras.has_details(), "a version alone is nothing to show behind the post");
    for bump in Bump::ALL {
        assert_eq!(Bump::from_key(bump.key()), Some(bump));
    }
    assert_eq!(serde_json::from_str::<VersionHead>(r#"{"version":"oops"}"#).unwrap().version, SemVer::ORIGINAL);
}

#[test]
fn an_edit_base_suggests_a_bump_and_names_the_next_free_version() {
    let base = EditBase { text: "The mind is a mirror.".into(), version: v(1, 2, 0), revision: None, taken: vec![v(1, 3, 0)], record: None };
    assert_eq!(base.suggested("The mind is a mirror!"), Bump::Patch);
    assert_eq!(base.suggested("Attention is the rarest form of generosity."), Bump::Major);
    assert_eq!(base.next(Bump::Minor), v(1, 4, 0));
    assert_eq!(base.next(Bump::Patch), v(1, 2, 1));
}
