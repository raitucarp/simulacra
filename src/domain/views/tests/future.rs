use super::{date, locked, post_at, utc_day};
use crate::domain::post::PostLifecycle;
use crate::domain::symbols::EntryState;
use crate::domain::views::calendar::YearMonth;
use crate::domain::views::future::{future_log, FutureItem};

fn scheduled(id: &str, text: &str, due: &str) -> crate::domain::Post {
    let mut post = post_at(id, text, "2026-11-01T09:00:00Z");
    post.lifecycle = PostLifecycle::scheduled(due.parse().expect("timestamp"));
    post
}

#[test]
fn scheduled_posts_and_dated_todos_group_by_month_from_today() {
    let todos = post_at("t", "Plan\n[ ] book the hall ^2026-12-03\n[x] done already ^2026-12-01\n[ ] past ^2026-11-02\n[ ] someday", "2026-11-01T09:00:00Z");
    let posts = [todos, scheduled("s", "Launch note", "2026-11-30T10:00:00Z"), scheduled("j", "New year", "2027-01-01T08:00:00Z")];
    let months = future_log(&posts, date("2026-11-20"), utc_day);
    assert_eq!(months.iter().map(|month| month.month).collect::<Vec<_>>(), [YearMonth { year: 2026, month: 11 }, YearMonth { year: 2026, month: 12 }, YearMonth { year: 2027, month: 1 }]);
    assert_eq!(months[0].entries[0].text, "Launch note");
    assert_eq!(months[0].entries[0].item, FutureItem::Post { locked: false });
    let december = &months[1].entries;
    assert_eq!(december.len(), 1, "done, past and undated to-dos stay out");
    assert_eq!((december[0].text.as_str(), december[0].day), ("book the hall", date("2026-12-03")));
    assert!(matches!(december[0].item, FutureItem::Todo { index: 0, state: EntryState::Open }));
}

#[test]
fn a_locked_scheduled_post_shows_only_its_placeholder() {
    let sealed = locked(scheduled("x", "the secret [ ] plan ^2026-12-24", "2026-12-24T10:00:00Z"));
    let months = future_log([&sealed], date("2026-11-20"), utc_day);
    let entries: Vec<_> = months.iter().flat_map(|month| &month.entries).collect();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].item, FutureItem::Post { locked: true });
    assert!(entries[0].text.is_empty());
}
