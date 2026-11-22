//! D9: the Tasks view across posts.

use super::super::tasks::{collect_tasks, task_contexts, DueFilter, TaskFilter};
use crate::domain::Post;
use jiff::civil::{date, Date};

/// Sunday 22 November 2026.
fn today() -> Date {
    date(2026, 11, 22)
}

fn posts() -> Vec<Post> {
    let texts = [
        "[ ] pay the bill ^2026-11-20 !\n[x] post the letter ^2026-11-21",
        "[ ] call mum ^today !!! &phone\n[ ] tidy up &Home\n[ ] read ^fri &home !!",
        "plain words",
    ];
    texts.iter().enumerate().map(|(n, text)| Post::create(format!("p{n}"), "net", "me", text).expect("post")).collect()
}

fn texts(filter: &TaskFilter) -> Vec<String> {
    let posts = posts();
    collect_tasks(posts.iter().map(|post| (post, today())), filter, today()).into_iter().map(|row| row.entry.item.text).collect()
}

#[test]
fn every_open_item_reads_soonest_first_with_undated_last() {
    assert_eq!(texts(&TaskFilter::default()), vec!["pay the bill", "call mum", "read", "tidy up"]);
    let all = TaskFilter { show_closed: true, ..TaskFilter::default() };
    assert_eq!(texts(&all), vec!["pay the bill", "post the letter", "call mum", "read", "tidy up"]);
}

#[test]
fn due_filters_pick_their_days() {
    let by = |due| texts(&TaskFilter { due, ..TaskFilter::default() });
    assert_eq!(by(DueFilter::Overdue), vec!["pay the bill"]);
    assert_eq!(by(DueFilter::Today), vec!["call mum"]);
    assert_eq!(by(DueFilter::Week), vec!["pay the bill", "call mum", "read"]);
    assert_eq!(by(DueFilter::NoDate), vec!["tidy up"]);
    assert!(DueFilter::Week.admits(Some(date(2026, 11, 28)), today()));
    assert!(!DueFilter::Week.admits(Some(date(2026, 11, 29)), today()));
}

#[test]
fn priority_and_context_narrow_the_list() {
    assert_eq!(texts(&TaskFilter { priority: Some(2), ..TaskFilter::default() }), vec!["call mum", "read"]);
    assert_eq!(texts(&TaskFilter { context: Some("home".into()), ..TaskFilter::default() }), vec!["read", "tidy up"]);
    assert_eq!(texts(&TaskFilter { context: Some("phone".into()), priority: Some(1), due: DueFilter::Today, show_closed: false }), vec!["call mum"]);
}

#[test]
fn contexts_are_listed_once_whatever_their_case() {
    assert_eq!(task_contexts(&posts()), vec!["phone".to_string(), "Home".to_string()]);
}
