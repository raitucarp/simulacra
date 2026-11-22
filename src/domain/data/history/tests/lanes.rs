//! The version tree (V5): lanes per row and the list order they are drawn in.

use super::super::*;

fn node<'a>(id: &'a str, parent: Option<&'a str>) -> TreeNode<'a> {
    TreeNode { id, parent }
}

fn row(lane: usize, through: &[usize], forks: &[usize], continued: bool, continues: bool) -> LaneRow {
    LaneRow { lane, through: through.to_vec(), forks: forks.to_vec(), continued, continues }
}

#[test]
fn a_linear_history_is_one_lane() {
    let rows = lane_rows(&[node("c", Some("b")), node("b", Some("a")), node("a", None)]);
    assert_eq!(rows, vec![row(0, &[], &[], false, true), row(0, &[], &[], true, true), row(0, &[], &[], true, false)]);
    assert_eq!(lane_count(&rows), 1);
    assert_eq!(lane_count(&[]), 0);
}

#[test]
fn a_branch_gets_its_own_lane_and_joins_where_it_forked() {
    let rows = lane_rows(&[node("c", Some("b")), node("x", Some("a")), node("b", Some("a")), node("a", None)]);
    assert_eq!(rows[1], row(1, &[0], &[], false, true), "the branch opens a second lane");
    assert_eq!(rows[2], row(0, &[1], &[], true, true), "the main line passes the branch");
    assert_eq!(rows[3], row(0, &[], &[1], true, false), "the branch ends at the version it was edited from");
    assert_eq!(lane_count(&rows), 2);
    assert_eq!(lane_cell(&rows[3], 0), LaneCell { up: true, down: false, dot: true, left: false, right: true });
    assert_eq!(lane_cell(&rows[3], 1), LaneCell { up: true, down: false, dot: false, left: true, right: false });
    assert_eq!(lane_cell(&rows[2], 1), LaneCell { up: true, down: true, ..LaneCell::default() });
}

#[test]
fn freed_lanes_are_reused_and_unknown_parents_end_a_line() {
    let rows = lane_rows(&[node("y", Some("b")), node("x", Some("gone")), node("z", Some("b")), node("b", None)]);
    assert_eq!(rows[1], row(1, &[0], &[], false, false), "a parent that is not listed ends the line");
    assert_eq!(rows[2].lane, 1, "the lane freed by x is reused");
    assert_eq!(rows[3], row(0, &[], &[1], true, false));
    let wide = LaneRow { lane: 0, forks: vec![2], ..LaneRow::default() };
    assert_eq!(lane_cell(&wide, 1), LaneCell { left: true, right: true, ..LaneCell::default() }, "the connector crosses the lanes between");
}

#[test]
fn the_list_is_drawn_newest_edit_first_with_the_head_wherever_it_sits() {
    let at = |hour| crate::time::utc(2026, 11, 20, hour, 0, 0);
    let subject = RevisionSubject::Post(crate::domain::PostId::new("p"));
    let original = Revision { id: RevisionId::new("a"), version: Some(SemVer::ORIGINAL), edited_at: Some(at(8)), ..Revision::of(subject.clone(), "one", at(9)) };
    let later = Revision { id: RevisionId::new("b"), parent_id: Some(RevisionId::new("a")), version: Some(SemVer::new(1, 1, 0)), edited_at: Some(at(10)), note: "more".into(), ..Revision::of(subject, "two", at(11)) };
    let head = VersionHead { id: RevisionId::new("h"), version: SemVer::new(1, 2, 0), parent: Some(RevisionId::new("a")), bump: Some(Bump::Minor), note: String::new(), edited_at: Some(at(9)) };
    let list = version_list("", "three", Some(&head), &[original, later]);
    let ids: Vec<&str> = list.iter().map(|v| v.id.as_ref().unwrap().as_str()).collect();
    assert_eq!(ids, vec!["b", "h", "a"], "the head was edited before b");
    assert_eq!(current_index(&list), 1);
    assert_eq!((list[0].kind, list[0].note.as_str(), list[0].bump), (VersionKind::Earlier, "more", Some(Bump::Minor)));
    assert_eq!((list[2].kind, list[2].bump, list[1].number), (VersionKind::Original, None, Some(SemVer::new(1, 2, 0))));
}

#[test]
fn any_two_versions_compare_in_either_direction() {
    let at = |hour| crate::time::utc(2026, 11, 20, hour, 0, 0);
    let subject = RevisionSubject::Post(crate::domain::PostId::new("p"));
    let a = Revision { id: RevisionId::new("a"), edited_at: Some(at(8)), ..Revision::of(subject.clone(), "The mind is a mirror.", at(9)) };
    let b = Revision { id: RevisionId::new("b"), parent_id: Some(RevisionId::new("a")), edited_at: Some(at(9)), ..Revision::of(subject, "The mind is a lamp.", at(10)) };
    let list = version_list("", "The mind is a lamp, not a mirror.", None, &[a, b]);
    let texts: Vec<&str> = list.iter().map(|v| v.content.as_str()).collect();
    assert_eq!(texts, vec!["The mind is a lamp, not a mirror.", "The mind is a lamp.", "The mind is a mirror."]);
    let older_to_newer = compare_versions(&list, 2, 1).expect("two versions");
    assert_eq!(older_to_newer, word_diff("The mind is a mirror.", "The mind is a lamp."));
    assert_eq!(compare_versions(&list, 1, 2), Some(word_diff("The mind is a lamp.", "The mind is a mirror.")));
    assert_eq!(compare_versions(&list, 1, 1), None, "a version against itself");
    assert_eq!(compare_versions(&list, 0, 9), None);
}

#[test]
fn the_edit_log_lists_numbered_versions_newest_edit_first() {
    let at = |hour| crate::time::utc(2026, 11, 20, hour, 0, 0);
    let subject = RevisionSubject::Post(crate::domain::PostId::new("p"));
    let original = Revision { id: RevisionId::new("a"), version: Some(SemVer::ORIGINAL), edited_at: Some(at(8)), ..Revision::of(subject.clone(), "one", at(9)) };
    let fix = Revision { id: RevisionId::new("b"), parent_id: Some(RevisionId::new("a")), version: Some(SemVer::new(1, 0, 1)), bump: Bump::Patch, note: "typo".into(), edited_at: Some(at(9)), ..Revision::of(subject, "two", at(10)) };
    let head = VersionHead { id: RevisionId::new("h"), version: SemVer::new(1, 1, 0), parent: Some(RevisionId::new("b")), bump: Some(Bump::Minor), note: "added a line".into(), edited_at: Some(at(10)) };
    let log = edit_log(&version_list("", "three", Some(&head), &[original, fix]));
    let lines: Vec<(String, Option<Bump>, &str, bool)> = log.iter().map(|e| (e.version.to_string(), e.bump, e.note.as_str(), e.current)).collect();
    assert_eq!(lines, vec![("1.1.0".into(), Some(Bump::Minor), "added a line", true), ("1.0.1".into(), Some(Bump::Patch), "typo", false), ("1.0.0".into(), None, "", false)]);
    assert_eq!(log[0].edited_at, Some(at(10)));
    let document = version_list("Title", "body", None, &[Revision::of(RevisionSubject::Document("d".into()), "old", at(9))]);
    assert!(edit_log(&document).is_empty(), "documents keep no numbered log");
}
