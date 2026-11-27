use super::pick::pick_sparks;
use crate::domain::{NetworkId, Post};

fn post(id: &str, network: &str, day: i8) -> Post {
    let mut post = Post::create(id, network, "user_main", "a thought").unwrap();
    post.created_at = crate::time::utc(2026, 11, day, 12, 0, 0);
    post
}

fn ids(picked: &[&Post]) -> Vec<String> {
    picked.iter().map(|post| post.id.to_string()).collect()
}

fn network(n: usize) -> Vec<Post> {
    (1..=n).map(|day| post(&format!("p{day}"), "net_a", day as i8)).collect()
}

#[test]
fn only_the_active_network_surfaces() {
    let posts = vec![post("a1", "net_a", 1), post("b1", "net_b", 2), post("a2", "net_a", 3)];
    let picked = pick_sparks(&posts, &NetworkId::new("net_a"), 7, 4);
    let mut got = ids(&picked);
    got.sort();
    assert_eq!(got, vec!["a1", "a2"]);
}

#[test]
fn the_newest_thoughts_are_left_to_the_feed() {
    let posts = network(20);
    let picked = pick_sparks(&posts, &NetworkId::new("net_a"), 7, 4);
    assert_eq!(picked.len(), 4);
    assert!(picked.iter().all(|post| post.created_at < crate::time::utc(2026, 11, 15, 0, 0, 0)), "{:?}", ids(&picked));
}

#[test]
fn the_same_day_gives_the_same_picks_and_another_day_others() {
    let posts = network(20);
    let net = NetworkId::new("net_a");
    assert_eq!(ids(&pick_sparks(&posts, &net, 7, 4)), ids(&pick_sparks(&posts, &net, 7, 4)));
    assert_ne!(ids(&pick_sparks(&posts, &net, 7, 4)), ids(&pick_sparks(&posts, &net, 8, 4)));
}

#[test]
fn a_small_network_shows_what_it_has() {
    let posts = network(3);
    assert_eq!(pick_sparks(&posts, &NetworkId::new("net_a"), 7, 4).len(), 3);
}
