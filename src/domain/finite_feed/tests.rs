use super::*;

fn ids(names: &[&str]) -> Vec<PostId> {
    names.iter().map(|name| PostId::new(*name)).collect()
}

fn order(posts: &[PostId], own: &[&str]) -> Vec<(PostId, usize, bool)> {
    posts.iter().enumerate().map(|(at, id)| (id.clone(), at, !own.contains(&id.as_str()))).collect()
}

fn window(posts: &[(PostId, usize, bool)], seen: &[PostId], budget: usize) -> BudgetWindow {
    let seen: std::collections::HashSet<PostId> = seen.iter().cloned().collect();
    budget_window(posts.iter().map(|(id, at, counts)| (id, *at, *counts)), &seen, budget)
}

#[test]
fn without_a_budget_the_feed_never_ends() {
    let posts = order(&ids(&["a", "b", "c"]), &[]);
    assert_eq!(window(&posts, &[], 0), BudgetWindow { positions: vec![0, 1, 2], ended: false });
}

#[test]
fn the_budget_counts_new_posts_and_ends_the_feed_once_spent() {
    let posts = order(&ids(&["a", "b", "c", "d"]), &[]);
    assert_eq!(window(&posts, &[], 2), BudgetWindow { positions: vec![0, 1], ended: true });
    assert_eq!(window(&posts, &[], 9), BudgetWindow { positions: vec![0, 1, 2, 3], ended: false });
}

#[test]
fn posts_seen_today_stay_and_own_posts_are_free() {
    let posts = order(&ids(&["new", "mine", "a", "b"]), &["mine"]);
    let seen = ids(&["a", "b"]);
    assert_eq!(window(&posts, &seen, 2), BudgetWindow { positions: vec![1, 2, 3], ended: true }, "a new post waits for tomorrow");
}

#[test]
fn a_day_starts_afresh_and_counts_each_post_once() {
    let (monday, tuesday) = (jiff::civil::date(2026, 11, 23), jiff::civil::date(2026, 11, 24));
    let mut day = FeedDay::default().on(monday);
    assert!(day.count(ids(&["a", "b", "a"])));
    assert!(!day.count(ids(&["b"])));
    day.answer = Some(PostId::new("answer"));
    assert_eq!(day.clone().on(monday).seen, ids(&["a", "b"]));
    let fresh = day.on(tuesday);
    assert_eq!((fresh.day, fresh.seen.len(), fresh.answer), (Some(tuesday), 0, None));
}

#[test]
fn the_answer_is_one_short_line() {
    assert_eq!(one_line_answer("  That habits\ncompound.  ").as_deref(), Some("That habits compound."));
    assert_eq!(one_line_answer(" \n "), None);
    assert_eq!(one_line_answer(&"x".repeat(500)).map(|line| line.chars().count()), Some(MAX_ANSWER_CHARS));
}
