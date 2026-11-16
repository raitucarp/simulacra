use crate::domain::{NetworkId, PostId};
use crate::state::store::AppState;
use chrono::{DateTime, NaiveDate, TimeZone, Utc};

fn date(y: i32, m: u32, d: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, d).unwrap()
}

fn at(d: NaiveDate, hour: u32) -> DateTime<Utc> {
    Utc.from_utc_datetime(&d.and_hms_opt(hour, 0, 0).unwrap())
}

fn restamp(state: &mut AppState, id: &PostId, when: DateTime<Utc>) {
    state.posts.iter_mut().find(|p| &p.id == id).unwrap().created_at = when;
}

#[test]
fn day_filter_keeps_only_that_days_posts() {
    let mut state = AppState::new_in_memory();
    let day = date(2001, 2, 3);
    let on_day = state.submit_post("Morning on the day").unwrap();
    let late = state.submit_post("Late on the day").unwrap();
    let next = state.submit_post("Next day thought").unwrap();
    restamp(&mut state, &on_day, at(day, 8));
    restamp(&mut state, &late, at(day, 23));
    restamp(&mut state, &next, at(date(2001, 2, 4), 1));

    let ids: Vec<PostId> = state.day_filtered_posts_in(day, "", Utc).map(|p| p.id.clone()).collect();
    assert_eq!(ids, vec![late.clone(), on_day.clone()], "newest first, like the stream");
    let matching: Vec<PostId> = state.day_filtered_posts_in(day, "morning", Utc).map(|p| p.id.clone()).collect();
    assert_eq!(matching, vec![on_day]);
}

#[test]
fn day_filter_is_scoped_to_the_active_network() {
    let mut state = AppState::new_in_memory();
    let day = date(2001, 2, 3);
    let moved = state.submit_post("Belongs elsewhere").unwrap();
    restamp(&mut state, &moved, at(day, 12));
    state.posts.iter_mut().find(|p| p.id == moved).unwrap().network_id = NetworkId::new("net_elsewhere");
    assert_eq!(state.day_filtered_posts_in(day, "", Utc).count(), 0);
}

#[test]
fn toggling_the_same_day_clears_the_filter() {
    let mut state = AppState::new_in_memory();
    let day = date(2001, 2, 3);
    assert_eq!(state.toggle_feed_day_filter(day), Some(day));
    assert_eq!(state.toggle_feed_day_filter(date(2001, 2, 4)), Some(date(2001, 2, 4)));
    assert_eq!(state.toggle_feed_day_filter(date(2001, 2, 4)), None);
    state.toggle_feed_day_filter(day);
    state.clear_feed_day_filter();
    assert_eq!(state.feed_day_filter, None);
}

#[test]
fn heatmap_counts_the_active_network_today() {
    let mut state = AppState::new_in_memory();
    let before = state.current_activity_heatmap().total();
    state.submit_post("Fresh thought for the cadence").unwrap();
    let heatmap = state.current_activity_heatmap();
    assert_eq!(heatmap.total(), before + 1);
    assert!(heatmap.count_on(heatmap.today()) >= 1);
}

#[test]
fn grouped_feed_entries_respect_the_day_filter() {
    use crate::state::{feed_entries, FeedTab};
    use chrono::Local;
    let mut state = AppState::new_in_memory();
    let day = date(2001, 2, 3);
    let id = state.submit_post("A thought from long ago").unwrap();
    let local_noon = Local.from_local_datetime(&day.and_hms_opt(12, 0, 0).unwrap()).earliest().unwrap();
    restamp(&mut state, &id, local_noon.with_timezone(&Utc));
    let unfiltered = feed_entries(&state, FeedTab::ForYou).len();

    state.toggle_feed_day_filter(day);
    let ids: Vec<PostId> = feed_entries(&state, FeedTab::ForYou).iter().map(|p| p.id.clone()).collect();
    assert_eq!(ids, vec![id]);
    assert_eq!(state.day_scoped(state.filtered_posts("long ago")).count(), 1);
    assert_eq!(state.day_scoped(state.filtered_posts("nothing like this")).count(), 0);

    state.clear_feed_day_filter();
    assert_eq!(feed_entries(&state, FeedTab::ForYou).len(), unfiltered);
    assert!(unfiltered > 1);
}
