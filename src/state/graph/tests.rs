use super::*;
use crate::domain::graph::NodeKind;
use std::rc::Rc;

#[test]
fn scene_is_reused_until_the_store_changes() {
    let mut state = AppState::new_in_memory();
    let mut cache = GraphCache::default();
    let first = cache.scene(&state);
    assert!(Rc::ptr_eq(&first, &cache.scene(&state)));

    state.submit_post("A new #idea for the graph").unwrap();
    let second = cache.scene(&state);
    assert!(!Rc::ptr_eq(&first, &second));
    assert_eq!(second.model.count_of(NodeKind::Post), first.model.count_of(NodeKind::Post) + 1);
    assert_eq!(second.layout.positions().len(), second.model.node_count());
    assert_eq!(second.degrees.len(), second.model.node_count());
}

#[test]
fn scene_follows_the_active_network() {
    let mut state = AppState::new_in_memory();
    let mut cache = GraphCache::default();
    let first = cache.scene(&state);
    let other = state.networks.iter().map(|n| n.id.clone()).find(|id| id != &state.active_network_id);
    let Some(other) = other else { return };
    state.switch_network(&other).unwrap();
    let second = cache.scene(&state);
    assert!(!Rc::ptr_eq(&first, &second));
    let expected = state.current_network_posts().count();
    assert_eq!(second.model.count_of(NodeKind::Post), expected);
}

#[test]
fn background_layouts_install_in_order_and_keep_the_previous_scene_meanwhile() {
    let mut state = AppState::new_in_memory();
    let mut cache = GraphCache::default();
    let (first_gen, first_job) = cache.refresh(&state).expect("initial job");
    assert!(cache.refresh(&state).is_none(), "unchanged store needs no new layout");
    assert!(cache.is_pending());
    assert_eq!(cache.current().model.node_count(), 0, "nothing installed yet");

    state.submit_post("Another #node").unwrap();
    let (second_gen, second_job) = cache.refresh(&state).expect("store changed");
    assert!(cache.install(second_gen, second_job.run()));
    let newest = cache.current();
    assert!(!cache.install(first_gen, first_job.run()), "a stale layout never replaces a newer one");
    assert!(Rc::ptr_eq(&newest, &cache.current()));
    assert!(!cache.is_pending());
}
