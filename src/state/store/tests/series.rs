//! N10: numbered parts of a series, their previous and next part and progress, the series of
//! a network, and a locked part that keeps its place.

use super::locked::locked_post;
use crate::state::store::AppState;

#[test]
fn parts_are_numbered_in_turn_and_know_their_neighbours() {
    let mut state = AppState::new_in_memory();
    let ids: Vec<_> = ["Day 1: hello, world.", "Day 2: ownership.", "Day 3: borrowing."].iter().map(|text| state.submit_post(text).expect("posted")).collect();
    let first = state.add_to_series(&ids[0], "  Learning   Rust ", Some(30)).expect("first");
    assert_eq!((first.title.as_str(), first.number, first.goal), ("Learning Rust", 1, Some(30)));
    let second = state.add_to_series(&ids[1], "learning rust", None).expect("second");
    assert_eq!((second.number, second.goal, second.id.clone()), (2, Some(30), first.id.clone()), "same series, goal kept");
    state.add_to_series(&ids[2], "Learning Rust", None).expect("third");
    let middle = state.series_nav(state.get_post(&ids[1]).expect("post")).expect("nav");
    assert_eq!((middle.position, middle.posted), (2, 3));
    assert_eq!((middle.previous.as_ref(), middle.next.as_ref()), (Some(&ids[0]), Some(&ids[2])));
    let last = state.series_nav(state.get_post(&ids[2]).expect("post")).expect("nav");
    assert_eq!(last.next, None);
    let listings = state.series_listings();
    assert_eq!(listings.len(), 1);
    assert_eq!((listings[0].title.as_str(), listings[0].goal, listings[0].parts.clone()), ("Learning Rust", Some(30), ids.clone()));
    assert!(state.add_to_series(&ids[0], "   ", None).is_err());
}

#[test]
fn leaving_keeps_the_other_numbers_and_the_next_part_follows_the_highest() {
    let mut state = AppState::new_in_memory();
    let ids: Vec<_> = ["One.", "Two.", "Three."].iter().map(|text| state.submit_post(text).expect("posted")).collect();
    for id in &ids[..2] {
        state.add_to_series(id, "Sketches", None).expect("added");
    }
    state.leave_series(&ids[0]).expect("left");
    assert_eq!(state.get_post(&ids[0]).and_then(|p| p.extras.series.clone()), None);
    assert_eq!(state.add_to_series(&ids[2], "Sketches", None).expect("added").number, 3);
    assert_eq!(state.series_listings()[0].parts, vec![ids[1].clone(), ids[2].clone()]);
}

#[test]
fn a_locked_part_keeps_its_place_but_only_its_id_is_listed() {
    let mut state = AppState::new_in_memory();
    let open = state.submit_post("Day 1.").expect("open");
    state.add_to_series(&open, "Journal", None).expect("added");
    let locked = locked_post(&mut state);
    state.add_to_series(&locked, "Journal", None).expect("a locked post can be numbered");
    let nav = state.series_nav(state.get_post(&open).expect("open")).expect("nav");
    assert_eq!(nav.next.as_ref(), Some(&locked), "its neighbour is named by id; the card shows its placeholder");
    state.user_profiles.push(crate::domain::Profile::new_agent("agent_x", "Xi", "@xi", "thinker"));
    let agent_post = crate::domain::Post::create("post_agent", state.active_network_id.clone(), "agent_x", "Not mine.").expect("post");
    state.insert_reply_after_parent(agent_post).expect("stored");
    assert!(state.add_to_series(&"post_agent".into(), "Journal", None).is_err(), "only the user's own posts");
}
