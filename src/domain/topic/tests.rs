use super::rank_tags;
use crate::domain::Post;
use jiff::Timestamp;

fn at(day: i8) -> Timestamp {
    crate::time::utc(2026, 11, day, 12, 0, 0)
}

fn post(id: &str, text: &str, day: i8) -> Post {
    let mut post = Post::create(id, "net_main", "user_main", text).unwrap();
    post.created_at = at(day);
    post
}

fn tags(posts: &[Post], since: Timestamp, limit: usize) -> Vec<(String, usize)> {
    let refs: Vec<&Post> = posts.iter().collect();
    rank_tags(&refs, since, limit).into_iter().map(|topic| (topic.tag, topic.post_count)).collect()
}

#[test]
fn counts_each_post_once_per_tag_and_ignores_case() {
    let posts = [post("p1", "#Mind and #mind again", 10), post("p2", "the #MIND, #Ethics", 11), post("p3", "#ethics", 12)];
    // Both used twice; #Ethics was used last. A tag keeps the spelling it was first seen with.
    assert_eq!(tags(&posts, at(1), 5), vec![("#Ethics".to_string(), 2), ("#Mind".to_string(), 2)]);
}

#[test]
fn ties_go_to_the_most_recently_used_tag() {
    let posts = [post("p1", "#old", 5), post("p2", "#new", 9)];
    assert_eq!(tags(&posts, at(1), 5), vec![("#new".to_string(), 1), ("#old".to_string(), 1)]);
}

#[test]
fn only_recent_posts_count_while_the_window_has_tags() {
    let posts = [post("p1", "#ancient #ancient2", 2), post("p2", "#fresh", 20)];
    assert_eq!(tags(&posts, at(15), 5), vec![("#fresh".to_string(), 1)]);
}

#[test]
fn an_empty_window_falls_back_to_every_post() {
    let posts = [post("p1", "#ancient", 2), post("p2", "no tags here", 20)];
    assert_eq!(tags(&posts, at(15), 5), vec![("#ancient".to_string(), 1)]);
}

#[test]
fn keeps_at_most_the_limit() {
    let posts = [post("p1", "#a #b #c", 3)];
    assert_eq!(tags(&posts, at(1), 2).len(), 2);
}

#[test]
fn no_posts_means_no_topics() {
    assert!(tags(&[], at(1), 5).is_empty());
}
