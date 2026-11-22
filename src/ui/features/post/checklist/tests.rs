use super::chips::{due_colour, due_label, due_tone, estimate_label, DueTone};
use super::render_post_content;
use crate::domain::post::Post;
use crate::theme::ThemeTokens;
use crate::ui::features::post::BodyLinks;
use jiff::civil::date;
use rust_i18n::t;

#[test]
fn overdue_needs_an_open_item_past_its_day() {
    let today = date(2026, 11, 22);
    assert_eq!(due_tone(date(2026, 11, 21), today, false), DueTone::Overdue);
    assert_eq!(due_tone(date(2026, 11, 21), today, true), DueTone::Later, "done items are not overdue");
    assert_eq!(due_tone(today, today, false), DueTone::Today);
    assert_eq!(due_tone(date(2026, 11, 23), today, false), DueTone::Later);
    let theme = ThemeTokens::default();
    assert_eq!(due_colour(DueTone::Overdue, &theme), theme.accent_danger);
    assert_ne!(due_colour(DueTone::Later, &theme), theme.accent_danger);
}

#[test]
fn due_days_near_today_read_as_words() {
    let today = date(2026, 11, 22);
    assert_eq!(due_label(today, today), t!("todo.due.today"));
    assert_eq!(due_label(date(2026, 11, 23), today), t!("todo.due.tomorrow"));
    assert_eq!(due_label(date(2026, 11, 21), today), t!("todo.due.yesterday"));
    assert!(due_label(date(2026, 12, 1), today).contains("2026-12-01"));
}

#[test]
fn estimates_read_in_hours_and_minutes() {
    assert_eq!(estimate_label(25), t!("todo.estimate.minutes", minutes = 25));
    assert_eq!(estimate_label(120), t!("todo.estimate.hours", hours = 2));
    assert_eq!(estimate_label(90), t!("todo.estimate.hours_minutes", hours = 1, minutes = 30));
}

#[test]
fn a_checklist_and_a_plain_body_both_render() {
    let theme = ThemeTokens::default();
    let posts: Vec<Post> = ["Plan\n[ ] pack ^fri !! &home ~1h @ada ^^weekly\n  [x] **socks**\nDone.", "Just words"]
        .iter()
        .enumerate()
        .map(|(n, text)| Post::create(format!("p{n}"), "net", "me", text).expect("post"))
        .collect();
    for post in &posts {
        let _body = render_post_content(post, &posts, &theme, &BodyLinks::default(), None);
    }
}
