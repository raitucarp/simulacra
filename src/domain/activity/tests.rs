use super::levels::quantile_levels;
use super::*;
use crate::domain::{NetworkId, Post, Profile};
use chrono::{DateTime, Datelike, FixedOffset, NaiveDate, TimeZone, Utc, Weekday};

const NET: &str = "net_a";
const OTHER_NET: &str = "net_b";

fn date(y: i32, m: u32, d: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, d).unwrap()
}

/// Monday, 16 November 2026.
fn today() -> NaiveDate {
    date(2026, 11, 16)
}

fn post_at(id: &str, net: &str, at: DateTime<Utc>) -> Post {
    let author = Profile::new_primary("u", "User", "@user");
    let mut post = Post::create(id, net, author, "thought").unwrap();
    post.created_at = at;
    post
}

fn noon(d: NaiveDate) -> DateTime<Utc> {
    Utc.from_utc_datetime(&d.and_hms_opt(12, 0, 0).unwrap())
}

fn build(posts: &[Post]) -> ActivityHeatmap {
    ActivityHeatmap::build(posts, &NetworkId::new(NET), today(), &Utc)
}

#[test]
fn grid_is_weekday_aligned_and_ends_today() {
    let heatmap = build(&[]);
    assert_eq!(heatmap.start().weekday(), Weekday::Sun);
    assert_eq!(heatmap.date_at(HEATMAP_WEEKS - 1, 1), today());
    assert_eq!(weekday_row(today()), 1);
    assert!(heatmap.day(HEATMAP_WEEKS - 1, 2).is_none(), "future days are hidden");
    assert_eq!(heatmap.day(HEATMAP_WEEKS - 1, 0).unwrap().date, date(2026, 11, 15));
    assert_eq!(heatmap.day(HEATMAP_WEEKS - 2, 6).unwrap().date, date(2026, 11, 14));
    for week in 0..HEATMAP_WEEKS {
        for row in 0..DAYS_PER_WEEK {
            if let Some(day) = heatmap.day(week, row) {
                assert_eq!(weekday_row(day.date), row);
            }
        }
    }
}

#[test]
fn posts_land_on_their_day_and_window_is_bounded() {
    let posts = vec![
        post_at("a", NET, noon(today())),
        post_at("b", NET, noon(today())),
        post_at("c", NET, noon(date(2026, 11, 14))),
        post_at("too_old", NET, noon(date(2025, 1, 1))),
        post_at("future", NET, noon(date(2026, 11, 20))),
    ];
    let heatmap = build(&posts);
    assert_eq!(heatmap.count_on(today()), 2);
    assert_eq!(heatmap.day(HEATMAP_WEEKS - 2, 6).unwrap().count, 1);
    assert_eq!(heatmap.total(), 3);
}

#[test]
fn only_the_requested_network_is_counted() {
    let posts = vec![
        post_at("mine", NET, noon(today())),
        post_at("theirs", OTHER_NET, noon(today())),
        post_at("theirs_2", OTHER_NET, noon(today())),
    ];
    let heatmap = build(&posts);
    assert_eq!(heatmap.total(), 1);
    assert_eq!(heatmap.count_on(today()), 1);
}

#[test]
fn buckets_use_the_local_calendar_date() {
    let early_utc = Utc.from_utc_datetime(&today().and_hms_opt(2, 0, 0).unwrap());
    let posts = vec![post_at("late_evening", NET, early_utc)];
    let west = FixedOffset::west_opt(5 * 3600).unwrap();
    let heatmap = ActivityHeatmap::build(&posts, &NetworkId::new(NET), today(), &west);
    assert_eq!(heatmap.count_on(date(2026, 11, 15)), 1);
    assert_eq!(heatmap.count_on(today()), 0);
    assert_eq!(local_date(&posts[0], &west), date(2026, 11, 15));
}

#[test]
fn quantile_levels_rank_active_days() {
    assert_eq!(quantile_levels(&[0, 0]), vec![0, 0]);
    assert_eq!(quantile_levels(&[0, 3, 3]), vec![0, 4, 4]);
    assert_eq!(quantile_levels(&[1, 1, 1, 2, 3, 10, 0]), vec![1, 1, 1, 3, 3, 4, 0]);
    let levels = quantile_levels(&(0..=20).collect::<Vec<u32>>());
    assert!(levels.windows(2).all(|w| w[0] <= w[1]), "levels are monotonic in count");
    assert_eq!(levels[0], 0);
    assert_eq!(levels[20], MAX_ACTIVITY_LEVEL);
    assert!(levels.iter().all(|&l| l <= MAX_ACTIVITY_LEVEL));
}

#[test]
fn heatmap_levels_follow_counts() {
    let mut posts = Vec::new();
    for i in 0..4 {
        posts.push(post_at(&format!("busy_{i}"), NET, noon(today())));
    }
    posts.push(post_at("quiet", NET, noon(date(2026, 11, 10))));
    let heatmap = build(&posts);
    let busy = heatmap.day(HEATMAP_WEEKS - 1, 1).unwrap();
    let quiet = heatmap.day(HEATMAP_WEEKS - 2, 2).unwrap();
    assert_eq!((busy.count, busy.level), (4, MAX_ACTIVITY_LEVEL));
    assert_eq!(quiet.date, date(2026, 11, 10));
    assert_eq!((quiet.count, quiet.level), (1, 1));
}

#[test]
fn month_labels_mark_month_starts_without_crowding() {
    let heatmap = build(&[]);
    let labels = heatmap.month_labels();
    assert!(labels.len() >= 12);
    for pair in labels.windows(2) {
        assert!(pair[1].week >= pair[0].week + 3, "labels keep a gap: {pair:?}");
        assert_ne!(pair[0].month, pair[1].month);
    }
    for label in &labels {
        assert_eq!(heatmap.date_at(label.week, 0).month(), label.month);
    }
    assert_eq!(labels.last().unwrap().month, 11);
}
